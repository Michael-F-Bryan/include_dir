use std::io::{Write, Read, BufReader, BufWriter};
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
        // TODO: Use a buffered reader here for easy perf gains
        let contents = BufReader::new(f.contents()?);
        write!(self.writer,
               r#"File {{ path: "{}", contents: &["#,
               f.name().relative_to(&self.root)?.display())?;

        for byte in contents.bytes() {
            write!(self.writer, "{}, ", byte?)?;
        }

        writeln!(self.writer, "]")?;
        writeln!(self.writer, "}}")?;

        Ok(self)
    }

    pub fn dir_as_static(&mut self, name: &str, d: &Dir) -> Result<&mut Self> {
        write!(self.writer, "pub static {}: Dir = ", name)?;
        self.write_dir(d)?;
        writeln!(self.writer, ";")?;

        Ok(self)
    }

    fn write_dir(&mut self, d: &Dir) -> Result<&mut Self> {
        write!(self.writer,
               r#"Dir {{ path: "{}", files: &["#,
               d.path().relative_to(&self.root)?.display())?;

        for file in d.files() {
            self.write_file(file)?;
            writeln!(self.writer, ",")?;
        }

        write!(self.writer, "], subdirs: &[")?;
        for dir in d.subdirs() {
            self.write_dir(dir)?;
            writeln!(self.writer, ",")?;
        }
        writeln!(self.writer, "]")?;
        writeln!(self.writer, "}}")?;

        Ok(self)
    }

    fn write_file_definition(&mut self) -> Result<&mut Self> {
        writeln!(self.writer, "/// A single static asset.")?;

        writeln!(self.writer, "#[derive(Clone, Debug, Hash, PartialEq)]")?;
        writeln!(self.writer,
                 "pub struct File {{
                    pub path: &'static str,
                    pub contents: &'static [u8]
                }}")?;

        writeln!(self.writer,
                 "{}",
                 "impl File {
                     /// Get the file's path.
                     #[inline]
                     pub fn path(&self) -> &::std::path::Path {
                         self.path.as_ref()
                     }

                    /// The file's name (everything after the last slash).
                    pub fn name(&self) -> &str {
                        self.path().file_name().unwrap().to_str().unwrap()
                    }

                    /// Get a Reader over the file's contents.
                    pub fn as_reader(&self) -> ::std::io::Cursor<&[u8]> {
                        ::std::io::Cursor::new(&self.contents)
                    }

                    /// The total size of this file in bytes.
                    pub fn size(&self) -> usize {
                        self.contents.len()
                    }
                }")?;

        Ok(self)
    }

    fn write_dir_definition(&mut self) -> Result<&mut Self> {
        // docs
        writeln!(self.writer, "/// A directory embedded as a static asset.")?;

        // struct definition
        writeln!(self.writer, "#[derive(Clone, Debug, Hash, PartialEq)]")?;
        writeln!(self.writer,
                 "pub struct Dir {{
                    pub path: &'static str,
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
                    if file.name() == name {
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

            /// Recursively walk the various sub-directories and files inside
            /// the bundled asset.
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// for entry in ASSET.walk() {
            ///   match entry {
            ///     DirEntry::File(f) => println!("{} ({} bytes)",
            ///                                   f.path().display(),
            ///                                   f.contents.len()),
            ///     DirEntry::Dir(d) => println!("{} (files: {}, subdirs: {})",
            ///                                  d.path().display(),
            ///                                  d.files.len(),
            ///                                  d.subdirs.len()),
            ///   }
            /// }
            /// ```
            pub fn walk<'a>(&'a self) -> DirWalker<'a>
            {
                DirWalker::new(self)
            }

            /// Get the directory's name.
            pub fn name(&self) -> &str {
                self.path().file_name().map(|s| s.to_str().unwrap()).unwrap_or("")
            }

            /// The directory's full path relative to the root.
            pub fn path(&self) -> &::std::path::Path {
                self.path.as_ref()
            }

            /// Get the total size of this directory and its contents in bytes.
            pub fn size(&self) -> usize {
                let file_size = self.files.iter().map(|f| f.size()).sum();
                
                self.subdirs.iter().fold(file_size, |acc, d| acc + d.size())
            }
        }"#)?;

        Ok(self)
    }

    fn write_direntry_definition(&mut self) -> Result<&mut Self> {
        writeln!(self.writer, "{}", "/// A directory entry.")?;

        writeln!(self.writer, "#[derive(Debug, PartialEq, Clone)]")?;
        writeln!(self.writer,
                 "{}",
                 "pub enum DirEntry<'a> {
                     Dir(&'a Dir),
                     File(&'a File),
                 }")?;

        writeln!(self.writer,
                 "{}",
                 "impl<'a> DirEntry<'a> {
                     /// Get the entry's name.
                     pub fn name(&self) -> &str {
                         match *self {
                             DirEntry::Dir(d) => d.name(),
                             DirEntry::File(f) => f.name(),
                         }
                     }
                     
                     /// Get the entry's path relative to the root directory.
                     pub fn path(&self) -> &::std::path::Path {
                         match *self {
                             DirEntry::Dir(d) => d.path(),
                             DirEntry::File(f) => f.path(),
                         }
                     }
                 }")?;

        Ok(self)
    }

    fn write_dirwalker_definition(&mut self) -> Result<&mut Self> {
        writeln!(self.writer,
                 "{}",
                 "/// A directory walker.
                  ///
                  /// `DirWalker` is an iterator which will recursively traverse
                  /// the embedded directory, allowing you to inspect each item.
                  /// It is largely modelled on the API used by the `walkdir`
                  /// crate.
                  ///
                  /// You probably won't create one of these directly, instead
                  /// prefer to use the `Dir::walk()` method.")?;

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

        writeln!(self.writer,
                 "{}",
                 "impl<'a> Iterator for DirWalker<'a> {
                    type Item = DirEntry<'a>;

                    fn next(&mut self) -> Option<Self::Item> {
                        let entry = self.entries_to_visit.pop_front();

                        if let Some(DirEntry::Dir(d)) = entry {
                            self.extend_contents(d);
                            Some(DirEntry::Dir(d))
                        } else {
                            entry
                        }
                    }
                }")?;

        Ok(self)
    }

    #[cfg(feature = "globs")]
    fn write_globs(&mut self) -> Result<&mut Self> {
        writeln!(self.writer,
                 "{}",
                 "
                /// An iterator over all `DirEntries` which match the specified
                /// pattern.
                ///
                /// # Note
                /// 
                /// You probably don't want to use this directly. Instead, you'll
                /// want the [`Dir::glob()`] method.
                /// 
                /// [`Dir::glob()`]: struct.Dir.html#method.glob
                pub struct Globs<'a> {
                    walker: DirWalker<'a>,
                    pattern: ::glob::Pattern,
                }")?;
        writeln!(self.writer,
                 "{}",
                 r#"
                 impl<'a> Iterator for Globs<'a> {
                    type Item = DirEntry<'a>;
                    fn next(&mut self) -> Option<Self::Item> {
                        while let Some(entry) = self.walker.next() {
                            if self.pattern.matches_path(entry.path()) {
                                return Some(entry);
                            }
                        }

                        None
                    }
                 }
        "#)?;

        writeln!(self.writer,
                 "{}",
                 r#"impl Dir {
                    /// Find all `DirEntries` which match a glob pattern.
                    ///
                    /// # Note
                    /// 
                    /// This may fail if you pass in an invalid glob pattern,
                    /// consult the [glob docs] for more info on what a valid
                    /// pattern is.
                    ///
                    /// # Examples
                    ///
                    /// ```rust,ignore
                    /// use handlebars::Handlebars;
                    /// let mut handlebars = Handlebars::new();
                    ///
                    /// for entry in ASSETS.glob("*.hbs")? {
                    ///     if let DirEntry::File(f) = entry {
                    ///         let template_string = String::from_utf8(f.contents.to_vec())?;
                    ///         handlebars.register_template_string(f.name(),template_string)?;
                    ///     }
                    /// }
                    /// ```
                    /// 
                    /// [glob docs]: https://doc.rust-lang.org/glob/glob/struct.Pattern.html
                    pub fn glob<'a>(&'a self, pattern: &str) -> Result<Globs<'a>, Box<::std::error::Error>> {
                        let pattern = ::glob::Pattern::new(pattern)?;
                        Ok(Globs {
                            walker: self.walk(),
                            pattern: pattern,
                        })
                    }
            }"#)?;

        Ok(self)
    }

    #[cfg(not(feature = "globs"))]
    fn write_globs(&mut self) -> Result<&mut Self> {
        Ok(self)
    }

    pub fn write_definitions(&mut self) -> Result<&mut Self> {
        self.write_file_definition()?
            .write_dir_definition()?
            .write_dirwalker_definition()?
            .write_globs()?
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

    #[test]
    fn serialize_file_definition() {
        let mut writer = Vec::new();

        {
            let mut serializer = Serializer::new("", &mut writer);
            serializer.write_file_definition().unwrap();
        }

        let got = String::from_utf8(writer).unwrap();
        assert!(got.contains("pub struct File {"));
    }

    #[test]
    fn serialize_dir_definition() {
        let mut writer = Vec::new();

        {
            let mut serializer = Serializer::new("", &mut writer);
            serializer.write_dir_definition().unwrap();
        }

        let got = String::from_utf8(writer).unwrap();
        assert!(got.contains("pub struct Dir {"));
    }

    compile_and_test!(compile_file_definition, |ser| ser.write_file_definition()?);
    compile_and_test!(compile_all_definitions, |ser| ser.write_definitions()?);


    compile_and_test!(compile_a_dir_and_save_it_as_a_constant, |ser| {
        let temp = dummy_dir(false);

        let f = include_dir(temp.path())
            .chain_err(|| "Failed to load dummy dir")?;

        ser.dir_as_static("bar", &f)?.write_definitions()?;
    });
}
