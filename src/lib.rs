#[macro_use]
extern crate error_chain;
extern crate walkdir;

#[cfg(test)]
extern crate tempfile;

mod files;

pub use errors::*;
pub use files::File;


mod errors {
    error_chain!{
        foreign_links {
            IO(::std::io::Error);
        }
    }
}
