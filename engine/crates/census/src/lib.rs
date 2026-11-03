//! The coverage census (conductor/archive/coverage_census_20261026):
//! every function of the original (`source/`) classed against the
//! engine, with the evidence its class asks for.
//!
//! The census reads the original's symbol headers (its functions, their
//! addresses and sizes), the names the engine's Rust and Swift cite in
//! their comments, the original's call graph, and `engine/census.toml`,
//! which holds the classes. `opence census` writes `docs/census.md` and
//! the summary in `docs/engine.md`; the crate's tests are its check.

use std::path::{Path, PathBuf};

pub mod c;
mod census;
pub mod citations;
pub mod classes;
pub mod docs;
pub mod engine;
pub mod fields;
pub mod fixups;
pub mod graph;
pub mod importer;
pub mod layout;
pub mod original;
pub mod ranking;
pub mod ratchet;
pub mod scripts;
pub mod symbols;
pub mod traced;

pub use census::{
    census, check_traces, read_traced, Census, Function, Mode, MAIN, MANIFEST, TRACED,
};

/// What the census could not read.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A file could not be read.
    #[error("{path}: {source}")]
    Io {
        /// The file.
        path: PathBuf,
        /// Why.
        source: std::io::Error,
    },
    /// A symbol header or a census file that is not in the format the
    /// census reads.
    #[error("{file}:{line}: malformed ({why})")]
    Malformed {
        /// The file of `source/`.
        file: String,
        /// The line, from 1 (0 for the header as a whole).
        line: usize,
        /// What is wrong with it.
        why: String,
    },
    /// The census's requirements not met, one line each.
    #[error("the census has {} problems", .0.len())]
    Problems(Vec<String>),
    /// A path the walk found outside the tree it walked.
    #[error("{0} is outside the tree")]
    Outside(PathBuf),
}

/// The repository, from a crate's manifest directory
/// (`engine/crates/<crate>`).
pub fn repository(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("../../..")
}

/// A file's text; the original's files are read as Latin-1, since a few
/// hold bytes that are not UTF-8.
pub fn read_text(path: &Path) -> Result<String, Error> {
    let bytes = std::fs::read(path).map_err(|source| Error::Io {
        path: path.to_owned(),
        source,
    })?;
    Ok(match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(error) => error.into_bytes().into_iter().map(char::from).collect(),
    })
}

/// Every file under `root` with one of `extensions`, in a fixed order.
pub fn walk(root: &Path, extensions: &[&str]) -> Result<Vec<PathBuf>, Error> {
    let mut found = Vec::new();
    let mut folders = vec![root.to_owned()];
    while let Some(folder) = folders.pop() {
        let entries = std::fs::read_dir(&folder).map_err(|source| Error::Io {
            path: folder.clone(),
            source,
        })?;
        for entry in entries {
            let entry = entry.map_err(|source| Error::Io {
                path: folder.clone(),
                source,
            })?;
            let path = entry.path();
            if path.is_dir() {
                folders.push(path);
            } else if path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extensions.contains(&extension))
            {
                found.push(path);
            }
        }
    }
    found.sort();
    Ok(found)
}
