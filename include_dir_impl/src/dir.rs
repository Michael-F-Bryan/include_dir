use crate::file::File;
use crate::direntry::DirEntry;

use anyhow::{self, format_err, Context, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::path::{Path, PathBuf};


#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Dir {
    pub(crate) root_rel_path: PathBuf,
    abs_path: PathBuf,
    entries: Vec<DirEntry>
}

impl Dir {
    pub fn from_disk(root: impl AsRef<Path>, path: impl Into<PathBuf>) -> Result<Dir, Error> {
        let abs_path = path.into();
        let root = root.as_ref();

        let root_rel_path = abs_path.strip_prefix(&root).unwrap().to_path_buf();

        if !abs_path.exists() {
            return Err(format_err!("Path '{}' does not exist", abs_path.display()));
        }
        if !abs_path.is_dir() {
            return Err(format_err!("Path '{}' is not a directory", abs_path.display()))
        }

        let mut entries = Vec::new();

        let dir_iter = abs_path
            .read_dir()
            .context(format!("Could not read the directory '{}'", abs_path.display()))?;

        for entry in dir_iter {
            let entry = entry?.path();

            if entry.is_file() {
                entries.push(DirEntry::File(File::from_disk(&root, entry)?));
            } else if entry.is_dir() {
                entries.push(DirEntry::Dir(Dir::from_disk(&root, entry)?));
            }
        }

        entries.sort_unstable_by(
            |a, b| a.root_rel_path().cmp(&b.root_rel_path()
            ));

        Ok(Dir {
            root_rel_path,
            abs_path,
            entries
        })
    }
}

impl ToTokens for Dir {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let root_rel_path = self.root_rel_path.to_str()
            .unwrap_or_else(|| panic!("Path {} is not valid UTF-8", self.root_rel_path.display()));

        let entries = &self.entries;

        let tok = quote! {
            $crate::Dir::new(#root_rel_path, &[#(#entries),*])
        };

        tok.to_tokens(tokens);
    }
}
