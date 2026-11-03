//! The gaps ranked (`opence census --rank`): each Missing or Partial
//! function [`MAIN`](crate::MAIN) reaches, and each that owes evidence, by
//! the Missing bytes behind it and then by how many functions call it.
//!
//! The bytes behind a function are its own and those of every Missing or
//! Partial function it reaches through Missing or Partial ones: the gap
//! it heads. Counting every caller, near or far, says little, since most
//! of the reached original is one cycle through the main loop.

use std::collections::BTreeMap;

use crate::classes::Class;
use crate::docs::system;
use crate::Census;

/// One ranked function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rank {
    /// Its name.
    pub name: String,
    /// Its system (its directory of `source/`).
    pub system: String,
    /// Its class.
    pub class: Option<Class>,
    /// Whether it owes evidence.
    pub owed: bool,
    /// Its size in bytes.
    pub size: u32,
    /// The reached functions that call it.
    pub callers: usize,
    /// The Missing bytes of the gap it heads.
    pub gap_bytes: u64,
}

/// The ranking of `census`.
pub fn rank(census: &Census) -> Vec<Rank> {
    let functions: BTreeMap<&str, _> = census
        .functions
        .iter()
        .map(|function| (function.name.as_str(), function))
        .collect();
    let graph = &census.original.graph;
    let mut callers: BTreeMap<&str, usize> = BTreeMap::new();
    for (name, node) in &graph.nodes {
        if !census.reached.contains(name) || !functions.contains_key(name.as_str()) {
            continue;
        }
        for other in &node.refers_to {
            *callers.entry(other.as_str()).or_default() += 1;
        }
    }
    let gap = |name: &str| {
        functions
            .get(name)
            .is_some_and(|f| matches!(f.class, Some(Class::Missing | Class::Partial)))
    };
    let mut ranked: Vec<Rank> = census
        .functions
        .iter()
        .filter(|f| f.reached && (gap(&f.name) || f.owed.is_some()))
        .map(|f| {
            let gap_bytes = graph
                .reach(&[f.name.as_str()], gap)
                .iter()
                .filter_map(|name| functions.get(name.as_str()))
                .filter(|g| g.class == Some(Class::Missing))
                .map(|g| u64::from(g.size))
                .sum();
            Rank {
                name: f.name.clone(),
                system: system(f).to_owned(),
                class: f.class,
                owed: f.owed.is_some(),
                size: f.size,
                callers: callers.get(f.name.as_str()).copied().unwrap_or(0),
                gap_bytes,
            }
        })
        .collect();
    ranked.sort_by(|a, b| {
        b.gap_bytes
            .cmp(&a.gap_bytes)
            .then(b.callers.cmp(&a.callers))
            .then(a.name.cmp(&b.name))
    });
    ranked
}

/// The ranking as tab-separated lines under a header.
pub fn write_ranking(ranked: &[Rank]) -> String {
    let mut text = String::from("name\tsystem\tclass\towed\tbytes\tcallers\tgap_bytes\n");
    for r in ranked {
        text.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            r.name,
            r.system,
            r.class.map_or("-", Class::name),
            if r.owed { "yes" } else { "no" },
            r.size,
            r.callers,
            r.gap_bytes
        ));
    }
    text
}
