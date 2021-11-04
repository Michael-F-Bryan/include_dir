#[cfg(feature = "metadata")]
use std::time::{Duration, SystemTime};
use std::{
    fmt::{self, Debug, Formatter},
    path::Path,
};

/// A file with its contents stored in a `&'static [u8]`.
#[derive(Clone, PartialEq, Eq)]
pub struct File<'a> {
    path: &'a str,
    contents: &'a [u8],
    #[cfg(feature = "metadata")]
    metadata: Option<Metadata>,
}

impl<'a> File<'a> {
    /// Create a new [`File`].
    pub const fn new(path: &'a str, contents: &'a [u8]) -> Self {
        File {
            path,
            contents,
            #[cfg(feature = "metadata")]
            metadata: None,
        }
    }

    /// The full path for this [`File`], relative to the directory passed to
    /// [`crate::include_dir!()`].
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    /// The file's raw contents.
    pub fn contents(&self) -> &[u8] {
        self.contents
    }

    /// The file's contents interpreted as a string.
    pub fn contents_utf8(&self) -> Option<&str> {
        std::str::from_utf8(self.contents()).ok()
    }

    /// Get a new [`File`] associated with the provided [`Metadata`].
    #[cfg(feature = "metadata")]
    pub const fn with_metadata(self, metadata: Metadata) -> Self {
        let File { path, contents, .. } = self;

        File {
            path,
            contents,
            metadata: Some(metadata),
        }
    }

    /// Get the [`File`]'s [`Metadata`], if available.
    #[cfg(feature = "metadata")]
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

impl<'a> Debug for File<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let File {
            path,
            contents,
            #[cfg(feature = "metadata")]
            metadata,
        } = self;

        let mut d = f.debug_struct("File");

        d.field("path", path)
            .field("contents", &format!("<{} bytes>", contents.len()));

        #[cfg(feature = "metadata")]
        d.field("metadata", metadata);

        d.finish()
    }
}

/// Basic metadata for a file.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg(feature = "metadata")]
pub struct Metadata {
    accessed: Duration,
    created: Duration,
    modified: Duration,
}

#[cfg(feature = "metadata")]
impl Metadata {
    /// Create a new [`Metadata`] using the number of seconds since the
    /// [`SystemTime::UNIX_EPOCH`].
    pub const fn new(accessed: Duration, created: Duration, modified: Duration) -> Self {
        Metadata {
            accessed,
            created,
            modified,
        }
    }

    /// Get the time this file was last accessed.
    ///
    /// See also: [`std::fs::Metadata::accessed()`].
    pub fn accessed(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + self.accessed
    }

    /// Get the time this file was created.
    ///
    /// See also: [`std::fs::Metadata::created()`].
    pub fn created(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + self.created
    }

    /// Get the time this file was last modified.
    ///
    /// See also: [`std::fs::Metadata::modified()`].
    pub fn modified(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + self.modified
    }
}
