# include_dir

[![Build Status](https://travis-ci.org/Michael-F-Bryan/include_dir.svg?branch=master)](https://travis-ci.org/Michael-F-Bryan/include_dir)
[![Build status](https://ci.appveyor.com/api/projects/status/3a4actkllivtsytk?svg=true)](https://ci.appveyor.com/project/Michael-F-Bryan/include-dir)
[![license](https://img.shields.io/github/license/michael-f-bryan/include_dir.svg)]()
[![Crates.io](https://img.shields.io/crates/v/include_dir.svg)](https://crates.io/crates/include_dir)
[![Docs.rs](https://docs.rs/include_dir/badge.svg)](https://docs.rs/include_dir/)



An extension to the `include_str!()` macro for embedding an entire directory
tree into your binary.


## Getting Started

To embed a directory and its contents into your binary, you'll need to add the
following to your `build.rs` script.

```rust
extern crate include_dir;

use std::env;
use std::path::Path;
use include_dir::include_dir;

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&outdir).join("assets.rs");

    include_dir("assets")
        .as_variable("SRC")
        .to_file(dest_path)
        .unwrap();
    }
```


## Features

- Embed a directory tree into your binary at compile time
- Find a file in the embedded directory
- Walk the directory tree, similar to the [walkdir] crate
- Search for files using a glob pattern (requires the `globs` feature)
- Ignore items matching a provided pattern (still has a couple bugs, YMMV)
- Rustdoc documentation for all generated types and their methods

To-Do list:

- Embed multiple directories without doubling up any struct definitions
- File metadata
- Compression
- `include_dir!()` proc macro?


## Integration Tests

Because a large part of this crate's functionality depends on generated code,
it's easier to test functionality from the point-of-view of an end user.
Therefore, a large proportion of the crate's tests are orchestrated by a
Python script.

For each `*.rs` file in the `integration_tests/` directory, this Python script
will:

- Create a new `--bin` crate in a temporary directory
- Copy the `*.rs` file into this new crate and rename it to `main.rs`.
- Scan the `*.rs` file for a special pattern indicating which asset
  directory will be included (relative to this crate's root directory). If the
  pattern isn't found, use this crate's `src/` directory.
- Generate a `build.rs` file which will compile in the specified file tree.
- Compile and run the new binary test crate.


[walkdir]: https://docs.rs/walkdir/
