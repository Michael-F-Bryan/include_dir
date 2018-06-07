//! Implementation crate for the [include_dir!()] macro.
//!
//! [include_dir!()]: https://github.com/Michael-F-Bryan/include_dir

#[macro_use]
extern crate proc_macro_hack;
extern crate failure;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

mod dir;
mod file;

use dir::Dir;
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
            .join(input.value());

        if !path.exists() {
            panic!("\"{}\" doesn't exist", path.display());
        }

        let path = path.canonicalize().expect("Can't normalize the path");

        let mut dir = Dir::from_disk(&path).expect("Couldn't load the directory");
        dir.normalize(&path);

        let tokens = quote!({
                __include_dir_use_everything!();
                #dir
            });

        tokens.to_string()
    }
}
