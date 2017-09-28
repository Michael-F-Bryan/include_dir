//! A "script" used to automatically generate `src/generated_example.rs`. You
//! probably don't want to use this yourself...

extern crate include_dir;

use std::path::{Path, PathBuf};
use std::env;
use include_dir::include_dir;

fn main() {
    let args = parse_args();

    // make sure we're in the right directory
    env::set_current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("src")).unwrap();

    include_dir(&args.target)
        .as_variable(&args.variable_name)
        .to_file(args.dest)
        .unwrap();
}

fn parse_args() -> Args {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 3 {
        usage();
    }

    Args {
        target: PathBuf::from(&args[0]),
        variable_name: args[1].clone(),
        dest: PathBuf::from(&args[2]),
    }
}

fn usage() {
    panic!("USAGE: generate_example <dir> <out_name> <variable>");
}


struct Args {
    target: PathBuf,
    variable_name: String,
    dest: PathBuf,
}
