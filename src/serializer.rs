use std::io::Write;

use files::File;
use dirs::Dir;
use errors::*;


/// The object in charge of serializing `Files` and `Dirs` to some `io::Writer`.
#[derive(Debug)]
pub struct Serializer<W>
    where W: Write
{
    writer: W,
}

impl<W> Serializer<W>
    where W: Write
{
    /// Create a new Serializer and write to the provided writer.
    pub fn new(writer: W) -> Serializer<W> {
        Serializer { writer }
    }

    fn write_file(&mut self, f: &File) -> Result<&mut Self> {
        write!(self.writer,
               r#"File {{ name: "{}", contents: &{:?} }}"#,
               f.name(),
               f.contents())?;

        Ok(self)
    }

    pub fn dir_as_static(&mut self, name: &str, d: &Dir) -> Result<&mut Self> {
        write!(self.writer, "pub static {}: Dir = ", name)?;
        self.write_dir(d)?;
        writeln!(self.writer, ";")?;

        Ok(self)
    }

    fn write_dir(&mut self, d: &Dir) -> Result<&mut Self> {
        write!(self.writer, r#"Dir {{ name: "{}", files: &["#, d.name())?;

        for file in d.files() {
            self.write_file(file)?;
            writeln!(self.writer, ",")?;
        }

        write!(self.writer, "], subdirs: &[")?;
        for dir in d.subdirs() {
            self.write_dir(dir)?;
            writeln!(self.writer, ",")?;
        }
        write!(self.writer, "] }}")?;

        Ok(self)
    }

    fn write_file_definition(&mut self) -> Result<&mut Self> {
        writeln!(self.writer, "/// A static asset")?;

        writeln!(self.writer, "#[derive(Clone, Debug, Hash, PartialEq)]")?;
        writeln!(self.writer,
                 "pub struct File {{
                    pub name: &'static str,
                    pub contents: &'static [u8]
                }}")?;

        // writeln!(self.writer, "{}",
        //          "impl File {
        //              fn as_string(&self) -> Result<
        //          }")?;

        Ok(self)
    }

    fn write_dir_definition(&mut self) -> Result<&mut Self> {
        // docs
        writeln!(self.writer, "/// A directory embedded as a static asset.")?;

        // struct definition
        writeln!(self.writer, "#[derive(Clone, Debug, Hash, PartialEq)]")?;
        writeln!(self.writer,
                 "pub struct Dir {{
                    pub name: &'static str,
                    pub files: &'static [File],
                    pub subdirs: &'static [Dir]
                  }}")?;

        // method impls
        writeln!(self.writer,
                 "{}",
                 r#"
        impl Dir {
            /// Find a file which *exactly* matches the provided name.
            pub fn find(&'static self, name: &str) -> Option<&'static File> {
                for file in self.files {
                    if file.name == name {
                        return Some(file);
                    }
                }

                for dir in self.subdirs {
                    if let Some(f) = dir.find(name) {
                        return Some(f);
                    }
                }

                None
            }

            pub fn walk<'a>(&'a self) -> DirWalker<'a>
            {
                DirWalker::new(self)
            }
        }"#)?;

        Ok(self)
    }

    fn write_direntry_definition(&mut self) -> Result<&mut Self> {
        writeln!(self.writer, "#[derive(Debug, PartialEq, Clone)]")?;
        writeln!(self.writer,
                 "{}",
                 "pub enum DirEntry<'a> {
                     Dir(&'a Dir),
                     File(&'a File),
                 }")?;

        Ok(self)
    }

    fn write_dirwalker_definition(&mut self) -> Result<&mut Self> {
        writeln!(self.writer, "#[derive(Debug, PartialEq, Clone)]")?;
        writeln!(self.writer,
                 "{}",
                 "pub struct DirWalker<'a> {
                     root: &'a Dir,
                     entries_to_visit: ::std::collections::VecDeque<DirEntry<'a>>,
                 }")?;

        writeln!(self.writer,
                 "{}",
                 "impl<'a> DirWalker<'a> {
                     fn new(root: &'a Dir) -> DirWalker<'a> {
                         let mut walker = DirWalker{
                            root: root,
                            entries_to_visit: ::std::collections::VecDeque::new(),
                         };
                         walker.extend_contents(root);
                         walker
                     }

                     fn extend_contents(&mut self, from: &Dir) {
                         for file in from.files {
                             self.entries_to_visit.push_back(DirEntry::File(file));
                         }

                         for dir in from.subdirs {
                             self.entries_to_visit.push_back(DirEntry::Dir(dir));
                         }
                     }
                 }")?;

        Ok(self)
    }

    pub fn write_definitions(&mut self) -> Result<&mut Self> {
        self.write_file_definition()?
            .write_dir_definition()?
            .write_dirwalker_definition()?
            .write_direntry_definition()?;

        Ok(self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Output};
    use std::fs;
    use std::path::Path;
    use tempdir::TempDir;
    use tempfile::NamedTempFile;
    use dirs::include_dir;

    macro_rules! compile_and_test {
        ($name:ident, |$ser:ident| $setup_serializer:expr) => (
            #[test]
            fn $name() {
                fn inner() -> Result<()> {
                    let mut buffer = Vec::new();
                    {
                        let mut $ser = Serializer::new(&mut buffer);
                        $setup_serializer;
                    }

                    let src = String::from_utf8(buffer).unwrap();

                    let (got, _dir) = compile(&src).unwrap();

                    println!("{}", src);
                    println!();
                    println!();
                    println!();

                    println!("{}", String::from_utf8(got.stdout).unwrap());
                    println!("{}", String::from_utf8(got.stderr).unwrap());
                    println!("Status: {:?}", got.status);
                    assert!(got.status.success());

                    ::std::thread::sleep(::std::time::Duration::from_millis(30));

                    Ok(())
                }

                inner().unwrap();
            }
        )
    }

    /// Tries to compile the given source code, adding a `main` function if
    /// necessary.
    fn compile(src: &str) -> Result<(Output, TempDir)> {
        let dir = TempDir::new("temp").unwrap();

        let main = dir.path().join("main.rs");

        let mut f = fs::File::create(&main)?;
        writeln!(f, "{}", src)?;

        if !src.contains("fn main()") {
            writeln!(f, "fn main() {{}}")?;
        }

        Command::new("rustc")
            .arg(format!("{}", main.display()))
            .arg("--out-dir")
            .arg(format!("{}", dir.path().display()))
            .output()
            .map_err(|e| e.into())
            .map(|o| (o, dir))
    }

    fn dummy_file(parent: &Path) -> NamedTempFile {
        let mut temp = NamedTempFile::new_in(parent).unwrap();
        write!(temp, "Hello World!").unwrap();

        temp
    }

    fn dummy_dir(with_children: bool) -> TempDir {
        let root = TempDir::new("temp").unwrap();

        if with_children {
            dummy_file(root.path());

            TempDir::new_in(root.path(), "child").unwrap();
        }

        root
    }

    #[test]
    fn serialize_file_definition() {
        let mut writer = Vec::new();

        {
            let mut serializer = Serializer::new(&mut writer);
            serializer.write_file_definition().unwrap();
        }

        let got = String::from_utf8(writer).unwrap();
        assert!(got.contains("pub struct File {"));
    }

    #[test]
    fn serialize_dir_definition() {
        let mut writer = Vec::new();

        {
            let mut serializer = Serializer::new(&mut writer);
            serializer.write_dir_definition().unwrap();
        }

        let got = String::from_utf8(writer).unwrap();
        assert!(got.contains("pub struct Dir {"));
    }

    compile_and_test!(compile_file_definition, |ser| ser.write_file_definition()?);

    compile_and_test!(compile_dir_definition, |ser| ser.write_definitions()?);


    compile_and_test!(compile_a_dir_and_save_it_as_a_constant, |ser| {
        let temp = dummy_dir(false);

        let f = include_dir(temp.path())
            .chain_err(|| "Failed to load dummy dir")?;

        ser.dir_as_static("bar", &f)?.write_definitions()?;
    });
}
