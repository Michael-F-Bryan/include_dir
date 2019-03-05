use glob::Pattern;

use crate::dir::DirEntry;

#[derive(Debug, Clone, PartialEq)]
pub struct Globs<'a> {
    stack: Vec<DirEntry<'a>>,
    pattern: Pattern,
}

impl<'a> Globs<'a> {
    pub(crate) fn new(pattern: Pattern, item: DirEntry<'a>) -> Globs<'a> {
        let mut globs = Globs {
            stack: vec![],
            pattern,
        };

        globs.fill_buffer(item);

        globs
    }

    fn fill_buffer(&mut self, item: DirEntry<'a>) {
        if let DirEntry::Dir(dir) = item {
            self.stack.extend(dir.entries());
        }
    }
}

impl<'a> Iterator for Globs<'a> {
    type Item = DirEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            self.fill_buffer(item);

            if self.pattern.matches_path(item.path()) {
                return Some(item);
            }
        }

        None
    }
}
