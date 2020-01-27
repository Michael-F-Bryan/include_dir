//! Implementation crate for the [include_dir!()] macro.
//!
//! [include_dir!()]: https://github.com/Michael-F-Bryan/include_dir

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use crate::dir::Dir;
use std::env;
use std::path::PathBuf;

mod dir;
mod file;

#[proc_macro_hack]
pub fn include_dir(input: TokenStream) -> TokenStream {
    let input: LitStr = parse_macro_input!(input as LitStr);
    let crate_root = env::var("CARGO_MANIFEST_DIR").unwrap();

    let path = PathBuf::from(crate_root).join(input.value());

    if !path.exists() {
        panic!("\"{}\" doesn't exist", path.display());
    }

    let path = path.canonicalize().expect("Can't normalize the path");

    let dir = Dir::from_disk(&path, &path).expect("Couldn't load the directory");

    TokenStream::from(quote! {
        #dir
    })
}
