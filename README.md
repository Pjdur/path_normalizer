# path_normalizer

[![Crates.io](https://img.shields.io/crates/v/path_normalizer.svg)](https://crates.io/crates/path_normalizer)
[![Docs.rs](https://docs.rs/path_normalizer/badge.svg)](https://docs.rs/path_normalizer)
[![CI](https://github.com/pjdur/path_normalizer/actions/workflows/ci.yml/badge.svg)](https://github.com/pjdur/path_normalizer/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

🧹 A lightweight Rust crate for **lexically normalizing paths** — no filesystem access required.

## Why?

This crate is useful for scenarios where you need to manipulate or validate paths without relying on the filesystem. It's particularly handy for:

- **Virtual Filesystems**: When working with in-memory or virtual filesystems, you can't rely on actual paths existing on disk.
- **Configuration Parsing**: When reading configuration files, you may need to normalize paths specified by users.
- **Sandboxed Environments**: In environments where filesystem access is restricted, you can still perform path manipulations.

> **Note**: The features of this crate exist in the Rust standard libary (`std`), but only on the nightly toolchain. This crate provides its features but available regardless of Rust toolchain.
## ✨ Features

- Normalize paths like `"foo/./bar/../baz"` to `"foo/baz"`
- Works purely on string logic — doesn't touch the filesystem
- Cross-platform and fast

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
path_normalizer = "0.1"
```

## 🚀 Usage

```rust
use path_normalizer::PathNormalizeExt;
use std::path::Path;

fn main() {
    let path = Path::new("foo/./bar/../baz");
    let normalized = path.normalize_path().unwrap();
    println!("Normalized: {}", normalized.display()); // Outputs: foo/baz
}
```

## 🔒 Why Lexical?

Unlike `canonicalize()`, this crate doesn't resolve symlinks or check the filesystem. It simply rewrites the path using lexical rules — making it ideal for virtual paths, config parsing, or sandboxed environments.

## 📚 License

MIT

---

Made with ❤️ for Rustaceans who like their paths tidy.
