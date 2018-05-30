extern crate include_dir_core;
#[allow(unused_imports)]
#[macro_use]
extern crate include_dir_impl;
#[macro_use]
extern crate proc_macro_hack;

#[doc(inline)]
pub use include_dir_core::{Dir, File};
#[doc(hidden)]
pub use include_dir_impl::*;

proc_macro_expr_decl! {
    include_dir! => include_dir_impl
}
