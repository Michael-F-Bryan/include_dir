use failure::{Error, ResultExt};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use utils;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub path: PathBuf,
    pub contents: Vec<u8>,
}

impl File {
    pub fn from_disk<P: Into<PathBuf>>(path: P) -> Result<File, Error> {
        let path = path.into();

        let mut contents = Vec::new();
        fs::File::open(&path)
            .context("Unable to open the file")?
            .read_to_end(&mut contents)
            .context("Couldn't read the file")?;

        Ok(File { path, contents })
    }
}

impl ToTokens for File {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = utils::escape(&self.path);
        let contents = &self.contents;

        let tok = quote!{
            ::include_dir::File {
                path: #path,
                contents: &[#(
                    #contents
                 ),*],
            }
        };

        tok.to_tokens(tokens);
    }
}
