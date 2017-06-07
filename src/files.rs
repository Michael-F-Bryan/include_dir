use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Write, Read};

use errors::*;


#[derive(PartialEq, Clone, Default, Debug)]
pub struct File {
    name: String,
    contents: Vec<u8>,
}


impl File {
    fn definition() -> &'static str {
        "pub struct File { name: &'static str, contents: &'static [u8], }"
    }

    pub fn new<P: AsRef<Path>>(filename: P) -> Result<File> {
        let full_name = PathBuf::from(filename.as_ref());
        let name = match full_name.file_name().and_then(|s| s.to_str()) {
            Some(ref s) => s.to_string(),
            None => bail!("Filename is invalid"),
        };

        if !full_name.is_file() {
            bail!("{} is not a file", full_name.display());
        }

        let mut contents = Vec::new();
        fs::File::open(&full_name)?.read_to_end(&mut contents)?;

        Ok(File { name, contents })
    }

    /// Writes a representation of the `File` to some writer.
    ///
    /// This representation **must** be valid Rust code and result in an
    /// identical version to the original!
    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<()> {
        writeln!(writer, "File {{")?;
        writeln!(writer, "    name: {:?},", self.name)?;
        writeln!(writer, "    contents: &{:?},", self.contents)?;
        writeln!(writer, "}}")?;
        Ok(())
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
    use std::io::{Seek, SeekFrom, Read};
    use std::process::{Command, Output};
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
    fn check_static_representation() {
        let (path, mut f) = dummy_file();

        let should_be = format!(r#"File {{
    name: "{}",
    contents: &{:?},
}}
"#,
                                path.file_name().and_then(|s| s.to_str()).unwrap(),
                                "Hello World!".as_bytes());

        let file = File::new(&path).unwrap();

        let mut buffer = Vec::new();
        file.write_to(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn file_only_works_on_files() {
        let t = TempDir::new("blah").unwrap();

        assert!(File::new(t.path()).is_err());
    }

    #[test]
    #[ignore]
    fn make_sure_file_compiles() {
        let (path, _f) = dummy_file();
        let file = File::new(&path).unwrap();

        let output_dir = TempDir::new("temp").unwrap();
        let mut temp_output = fs::File::create(output_dir.path().join("main.rs")).unwrap();

        writeln!(temp_output, "{}", File::definition());
        write!(temp_output, "const FILE: File = ");
        file.write_to(&mut temp_output).unwrap();
        write!(temp_output, ";");

        let output = compile(output_dir.path().join("main.rs")).unwrap();
        println!("{:?}", output);

        let mut buffer = Vec::new();
        temp_output.seek(SeekFrom::Start(0)).unwrap();
        temp_output.read_to_end(&mut buffer).unwrap();
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
