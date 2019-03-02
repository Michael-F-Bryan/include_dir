//! An extension to the `include_str!()` and `include_bytes!()` macro for embedding an entire
//! directory tree into your binary.
//!
//! # Examples
//!
//! The `include_dir!()` macro will include a directory **relative to the
//! project root** (using the `CARGO_MANIFEST_DIR` variable), in this example
//! the source code for the `include_dir` crate has been included inside itself.
//!
//! ```rust
//! #[macro_use]
//! extern crate include_dir;
//!
//! use include_dir::Dir;
//! use std::path::Path;
//!
//! const PROJECT_DIR: Dir = include_dir!(".");
//!
//! # fn main() {
//! // of course, you can retrieve a file by its full path
//! let lib_rs = PROJECT_DIR.get_file("src/lib.rs").unwrap();
//!
//! // you can also inspect the file's contents
//! let body = lib_rs.contents_utf8().unwrap();
//! assert!(body.contains("SOME_INTERESTING_STRING"));
//!
//! // you can search for files (and directories) using glob patterns
//! let glob = "**/*.rs";
//! for entry in PROJECT_DIR.find(glob).unwrap() {
//!     println!("Found {}", entry.path().display());
//! }
//! # }
//! ```

#![deny(missing_docs, missing_copy_implementations, missing_debug_implementations)]

use proc_macro_hack::proc_macro_hack;

mod dir;
mod file;
mod globs;

pub use crate::dir::Dir;
pub use crate::file::File;

#[doc(hidden)]
#[proc_macro_hack]
pub use include_dir_impl::include_dir;
