// FEATURE: globs
// IGNORE: .git target

extern crate glob;
use assets::{DirEntry, ASSETS};

fn main() {
    for entry in ASSETS.glob("*.rs").unwrap() {
        match entry {
            DirEntry::Dir(d) => println!(
                "{}\tfiles: {}, subdirs: {}",
                d.path().display(),
                d.files.len(),
                d.subdirs.len()
            ),
            DirEntry::File(f) => println!("{}\t({} bytes)", f.path().display(), f.contents.len()),
        }
    }
}

#[allow(dead_code)]
mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
