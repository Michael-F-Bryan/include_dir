use assets::ASSETS;

fn main() {
    println!("Asset directory: {}", ASSETS.name);

    for file in ASSETS.files {
        println!("\t{} ({} bytes)", file.name, file.contents.len());
    }

    panic!();
}

mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
