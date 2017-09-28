use std::path::{Path, PathBuf};

use errors::*;

pub trait Locatable {
    fn path(&self) -> &Path;

    fn relative_to<P: AsRef<Path>>(&self, to: P) -> Result<PathBuf> {
        self.path()
            .strip_prefix(to.as_ref())
            .map(|p| PathBuf::from(p))
            .chain_err(|| {
                format!(
                    "Unable to resolve relative path for {}",
                    self.path().display()
                )
            })
    }
}

impl<P> Locatable for P
where
    P: AsRef<Path>,
{
    fn path(&self) -> &Path {
        self.as_ref()
    }
}
