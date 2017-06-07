use std::path::{Path, PathBuf};

use files::File;
use errors::*;


pub struct Dir {
    name: String,
    files: Vec<File>,
    subdirs: Vec<Dir>,
}


impl Dir {
    pub fn new<P: AsRef<Path>>(dirname: P) -> Result<Dir> {
        let full_name = PathBuf::from(dirname.as_ref());
        let name = match full_name.file_name().and_then(|s| s.to_str()) {
            Some(ref s) => s.to_string(),
            None => bail!("Directory name is invalid"),
        };

        if !full_name.is_dir() {
            bail!("{} is not a directory", full_name.display());
        }

        let subdirs = Vec::new();
        let files = Vec::new();

        Ok(Dir {
               name,
               subdirs,
               files,
           })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn dir_only_works_on_directories() {
        let f = NamedTempFile::new().unwrap();

        assert!(Dir::new(f.path()).is_err());
    }
}
