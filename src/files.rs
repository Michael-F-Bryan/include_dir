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
            Some(ref s) => s.to_string(),
            None => bail!("Filename is invalid"),
        };

        let mut contents = Vec::new();
        fs::File::open(&full_name)?.read_to_end(&mut contents)?;

        Ok(File { name, contents })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::io::Read;

    fn this_binary() -> (PathBuf, fs::File) {
        let filename = env::args().next().unwrap();
        let f = fs::File::open(&filename).unwrap();

        (PathBuf::from(filename), f)
    }

    #[test]
    fn new_file() {
        let (this, mut f) = this_binary();

        let file = File::new(&this).unwrap();

        let mut file_contents = Vec::new();
        f.read_to_end(&mut file_contents).unwrap();

        assert_eq!(file.contents, &file_contents[..]);
        assert_eq!(file.name,
                   this.file_name().and_then(|s| s.to_str()).unwrap());
    }
}
