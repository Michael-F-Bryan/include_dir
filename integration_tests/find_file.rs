use assets::ASSETS;

fn main() {
    let _lib_rs = ASSETS.find("lib.rs").expect("Couldn't find libs.rs");

    // you need to provide the full name
    assert!(ASSETS.find("lib.").is_none());

    assert!(ASSETS.find("non-existent-file.php").is_none());
}

#[allow(dead_code)]
mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
