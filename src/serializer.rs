use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};

use files::File;
use dirs::Dir;
use errors::*;
use helpers::Locatable;

/// The object in charge of serializing `Files` and `Dirs` to some `io::Writer`.
#[derive(Debug)]
pub struct Serializer<W>
    where W: Write
{
    root: PathBuf,
    writer: BufWriter<W>,
}

impl<W> Serializer<W>
    where W: Write
{
    /// Create a new Serializer and write to the provided writer.
    pub fn new<P: AsRef<Path>>(root: P, writer: W) -> Serializer<W> {
        Serializer {
            root: PathBuf::from(root.as_ref()),
            writer: BufWriter::new(writer),
        }
    }

    fn write_file(&mut self, f: &File) -> Result<&mut Self> {
        write!(self.writer,
               r#"File {{ 
                path: r"{0}", 
                contents: include_bytes!(r"{0}"),
                }}"#,
               f.name().display())?;

        Ok(self)
    }

    pub fn dir_as_static(&mut self, name: &str, d: &Dir) -> Result<&mut Self> {
        write!(self.writer, "pub static {}: Dir = ", name)?;
        self.write_dir(d)?;
        writeln!(self.writer, ";")?;

        Ok(self)
    }

    fn write_dir(&mut self, d: &Dir) -> Result<&mut Self> {
        writeln!(self.writer,
               r#"Dir {{
    path: r"{}",
    files: &["#,
               d.path().relative_to(&self.root)?.display())?;

        for file in d.files() {
            self.write_file(file)?;
            writeln!(self.writer, ",")?;
        }

        writeln!(self.writer, r#"    ],
    subdirs: &["#)?;
        for dir in d.subdirs() {
            self.write_dir(dir)?;
            writeln!(self.writer, ",")?;
        }
        writeln!(self.writer, "    ]")?;
        write!(self.writer, "}}")?;

        Ok(self)
    }

    fn write_std_definitions(&mut self) -> Result<&mut Self> {
        self.writer.write(include_bytes!("serialized_std_definitions.rs"))?;
        Ok(self)
    }

    #[cfg(feature = "globs")]
    fn write_globs_definitions(&mut self) -> Result<&mut Self> {
        self.writer.write(include_bytes!("serialized_globs_definitions.rs"))?;        
        Ok(self)
    }

    #[cfg(not(feature = "globs"))]
    fn write_globs_definitions(&mut self) -> Result<&mut Self> {
        Ok(self)
    }

    pub fn write_definitions(&mut self) -> Result<&mut Self> {
        self.write_std_definitions()?
            .write_globs_definitions()?;

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
        ($( #[$attr:meta] )* $name:ident, |$ser:ident| $setup_serializer:expr) => (
            $(
                #[$attr]
            )*
            #[test]
            fn $name() {
                fn inner() -> Result<()> {
                    let mut buffer = Vec::new();
                    {
                        let mut $ser = Serializer::new("", &mut buffer);
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

    compile_and_test!(compile_all_definitions, |ser| ser.write_definitions()?);


    compile_and_test!(compile_a_dir_and_save_it_as_a_constant, |ser| {
        let temp = dummy_dir(false);

        let f = include_dir(temp.path())
            .chain_err(|| "Failed to load dummy dir")?;

        ser.dir_as_static("bar", &f)?.write_definitions()?;
    });
}
