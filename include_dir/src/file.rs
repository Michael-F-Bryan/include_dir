use std::fmt::{self, Debug, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct File {
    pub path: &'static str,
    pub contents: &'static [u8],
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("File")
            .field("path", &self.path)
            .field("contents", &format!("<{} bytes>", self.contents.len()))
            .finish()
    }
}
