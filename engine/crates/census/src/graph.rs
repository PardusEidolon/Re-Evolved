//! The original's call graph: which of its definitions refers to which.
//!
//! A node is a top-level function, initialised data or macro of `source/`
//! (its `.c` and `.h` files); an edge is a name in a node's body that
//! names another node. Data and macros are nodes so that a function
//! reached only through a table of functions (the script functions, the
//! object types' functions) or through a macro is still reached. Static
//! definitions of the same name in several files are one node.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::c::{Definition, Kind};

/// One node of the graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// What it is (a function, when any of its definitions is one).
    pub kind: Kind,
    /// The files that define it, and the line in each.
    pub places: Vec<(String, usize)>,
    /// The nodes it refers to.
    pub refers_to: BTreeSet<String>,
}

/// The graph, by node name.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Graph {
    /// Its nodes.
    pub nodes: BTreeMap<String, Node>,
}

impl Graph {
    /// The graph of every file's definitions (`(file, definitions)`).
    pub fn new(files: &[(String, Vec<Definition>)]) -> Self {
        let mut nodes: BTreeMap<String, Node> = BTreeMap::new();
        let mut names: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for (file, definitions) in files {
            for definition in definitions {
                let node = nodes.entry(definition.name.clone()).or_insert(Node {
                    kind: definition.kind,
                    places: Vec::new(),
                    refers_to: BTreeSet::new(),
                });
                node.kind = node.kind.min(definition.kind);
                node.places.push((file.clone(), definition.line));
                names
                    .entry(definition.name.clone())
                    .or_default()
                    .extend(definition.refers_to.iter().cloned());
            }
        }
        for (name, referred) in names {
            let known: BTreeSet<String> = referred
                .into_iter()
                .filter(|other| *other != name && nodes.contains_key(other))
                .collect();
            if let Some(node) = nodes.get_mut(&name) {
                node.refers_to = known;
            }
        }
        Self { nodes }
    }

    /// Every node reached from `entries`, walking on only from the nodes
    /// `through` accepts (an entry is always walked from). The entries
    /// the graph does not hold are left out.
    pub fn reach(&self, entries: &[&str], through: impl Fn(&str) -> bool) -> BTreeSet<String> {
        let mut reached = BTreeSet::new();
        let mut queue: VecDeque<(&str, bool)> = entries
            .iter()
            .filter(|entry| self.nodes.contains_key(**entry))
            .map(|entry| (*entry, true))
            .collect();
        while let Some((name, entry)) = queue.pop_front() {
            if !reached.insert(name.to_owned()) && !entry {
                continue;
            }
            if !entry && !through(name) {
                continue;
            }
            if let Some(node) = self.nodes.get(name) {
                for other in &node.refers_to {
                    if !reached.contains(other) {
                        queue.push_back((other, false));
                    }
                }
            }
        }
        reached
    }
}
