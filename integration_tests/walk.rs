use assets::ASSETS;
use assets::DirEntry;

fn main() {
    let mut number_of_files = 0;
    let mut number_of_dirs = 0;

    for entry in ASSETS.walk() {
        println!("{}", entry.path().display());

        match entry {
            DirEntry::Dir(_) => number_of_dirs += 1,
            DirEntry::File(_) => number_of_files += 1,
        }
    }
}

#[allow(dead_code)]
mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
