#!/usr/bin/env python3
"""
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
  pattern isn't found, use this crate's `src/` directory.
- Generate a `build.rs` file which will compile in the specified file tree.
- Compile and run the new binary test crate.
"""

import os
from pathlib import Path
import subprocess
import tempfile
import logging
import shutil
import re
import time

project_root = Path(os.path.abspath(__file__)).parent

logging.basicConfig(format='%(asctime)s %(levelname)6s: %(message)s', 
                    datefmt='%m/%d/%Y %I:%M:%S %p',
                    level=logging.DEBUG)

BUILD_RS_TEMPLATE = """
extern crate include_dir;

use std::env;
use std::path::Path;
use include_dir::include_dir;

fn main() {{
    let outdir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&outdir).join("assets.rs");

    include_dir("{}")
        .as_variable("ASSETS")
        .to_file(dest_path)
        .unwrap();
    }}
"""

CARGO_TOML_TEMPLATE = """
[package]
authors = ["Michael-F-Bryan <michaelfbryan@gmail.com>"]
name = "{}"
version = "0.1.0"

[build-dependencies]
include_dir = {{path = "{}"}}
"""

def pretty_print_output(output):
    logging.debug("return code: %d", output.returncode)
    if output.stdout:
        logging.debug("stdout:")
        for line in output.stdout.decode().split("\n"):
            logging.debug("%s", line)

    if output.stderr:
        logging.debug("stderr:")
        for line in output.stderr.decode().split("\n"):
            logging.debug("%s", line)


class IntegrationTest:
    def __init__(self, filename):
        self.script = Path(os.path.abspath(filename))
        self.name = self.script.stem
        self.temp_dir = tempfile.TemporaryDirectory(prefix="include_dir_test-")
        self.crate = None

    def initialize(self):
        logging.info("Initializing test crate in %s", self.temp_dir.name)
        crate_name = self.name

        output = subprocess.run(["cargo", "new", "--bin", crate_name],
                       cwd=self.temp_dir.name,
                       stdout=subprocess.PIPE,
                       stderr=subprocess.PIPE)

        if output.returncode != 0:
            logging.error("Unable to create a new crate")
            pretty_print_output(output)
            return

        self.crate = Path(self.temp_dir.name) / crate_name

        shutil.copy(self.script, self.crate / "src" / "main.rs")

        self.generate_build_rs()
        self.update_cargo_toml()
        self.copy_across_cache()

    def run(self):
        logging.info('Running test "%s" in "%s"', 
                     self.name, 
                     self.temp_dir.name)

        output = subprocess.run(["cargo", "run", "--verbose"],
                       cwd=self.crate,
                       stdout=subprocess.PIPE,
                       stderr=subprocess.PIPE)

        if output.returncode != 0:
            logging.error("Test failed ✘")
            pretty_print_output(output)
            return
        else:
            logging.info("Pass ✔")


    def generate_build_rs(self):
        asset_dir = self.assets_to_embed()
        logging.info("Generating build.rs (assets: %s)", asset_dir)

        build_rs = self.crate / "build.rs"

        with open(build_rs, "w") as f:
            f.write(BUILD_RS_TEMPLATE.format(asset_dir))

    def update_cargo_toml(self):
        logging.info("Updating Cargo.toml")
        cargo_toml = self.crate / "Cargo.toml"

        with open(cargo_toml, "w") as f:
            f.write(CARGO_TOML_TEMPLATE.format(self.name, project_root))

    def assets_to_embed(self):
        # Search for the "special" pattern -> include_dir!("path/to/assets")
        pattern = re.compile(r'include_dir!\("([\w\d./]+)"\)')

        with open(self.script) as f:
            got = pattern.search(f.read())
            if got is None:
                return project_root / "src"
            else:
                return Path(abspath(got.groups(1)))

    def copy_across_cache(self):
        logging.info('Copying across "target/" dir')
        shutil.copytree(project_root / "target", self.crate / "target")

    def __repr__(self):
        return '<{}: filename="{}">'.format(
            self.__class__.__name__,
            self.filename)


def discover_integration_tests():
    test_dir = project_root / "integration_tests"

    for file in test_dir.glob("*.rs"):
        test = IntegrationTest(file)
        yield test


def main():
    for test in discover_integration_tests():
        test.initialize()
        # time.sleep(12000)
        test.run()

if __name__ == "__main__":
    main()

