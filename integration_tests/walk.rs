use assets::ASSETS;
use assets::DirEntry;

fn main() {
    let mut number_of_files = 0;
    let mut number_of_dirs = 0;

    for entry in ASSETS.walk() {
        println!("{}", entry.name);
    }
}

mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
