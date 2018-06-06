use std::fmt::{self, Debug, Formatter};
use std::path::Path;
use std::str;

#[derive(Copy, Clone, PartialEq)]
pub struct File<'a> {
    #[doc(hidden)]
    pub path: &'a str,
    #[doc(hidden)]
    pub contents: &'a [u8],
}

impl<'a> File<'a> {
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    pub fn contents(&self) -> &'a [u8] {
        self.contents
    }

    pub fn contents_utf8(&self) -> Option<&'a str> {
        str::from_utf8(self.contents()).ok()
    }
}

impl<'a> Debug for File<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("File")
            .field("path", &self.path)
            .field("contents", &format!("<{} bytes>", self.contents.len()))
            .finish()
    }
}
