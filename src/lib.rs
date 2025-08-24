//! A small crate for lexical path normalization.
//!
//! This crate provides a trait extension for [`std::path::Path`] to normalize paths
//! without accessing the filesystem. A `normalize_lexically` method exists in the
//! nightly Rust toolchain, but this crate allows for using it in the stable toolchain.

use std::path::{Component, Path, PathBuf};

/// Error type returned when path normalization fails.
#[derive(Debug)]
pub struct NormalizeError;

impl std::fmt::Display for NormalizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path normalization failed")
    }
}

impl std::error::Error for NormalizeError {}

/// Extension trait to add `normalize_lexically` to [`Path`].
pub trait PathNormalizeExt {
    /// Normalize a path lexically, resolving `.` and `..` without accessing the filesystem.
    ///
    /// # Errors
    ///
    /// Returns [`NormalizeError`] if the path attempts to traverse above its root.
    fn normalize_path(&self) -> Result<PathBuf, NormalizeError>;
}

impl PathNormalizeExt for Path {
    fn normalize_path(&self) -> Result<PathBuf, NormalizeError> {
        let mut lexical = PathBuf::new();
        let mut iter = self.components().peekable();

        let root = match iter.peek() {
            Some(Component::ParentDir) => return Err(NormalizeError),
            Some(p @ Component::RootDir) | Some(p @ Component::CurDir) => {
                lexical.push(p);
                iter.next();
                lexical.as_os_str().len()
            }
            Some(Component::Prefix(prefix)) => {
                lexical.push(prefix.as_os_str());
                iter.next();
                if let Some(p @ Component::RootDir) = iter.peek() {
                    lexical.push(p);
                    iter.next();
                }
                lexical.as_os_str().len()
            }
            None => return Ok(PathBuf::new()),
            Some(Component::Normal(_)) => 0,
        };

        for component in iter {
            match component {
                Component::RootDir => unreachable!(),
                Component::Prefix(_) => return Err(NormalizeError),
                Component::CurDir => continue,
                Component::ParentDir => {
                    if lexical.as_os_str().len() == root {
                        return Err(NormalizeError);
                    } else {
                        lexical.pop();
                    }
                }
                Component::Normal(path) => lexical.push(path),
            }
        }

        Ok(lexical)
    }
}
