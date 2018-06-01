use file::File;
use std::path::Path;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dir {
    pub path: &'static str,
    pub files: &'static [File],
    pub dirs: &'static [Dir],
}

impl Dir {
    pub fn contains<S: AsRef<Path>>(&self, path: S) -> bool {
        let path = path.as_ref();

        self.get_file(path).is_some() || self.get_dir(path).is_some()
    }

    pub fn get_dir<S: AsRef<Path>>(&self, path: S) -> Option<Dir> {
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

    pub fn get_file<S: AsRef<Path>>(&self, path: S) -> Option<File> {
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
}
