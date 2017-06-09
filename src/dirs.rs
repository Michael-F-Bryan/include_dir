use std::path::{Path, PathBuf};

use files::{include_file, File};
use errors::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Options {
    ignore: Vec<String>,
}

impl Options {
    pub fn new() -> Options {
        Options::default()
    }

    pub fn ignore(&mut self, name: &str) -> &mut Self {
        self.ignore.push(name.to_string());
        self
    }

    fn is_ignored<P: AsRef<Path>>(&self, path: P) -> bool {
        for ignore in &self.ignore {
            println!("Checking if {} is ignored by {}",
                     path.as_ref().display(),
                     ignore);
            if path.as_ref() == PathBuf::from(ignore) {
                return true;
            }
        }

        false
    }
}

pub fn include_dir_with_options<P: AsRef<Path>>(root: P, options: Options) -> Result<Dir> {
    let full_name = PathBuf::from(root.as_ref());
    let name = match full_name.file_name().and_then(|s| s.to_str()) {
        Some(s) => s.to_string(),
        None => bail!("Directory name is invalid"),
    };

    if !full_name.is_dir() {
        bail!("{} is not a directory", full_name.display());
    }


    let files = files_in_dir(&full_name, &options)?;
    let subdirs = dirs_in_dir(&full_name, &options)?;

    Ok(Dir {
           name,
           subdirs,
           files,
       })
}

/// Traverse a file tree, building up an in-memory representation of it.
pub fn include_dir<P: AsRef<Path>>(root: P) -> Result<Dir> {
    include_dir_with_options(root, Options::new())
}

fn dirs_in_dir<P: AsRef<Path>>(root: P, options: &Options) -> Result<Vec<Dir>> {
    let dirs = root.as_ref()
        .read_dir()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| p.is_dir())
        .inspect(|p| println!("{}", p.display()));

    dirs.filter(|p| {
                    let relative_path = p.strip_prefix(root.as_ref()).unwrap();
                    !options.is_ignored(relative_path)
                })
        .map(include_dir)
        .collect()
}

fn files_in_dir<P: AsRef<Path>>(root: P, options: &Options) -> Result<Vec<File>> {
    let file_paths = root.as_ref()
        .read_dir()?
        .filter_map(|dir_entry| dir_entry.ok())
        .map(|dir_entry| dir_entry.path())
        .filter(|p| p.is_file());

    file_paths
        .filter(|p| {
                    let relative_path = p.strip_prefix(root.as_ref()).unwrap();
                    !options.is_ignored(relative_path)
                })
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

        let got_files = files_in_dir(&temp_path, &Options::new()).unwrap();

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

    #[test]
    fn ignore_files() {
        let mut options = Options::new();
        options.ignore("lib.rs");

        let src_directory = concat!(env!("CARGO_MANIFEST_DIR"), "/src/");
        let files = files_in_dir(&src_directory, &options).unwrap();

        assert!(files.iter().all(|f| f.name() != "libs.rs"));
    }

    #[test]
    fn ignore_dirs() {
        let mut options = Options::new();
        options.ignore(".git").ignore("target");

        let src_directory = env!("CARGO_MANIFEST_DIR");
        let dirs = dirs_in_dir(&src_directory, &options).unwrap();

        assert!(dirs.iter().all(|d| d.name != ".git" && d.name != "target"));
    }
}
