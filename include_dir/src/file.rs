use failure::{Error, ResultExt};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::borrow::Cow;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct File<'a> {
    pub path: Cow<'a, Path>,
    pub contents: Cow<'a, [u8]>,
}

impl File<'static> {
    pub fn from_disk<P: AsRef<Path>>(path: P) -> Result<File<'static>, Error> {
        let path = path.as_ref();

        let mut buffer = Vec::new();
        fs::File::open(path)
            .context("Unable to open the file")?
            .read_to_end(&mut buffer)
            .context("Unable to read the file")?;

        Ok(File {
            path: Cow::Owned(path.to_owned()),
            contents: Cow::Owned(buffer),
        })
    }
}

impl<'a> ToTokens for File<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        unimplemented!()
    }
}
