use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Literal;
use quote::quote;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
};

#[proc_macro]
pub fn include_dir(input: TokenStream) -> TokenStream {
    let tokens: Vec<_> = input.into_iter().collect();

    let path = match tokens.as_slice() {
        [TokenTree::Literal(lit)] => unwrap_string_literal(lit),
        _ => panic!("This macro only accepts a single, non-empty string argument"),
    };

    let path = resolve_path(&path, get_env).unwrap();

    expand_dir(&path, &path).into()
}

fn unwrap_string_literal(lit: &proc_macro::Literal) -> String {
    let mut repr = lit.to_string();
    if !repr.starts_with('"') || !repr.starts_with('"') {
        panic!("This macro only accepts a single, non-empty string argument")
    }

    repr.remove(0);
    repr.pop();

    repr
}

fn expand_dir(root: &Path, path: &Path) -> proc_macro2::TokenStream {
    let children = read_dir(path).unwrap_or_else(|e| {
        panic!(
            "Unable to read the entries in \"{}\": {}",
            path.display(),
            e
        )
    });

    let mut child_tokens = Vec::new();

    for child in children {
        if child.is_dir() {
            let tokens = expand_dir(root, &child);
            child_tokens.push(quote! {
                include_dir::DirEntry::Dir(#tokens)
            });
        } else if child.is_file() {
            let tokens = expand_file(&child);
            child_tokens.push(quote! {
                include_dir::DirEntry::File(#tokens)
            });
        } else {
            panic!("\"{}\" is neither a file nor a directory", child.display());
        }
    }

    let path = path.strip_prefix(root).unwrap().to_string_lossy();

    quote! {
        include_dir::Dir::new(#path, &[ #(#child_tokens),* ])
    }
}

fn expand_file(path: &Path) -> proc_macro2::TokenStream {
    let contents = std::fs::read(path)
        .unwrap_or_else(|e| panic!("Unable to read \"{}\": {}", path.display(), e));
    let literal = Literal::byte_string(&contents);
    let path = path
        .file_name()
        .expect("Files always have a name")
        .to_string_lossy();

    let tokens = quote! {
        include_dir::File::new(#path, #literal)
    };

    tokens
}

fn read_dir(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if !dir.is_dir() {
        panic!("\"{}\" is not a directory", dir.display());
    }

    let mut paths = Vec::new();

    for entry in dir.read_dir()? {
        let entry = entry?;
        paths.push(entry.path());
    }

    paths.sort();

    Ok(paths)
}

fn get_env(variable: &str) -> Option<String> {
    std::env::var(variable).ok()
}

fn resolve_path(
    raw: &str,
    get_env: impl Fn(&str) -> Option<String>,
) -> Result<PathBuf, Box<dyn Error>> {
    let mut unprocessed = raw;
    let mut resolved = String::new();

    while let Some(dollar_sign) = unprocessed.find('$') {
        let (head, tail) = unprocessed.split_at(dollar_sign);
        resolved.push_str(head);

        match parse_identifier(&tail[1..]) {
            Some((variable, rest)) => {
                let value = get_env(variable).ok_or_else(|| MissingVariable {
                    variable: variable.to_string(),
                })?;
                resolved.push_str(&value);
                unprocessed = rest;
            }
            None => {
                return Err(UnableToParseVariable { rest: tail.into() }.into());
            }
        }
    }
    resolved.push_str(unprocessed);

    Ok(PathBuf::from(resolved))
}

#[derive(Debug, PartialEq)]
struct MissingVariable {
    variable: String,
}

impl Error for MissingVariable {}

impl Display for MissingVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to resolve ${}", self.variable)
    }
}

#[derive(Debug, PartialEq)]
struct UnableToParseVariable {
    rest: String,
}

impl Error for UnableToParseVariable {}

impl Display for UnableToParseVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse a variable from \"{}\"", self.rest)
    }
}

fn parse_identifier(text: &str) -> Option<(&str, &str)> {
    let mut calls = 0;

    let (head, tail) = take_while(text, |c| {
        calls += 1;

        match c {
            '_' => true,
            letter if letter.is_ascii_alphabetic() => true,
            digit if digit.is_ascii_digit() && calls > 1 => true,
            _ => false,
        }
    });

    if head.is_empty() {
        None
    } else {
        Some((head, tail))
    }
}

fn take_while(s: &str, mut predicate: impl FnMut(char) -> bool) -> (&str, &str) {
    let mut index = 0;

    for c in s.chars() {
        if predicate(c) {
            index += c.len_utf8();
        } else {
            break;
        }
    }

    s.split_at(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_path_with_no_environment_variables() {
        let path = "./file.txt";

        let resolved = resolve_path(path, |_| unreachable!()).unwrap();

        assert_eq!(resolved.to_str().unwrap(), path);
    }

    #[test]
    fn simple_environment_variable() {
        let path = "./$VAR";

        let resolved = resolve_path(path, |name| {
            assert_eq!(name, "VAR");
            Some("file.txt".to_string())
        })
        .unwrap();

        assert_eq!(resolved.to_str().unwrap(), "./file.txt");
    }

    #[test]
    fn dont_resolve_recursively() {
        let path = "./$TOP_LEVEL.txt";

        let resolved = resolve_path(path, |name| match name {
            "TOP_LEVEL" => Some("$NESTED".to_string()),
            "$NESTED" => unreachable!("Shouln't resolve recursively"),
            _ => unreachable!(),
        })
        .unwrap();

        assert_eq!(resolved.to_str().unwrap(), "./$NESTED.txt");
    }

    #[test]
    fn parse_valid_identifiers() {
        let inputs = vec![
            ("a", "a"),
            ("a_", "a_"),
            ("_asf", "_asf"),
            ("a1", "a1"),
            ("a1_#sd", "a1_"),
        ];

        for (src, expected) in inputs {
            let (got, rest) = parse_identifier(src).unwrap();
            assert_eq!(got.len() + rest.len(), src.len());
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn unknown_environment_variable() {
        let path = "$UNKNOWN";

        let err = resolve_path(path, |_| None).unwrap_err();

        let missing_variable = err.downcast::<MissingVariable>().unwrap();
        assert_eq!(
            *missing_variable,
            MissingVariable {
                variable: String::from("UNKNOWN"),
            }
        );
    }

    #[test]
    fn invalid_variables() {
        let inputs = &["$1", "$"];

        for input in inputs {
            let err = resolve_path(input, |_| unreachable!()).unwrap_err();

            let err = err.downcast::<UnableToParseVariable>().unwrap();
            assert_eq!(
                *err,
                UnableToParseVariable {
                    rest: input.to_string(),
                }
            );
        }
    }
}