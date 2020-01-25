use crate::file::File;
use anyhow::{self, format_err, Context, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Dir {
    root_rel_path: PathBuf,
    abs_path: PathBuf,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl Dir {
    pub fn from_disk<Q: AsRef<Path>, P: Into<PathBuf>>(root: Q, path: P) -> Result<Dir, Error> {
        let abs_path = path.into();
        let root = root.as_ref();

        let root_rel_path = abs_path.strip_prefix(&root).unwrap().to_path_buf();

        if !abs_path.exists() {
            return Err(format_err!("The directory doesn't exist"));
        }

        let mut files = Vec::new();
        let mut dirs = Vec::new();

        for entry in abs_path.read_dir().context("Couldn't read the directory")? {
            let entry = entry?.path();

            if entry.is_file() {
                files.push(File::from_disk(&root, entry)?);
            } else if entry.is_dir() {
                dirs.push(Dir::from_disk(&root, entry)?);
            }
        }

        Ok(Dir {
            root_rel_path,
            abs_path,
            files,
            dirs,
        })
    }
}

impl ToTokens for Dir {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let root_rel_path = self.root_rel_path.display().to_string();
        let files = &self.files;
        let dirs = &self.dirs;

        let tok = quote! {
            $crate::Dir {
                path: #root_rel_path,
                files: &[#(
                    #files
                 ),*],
                dirs: &[#(
                    #dirs
                 ),*],
            }
        };

        tok.to_tokens(tokens);
    }
}
