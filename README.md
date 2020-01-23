# include_dir

[![Build Status](https://travis-ci.org/Michael-F-Bryan/include_dir.svg?branch=master)](https://travis-ci.org/Michael-F-Bryan/include_dir)
[![Build status](https://ci.appveyor.com/api/projects/status/3a4actkllivtsytk?svg=true)](https://ci.appveyor.com/project/Michael-F-Bryan/include-dir)
[![license](https://img.shields.io/github/license/Michael-F-Bryan/include_dir.svg)]()
[![Crates.io](https://img.shields.io/crates/v/include_dir.svg)](https://crates.io/crates/include_dir)
[![Docs.rs](https://docs.rs/include_dir/badge.svg)](https://docs.rs/include_dir/)

An evolution of the `include_str!()` and `include_bytes!()` macros for embedding 
an entire directory tree into your binary.

Rendered Documentation:

- [master](https://michael-f-bryan.github.io/include_dir)
- [Latest Release](https://docs.rs/include_dir/)

## Getting Started

The `include_dir!()` macro works very similarly to the normal `include_str!()`
and `include_bytes!()` macros. You pass the macro a file path and assign the 
returned value to some `static` variable.

Most importantly, the file path **must be relative to the project root** as
indicated by the `CARGO_MANIFEST_DIR` environment variable.

```rust
#[macro_use]
extern crate include_dir;

use include_dir::Dir;
use std::path::Path;

static PROJECT_DIR: Dir = include_dir!(".");

// of course, you can retrieve a file by its full path
let lib_rs = PROJECT_DIR.get_file("src/lib.rs").unwrap();

// you can also inspect the file's contents
let body = lib_rs.contents_utf8().unwrap();
assert!(body.contains("SOME_INTERESTING_STRING"));

// you can search for files (and directories) using glob patterns
#[cfg(feature = "search")]
{
    let glob = "**/*.rs";
    for entry in PROJECT_DIR.find(glob).unwrap() {
        println!("Found {}", entry.path().display());
    }
}
```

## Features

- Embed a directory tree into your binary at compile time
- Find a file in the embedded directory
- Search for files using a glob pattern (requires the `globs` feature)

To-Do list:

- File metadata
- Compression?
