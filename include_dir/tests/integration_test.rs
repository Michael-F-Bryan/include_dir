use include_dir::{include_dir, Dir};
use std::path::Path;
use tempdir::TempDir;

const PARENT_DIR: Dir<'_> = include_dir!(".");

#[test]
fn included_all_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("{:#?}", PARENT_DIR);

    validate_directory(PARENT_DIR, root, root);
}

#[test]
fn extract_all_files() {
    let tmpdir = TempDir::new(
        format!(
            "{}-{}-test",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
        .as_str(),
    )
    .unwrap();
    let root = tmpdir.path();
    PARENT_DIR.extract(root).unwrap();

    validate_directory(PARENT_DIR, root, root);
}

fn validate_directory(dir: Dir<'_>, path: &Path, root: &Path) {
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap().path();
        let entry = entry.strip_prefix(root).unwrap();

        let name = entry.file_name().unwrap();

        assert!(dir.contains(entry), "Can't find {}", entry.display());

        if entry.is_dir() {
            let child_path = path.join(name);
            validate_directory(dir.get_dir(entry).unwrap(), &child_path, root);
        }
    }
}
