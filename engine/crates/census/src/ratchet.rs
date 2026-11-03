//! The ratchet (`engine/census/ratchet.tsv`): each system's Missing,
//! Partial and owed counts. `--check` fails when a count rises above what
//! the ratchet records, and when one falls below it until
//! `opence census --ratchet` records the lower count, so the coverage
//! tracks can only bring the counts down.

use std::collections::BTreeMap;

use crate::classes::Class;
use crate::docs::system;
use crate::{Census, Error};

/// The ratchet, from `engine/`.
pub const RATCHET: &str = "census/ratchet.tsv";

/// Its header.
const HEADER: &str = "system\tmissing\tpartial\towed";

/// A system's counts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Its Missing functions.
    pub missing: usize,
    /// Its Partial functions.
    pub partial: usize,
    /// Its functions that owe evidence.
    pub owed: usize,
}

impl Counts {
    /// The counts with their names, in the ratchet's order.
    fn named(self) -> [(&'static str, usize); 3] {
        [
            ("Missing", self.missing),
            ("Partial", self.partial),
            ("owed", self.owed),
        ]
    }
}

/// Every system's counts, by system.
pub type Ratchet = BTreeMap<String, Counts>;

/// The census's counts. A system with nothing to count is left out.
pub fn counts(census: &Census) -> Ratchet {
    let mut ratchet = Ratchet::new();
    for function in &census.functions {
        let missing = usize::from(function.class == Some(Class::Missing));
        let partial = usize::from(function.class == Some(Class::Partial));
        let owed = usize::from(function.owed.is_some());
        if missing + partial + owed == 0 {
            continue;
        }
        let counts = ratchet.entry(system(function).to_owned()).or_default();
        counts.missing += missing;
        counts.partial += partial;
        counts.owed += owed;
    }
    ratchet
}

/// The ratchet's text.
pub fn write_ratchet(ratchet: &Ratchet) -> String {
    let mut text = format!("{HEADER}\n");
    for (system, counts) in ratchet {
        text.push_str(&format!(
            "{system}\t{}\t{}\t{}\n",
            counts.missing, counts.partial, counts.owed
        ));
    }
    text
}

/// The ratchet of `text`.
pub fn read_ratchet(text: &str) -> Result<Ratchet, Error> {
    let malformed = |line: usize, why: &str| Error::Malformed {
        file: format!("engine/{RATCHET}"),
        line,
        why: why.to_owned(),
    };
    let mut lines = text.lines().enumerate();
    if lines.next().map(|(_, header)| header) != Some(HEADER) {
        return Err(malformed(1, "the header is not the ratchet's"));
    }
    let mut ratchet = Ratchet::new();
    for (index, line) in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        let [system, missing, partial, owed] = fields.as_slice() else {
            return Err(malformed(index + 1, "a line has four fields"));
        };
        let count = |field: &str| {
            field
                .parse::<usize>()
                .map_err(|_| malformed(index + 1, "a count is not a number"))
        };
        ratchet.insert(
            (*system).to_owned(),
            Counts {
                missing: count(missing)?,
                partial: count(partial)?,
                owed: count(owed)?,
            },
        );
    }
    Ok(ratchet)
}

/// What differs between the `recorded` ratchet and the `current` counts:
/// first the counts that rose, then those that changed otherwise. A
/// system either does not name counts zero.
///
/// The counts are stages a function moves through, so a function that
/// moves forward is not a rise: Missing and Partial together rise by no
/// more than owed fell (an owed function its check finds short becomes
/// Partial, and one it finds a stub becomes Missing), and owed rises by no
/// more than Missing fell (a Missing function becomes Matched or Partial
/// and owes its check).
pub fn compare(recorded: &Ratchet, current: &Ratchet) -> (Vec<String>, Vec<String>) {
    let mut rose = Vec::new();
    let mut fell = Vec::new();
    let systems: std::collections::BTreeSet<&String> =
        recorded.keys().chain(current.keys()).collect();
    for system in systems {
        let was = recorded.get(system).copied().unwrap_or_default();
        let now = current.get(system).copied().unwrap_or_default();
        let freed = was.missing.saturating_sub(now.missing);
        let checked = was.owed.saturating_sub(now.owed);
        let found = now.missing + now.partial > was.missing + was.partial + checked;
        let rises = [
            now.missing > was.missing && found,
            now.partial > was.partial && found,
            now.owed > was.owed + freed,
        ];
        for (((name, was), (_, now)), rises) in was.named().into_iter().zip(now.named()).zip(rises)
        {
            if rises {
                rose.push(format!("{system}: {name} rose from {was} to {now}"));
            } else if now < was {
                fell.push(format!(
                    "{system}: {name} fell from {was} to {now}: run opence census --ratchet"
                ));
            } else if now > was {
                fell.push(format!(
                    "{system}: {name} rose from {was} to {now} as Missing fell: run opence census --ratchet"
                ));
            }
        }
    }
    (rose, fell)
}
