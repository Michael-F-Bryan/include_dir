use std::fmt::{self, Debug, Formatter};
use std::path::Path;
use std::str;

/// A file with its contents stored in a `&'static [u8]`.
#[derive(Copy, Clone, PartialEq)]
pub struct File<'a> {
    path: &'a str,
    contents: &'a [u8],
}

impl<'a> File<'a> {
    /// Create a new [`File`]
    pub const fn new(path: &'a str, contents: &'a [u8]) -> Self {
        Self {
            path,
            contents,
        }
    }
    /// The file's raw contents.
    pub fn contents(&self) -> &[u8] {
        self.contents
    }

    /// The file's contents interpreted as a string.
    pub fn contents_utf8(&self) -> Option<&str> {
        str::from_utf8(self.contents()).ok()
    }

    /// Returns the File's path relative to the directory included with `include_dir!()`.
    pub fn path(&self) -> &Path {
        Path::new(self.path)
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
