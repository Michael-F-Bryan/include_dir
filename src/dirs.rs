use std::path::{Path, PathBuf};
use std::io::Write;

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
        .map(|p| include_dir(p))
        .collect()
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

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<()> {
        writeln!(writer, "Dir {{")?;
        writeln!(writer, r#"    name: "{}","#, self.name)?;

        write!(writer, "    files: vec![")?;
        for file in &self.files {
            file.write_to(writer)?;
        }
        writeln!(writer, "],")?;

        write!(writer, "    subdirs: vec![")?;
        for subdir in &self.subdirs {
            subdir.write_to(writer)?;
        }
        writeln!(writer, "],")?;

        writeln!(writer, "}}")?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tempdir::TempDir;
    use std::fs;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::process::{Command, Output};

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
        let child = TempDir::new_in(root.path(), "child").unwrap();

        let dir = include_dir(root.path()).unwrap();

        assert_eq!(dir.files.len(), 0);
        assert_eq!(dir.subdirs.len(), 1);

        assert_eq!(dir.subdirs[0].subdirs.len(), 0);
        assert_eq!(dir.subdirs[0].files.len(), 0);
    }

    #[test]
    fn write_empty_directory_to_string() {
        let temp = TempDir::new("temp").unwrap();
        let dir = include_dir(temp.path()).unwrap();

        let mut buffer = Vec::new();
        dir.write_to(&mut buffer).unwrap();

        let should_be = format!(r#"Dir {{
    name: "{}",
    files: vec![],
    subdirs: vec![],
}}
"#,
                                dir.name);

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn make_sure_dir_compiles() {
        let mut temp = NamedTempFile::new().unwrap();

        let dir = include_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap();

        dir.write_to(&mut temp).unwrap();

        let output = compile(temp.path()).unwrap();
        println!("{:?}", output);

        let mut buffer = Vec::new();
        temp.seek(SeekFrom::Start(0)).unwrap();
        temp.read_to_end(&mut buffer).unwrap();
        println!("{}", String::from_utf8(buffer).unwrap());
        panic!();
    }

    fn compile<P: AsRef<Path>>(s: P) -> Result<Output> {
        Command::new("rustc")
            .arg(s.as_ref().to_str().unwrap())
            .output()
            .map_err(|e| e.into())
    }

}
