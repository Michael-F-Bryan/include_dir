use std::time::{Duration, SystemTime};

use crate::File;

impl<'a> File<'a> {
    /// Set the [`Metadata`] associated with a [`File`].
    pub const fn with_metadata(self, metadata: Metadata) -> Self {
        let File { path, contents, .. } = self;

        File {
            path,
            contents,
            metadata: Some(metadata),
        }
    }

    /// Get the [`File`]'s [`Metadata`], if available.
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

/// Basic metadata for a file.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[doc(cfg(feature = "metadata"))]
pub struct Metadata {
    accessed: Duration,
    created: Duration,
    modified: Duration,
}

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
