//! Implementation crate for the [include_dir!()] macro.
//!
//! [include_dir!()]: https://github.com/Michael-F-Bryan/include_dir

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::dir::Dir;
use std::env;
use std::path::PathBuf;

mod dir;
mod file;

pub fn include_dir(input: LitStr) -> TokenStream {
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
