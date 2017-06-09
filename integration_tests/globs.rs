// FEATURE: globs
// IGNORE: .git target

extern crate glob;
use assets::ASSETS;

fn main() {
    // Using globs gives you back an iterator over all the matches
    let rust_files: Vec<_> = ASSETS.glob("*.rs").unwrap().collect();
    assert!(rust_files.len() > 0,
            "I'm pretty sure there should be some rust files around here somewhere...");

    for file in rust_files {
        println!("{}", file.path().display());
    }

    let lib_star: Vec<_> = ASSETS.glob("lib.*").unwrap().collect();
    assert_eq!(lib_star.len(), 1);

    panic!();
}

#[allow(dead_code)]
mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
