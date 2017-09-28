extern crate include_dir;

use std::path::Path;
use std::env;
use include_dir::include_dir;

fn main() {
    let dest_path = Path::new("assets.rs");
    let dir_to_include = Path::new("./integration_tests");

    include_dir(&dir_to_include)
        .as_variable("INTEGRATION_TESTS")
        .to_file(dest_path)
        .unwrap();
}
