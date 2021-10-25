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
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub(crate) fn timestamp_to_tokenstream(
    time: std::io::Result<SystemTime>,
) -> proc_macro2::TokenStream {
    time.ok()
        .and_then(|m| m.duration_since(UNIX_EPOCH).ok())
        .map(|dur| dur.as_secs_f64())
        .map(|secs| quote! { Some(#secs) }.to_token_stream())
        .unwrap_or_else(|| quote! { None }.to_token_stream())
}
