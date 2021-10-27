use include_dir::{dir::ExtractMode, include_dir, Dir};
use std::path::Path;
use tempdir::TempDir;

const PARENT_DIR: Dir<'_> = include_dir!(".");

#[test]
fn included_all_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("{:#?}", PARENT_DIR);

    validate_included(PARENT_DIR, root, root);
}

fn tempdir() -> Result<TempDir, std::io::Error> {
    TempDir::new(
        format!(
            "{}-{}-test",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
        .as_str(),
    )
}

#[test]
fn extract_all_files_fail() {
    let tmpdir = tempdir().unwrap();
    let root = tmpdir.path();
    PARENT_DIR.extract(root, ExtractMode::FailIfExists).unwrap();
    validate_extracted(PARENT_DIR, root);

    assert!(PARENT_DIR.extract(root, ExtractMode::FailIfExists).is_err());
}

#[test]
fn extract_all_files_overwrite() {
    let tmpdir = tempdir().unwrap();
    let root = tmpdir.path();

    PARENT_DIR.extract(root, ExtractMode::Overwrite).unwrap();
    validate_extracted(PARENT_DIR, root);

    PARENT_DIR.extract(root, ExtractMode::Overwrite).unwrap();
    validate_extracted(PARENT_DIR, root);
}

// Validates that all files on the filesystem exist in the inclusion
fn validate_included(dir: Dir<'_>, path: &Path, root: &Path) {
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap().path();
        let entry = entry.strip_prefix(root).unwrap();

        let name = entry.file_name().unwrap();

        assert!(dir.contains(entry), "Can't find {}", entry.display());

        if entry.is_dir() {
            let child_path = path.join(name);
            validate_included(dir.get_dir(entry).unwrap(), &child_path, root);
        }
    }
}

// Validates that all files in the inclusion were extracted to the filesystem
fn validate_extracted(dir: Dir, path: &Path) {
    // Check if all the subdirectories exist, recursing on each
    for subdir in dir.dirs() {
        let subdir_path = path.join(dir.path());
        assert!(subdir_path.exists());
        validate_extracted(*subdir, &subdir_path);
    }

    // Check if the files at the root of this directory exist
    for file in dir.files() {
        let file_path = path.join(file.path());
        assert!(file_path.exists());
    }
}
