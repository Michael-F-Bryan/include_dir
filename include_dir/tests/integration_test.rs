#[macro_use]
extern crate include_dir;

use include_dir::FileSystem;
use std::path::Path;

const FS: FileSystem = include_dir!(".");

#[test]
fn included_all_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("{:#?}", FS);

    validate_directory(&FS, root, root);
}

fn validate_directory(fs: &FileSystem, path: &Path, root: &Path) {
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap().path();
        let entry = entry.strip_prefix(root).unwrap();

        let name = entry.file_name().unwrap();

        assert!(fs.contains(entry), "Can't find {}", entry.display());

        if entry.is_dir() {
            let child_path = path.join(name);
            validate_directory(fs, &child_path, root);
        }
    }
}
