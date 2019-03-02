use glob::{Pattern, PatternError};
use std::path::Path;

use crate::file::File;
use crate::globs::{DirEntry, Globs};

/// A directory entry.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dir<'a> {
    #[doc(hidden)]
    pub path: &'a str,
    #[doc(hidden)]
    pub files: &'a [File<'a>],
    #[doc(hidden)]
    pub dirs: &'a [Dir<'a>],
}

impl<'a> Dir<'a> {
    /// Get the directory's path.
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    /// Get a list of the files in this directory.
    pub fn files(&self) -> &'a [File<'a>] {
        self.files
    }

    /// Get a list of the sub-directories inside this directory.
    pub fn dirs(&self) -> &'a [Dir<'a>] {
        self.dirs
    }

    /// Does this directory contain `path`?
    pub fn contains<S: AsRef<Path>>(&self, path: S) -> bool {
        let path = path.as_ref();

        self.get_file(path).is_some() || self.get_dir(path).is_some()
    }

    /// Fetch a sub-directory by *exactly* matching its path relative to the
    /// directory included with `include_dir!()`.
    pub fn get_dir<S: AsRef<Path>>(&self, path: S) -> Option<Dir> {
        let path = path.as_ref();

        for dir in self.dirs {
            if Path::new(dir.path) == path {
                return Some(*dir);
            }

            if let Some(d) = dir.get_dir(path) {
                return Some(d);
            }
        }

        None
    }

    /// Fetch a sub-directory by *exactly* matching its path relative to the
    /// directory included with `include_dir!()`.
    pub fn get_file<S: AsRef<Path>>(&self, path: S) -> Option<File> {
        let path = path.as_ref();

        for file in self.files {
            if Path::new(file.path) == path {
                return Some(*file);
            }
        }

        for dir in self.dirs {
            if let Some(d) = dir.get_file(path) {
                return Some(d);
            }
        }

        None
    }

    /// Search for a file or directory with a glob pattern.
    pub fn find(&self, glob: &str) -> Result<impl Iterator<Item = DirEntry<'a>>, PatternError> {
        let pattern = Pattern::new(glob)?;

        Ok(Globs::new(pattern, *self))
    }

    pub(crate) fn dir_entries(&self) -> impl Iterator<Item = DirEntry<'a>> {
        let files = self.files().into_iter().map(|f| DirEntry::File(*f));
        let dirs = self.dirs().into_iter().map(|d| DirEntry::Dir(*d));

        files.chain(dirs)
    }
}
