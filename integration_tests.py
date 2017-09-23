#!/usr/bin/env python3
"""
Usage:
    ./integration_tests.py [options] [<patterns>...]

Options:
    -h --help               Print this help information.
    -d --debug              Enable debug mode.
    -s --sequential         Runs the tests sequentially instead of in parallel.


Because a large part of this crate's functionality depends on generated code,
it's easier to test functionality from the point-of-view of an end user.
Therefore, a large proportion of the crate's tests are orchestrated by a
Python script.

For each `*.rs` file in the `integration_tests/` directory, this Python script
will:

- Create a new `--bin` crate in a temporary directory
- Copy the `*.rs` file into this new crate and rename it to `main.rs`.
- Scan the `*.rs` file for a **special** pattern indicating which asset
  directory will be included (relative to this crate's root directory). If the
  pattern isn't found, use this crate's directory (ignoring ".git" and "target").
- Generate a `build.rs` file which will compile in the specified file tree.
- Compile and run the new binary test crate.
"""

import os
import sys
from pathlib import Path
import subprocess
import tempfile
import logging
import shutil
import re
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime

from dateutil.relativedelta import relativedelta
from docopt import docopt
import jinja2

DEBUG = False

project_root = Path(os.path.abspath(__file__)).parent

BUILD_RS_TEMPLATE = """
extern crate include_dir;

use std::env;
use std::path::Path;
use include_dir::include_dir;

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&outdir).join("assets.rs");

    include_dir("{{ root }}")
        .as_variable("ASSETS")
        {% for ignore in ignores %}
        .ignore("{{ ignore }}")
        {% endfor %}
        .to_file(dest_path)
        .unwrap();
    }
"""

CARGO_TOML_TEMPLATE = """
[package]
authors = ["Michael-F-Bryan <michaelfbryan@gmail.com>"]
name = "{{ name }}"
version = "0.1.0"

[build-dependencies.include_dir]
path = "{{ include_dir_path }}" 
{% if features %}
features = {{ features }}
{% endif %}

[dependencies]
{% for dep in dependencies %}
{{ dep }} = "*"
{% endfor %}
"""

def pretty_print_output(name, stdout, stderr):
    logging.warning("return code: %d", output.returncode)
    if stdout:
        logging.warn("stdout:")
        for line in stdout.decode().split("\n"):
            logging.warning("(%s) %s", name, line)

    if stderr:
        logging.warning("stderr:")
        for line in stderr.decode().split("\n"):
            logging.warning("(%s) %s", name, line)


class IntegrationTest:
    """
    A runner for an integration test.
    """
    def __init__(self, filename):
        self.script = filename.relative_to(project_root)
        self.name = self.script.stem
        self.temp_dir = tempfile.TemporaryDirectory(prefix="include_dir_test-")
        self.crate = None

    def initialize(self):
        """
        Do all the work necessary to create a new project, copy across the test
        file, add a build script, then adjust Cargo.toml accordingly.
        """
        logging.debug("(%s) Initializing test crate in %s", self.name, self.temp_dir.name)
        crate_name = self.name

        cmd = ["cargo", "new", "--bin", crate_name]
        if DEBUG:
            cmd.append("--verbose")

        proc = subprocess.Popen(cmd,
                                cwd=self.temp_dir.name,
                                stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE)
        proc.wait()
        stdout, stderr = proc.communicate()

        if proc.returncode != 0:
            logging.error("Unable to create a new crate")
            pretty_print_output(self.name, stdout, stderr)
            return

        self.crate = Path(self.temp_dir.name) / crate_name

        main_rs = self.crate / "src" / "main.rs"
        shutil.copy(str(self.script), str(main_rs))

        analysis = self._analyse_script()
        self._generate_build_rs(analysis)
        self._update_cargo_toml(analysis)
        self._copy_across_cache()

    def run(self):
        """
        Run the test, checking to see that it passes (non-zero exit code).
        """
        start = datetime.now()
        logging.info('Running test "%s"', self.name)

        cmd = ["cargo", "run"]

        if DEBUG:
            cmd.append("--verbose")

        proc = subprocess.Popen(cmd,
                                cwd=self.crate,
                                stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE)
        proc.wait()
        stdout, stderr = proc.communicate()

        duration = human_readable(relativedelta(datetime.now(), start))

        if proc.returncode != 0:
            logging.error("%-20s\t✘\t(%s)", self.name, duration)
            pretty_print_output(self.name, stdout, stderr)
            return False
        else:
            logging.info("%-20s\t✔\t(%s)", self.name, duration)
            return True

    def _generate_build_rs(self, analysis):
        logging.debug("(%s) Generating build.rs (assets: %s)", self.name, analysis["root"])

        build_rs = self.crate / "build.rs"

        with open(str(build_rs), "w") as f:
            build_template = jinja2.Template(BUILD_RS_TEMPLATE)
            f.write(build_template.render(analysis))

    def _update_cargo_toml(self, analysis):
        logging.debug("(%s) Updating Cargo.toml", self.name)
        cargo_toml = self.crate / "Cargo.toml"

        with open(str(cargo_toml), "w") as f:
            template = jinja2.Template(CARGO_TOML_TEMPLATE)
            context = {
                "name": self.name,
                "include_dir_path": project_root,
                "dependencies": analysis["dependencies"],
            }
            feat = analysis.get("features", [])
            context["features"] = feat if isinstance(feat, list) else [feat]
            f.write(template.render(context))

    def _analyse_script(self):
        keywords = {
            "features": "FEATURE",
            "root": "ROOT",
            "ignore": "IGNORE"
        }
        context = {}
        context["root"] = project_root / "src"
        context["dependencies"] = []

        with open(str(self.script)) as f:
            for line in f:
                for name, keyword in keywords.items():
                    pattern = re.compile(r"// {}:(\s+[^\s]+)+".format(keyword))
                    match = pattern.search(line)
                    if match:
                        values = [v.strip() for v in match.groups()]
                        context[name] = values if len(values) > 1 else values[0]

                match = re.search(r"extern crate ([\w_]+)", line)
                if match:
                    context["dependencies"].append(match.group(1))

        logging.debug("(%s) Analysis %s", self.name, context)

        # Make sure the "root" variable is a Path
        if not isinstance(context["root"], Path):
            context["root"] = Path(context["root"])

        if not context["root"].exists():
            logging.warning("(%s) embedded directory doesn't exist, \"%s\"",
                            self.name,
                            context["root"])
        return context

    def _copy_across_cache(self):
        logging.debug('(%s) Copying across "target/" dir', self.name)
        try:
            # os.symlink(project_root / "target", self.crate / "target")
            shutil.copytree(project_root / "target", self.crate / "target")
        except:
            pass

    def __repr__(self):
        return '<{}: filename="{}">'.format(
            self.__class__.__name__,
            self.name)


def discover_integration_tests(patterns):
    """
    Find all the tests in the integration_tests directory which satisfy the
    provided pattern. If no pattern is provided, use a default of `*.rs`.
    """
    if not patterns:
        patterns = ['*.rs']

    test_dir = project_root / "integration_tests"
    filenames = set()

    for pattern in patterns:
        matches = test_dir.glob(pattern)
        for match in matches:
            filenames.add(match)

    return [IntegrationTest(name) for name in filenames]


def run_test(test):
    test.initialize()
    return test.run()

def human_readable(delta):
    attrs = ['years', 'months', 'days', 'hours', 'minutes', 'seconds']
    ret = []
    for attr in attrs:
        value = getattr(delta, attr)
        if value:
            ret.append("{} {}".format(value, attr if value > 1 else attr[:-1]))
        
    return " and ".join(ret)


def main(args):
    start = datetime.now()

    patterns = args["<patterns>"]
    tests = list(discover_integration_tests(patterns))

    if not tests:
        logging.warning("No tests match the provided pattern")

    if args['--sequential']:
        for test in tests:
            run_test(test)
    else:
        with ThreadPoolExecutor() as pool:
            results = pool.map(run_test, tests)
        
    duration = relativedelta(datetime.now(), start)
    logging.info("Tests completed in %s", human_readable(duration))

    finished_successfully = all(results)
    if not finished_successfully:
        sys.exit(1)


if __name__ == "__main__":
    options = docopt(__doc__)
    if options["--debug"]:
        DEBUG = True

    logging.basicConfig(format='%(asctime)s %(levelname)6s: %(message)s',
                        datefmt='%m/%d/%Y %I:%M:%S %p',
                        level=logging.DEBUG if DEBUG else logging.INFO)
    main(options)

