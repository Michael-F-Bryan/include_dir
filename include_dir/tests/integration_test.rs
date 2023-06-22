use include_dir::{include_dir, Dir};
use std::path::Path;
use tempfile::TempDir;

static PARENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/tree");

#[test]
fn included_all_files_in_the_include_dir_crate() {
    let root = &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/tree");

    validate_included(&PARENT_DIR, root, root);
    assert!(PARENT_DIR.contains("a/i/x"));
}

#[test]
fn extract_all_files() {
    let tmpdir = TempDir::new().unwrap();
    let root = tmpdir.path();
    PARENT_DIR.extract(root).unwrap();

    validate_extracted(&PARENT_DIR, root);
}

// Validates that all files on the filesystem exist in the inclusion
fn validate_included(dir: &Dir<'_>, path: &Path, root: &Path) {
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap().path();
        let entry = entry.strip_prefix(root).unwrap();

        let name = entry.file_name().unwrap();

        assert!(dir.contains(entry), "Can't find {}", entry.display());

        if entry.is_dir() {
            let child_path = path.join(name);
            validate_included(
                dir.get_entry(entry).unwrap().as_dir().unwrap(),
                &child_path,
                root,
            );
        }
    }
}

// Validates that all files in the inclusion were extracted to the filesystem
fn validate_extracted(dir: &Dir, path: &Path) {
    // Check if all the subdirectories exist, recursing on each
    for subdir in dir.dirs() {
        let subdir_path = path.join(subdir.path());
        assert!(subdir_path.exists(), "Can't find {}", subdir_path.display());
        validate_extracted(subdir, path);
    }

    // Check if the files at the root of this directory exist
    for file in dir.files() {
        let file_path = path.join(file.path());
        assert!(file_path.exists(), "Can't find {}", file_path.display());
    }
}
