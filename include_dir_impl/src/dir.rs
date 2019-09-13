use failure::{self, Error, ResultExt};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

use crate::file::File;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Dir {
    abs_path: PathBuf,
    root_rel_path: PathBuf,
    entries: Vec<DirEntry>,
}

#[derive(Debug, Clone, PartialEq)]
enum DirEntry {
    File(File),
    Dir(Dir),
}

impl DirEntry {
    fn file_name(&self) -> Option<&OsStr> {
        match self {
            DirEntry::File(file) => file.file_name(),
            DirEntry::Dir(dir) => dir.file_name(),
        }
    }
}

impl ToTokens for DirEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tok = match self {
            DirEntry::File(file) => {
                quote!{
                    $crate::DirEntry::File(#file)
                }
            }
            DirEntry::Dir(dir) => {
                quote!{
                    $crate::DirEntry::Dir(#dir)
                }
            }
        };

        tok.to_tokens(tokens);
    }
}

impl Dir {
    pub fn from_disk<Q: AsRef<Path>, P: Into<PathBuf>>(root: Q, path: P) -> Result<Dir, Error> {
        let abs_path = path.into();
        let root = root.as_ref();

        let root_rel_path = abs_path.strip_prefix(&root).unwrap().to_path_buf();

        if !abs_path.exists() {
            return Err(failure::err_msg("The directory doesn't exist"));
        }

        let mut entries = Vec::new();

        for entry in abs_path.read_dir().context("Couldn't read the directory")? {
            let entry = entry?.path();

            if entry.is_file() {
                entries.push(DirEntry::File(File::from_disk(&root, entry)));
            } else if entry.is_dir() {
                entries.push(DirEntry::Dir(Dir::from_disk(&root, entry)?));
            }
        }

        entries.sort_unstable_by(|a, b| a.file_name().cmp(&b.file_name()));

        Ok(Dir { abs_path, root_rel_path, entries })
    }

    pub fn file_name(&self) -> Option<&OsStr> {
        self.root_rel_path.file_name()
    }
}

impl ToTokens for Dir {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let root_rel_path = self.root_rel_path.to_str()
            .expect("path should contain valid UTF-8 characters");

        let file_name = if let Some(file_name) = self.root_rel_path.file_name() {
            file_name.to_str()
                .expect("path cannot contain invalid UTF-8 characters")
        } else {
            ""
        };
        let entries = &self.entries;

        let tok = quote!{
            $crate::Dir {
                path: #root_rel_path,
                file_name: #file_name,
                entries: &[#(
                    #entries
                ),*],
            }
        };

        tok.to_tokens(tokens);
    }
}
