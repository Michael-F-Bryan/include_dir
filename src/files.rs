use std::path::{Path, PathBuf};
use std::fs;
use std::io::Read;

use errors::*;


#[derive(PartialEq, Clone, Default, Debug)]
pub struct File {
    name: String,
    contents: Vec<u8>,
}


impl File {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<File> {
        let full_name = PathBuf::from(filename.as_ref());
        let name = match full_name.file_name().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => bail!("Filename is invalid"),
        };

        if !full_name.is_file() {
            bail!("{} is not a file", full_name.display());
        }

        let mut contents = Vec::new();
        fs::File::open(&full_name)?.read_to_end(&mut contents)?;

        Ok(File { name, contents })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn contents(&self) -> &[u8] {
        &self.contents
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

        let file = File::new(&path).unwrap();

        let mut file_contents = Vec::new();
        f.read_to_end(&mut file_contents).unwrap();

        assert_eq!(file.contents, file_contents);
        assert_eq!(file.name,
                   path.file_name().and_then(|s| s.to_str()).unwrap());
    }

    #[test]
    fn file_only_works_on_files() {
        let t = TempDir::new("blah").unwrap();

        assert!(File::new(t.path()).is_err());
    }
}
