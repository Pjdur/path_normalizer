use path_normalizer::PathNormalizeExt;
use std::path::Path;

fn main() {
    let path = Path::new("foo/./bar/../baz");
    let normalized = path.normalize_path().unwrap();
    println!("Normalized: {}", normalized.display());
}
