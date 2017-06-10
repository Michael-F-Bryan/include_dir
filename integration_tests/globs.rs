// I'm using a django project on my local machine
// ROOT: /home/michael/Documents/website
// FEATURE: globs
// IGNORE: .git __pycache__

extern crate glob;
use assets::{ASSETS, DirEntry};

fn main() {
    for entry in ASSETS.glob("*.py").unwrap() {
        match entry {
            DirEntry::Dir(d) => {
                println!("{}\tfiles: {}, subdirs: {}",
                         d.path().display(),
                         d.files.len(),
                         d.subdirs.len())
            }
            DirEntry::File(f) => println!("{}\t({} bytes)", f.path().display(), f.contents.len()),
        }
    }
}

#[allow(dead_code)]
mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
