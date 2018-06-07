use failure::{Error, ResultExt};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

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

    pub fn normalize(&mut self, root: &Path) {
        self.path = self.path.strip_prefix(root).unwrap().to_path_buf();
    }
}

impl ToTokens for File {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.path.display().to_string();
        let contents = &self.contents;

        let tok = quote!{
            File {
                path: #path,
                contents: &[#(
                    #contents
                 ),*],
            }
        };

        tok.to_tokens(tokens);
    }
}
