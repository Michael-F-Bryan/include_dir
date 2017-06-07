#[macro_use]
extern crate error_chain;
extern crate walkdir;

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
