use crate::timestamp_to_tokenstream;
use anyhow::Error;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub(crate) struct File {
    root_rel_path: PathBuf,
    abs_path: PathBuf,
    metadata: Metadata,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.root_rel_path == other.root_rel_path && self.abs_path == other.abs_path
    }
}

impl File {
    pub fn from_disk<Q: AsRef<Path>, P: Into<PathBuf>>(root: Q, path: P) -> Result<File, Error> {
        let abs_path = path.into();
        let root = root.as_ref();
        let root_rel_path = abs_path.strip_prefix(&root).unwrap().to_path_buf();
        let metadata = std::fs::metadata(&abs_path)?;

        Ok(File {
            abs_path,
            root_rel_path,
            metadata,
        })
    }
}

impl ToTokens for File {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let root_rel_path = self.root_rel_path.display().to_string();
        let abs_path = self.abs_path.display().to_string();
        let modified = timestamp_to_tokenstream(self.metadata.modified());
        let created = timestamp_to_tokenstream(self.metadata.created());
        let accessed = timestamp_to_tokenstream(self.metadata.accessed());
        let tok = quote! {
            $crate::File {
                path: #root_rel_path,
                contents: include_bytes!(#abs_path),
                modified: #modified,
                created: #created,
                accessed: #accessed
            }
        };

        tok.to_tokens(tokens);
    }
}
