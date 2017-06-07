extern crate include_dir;

mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}

fn main() {
    println!("{:?}", assets::ASSETS.files());
}
