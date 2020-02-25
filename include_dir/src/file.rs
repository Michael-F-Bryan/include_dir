use std::fmt::{self, Debug, Formatter};
use std::path::Path;
use std::str;
use std::ffi::OsStr;

/// A file with its contents stored in a `&'static [u8]`.
#[derive(Copy, Clone, PartialEq)]
pub struct File<'a> {
    path: &'a str,
    file_name: &'a str,
    contents: &'a [u8],
}

impl<'a> File<'a> {
    /// FIXME: Docstring
    pub const fn new(path: &'a str, file_name: &'a str, contents: &'a [u8]) -> Self {
        Self {
            path,
            file_name,
            contents,
        }
    }
    /// The file's raw contents.
    pub fn contents(&self) -> &'_ [u8] {
        self.contents
    }

    /// The file's contents interpreted as a string.
    pub fn contents_utf8(&self) -> Option<&'_ str> {
        str::from_utf8(self.contents()).ok()
    }

    /// Returns the File's path relative to the directory included with `include_dir!()`.
    pub fn path(&self) -> &'_ Path {
        Path::new(self.path)
    }

    /// Returns the final component of the [`Path`] of the file
    pub fn file_name(&self) -> &'_ OsStr {
        OsStr::new(self.file_name)
    }
}

impl<'a> Debug for File<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
            .field("path", &self.path)
            .field("contents", &format!("<{} bytes>", self.contents.len()))
            .finish()
    }
}
