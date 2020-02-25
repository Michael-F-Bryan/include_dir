use std::path::Path;
use std::path;

use crate::{File, Dir};
use std::convert::TryFrom;

/// An entry within the embedded filesystem representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DirEntry<'a> {
    /// A directory
    Dir(Dir<'a>),
    /// A regular file
    File(File<'a>)
}


impl DirEntry<'_> {
    /// The [`Path`] that corresponds to the entry
    pub fn path(&self) -> &'_ Path {
        match self {
            DirEntry::Dir(dir) => dir.path(),
            DirEntry::File(file) => file.path(),
        }
    }

    /// Traverses the directory sub-tree from this entry
    fn traverse(&self, path_iter: &mut path::Iter<'_>) -> Option<&'_ DirEntry<'_>> {
        match (path_iter.next(), self) {
            // If there are no more components, this is the chosen path
            (None, _) => {
                Some(self)
            },
            // If there are more components and we are in a directory, keep searching if able
            (Some(child), DirEntry::Dir(current_dir)) => {
                current_dir.entries()
                    .binary_search_by_key(&child.into(), |entry| entry.path().file_name())
                    .ok()
                    .map(|index| &current_dir.entries()[index])
                    .and_then(|child_entry| child_entry.traverse(path_iter))
            }
            // otherwise we are a file then there is nowhere else to search, so we give up
            (Some(_), DirEntry::File(_)) => None,
        }
    }

    /// Attempts to retrieve the path from the sub-tree
    pub fn get(&self, path: impl AsRef<Path>) -> Option<&DirEntry<'_>> {
        self.traverse(&mut path.as_ref().iter())
    }

    /// Attempts to retrieve the path from the sub-tree as a [`Dir`]
    pub fn get_dir(&self, path: impl AsRef<Path>) -> Option<&Dir<'_>> {
        match self.traverse(&mut path.as_ref().iter()) {
            Some(DirEntry::Dir(dir)) => Some(dir),
            _ => None
        }
    }

    /// Attempts to retrieve a path from the sub-tree as a [`File`]
    pub fn get_file(&self, path: impl AsRef<Path>) -> Option<&File<'_>> {
        match self.traverse(&mut path.as_ref().iter()) {
            Some(DirEntry::File(file)) => Some(file),
            _=> None
        }
    }

    /// Returns true if the entry corresponds to a [`DirEntry::Dir`]
    pub fn is_dir(&self) -> bool {
        if let DirEntry::Dir(_) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if the entry corresponds to a regular [`DirEntry::File`]
    pub fn is_file(&self) -> bool {
        if let DirEntry::File(_) = *self {
            true
        } else {
            false
        }
    }
}

impl<'a> TryFrom<DirEntry<'a>> for Dir<'a> {
    type Error = ();

    fn try_from(entry: DirEntry<'a>) -> Result<Self, Self::Error> {
        if let DirEntry::Dir(dir) = entry {
            Ok(dir)
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a DirEntry<'a>> for &Dir<'a> {
    type Error = ();

    fn try_from(entry: &'a DirEntry<'a>) -> Result<Self, Self::Error> {
        if let DirEntry::Dir(dir) = entry {
            Ok(dir)
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<DirEntry<'a>> for File<'a> {
    type Error = ();

    fn try_from(entry: DirEntry<'a>) -> Result<Self, Self::Error> {
        if let DirEntry::File(file) = entry {
            Ok(file)
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a DirEntry<'a>> for &File<'a> {
    type Error = ();

    fn try_from(entry: &'a DirEntry<'a>) -> Result<Self, Self::Error> {
        if let DirEntry::File(file) = entry {
            Ok(file)
        } else {
            Err(())
        }
    }
}
