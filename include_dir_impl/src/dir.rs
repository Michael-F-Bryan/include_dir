use failure::{self, Error, ResultExt};
use file::File;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct Dir {
    pub path: PathBuf,
    pub files: Vec<File>,
    pub dirs: Vec<Dir>,
}

impl Dir {
    pub fn from_disk<P: Into<PathBuf>>(path: P) -> Result<Dir, Error> {
        let path = path.into();

        if !path.exists() {
            return Err(failure::err_msg("The directory doesn't exist"));
        }

        let mut files = Vec::new();
        let mut dirs = Vec::new();

        for entry in path.read_dir().context("Couldn't read the directory")? {
            let entry = entry?.path();

            if entry.is_file() {
                files.push(File::from_disk(entry)?);
            } else if entry.is_dir() {
                dirs.push(Dir::from_disk(entry)?);
            }
        }

        Ok(Dir { path, files, dirs })
    }

    pub fn normalize(&mut self, root: &Path) {
        self.path = self.path.strip_prefix(root).unwrap().to_path_buf();

        for file in &mut self.files {
            file.normalize(root);
        }

        for dir in &mut self.dirs {
            dir.normalize(root);
        }
    }
}

impl ToTokens for Dir {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.path.display().to_string();
        let files = &self.files;
        let dirs = &self.dirs;

        let tok = quote!{
            Dir {
                path: #path,
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
