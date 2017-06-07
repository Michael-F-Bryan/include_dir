use std::path::Path;
use std::fs::File;

use dirs::{self, Dir};
use serializer::Serializer;
use errors::*;


/// A builder object to give the `include_dir()` function a nice fluent API.
///
/// # Note
///
/// It is pretty much assumed that you'll use it in the order described in
/// include_dir's docs. Straying from that path will only lead to frustration.
#[derive(Debug)]
pub struct IncludeDirBuilder {
    dir: Option<Dir>,
    variable: Option<String>,
    err: Option<Error>,
}

/// Embed a file tree.
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
pub fn include_dir(root: &str) -> IncludeDirBuilder {
    let dir = dirs::include_dir(root);

    match dir {
        Ok(d) => {
            IncludeDirBuilder {
                dir: Some(d),
                variable: None,
                err: None,
            }
        }
        Err(e) => {
            IncludeDirBuilder {
                dir: None,
                variable: None,
                err: Some(e),
            }
        }
    }
}

impl IncludeDirBuilder {
    /// Set the variable name to save your assets under.
    pub fn as_variable(mut self, name: &str) -> Self {
        self.variable = Some(name.to_string());
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

        let f = File::create(path).chain_err(|| "Unable to open file")?;
        let mut serializer = Serializer::new(f);

        serializer
            .dir_as_const(&variable_name, &dir)?
            .write_file_definition()?
            .write_dir_definition()?;

        Ok(())
    }
}
