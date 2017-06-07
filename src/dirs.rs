use std::path::{Path, PathBuf};

use files::File;
use errors::*;


pub fn include_dir<P: AsRef<Path>>(root: P) -> Result<Dir> {
    let full_name = PathBuf::from(root.as_ref());
    let name = match full_name.file_name().and_then(|s| s.to_str()) {
        Some(ref s) => s.to_string(),
        None => bail!("Directory name is invalid"),
    };

    if !full_name.is_dir() {
        bail!("{} is not a directory", full_name.display());
    }

    let subdirs = Vec::new();
    let files = files_in_dir(&full_name)?;

    Ok(Dir {
           name,
           subdirs,
           files,
       })
}

fn files_in_dir<P: AsRef<Path>>(root: P) -> Result<Vec<File>> {
    root.as_ref()
        .read_dir()?
        .filter_map(|dir_entry| dir_entry.ok())
        .map(|dir_entry| dir_entry.path())
        .filter(|p| p.is_file())
        .map(|p| File::new(p))
        .collect()
}


pub struct Dir {
    name: String,
    files: Vec<File>,
    subdirs: Vec<Dir>,
}


impl Dir {
    fn new<S: AsRef<str>>(dirname: S) -> Dir {
        Dir {
            name: dirname.as_ref().to_string(),
            files: Vec::new(),
            subdirs: Vec::new(),
        }
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
}
