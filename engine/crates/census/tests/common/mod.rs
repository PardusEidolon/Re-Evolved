//! Fake trees for the census's tests: a folder under the system's
//! temporary folder, removed when the tree is dropped.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

/// A fake tree of files.
pub struct Tree {
    root: PathBuf,
}

impl Tree {
    /// An empty tree named for the test.
    pub fn new(name: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("opence-census-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        Self { root }
    }

    /// The tree with `text` at `path`.
    pub fn with(self, path: &str, text: &str) -> Self {
        let file = self.root.join(path);
        if let Some(parent) = file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(file, text);
        self
    }

    /// Its root.
    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// The repository, or nothing when its `source/` is missing (the
/// checks that read the real tree skip then).
pub fn repository() -> Option<PathBuf> {
    let root = opence_census::repository(Path::new(env!("CARGO_MANIFEST_DIR")));
    root.join("source/units/units.c").is_file().then_some(root)
}
