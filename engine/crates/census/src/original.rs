//! The original as the census reads it: its functions (from the symbol
//! headers, and the inlined ones from the C) and its call graph.
//!
//! The symbol headers know about thirteen hundred functions only by their
//! address (`code_000ad290`): the decompilation defines most of them as
//! static functions without giving their symbols a name. Each file's
//! address-only symbols, in address order, are taken to be its
//! definitions that have no symbol, in the order the C defines them (the
//! compiler lays a file's functions out in its order); a symbol the C
//! defines nothing for is known by its place.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::c::{definitions, Definition, Kind};
use crate::graph::Graph;
use crate::symbols::{self, Function};
use crate::{read_text, walk, Error};

/// The prefix of the names the symbol headers give a function they know
/// only by its address.
pub const ADDRESS_ONLY: &str = "code_";

/// How far the sizes of a file's paired symbols must rise with the
/// lengths of their definitions (as a rank correlation) to back the order.
const SIZE_AGREEMENT: f64 = 0.8;

/// The original.
#[derive(Debug, Clone)]
pub struct Original {
    /// Every function: the symbol headers' code, in address order, then
    /// the functions the C defines that have no symbol of their own
    /// (inlined by the compiler, with an address and a size of 0), by file.
    pub functions: Vec<Function>,
    /// The functions the old hosted port added to `source/`, which are not
    /// the original's (see [`crate::c::Definition::port`]).
    pub added: Vec<Function>,
    /// The call graph of its `.c` and `.h` files.
    pub graph: Graph,
    /// What each address-only symbol is taken to be, by its header name.
    pub names_by_address: BTreeMap<String, Name>,
}

/// What an address-only symbol is taken to be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    /// Its file of `source/`.
    pub file: String,
    /// The definition it is (`unit_kill`), or its place (`after
    /// unit_update`).
    pub name: String,
    /// How it was named.
    pub evidence: Evidence,
}

/// How an address-only symbol was named.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Evidence {
    /// The file has as many definitions without a symbol as address-only
    /// symbols, and the symbols' sizes rise with the definitions' lengths.
    OrderAndSize,
    /// Taken in order, without the sizes to back it.
    Order,
    /// The C defines nothing for it: it is known by the named function
    /// before it.
    Place,
}

impl Evidence {
    /// Its text in `names.tsv`.
    pub fn text(self) -> &'static str {
        match self {
            Self::OrderAndSize => "order and size",
            Self::Order => "order",
            Self::Place => "place",
        }
    }
}

impl Original {
    /// The original at `source`.
    pub fn read(source: &Path) -> Result<Self, Error> {
        let mut functions = symbols::functions(source)?;
        let mut files = Vec::new();
        for path in walk(source, &["c", "h"])? {
            let file = path
                .strip_prefix(source)
                .map_err(|_| Error::Outside(path.clone()))?
                .to_string_lossy()
                .replace('\\', "/");
            files.push((file, definitions(&read_text(&path)?)));
        }
        let named: BTreeSet<String> = functions.iter().map(|f| f.name.clone()).collect();
        // (each file's own definitions without a symbol, in its order)
        let mut seen = BTreeSet::new();
        let mut unsymbolled: BTreeMap<&str, Vec<&Definition>> = BTreeMap::new();
        for (file, found) in &files {
            for definition in found {
                if definition.kind == Kind::Function
                    && !named.contains(&definition.name)
                    && seen.insert(definition.name.clone())
                {
                    unsymbolled
                        .entry(file.as_str())
                        .or_default()
                        .push(definition);
                }
            }
        }
        let names_by_address = name_addresses(&mut functions, &unsymbolled);
        let paired: BTreeSet<&str> = names_by_address
            .values()
            .filter(|name| name.evidence != Evidence::Place)
            .map(|name| name.name.as_str())
            .collect();
        let mut added = Vec::new();
        let mut inlined = Vec::new();
        for (file, found) in &unsymbolled {
            for definition in found {
                if paired.contains(definition.name.as_str()) {
                    continue;
                }
                let function = Function {
                    address: 0,
                    size: 0,
                    name: definition.name.clone(),
                    file: (*file).to_owned(),
                };
                if definition.port {
                    added.push(function);
                } else {
                    inlined.push(function);
                }
            }
        }
        // (the inlined ones by file, in the order `walk` gives the files)
        let order: BTreeMap<&str, usize> = files
            .iter()
            .enumerate()
            .map(|(index, (file, _))| (file.as_str(), index))
            .collect();
        inlined.sort_by_key(|f| order.get(f.file.as_str()).copied().unwrap_or(usize::MAX));
        added.sort_by_key(|f| order.get(f.file.as_str()).copied().unwrap_or(usize::MAX));
        functions.extend(inlined);
        Ok(Self {
            functions,
            added,
            graph: Graph::new(&files),
            names_by_address,
        })
    }

    /// The names of its functions.
    pub fn names(&self) -> BTreeSet<String> {
        self.functions.iter().map(|f| f.name.clone()).collect()
    }
}

/// Names each address-only symbol of `functions` (renaming those taken to
/// be a definition), from each file's `unsymbolled` definitions.
fn name_addresses(
    functions: &mut [Function],
    unsymbolled: &BTreeMap<&str, Vec<&Definition>>,
) -> BTreeMap<String, Name> {
    let mut by_file: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, function) in functions.iter().enumerate() {
        if function.name.starts_with(ADDRESS_ONLY) {
            by_file
                .entry(function.file.clone())
                .or_default()
                .push(index);
        }
    }
    let mut names = BTreeMap::new();
    for (file, indices) in &by_file {
        // (a port's additions and inline functions have no symbol)
        let candidates: Vec<&Definition> = unsymbolled
            .get(file.as_str())
            .map(|found| {
                found
                    .iter()
                    .filter(|d| !d.port && !d.inline)
                    .copied()
                    .collect()
            })
            .unwrap_or_default();
        let pairs = indices.len().min(candidates.len());
        let sizes: Vec<f64> = indices
            .iter()
            .take(pairs)
            .filter_map(|&index| functions.get(index))
            .map(|f| f64::from(f.size))
            .collect();
        let lengths: Vec<f64> = candidates
            .iter()
            .take(pairs)
            .map(|d| d.refers_to.len() as f64)
            .collect();
        let evidence = if indices.len() == candidates.len()
            && rank_correlation(&sizes, &lengths) >= SIZE_AGREEMENT
        {
            Evidence::OrderAndSize
        } else {
            Evidence::Order
        };
        for (position, &index) in indices.iter().enumerate() {
            let Some(function) = functions.get(index) else {
                continue;
            };
            let address_name = function.name.clone();
            let name = match candidates.get(position) {
                Some(definition) => Name {
                    file: file.clone(),
                    name: definition.name.clone(),
                    evidence,
                },
                None => Name {
                    file: file.clone(),
                    name: place(functions, index),
                    evidence: Evidence::Place,
                },
            };
            if name.evidence != Evidence::Place {
                if let Some(function) = functions.get_mut(index) {
                    function.name = name.name.clone();
                }
            }
            names.insert(address_name, name);
        }
    }
    names
}

/// The place of the address-only symbol at `index`: after the named
/// function of its file before it, or first in its file.
fn place(functions: &[Function], index: usize) -> String {
    let Some(function) = functions.get(index) else {
        return String::new();
    };
    functions
        .get(..index)
        .unwrap_or(&[])
        .iter()
        .rev()
        .find(|f| f.file == function.file && !f.name.starts_with(ADDRESS_ONLY))
        .map_or_else(
            || format!("first in {}", function.file),
            |before| format!("after {}", before.name),
        )
}

/// Spearman's rank correlation of `a` and `b` (0 when there are fewer
/// than three pairs, or the lengths differ).
pub fn rank_correlation(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len();
    if n < 3 || n != b.len() {
        return 0.0;
    }
    let (ra, rb) = (ranks(a), ranks(b));
    let mean = (n as f64 - 1.0) / 2.0;
    let (mut covariance, mut va, mut vb) = (0.0, 0.0, 0.0);
    for (x, y) in ra.iter().zip(&rb) {
        covariance += (x - mean) * (y - mean);
        va += (x - mean) * (x - mean);
        vb += (y - mean) * (y - mean);
    }
    if va == 0.0 || vb == 0.0 {
        return 0.0;
    }
    covariance / (va * vb).sqrt()
}

/// The ranks of `values`, ties sharing their mean rank.
fn ranks(values: &[f64]) -> Vec<f64> {
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_by(|&i, &j| {
        values
            .get(i)
            .partial_cmp(&values.get(j))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut ranks = vec![0.0; values.len()];
    let mut start = 0;
    while start < order.len() {
        let value = order.get(start).and_then(|&i| values.get(i));
        let mut end = start;
        while end < order.len() && order.get(end).and_then(|&i| values.get(i)) == value {
            end += 1;
        }
        let rank = (start + end - 1) as f64 / 2.0;
        for &i in order.get(start..end).unwrap_or(&[]) {
            if let Some(slot) = ranks.get_mut(i) {
                *slot = rank;
            }
        }
        start = end;
    }
    ranks
}

/// The names table, under `engine/`.
pub const NAMES: &str = "census/names.tsv";

/// `names_by_address` as `names.tsv`: an address-only symbol a line, with
/// its file, its name and how it was named.
pub fn write_names(names_by_address: &BTreeMap<String, Name>) -> String {
    let mut text = String::from("address\tfile\tname\tevidence\n");
    for (address, name) in names_by_address {
        text.push_str(&format!(
            "{address}\t{}\t{}\t{}\n",
            name.file,
            name.name,
            name.evidence.text()
        ));
    }
    text
}

/// The written names table, if it is not what the original at `root`
/// gives.
pub fn stale(root: &Path) -> Result<Vec<String>, Error> {
    let original = Original::read(&root.join("source"))?;
    let written = read_text(&root.join("engine").join(NAMES)).unwrap_or_default();
    Ok(if written == write_names(&original.names_by_address) {
        Vec::new()
    } else {
        vec![format!(
            "engine/{NAMES} is stale: run opence census --write"
        )]
    })
}
