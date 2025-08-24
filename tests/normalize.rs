use path_normalizer::{PathNormalizeExt};
use std::path::{Path, PathBuf};

#[test]
fn test_normalize_simple() {
    let path = Path::new("a/b/../c");
    let normalized = path.normalize_path().unwrap();
    assert_eq!(normalized, PathBuf::from("a/c"));
}

#[test]
fn test_normalize_root_escape() {
    let path = Path::new("../a");
    assert!(path.normalize_path().is_err());
}

#[test]
fn test_normalize_empty() {
    let path = Path::new("");
    let normalized = path.normalize_path().unwrap();
    assert_eq!(normalized, PathBuf::new());
}
