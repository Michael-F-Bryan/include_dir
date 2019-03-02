#[macro_use]
extern crate include_dir;

use include_dir::Dir;

/// Example the output generated when running `include_dir!()` on itself.
pub static GENERATED_EXAMPLE: Dir = include_dir!("src");

fn main() {
    for file in GENERATED_EXAMPLE.files() {
        println!("{}: {}", file.path().display(), file.contents.len());
    }
}
