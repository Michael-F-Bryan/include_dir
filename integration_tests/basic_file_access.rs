//! This integration test makes sure you can access various basic methods and
//! attributes of `Dir` and `File`.
//!
//! At the moment, we're checking for:
//! - Dir
//!   - name()
//!   - path()
//!   - files
//!   - subdirs
//! - File
//!   - name()
//!   - path()
//!   - contents

use std::path::PathBuf;
use assets::ASSETS;

fn main() {
    println!("Asset directory: {}", ASSETS.name());
    assert_eq!(
        ASSETS.path(),
        PathBuf::from("src"),
        "The root directory pointed to by ASSETS should be the name of the included directory"
    );

    for file in ASSETS.files {
        println!(
            "\t{} at {} ({} bytes)",
            file.name(),
            file.path().display(),
            file.contents.len()
        );
    }

    for dir in ASSETS.subdirs {
        println!("\t{}", dir.path().display());
    }

    println!("Asset directory contains {} bytes", ASSETS.size());
}

#[allow(dead_code, unused_variables)]
mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
