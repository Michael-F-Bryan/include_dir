#[macro_use]
extern crate proc_macro_hack;
extern crate include_dir_core;
extern crate syn;
#[macro_use]
extern crate quote;

use include_dir_core::Dir;
use std::env;
use std::path::PathBuf;
use syn::LitStr;

proc_macro_expr_impl! {
    /// Add one to an expression.
    pub fn include_dir_impl(input: &str) -> String {
        let input: LitStr =  syn::parse_str(input)
            .expect("include_dir!() only accepts a single string argument");
        let crate_root = env::var("CARGO_MANIFEST_DIR").unwrap();

        let path = PathBuf::from(crate_root)
            .join(input.value())
            .canonicalize()
            .expect("Unable to normalize the path");

        let dir = Dir::from_disk(&path).expect("Couldn't load the directory");

        quote!(#dir).to_string()
    }
}
