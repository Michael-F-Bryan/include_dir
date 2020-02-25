use crate::dir::Dir;
use crate::file::File;
use std::path::{Path, PathBuf};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DirEntry {
    Dir(Dir),
    File(File),
}

impl DirEntry {
    pub(crate) fn root_rel_path(&self) -> &Path {
        match self {
            DirEntry::Dir(d) => d.root_rel_path.as_path(),
            DirEntry::File(f) => f.root_rel_path.as_path(),
        }
    }

    pub(crate) fn from_disk(root: impl AsRef<Path>, path: impl Into<PathBuf>) -> Result<DirEntry, anyhow::Error> {
        Ok(DirEntry::Dir(Dir::from_disk(root, path)?))
    }
}

impl ToTokens for DirEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {

        let tok = match self {
            DirEntry::Dir(dir) => {
                quote! {
                    $crate::DirEntry::Dir(#dir)
                }
            },
            DirEntry::File(file) => {
                quote! {
                    $crate::DirEntry::File(#file)
                }
            },
        };

        tok.to_tokens(tokens)
    }
}
