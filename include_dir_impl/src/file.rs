use anyhow::Error;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct File {
    root_rel_path: PathBuf,
    abs_path: PathBuf,
}

impl File {
    pub fn from_disk<Q: AsRef<Path>, P: Into<PathBuf>>(root: Q, path: P) -> Result<File, Error> {
        let abs_path = path.into();
        let root = root.as_ref();

        let root_rel_path = abs_path.strip_prefix(&root).unwrap().to_path_buf();

        Ok(File {
            abs_path,
            root_rel_path,
        })
    }
}

impl ToTokens for File {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let root_rel_path = self.root_rel_path.display().to_string();
        let abs_path = self.abs_path.display().to_string();

        let tok = quote! {
            $crate::File {
                path: #root_rel_path,
                contents: include_bytes!(#abs_path),
            }
        };

        tok.to_tokens(tokens);
    }
}
