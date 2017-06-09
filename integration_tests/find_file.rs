// INCLUDE: src
// FEATURE: glob
// IGNORE: .git target

extern crate glob;
use assets::ASSETS;

fn main() {
    let lib_rs = ASSETS.find("lib.rs").expect("Couldn't find libs.rs");

    // you need to provide the full name
    assert!(ASSETS.find("lib.").is_none());

    assert!(ASSETS.find("non-existent-file.php").is_none());

    // Using globs gives you back an iterator over all the matches
    let rust_files: Vec<_> = ASSETS.glob("*.rs").collect();
    assert!(rust_files.len() > 0,
            "I'm pretty sure there should be some rust files around here somewhere...");
}

#[allow(dead_code)]
mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
