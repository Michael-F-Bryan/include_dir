extern crate include_dir;

use std::path::Path;
use include_dir::include_dir;

fn main() {
    let dest_path = Path::new(".").join("assets.rs");
    let dir_to_include = Path::new(env!("CARGO_MANIFEST_DIR")).join("integration_tests");

    include_dir(&dir_to_include)
        .as_variable("INTEGRATION_TESTS")
        .to_file(dest_path)
        .unwrap();
}
