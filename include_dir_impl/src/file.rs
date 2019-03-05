use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct File {
    root_rel_path: PathBuf,
    abs_path: PathBuf,
}

impl File {
    pub fn from_disk<Q: AsRef<Path>, P: Into<PathBuf>>(root: Q, path: P) -> File {
        let abs_path = path.into();
        let root = root.as_ref();

        let root_rel_path = abs_path.strip_prefix(&root).unwrap().to_path_buf();

        File { abs_path, root_rel_path }
    }

    pub fn file_name(&self) -> Option<&OsStr> {
        self.root_rel_path.file_name()
    }
}

impl ToTokens for File {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let root_rel_path = self.root_rel_path.to_str()
            .expect("path should contain valid UTF-8 characters");
        let abs_path = self.abs_path.to_str()
            .expect("path should contain valid UTF-8 characters");
        let file_name = self.root_rel_path.file_name()
            .expect("path should contain a file name")
            .to_str()
            .expect("path should only contain valid UTF-8 characters");

        let tok = quote!{
            $crate::File {
                path: #root_rel_path,
                file_name: #file_name,
                contents: include_bytes!(#abs_path),
            }
        };

        tok.to_tokens(tokens);
    }
}
