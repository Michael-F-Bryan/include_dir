// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate error_chain;
extern crate walkdir;

#[cfg(test)]
extern crate tempfile;
#[cfg(test)]
extern crate tempdir;

mod files;
mod dirs;

pub use errors::*;
pub use files::File;
pub use dirs::Dir;


mod errors {
    error_chain!{
        foreign_links {
            IO(::std::io::Error);
        }
    }
}
