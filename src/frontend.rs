#![cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]

use std::path::{Path, PathBuf};
use std::fs::File;

use dirs::{self, Dir, Options};
use serializer::Serializer;
use errors::*;


/// A builder object to give the `include_dir()` function a nice fluent API.
///
/// # Note
///
/// It is pretty much assumed that you'll use it in the order described in
/// `include_dir`'s docs. Straying from that path will only lead to frustration.
#[derive(Debug, Default)]
pub struct IncludeDirBuilder {
    dir: Option<Dir>,
    root: Option<PathBuf>,
    variable: Option<String>,
    options: Options,
    err: Option<Error>,
}

/// The entire purpose of this crate.
///
/// This function actually creates a builder object which allows you to
/// configure the variable name, and where to save it.
///
/// # Examples
///
/// ```rust,no_run
/// use std::env;
/// use std::path::Path;
/// use include_dir::include_dir;
///
/// let outdir = env::var("OUT_DIR").unwrap();
/// let dest_path = Path::new(&outdir).join("assets.rs");
///
/// include_dir("src")
///   .as_variable("SRC")
///   .to_file(dest_path)
///   .unwrap();
/// ```
pub fn include_dir<P: AsRef<Path>>(dir_to_include: P) -> IncludeDirBuilder {
    let dir_to_include = dir_to_include.as_ref();
    let dir = dirs::include_dir(dir_to_include);

    let mut this = IncludeDirBuilder::default();
    this.root = Some(dir_to_include.to_path_buf());

    match dir {
        Ok(d) => this.dir = Some(d),
        Err(e) => this.err = Some(e),
    }

    this
}

impl IncludeDirBuilder {
    /// Set the variable name to save your assets under.
    pub fn as_variable(mut self, name: &str) -> Self {
        self.variable = Some(name.to_string());
        self
    }

    /// Ignore a directory or file.
    ///
    /// # Note
    ///
    /// For now, this will only match the path relative to the included
    /// directory.
    pub fn ignore<S: AsRef<str>>(mut self, name: S) -> Self {
        if let Err(e) = self.options.ignore(name.as_ref()) {
            self.err = Some(e);
        }
        self
    }

    /// Save the file tree to the provided path, performing all necessary
    /// error checking.
    pub fn to_file<P: AsRef<Path>>(self, path: P) -> Result<()> {
        if let Some(e) = self.err {
            return Err(e);
        }

        let variable_name = match self.variable {
            Some(v) => v,
            None => bail!("No variable name set (use the `as_variable` method)"),
        };

        let dir = match self.dir {
            Some(d) => d,
            None => bail!("No directory selected"),
        };

        let root = match self.root {
            Some(r) => r,
            None => bail!("No directory selected"),
        };

        let f = File::create(path).chain_err(|| "Unable to open file")?;
        let mut serializer = Serializer::new(root, f);

        serializer
            .dir_as_static(&variable_name, &dir)?
            .write_definitions()?;

        Ok(())
    }
}
