use glob::{Pattern, PatternError};
use std::path::Path;

use crate::file::File;
use crate::dir::{Dir, DirEntry};
use crate::globs::Globs;

/// The file system that includes all the files and directories included with `include_dir!()`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FileSystem<'a> {
    #[doc(hidden)]
    pub root: DirEntry<'a>,
}

impl<'a> FileSystem<'a> {
    /// Does this directory contain `path`?
    pub fn contains<S: AsRef<Path>>(&self, path: S) -> bool {
        self.get(path).is_some()
    }

    /// Does this file system contain `path`?
    pub fn get<S: AsRef<Path>>(&self, path: S) -> Option<&DirEntry> {
        self.root.get(path.as_ref().iter())
    }

    /// Does this file system contain a file `path`?
    pub fn get_file<S: AsRef<Path>>(&self, path: S) -> Option<&File> {
        if let Some(DirEntry::File(file)) = self.get(path) {
            Some(file)
        } else {
            None
        }
    }

    /// Does this file system contain a directory `path`?
    pub fn get_dir<S: AsRef<Path>>(&self, path: S) -> Option<&Dir> {
        if let Some(DirEntry::Dir(dir)) = self.get(path) {
            Some(dir)
        } else {
            None
        }
    }

    /// Search for a file or directory with a glob pattern.
    pub fn find(&self, glob: &str) -> Result<impl Iterator<Item = DirEntry<'a>>, PatternError> {
        let pattern = Pattern::new(glob)?;

        Ok(Globs::new(pattern, self.root))
    }
}
