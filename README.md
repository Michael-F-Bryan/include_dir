# include_dir

An extension to the `include_str!()` macro for embedding an entire directory
tree into your binary.


## Getting Started

To embed a directory and its contents into your binary, you'll need to add the
following to your `build.rs` script.

```rust
extern crate include_dir;

use std::env;
use std::fs::File;
use include_dir::include_dir;

fn main() {
    match run() {
        Ok(()) => {},
        Err(e) => panic!("Error including assets, {}", e),
    }
}

fn run() -> Result<(), Box<Error>> {
    let dir = include_dir("./path/to/assets/")?;

    let assets_rs = PathBuf::from(env::get("OUTPUT_DIR").unwrap())
        .join("assets.rs");
    dir.write_to(File::open(assets_rs))?;

    Ok(())
}
```
