//! An extension to the `include_str!()` and `include_bytes!()` macro for
//! embedding an entire directory tree into your binary.
//!
//! # Environment Variables
//!
//! # Examples
//!
//! Here is an example that embeds the `include_dir` crate's source code in a
//! `const` so we can play around with it.
//!
//! ```rust
//! use include_dir::{include_dir, Dir};
//! use std::path::Path;
//!
//! const PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR");
//!
//! // of course, you can retrieve a file by its full path
//! let lib_rs = PROJECT_DIR.get_file("src/lib.rs").unwrap();
//!
//! // you can also inspect the file's contents
//! let body = lib_rs.contents_utf8().unwrap();
//! assert!(body.contains("SOME_INTERESTING_STRING"));
//!
//! // if you enable the `glob` feature, you can for files (and directories) using glob patterns
//! #[cfg(feature = "glob")]
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
//! - `glob` - search for files using glob patterns
//! - `nightly` - enables nightly APIs like
//!   [`proc_macro_tracked_env`][tracked-env] and [`track_path`][track-path].
//!   This gives the compiler more information about what is accessed by the
//!   procedural macro, enabling better caching.
//!
//! [tracked-env]: https://github.com/rust-lang/rust/issues/74690
//! [track-path]: https://github.com/rust-lang/rust/issues/73921

#![deny(
    elided_lifetimes_in_paths,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]

mod dir;
mod dir_entry;
mod file;

#[cfg(feature = "glob")]
mod globs;

pub use crate::dir::Dir;
pub use crate::dir_entry::DirEntry;
pub use crate::file::File;

pub use include_dir_macros::include_dir;
