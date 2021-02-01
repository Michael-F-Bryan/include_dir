//! Implementation crate for the [include_dir!()] macro.
//!
//! [include_dir!()]: https://github.com/Michael-F-Bryan/include_dir

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use crate::dir::Dir;
use std::path::PathBuf;
use std::{env, ffi::OsString};

mod dir;
mod file;

#[proc_macro_hack]
pub fn include_dir(input: TokenStream) -> TokenStream {
    let root = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    include_dir_from_root(root, input)
}

#[proc_macro_hack]
pub fn include_dir_from_out_dir(input: TokenStream) -> TokenStream {
    let root = env::var_os("OUT_DIR")
        .unwrap_or_else(|| panic!("OUT_DIR environment variable is not defined"));
    include_dir_from_root(root, input)
}

fn include_dir_from_root(root: OsString, input: TokenStream) -> TokenStream {
    let input: LitStr = parse_macro_input!(input as LitStr);
    let path = PathBuf::from(root).join(input.value());

    if !path.exists() {
        panic!("\"{}\" doesn't exist", path.display());
    }

    let path = path.canonicalize().expect("Can't normalize the path");

    let dir = Dir::from_disk(&path, &path).expect("Couldn't load the directory");

    TokenStream::from(quote! {
        #dir
    })
}
