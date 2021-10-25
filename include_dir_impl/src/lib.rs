//! Implementation crate for the [include_dir!()] macro.
//!
//! [include_dir!()]: https://github.com/Michael-F-Bryan/include_dir

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, LitStr};

use crate::dir::Dir;
use std::env;
use std::path::{Path, PathBuf};

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

    Dir::from_disk(&path, &path)
        .expect("Couldn't load the directory")
        .to_token_stream()
        .into()
}

#[proc_macro_hack]
pub fn try_include_dir(input: TokenStream) -> TokenStream {
    let input: LitStr = parse_macro_input!(input as LitStr);
    let crate_root = env::var("CARGO_MANIFEST_DIR").unwrap();

    let path = PathBuf::from(crate_root).join(input.value());
    match load_dir(path) {
        Ok(dir) => quote! { Result::<$crate::Dir, &'static str>::Ok(#dir) },
        Err(err) => quote! { Result::<$crate::Dir, &'static str>::Err(#err) },
    }
    .into()
}

fn load_dir(path: impl AsRef<Path>) -> Result<Dir, String> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(format!("\"{}\" doesn't exist", path.display()));
    }

    let path = path
        .canonicalize()
        .map_err(|_| String::from("Can't normalize the path"))?;

    Dir::from_disk(&path, &path)
        .map_err(|e| format!("Couldn't load the directory: {}", e.to_string()))
}
