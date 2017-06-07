extern crate walkdir;

use walkdir::WalkDir;

use std::path::{Path, PathBuf};

#[derive(PartialEq, Clone, Default, Debug)]
struct File {
    contents: Vec<u8>,
    name: String,
}

impl File {
    pub fn new<P: AsRef<Path>>(filename: P) -> File {
        unimplemented!()
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
struct Dir {
    root: PathBuf,
    files: Vec<File>,
    subdirs: Vec<Dir>,
}

impl Dir {
    pub fn new<P: Into<PathBuf>>(root: P) -> Dir {
        construct(root)
    }
}


fn construct<P: Into<PathBuf>>(root: P) -> Dir {
    Dir::default()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::io::Read;

    fn this_binary() -> (String, fs::File) {
        let filename = env::args().next().unwrap();
        let f = fs::File::open(&filename).unwrap();

        (filename, f)
    }

    #[test]
    fn new_file() {
        let (this, mut f) = this_binary();

        let file = File::new(this);

        let mut file_contents = Vec::new();
        f.read_to_end(&mut file_contents).unwrap();

        assert_eq!(file.contents, file_contents);
    }
}
