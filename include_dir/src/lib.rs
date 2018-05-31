#[allow(unused_imports)]
#[macro_use]
extern crate include_dir_impl;
#[macro_use]
extern crate proc_macro_hack;

mod dir;
mod file;

pub use dir::Dir;
pub use file::File;

#[doc(hidden)]
pub use include_dir_impl::*;

proc_macro_expr_decl! {
    include_dir! => include_dir_impl
}
