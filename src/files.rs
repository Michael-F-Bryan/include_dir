use std::path::{Path, PathBuf};
use errors::*;
use helpers::Locatable;


/// Read a single file into memory.
pub fn include_file<P: AsRef<Path>>(filename: P) -> Result<File> {
    let name = PathBuf::from(filename.as_ref());

    if !name.is_file() {
        bail!("{} is not a file", name.display());
    }

    Ok(File { path: PathBuf::from(name) })
}


/// A basic representation of a file.
#[derive(PartialEq, Clone, Default, Debug)]
pub struct File {
    path: PathBuf,
}


impl File {
    /// Get the file's name.
    pub fn name(&self) -> &Path {
        &self.path
    }
}

impl Locatable for File {
    fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::NamedTempFile;

    fn dummy_file() -> NamedTempFile {
        let mut temp = NamedTempFile::new().unwrap();

        write!(temp, "Hello World!").unwrap();
        temp.seek(SeekFrom::Start(0)).unwrap();

        temp
    }

    #[test]
    fn new_file() {
        let f = dummy_file();

        let file = include_file(&f.path()).unwrap();

        assert_eq!(file.name(), f.path());
        f.close().unwrap();
    }

    #[test]
    fn file_only_works_on_files() {
        let path = "blah";

        assert!(include_file(path).is_err());
    }
}
