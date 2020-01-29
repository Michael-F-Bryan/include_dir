use crate::dir::Dir;
use crate::file::File;
use glob::{Pattern, PatternError};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Globs<'a> {
    stack: Vec<DirEntry<'a>>,
    pattern: Pattern,
}

impl<'a> Dir<'a> {
    /// Search for a file or directory with a glob pattern.
    pub fn find(&self, glob: &str) -> Result<impl Iterator<Item = DirEntry<'a>>, PatternError> {
        let pattern = Pattern::new(glob)?;

        Ok(Globs::new(pattern, *self))
    }

    pub(crate) fn dir_entries(&self) -> impl Iterator<Item = DirEntry<'a>> {
        let files = self.files().iter().map(|f| DirEntry::File(*f));
        let dirs = self.dirs().iter().map(|d| DirEntry::Dir(*d));

        files.chain(dirs)
    }
}

impl<'a> Globs<'a> {
    pub(crate) fn new(pattern: Pattern, root: Dir<'a>) -> Globs<'a> {
        let stack = vec![DirEntry::Dir(root)];
        Globs { stack, pattern }
    }

    fn fill_buffer(&mut self, item: &DirEntry<'a>) {
        if let DirEntry::Dir(ref dir) = *item {
            self.stack.extend(dir.dir_entries());
        }
    }
}

impl<'a> Iterator for Globs<'a> {
    type Item = DirEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            self.fill_buffer(&item);

            if self.pattern.matches_path(item.path()) {
                return Some(item);
            }
        }

        None
    }
}

/// Entries returned by the Globs iterator
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DirEntry<'a> {
    /// A file with its contents stored in a &'static [u8].
    File(File<'a>),
    /// A directory entry.
    Dir(Dir<'a>),
}

impl<'a> DirEntry<'a> {
    /// Get the entries's path
    pub fn path(&self) -> &'a Path {
        match *self {
            DirEntry::File(f) => f.path(),
            DirEntry::Dir(d) => d.path(),
        }
    }
}
