use std::path::{Iter as PathIter, Path};
use std::ffi::OsStr;

use crate::file::File;

/// A directory entry.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DirEntry<'a> {
    /// A directory.
    Dir(Dir<'a>),

    /// A file.
    File(File<'a>),
}

impl<'a> DirEntry<'a> {
    /// Get the directory's path.
    pub fn path(&self) -> &'a Path {
        match self {
            DirEntry::Dir(dir) => dir.path(),
            DirEntry::File(file) => file.path(),
        }
    }

    pub(crate) fn file_name(&self) -> &'a OsStr {
        match self {
            DirEntry::File(file) => file.file_name(),
            DirEntry::Dir(dir) => dir.file_name(),
        }
    }

    pub(crate) fn get(&self, mut iter: PathIter) -> Option<&DirEntry> {
        match (self, iter.next()) {
            (entry, None) => {
                Some(entry)
            }
            (DirEntry::File(_), Some(_)) => {
                // Can't desend into a file.
                None
            }
            (DirEntry::Dir(dir), Some(name)) => {
                if let Ok(pos) = dir.entries.binary_search_by_key(&name, |e| e.file_name()) {
                    dir.entries[pos].get(iter)
                } else {
                    None
                }
            }
        }
    }
}

/// A directory.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dir<'a> {
    #[doc(hidden)]
    pub path: &'a str,
    #[doc(hidden)]
    pub file_name: &'a str,
    #[doc(hidden)]
    pub entries: &'a [DirEntry<'a>],
}

impl<'a> Dir<'a> {
    /// Get the directory's path.
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    /// The directory's name.
    pub fn file_name(&self) -> &'a OsStr {
        OsStr::new(self.file_name)
    }

    /// Get a list of the entries in this directory.
    pub fn entries(&self) -> &'a [DirEntry<'a>] {
        self.entries
    }
}
