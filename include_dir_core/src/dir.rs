use failure::{Error, ResultExt};
use file::File;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::borrow::Cow;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Dir<'a> {
    path: Cow<'a, Path>,
    files: Cow<'a, [File<'a>]>,
    // dirs: Cow<'a, [Dir<'a>]>,
}

impl Dir<'static> {
    pub fn from_disk<P: AsRef<Path>>(path: P) -> Result<Dir<'static>, Error> {
        let root = path.as_ref().to_owned();

        let mut files = Vec::new();
        let mut dirs = Vec::new();

        for entry in root.read_dir().context("Unable to read the directory")? {
            let entry = entry.context("Unable to inspect the item")?.path();

            if entry.is_file() {
                let file = File::from_disk(entry)?;
                files.push(file);
            } else if entry.is_dir() {
                let dir = Dir::from_disk(entry)?;
                dirs.push(dir);
            }
        }

        let d = Dir {
            path: Cow::Owned(root),
            files: Cow::Owned(files),
            // dirs: Cow::Owned(dirs),
        };

        Ok(d)
    }
}

impl<'a> ToTokens for Dir<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        unimplemented!()
    }
}
