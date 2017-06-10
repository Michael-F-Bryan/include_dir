use std::path::{Path, PathBuf};
use glob::Pattern;

use files::{include_file, File};
use helpers::Locatable;
use errors::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Options {
    ignore: Vec<Pattern>,
}

impl Options {
    pub fn new() -> Options {
        Options::default()
    }

    pub fn ignore(&mut self, name: &str) -> Result<&mut Self> {
        let pattern = Pattern::new(name)
            .chain_err(|| "Invalid glob pattern for ignore")?;

        self.ignore.push(pattern);
        Ok(self)
    }

    fn is_ignored<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        self.ignore.iter().any(|ignore| ignore.matches_path(path))
    }
}

pub fn include_dir_with_options<P: AsRef<Path>>(root: P, options: Options) -> Result<Dir> {
    let full_name = PathBuf::from(root.as_ref());

    if !full_name.is_dir() {
        bail!("{} is not a directory", full_name.display());
    }


    let files = files_in_dir(&full_name, &options)?;
    let subdirs = dirs_in_dir(&full_name, &options)?;

    Ok(Dir {
           path: full_name,
           subdirs: subdirs,
           files: files,
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
        .filter(|p| {
                    let relative_path = p.relative_to(root.as_ref()).unwrap();
                    !options.is_ignored(relative_path)
                })
        .inspect(|p| println!("{}", p.display()));

    dirs.map(include_dir).collect()
}

fn files_in_dir<P: AsRef<Path>>(root: P, options: &Options) -> Result<Vec<File>> {
    let file_paths = root.as_ref()
        .read_dir()?
        .filter_map(|dir_entry| dir_entry.ok())
        .map(|dir_entry| dir_entry.path())
        .filter(|p| p.is_file());

    file_paths
        .map(include_file)
        .filter(|f| match *f {
                    Ok(ref f) => {
                        let relative_path = f.relative_to(root.as_ref()).unwrap();
                        !options.is_ignored(relative_path)
                    } 
                    _ => true,
                })
        .collect()
}


/// Representation of a directory in memory.
#[derive(Debug, Clone, PartialEq)]
pub struct Dir {
    path: PathBuf,
    files: Vec<File>,
    subdirs: Vec<Dir>,
}


impl Dir {
    /// Get the directory's name
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
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

impl Locatable for Dir {
    fn path(&self) -> &Path {
        &self.path
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
        assert_eq!(got_files[0].relative_to(&temp_path).unwrap(),
                   PathBuf::from("file_1.txt"));
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
        options.ignore("lib.rs").unwrap();

        let src_directory = concat!(env!("CARGO_MANIFEST_DIR"), "/src/");
        let files = files_in_dir(&src_directory, &options).unwrap();

        let files_contains_lib = files
            .iter()
            .any(|f| f.name().to_str().unwrap().contains("lib.rs"));
        assert!(!files_contains_lib);
    }

    #[test]
    fn ignore_dirs() {
        let mut options = Options::new();
        options.ignore(".git").unwrap().ignore("target").unwrap();

        let src_directory = env!("CARGO_MANIFEST_DIR");
        let dirs = dirs_in_dir(&src_directory, &options).unwrap();

        let dirs_contains_git_or_target = dirs.iter()
            .any(|d| d.name() == ".git" || d.name() == "target");
        assert!(!dirs_contains_git_or_target);
    }
}
