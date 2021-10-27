use crate::file::File;
use std::fs;
use std::io::Write;
use std::path::Path;

/// A directory entry.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dir<'a> {
    #[doc(hidden)]
    pub path: &'a str,
    #[doc(hidden)]
    pub files: &'a [File<'a>],
    #[doc(hidden)]
    pub dirs: &'a [Dir<'a>],
}

/// Modes in which a directory entry can be extracted to disk.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExtractMode {
    /// Halts the extraction on file name collision.
    FailIfExists,
    /// Overwrites all colliding files.
    Overwrite,
}

impl<'a> Dir<'a> {
    /// Get the directory's path.
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    /// Get a list of the files in this directory.
    pub fn files(&self) -> &'a [File<'a>] {
        self.files
    }

    /// Get a list of the sub-directories inside this directory.
    pub fn dirs(&self) -> &'a [Dir<'a>] {
        self.dirs
    }

    /// Does this directory contain `path`?
    pub fn contains<S: AsRef<Path>>(&self, path: S) -> bool {
        let path = path.as_ref();

        self.get_file(path).is_some() || self.get_dir(path).is_some()
    }

    /// Fetch a sub-directory by *exactly* matching its path relative to the
    /// directory included with `include_dir!()`.
    pub fn get_dir<S: AsRef<Path>>(&self, path: S) -> Option<Dir<'a>> {
        let path = path.as_ref();

        for dir in self.dirs {
            if Path::new(dir.path) == path {
                return Some(*dir);
            }

            if let Some(d) = dir.get_dir(path) {
                return Some(d);
            }
        }

        None
    }

    /// Fetch a sub-directory by *exactly* matching its path relative to the
    /// directory included with `include_dir!()`.
    pub fn get_file<S: AsRef<Path>>(&self, path: S) -> Option<File<'a>> {
        let path = path.as_ref();

        for file in self.files {
            if Path::new(file.path) == path {
                return Some(*file);
            }
        }

        for dir in self.dirs {
            if let Some(d) = dir.get_file(path) {
                return Some(d);
            }
        }

        None
    }

    /// Create directories and extract all files to real filesystem.
    /// Creates parent directories of `path` if they do not already exist.
    /// In case of error, partially extracted directory may remain on the filesystem.
    pub fn extract<S: AsRef<Path>>(&self, path: S, mode: ExtractMode) -> std::io::Result<()> {
        // Extracts the given directory entry to the given path
        // We use this internally for recursing on subdirectories
        fn extract_dir<S: AsRef<Path>>(
            dir: Dir<'_>,
            path: S,
            mode: ExtractMode,
        ) -> std::io::Result<()> {
            let path = path.as_ref();
            // Create all the subdirectories in here (but not their files yet)
            for dir in dir.dirs() {
                fs::create_dir_all(path.join(dir.path()))?;
                extract_dir(*dir, path, mode)?;
            }

            // Only write files at the root of this directory (we recurse on subdirectories)
            for file in dir.files() {
                let mut fsf = fs::OpenOptions::new()
                    .write(true)
                    .create_new(mode == ExtractMode::FailIfExists)
                    .create(mode == ExtractMode::Overwrite)
                    .truncate(mode == ExtractMode::Overwrite)
                    .open(path.join(file.path()))?;
                fsf.write_all(file.contents())?;
                fsf.sync_all()?;
            }

            Ok(())
        }

        extract_dir(*self, path, mode)
    }
}
