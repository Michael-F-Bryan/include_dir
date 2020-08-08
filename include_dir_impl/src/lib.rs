//! Implementation crate for the [include_dir!()] macro.
//!
//! [include_dir!()]: https://github.com/Michael-F-Bryan/include_dir

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, LitStr, Token};

use crate::dir::Dir;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

mod dir;
mod file;

struct Args {
    dir: String,
    exclude: HashSet<String>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let strings = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
        let mut iter = strings.into_iter();
        let dir = iter.next().unwrap().value();
        let exclude = iter.map(|x| x.value()).collect();
        Ok(Args { dir, exclude })
    }
}

#[proc_macro_hack]
pub fn include_dir(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as Args);
    let crate_root = env::var("CARGO_MANIFEST_DIR").unwrap();

    let path = PathBuf::from(crate_root).join(args.dir);

    if !path.exists() {
        panic!("\"{}\" doesn't exist", path.display());
    }

    let path = path.canonicalize().expect("Can't normalize the path");

    let dir = Dir::from_disk(&path, &path, &args.exclude).expect("Couldn't load the directory");

    TokenStream::from(quote! {
        #dir
    })
}
