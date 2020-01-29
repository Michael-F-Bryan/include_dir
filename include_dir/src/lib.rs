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
//! use include_dir::{include_dir, Dir};
//! use std::path::Path;
//!
//! const PROJECT_DIR: Dir = include_dir!(".");
//!
//! // of course, you can retrieve a file by its full path
//! let lib_rs = PROJECT_DIR.get_file("src/lib.rs").unwrap();
//!
//! // you can also inspect the file's contents
//! let body = lib_rs.contents_utf8().unwrap();
//! assert!(body.contains("SOME_INTERESTING_STRING"));
//!
//! // if you enable the `search` feature, you can for files (and directories) using glob patterns
//! #[cfg(feature = "search")]
//! {
//!     let glob = "**/*.rs";
//!     for entry in PROJECT_DIR.find(glob).unwrap() {
//!         println!("Found {}", entry.path().display());
//!     }
//! }
//! ```
//!
//! # Features
//!
//! This library exposes a couple feature flags for enabling and disabling extra
//! functionality. These are:
//!
//! - **example:** compile in an example of the embedded directory tree

#![deny(
    elided_lifetimes_in_paths,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
)]

#[allow(unused_imports)]
#[macro_use]
extern crate include_dir_impl;
#[macro_use]
extern crate proc_macro_hack;

mod dir;
mod file;

#[cfg(feature = "search")]
mod globs;

pub use crate::dir::Dir;
pub use crate::file::File;
#[cfg(feature = "search")]
pub use crate::globs::DirEntry;

#[doc(hidden)]
#[proc_macro_hack]
pub use include_dir_impl::include_dir;

/// Example the output generated when running `include_dir!()` on itself.
#[cfg(feature = "example-output")]
pub static GENERATED_EXAMPLE: Dir<'_> = include_dir!(".");
