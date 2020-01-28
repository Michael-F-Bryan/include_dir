use crate::direntry::DirEntry;
use glob::{Pattern, PatternError};

#[derive(Debug, Clone, PartialEq)]
pub struct Globs<'a> {
    stack: Vec<&'a DirEntry<'a>>,
    pattern: Pattern,
}

impl DirEntry<'_> {
    /// Search for a file or directory with a glob pattern.
    pub fn find(&self, glob: &str) -> Result<impl Iterator<Item = &'_ DirEntry<'_>>, PatternError> {
        let pattern = Pattern::new(glob)?;

        Ok(Globs::new(pattern, self))
    }
}

impl<'a> Globs<'a> {
    pub(crate) fn new(pattern: Pattern, root: &'a DirEntry<'a>) -> Globs<'a> {
        let stack = vec![root];
        Globs { stack, pattern }
    }

    fn fill_buffer(&mut self, item: &'a DirEntry<'a>) {
        if let DirEntry::Dir(dir) = item {
            self.stack.extend(dir.entries());
        }
    }
}

impl<'a> Iterator for Globs<'a> {
    type Item = &'a DirEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            self.fill_buffer(&item);

            if self.pattern.matches_path(item.path()) {
                return Some(item);
            }
        }
        None
    }
}
