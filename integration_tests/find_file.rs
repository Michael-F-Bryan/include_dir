use assets::ASSETS;

fn main() {
    let lib_rs = ASSETS.find("lib.rs").expect("Couldn't find libs.rs");

    // Note: you need to provide the full name
    assert!(ASSETS.find("lib.").is_none());

    assert!(ASSETS.find("non-existent-file.php").is_none());
}

mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
