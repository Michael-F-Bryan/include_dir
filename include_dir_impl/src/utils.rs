use std::path::Path;

pub fn escape(path: &Path) -> String {
    let mut path = path.display().to_string();

    path.insert(0, 'r');
    path.insert(1, '"');
    path.push('"');

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_paths() {
        let inputs = vec![
            (Path::new("foo/bar/baz"), r#"r"foo/bar/baz""#),
            (Path::new(r"C:\foo\bar\baz"), r##"r"C:\foo\bar\baz""##),
        ];

        for (src, should_be) in inputs {
            let got = escape(src);
            assert_eq!(got, should_be);
        }
    }
}
