use std::fmt::{self, Debug, Formatter};
use std::path::Path;
use std::str;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// A file with its contents stored in a `&'static [u8]`.
#[derive(Copy, Clone, PartialEq)]
pub struct File<'a> {
    #[doc(hidden)]
    pub path: &'a str,
    #[doc(hidden)]
    pub contents: &'a [u8],

    #[doc(hidden)]
    pub modified: Option<f64>,

    #[doc(hidden)]
    pub accessed: Option<f64>,

    #[doc(hidden)]
    pub created: Option<f64>,
}

impl<'a> File<'a> {
    /// The file's path, relative to the directory included with
    /// `include_dir!()`.
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    /// The file's raw contents.
    pub fn contents(&self) -> &'a [u8] {
        self.contents
    }

    /// The file's contents interpreted as a string.
    pub fn contents_utf8(&self) -> Option<&'a str> {
        str::from_utf8(self.contents()).ok()
    }

    /// The file's created timestamp as of compilation time, if available
    pub fn created(&self) -> Option<SystemTime> {
        self.created
            .map(|secs| UNIX_EPOCH + Duration::from_secs_f64(secs))
    }

    /// The file's last modified timestamp as of compilation time, if available
    pub fn modified(&self) -> Option<SystemTime> {
        self.modified
            .map(|secs| UNIX_EPOCH + Duration::from_secs_f64(secs))
    }

    /// The file's last accessed timestamp as of compilation time, if available
    pub fn accessed(&self) -> Option<SystemTime> {
        self.accessed
            .map(|secs| UNIX_EPOCH + Duration::from_secs_f64(secs))
    }
}

impl<'a> Debug for File<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
            .field("path", &self.path)
            .field("contents", &format!("<{} bytes>", self.contents.len()))
            .field("created", &self.created())
            .field("modified", &self.modified())
            .field("accessed", &self.accessed())
            .finish()
    }
}
