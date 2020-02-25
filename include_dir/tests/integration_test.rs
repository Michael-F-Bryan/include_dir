use include_dir::{include_dir, DirEntry};
use std::path::Path;

const PARENT_DIR: DirEntry<'_> = include_dir!(".");

#[test]
fn included_all_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("{:#?}", PARENT_DIR);

    validate_directory(&PARENT_DIR, root, root);
}

fn validate_directory(include_dir_entry: &DirEntry<'_>, path: &Path, root: &Path) {
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap().path();
        let entry = entry.strip_prefix(root).unwrap();

        let name = entry.file_name().unwrap();

        assert!(include_dir_entry.get(entry).is_some(), "Can't find {}", entry.display());

        if entry.is_dir() {
            let child_path = path.join(name);
            validate_directory(include_dir_entry, &child_path, root);
        }
    }
}
