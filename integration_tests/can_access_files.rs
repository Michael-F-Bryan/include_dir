
fn main() {
    println!("{:?}", assets::ASSETS.files);
}

mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
