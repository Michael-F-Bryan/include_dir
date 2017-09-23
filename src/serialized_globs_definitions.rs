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
}

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

impl Dir {
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
}
