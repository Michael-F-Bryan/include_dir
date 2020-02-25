use crate::file::File;
use std::path::Path;

use crate::DirEntry;

/// A directory entry.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dir<'a> {
    path: &'a str,
    entries: &'a [DirEntry<'a>]
}

impl<'a> Dir<'a> {

    /// Create a new [`Dir`]
    pub const fn new(path: &'a str, entries: &'a [DirEntry<'_>]) -> Self {
        Self {
            path,
            entries
        }
    }

    /// The directory's path relative to the directory included with [include_dir!()]
    pub fn path(&self) -> &'_ Path {
        Path::new(self.path)
    }

    /// Retrieve the entries within the directory
    pub fn entries(&self) -> &'_ [DirEntry<'_>] {
        self.entries
    }

    /// Return an iterator over all files contained within the directory
    pub fn files(&self) -> impl Iterator<Item=&File<'_>> {
        self
            .entries
            .iter()
            .filter_map(Into::into)
   }

    /// Return an iterator over all sub-directories within the directory
    pub fn dirs(&self) -> impl Iterator<Item=&Dir<'_>> {
        self
            .entries
            .iter()
            .filter_map(Into::into)
    }
}
