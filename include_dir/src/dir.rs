use file::File;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dir {
    pub path: &'static str,
    pub files: &'static [File],
    pub dirs: &'static [Dir],
}
