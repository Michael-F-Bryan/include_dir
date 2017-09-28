pub static SRC: Dir = Dir {
    path: r"",
    files: &[
File { 
    path: r"globs.rs", 
    contents: include_bytes!(r"/home/michael/Documents/include_dir/integration_tests/globs.rs"), 
},
File { 
    path: r"walk.rs", 
    contents: include_bytes!(r"/home/michael/Documents/include_dir/integration_tests/walk.rs"), 
},
File { 
    path: r"basic_file_access.rs", 
    contents: include_bytes!(r"/home/michael/Documents/include_dir/integration_tests/basic_file_access.rs"), 
},
File { 
    path: r"find_file.rs", 
    contents: include_bytes!(r"/home/michael/Documents/include_dir/integration_tests/find_file.rs"), 
},
    ],
    subdirs: &[
    ]
};

/// A single static asset.
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct File {
    pub path: &'static str,
    pub contents: &'static [u8],
}

impl File {
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
}

/// A directory embedded as a static asset.
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Dir {
    pub path: &'static str,
    pub files: &'static [File],
    pub subdirs: &'static [Dir],
}

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
    pub fn walk<'a>(&'a self) -> DirWalker<'a> {
        DirWalker::new(self)
    }

    /// Get the directory's name.
    pub fn name(&self) -> &str {
        self.path()
            .file_name()
            .map(|s| s.to_str().unwrap())
            .unwrap_or("")
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
}

/// A directory walker.
///
/// `DirWalker` is an iterator which will recursively traverse
/// the embedded directory, allowing you to inspect each item.
/// It is largely modelled on the API used by the `walkdir`
/// crate.
///
/// You probably won't create one of these directly, instead
/// prefer to use the `Dir::walk()` method.
#[derive(Debug, PartialEq, Clone)]
pub struct DirWalker<'a> {
    root: &'a Dir,
    entries_to_visit: ::std::collections::VecDeque<DirEntry<'a>>,
}

impl<'a> DirWalker<'a> {
    fn new(root: &'a Dir) -> DirWalker<'a> {
        let mut walker = DirWalker {
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
}

impl<'a> Iterator for DirWalker<'a> {
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
}

/// A directory entry.
#[derive(Debug, PartialEq, Clone)]
pub enum DirEntry<'a> {
    Dir(&'a Dir),
    File(&'a File),
}

impl<'a> DirEntry<'a> {
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
}
