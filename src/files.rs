use std::path::{Path, PathBuf};
use std::fs;
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

    /// The file's contents.
    pub fn contents(&self) -> Result<fs::File> {
        fs::File::open(&self.path).map_err(|e| e.into())
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
    use std::io::{Seek, SeekFrom, Write, Read};
    use tempfile::NamedTempFile;
    use tempdir::TempDir;

    fn dummy_file() -> (PathBuf, NamedTempFile) {
        let mut temp = NamedTempFile::new().unwrap();

        write!(temp, "Hello World!").unwrap();
        temp.seek(SeekFrom::Start(0)).unwrap();

        (PathBuf::from(temp.path()), temp)
    }

    #[test]
    fn new_file() {
        let (path, mut f) = dummy_file();

        let file = include_file(&path).unwrap();

        let mut file_contents = Vec::new();
        f.read_to_end(&mut file_contents).unwrap();

        let mut got = Vec::new();
        file.contents().unwrap().read_to_end(&mut got).unwrap();
        assert_eq!(got, file_contents);
        assert_eq!(file.name(), path);
    }

    #[test]
    fn file_only_works_on_files() {
        let t = TempDir::new("blah").unwrap();

        assert!(include_file(t.path()).is_err());
    }
}
