use std::path::{Path, PathBuf};

use files::{include_file, File};
use errors::*;


/// Traverse a file tree, building up an in-memory representation of it.
pub fn include_dir<P: AsRef<Path>>(root: P) -> Result<Dir> {
    let full_name = PathBuf::from(root.as_ref());
    let name = match full_name.file_name().and_then(|s| s.to_str()) {
        Some(s) => s.to_string(),
        None => bail!("Directory name is invalid"),
    };

    if !full_name.is_dir() {
        bail!("{} is not a directory", full_name.display());
    }


    let files = files_in_dir(&full_name)?;
    let subdirs = dirs_in_dir(&full_name)?;

    Ok(Dir {
           name,
           subdirs,
           files,
       })
}

fn dirs_in_dir<P: AsRef<Path>>(root: P) -> Result<Vec<Dir>> {
    root.as_ref()
        .read_dir()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| p.is_dir())
        .map(include_dir)
        .collect()
}

fn files_in_dir<P: AsRef<Path>>(root: P) -> Result<Vec<File>> {
    root.as_ref()
        .read_dir()?
        .filter_map(|dir_entry| dir_entry.ok())
        .map(|dir_entry| dir_entry.path())
        .filter(|p| p.is_file())
        .map(include_file)
        .collect()
}


/// Representation of a directory in memory.
#[derive(Debug, Clone, PartialEq)]
pub struct Dir {
    name: String,
    files: Vec<File>,
    subdirs: Vec<Dir>,
}


impl Dir {
    /// Get the directory's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The files in this directory.
    pub fn files(&self) -> &[File] {
        &self.files
    }

    /// The subdirectories in this directory.
    pub fn subdirs(&self) -> &[Dir] {
        &self.subdirs
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tempdir::TempDir;
    use std::fs;
    use std::io::Write;

    #[test]
    fn dir_only_works_on_directories() {
        let f = NamedTempFile::new().unwrap();

        assert!(include_dir(f.path()).is_err());
    }

    #[test]
    fn get_files_in_dir() {
        let temp = TempDir::new("temp").unwrap();
        let temp_path = temp.path();

        fs::File::create(temp_path.join("file_1.txt"))
            .unwrap()
            .write("File 1".as_bytes())
            .unwrap();

        let got_files = files_in_dir(&temp_path).unwrap();

        assert_eq!(got_files.len(), 1);
        assert_eq!(got_files[0].name(), "file_1.txt");
    }

    #[test]
    fn get_sub_directories() {
        let root = TempDir::new("temp").unwrap();
        let _child = TempDir::new_in(root.path(), "child").unwrap();

        let dir = include_dir(root.path()).unwrap();

        assert_eq!(dir.files.len(), 0);
        assert_eq!(dir.subdirs.len(), 1);

        assert_eq!(dir.subdirs[0].subdirs.len(), 0);
        assert_eq!(dir.subdirs[0].files.len(), 0);
    }
}
