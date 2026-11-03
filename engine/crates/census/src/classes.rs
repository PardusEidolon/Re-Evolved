//! The classes (`engine/census/census.toml`): what each function of the
//! original is to the engine, and the evidence its class asks for.
//!
//! ```toml
//! [[rule]]                      # every function of a directory, file or prefix
//! directory = "rasterizer"
//! class = "replaced"
//! because = "renderer"
//! stands_in = "app/Sources/OpenCEEngineRender"
//!
//! [function.unit_kill]          # one function; it wins over every rule
//! class = "matched"
//! check = "sim/tests/damage_reference.rs"
//! against = "trace"
//! ```
//!
//! A function takes the class of its own entry, or else of the most
//! specific rule that matches it (a file is more specific than a
//! directory, a deeper directory than its parent, and a prefix adds to
//! either; of two equally specific rules the later wins; a rule that
//! selects nothing is the default). A directory names its files and those
//! of its subdirectories. Paths name the engine's files from `engine/crates`
//! (`sim/src/units.rs`) or from `engine` for the app (`app/Sources/...`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{read_text, walk, Error};

/// What a function of the original is to the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Class {
    /// Ported line for line: a reference trace compiles it, or a trace of
    /// a caller covers it (`why` and `through`).
    Ported,
    /// Compiled unchanged into game.wasm: its file is one
    /// `port/linux/port.json`'s game compiles ([`Game`]).
    Compiled,
    /// Rewritten: the engine does its job its own way, and a check against
    /// the original (a trace, a golden comparison or a measurement) shows
    /// that the behaviour matches.
    Matched,
    /// A helper the engine does not port, with the Rust primitive used in
    /// its place (`primitive`).
    Native,
    /// Started, not finished: what it leaves out (`leaves_out`), and the
    /// evidence for the part that is done.
    Partial,
    /// Replaced on purpose, for one of the listed reasons (`because`), with
    /// what stands in for it (`stands_in`).
    Replaced,
    /// Not started.
    Missing,
}

impl Class {
    /// Every class, in order.
    pub const ALL: [Self; 7] = [
        Self::Ported,
        Self::Compiled,
        Self::Matched,
        Self::Native,
        Self::Partial,
        Self::Replaced,
        Self::Missing,
    ];

    /// Its name, as the census writes it.
    pub fn name(self) -> &'static str {
        match self {
            Self::Ported => "Ported",
            Self::Compiled => "Compiled",
            Self::Matched => "Matched",
            Self::Native => "Native",
            Self::Partial => "Partial",
            Self::Replaced => "Replaced",
            Self::Missing => "Missing",
        }
    }
}

/// The reasons a function may be Replaced: the fixed list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Because {
    /// The enhanced lighting and the Metal renderer.
    Renderer,
    /// The Xbox's own hardware.
    Xbox,
    /// Keyboard and mouse only.
    KeyboardMouse,
    /// Maps imported to a typed format.
    Imported,
    /// The editor and the tools.
    Tools,
    /// Networking and Xbox Live.
    Network,
    /// Bink's videos.
    Bink,
    /// A library does the job (Jolt's collision for the collision BSP), so
    /// what the player sees has to be said (`lives_on`).
    Library,
}

impl Because {
    /// The reason, as the census writes it.
    pub fn text(self) -> &'static str {
        match self {
            Self::Renderer => "the enhanced lighting and Metal renderer",
            Self::Xbox => "Xbox-only hardware",
            Self::KeyboardMouse => "keyboard and mouse only",
            Self::Imported => "maps imported to a typed format",
            Self::Tools => "the editor and tools",
            Self::Network => "networking and Xbox Live",
            Self::Bink => "Bink",
            Self::Library => "a library does the job",
        }
    }
}

/// What a check compares the engine with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Against {
    /// A reference trace of the original's own functions (a
    /// `*_reference.rs` test).
    Trace,
    /// Golden frames or sounds of the stock game.
    Golden,
    /// A measurement of the stock game.
    Measure,
}

/// A rule or an entry: what it selects (rules only), its class and its
/// evidence.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// A rule's directory of `source/` (`rasterizer`).
    pub directory: Option<String>,
    /// A rule's file of `source/` (`units/units.c`).
    pub file: Option<String>,
    /// A rule's prefix of the function's name (`rasterizer_`).
    pub prefix: Option<String>,
    /// The class.
    pub class: Option<Class>,
    /// The engine's file that does its job, when no citation names it.
    pub engine: Option<String>,
    /// Ported without a trace of its own: why.
    pub why: Option<String>,
    /// Ported without a trace of its own: the traced caller that covers it.
    pub through: Option<String>,
    /// Matched: the test that checks it against the original.
    pub check: Option<String>,
    /// Matched: what that test compares the engine with.
    pub against: Option<Against>,
    /// Native: the primitive in its place (`libm::sinf`, `Vec`).
    pub primitive: Option<String>,
    /// Partial: what it leaves out.
    #[serde(default)]
    pub leaves_out: Vec<String>,
    /// Replaced: why.
    pub because: Option<Because>,
    /// Replaced: the engine's code that stands in for it.
    pub stands_in: Option<String>,
    /// Replaced: the behaviour the player sees that has to live on
    /// elsewhere, and where.
    pub lives_on: Option<String>,
    /// The evidence known to be missing: the check accepts it, and the
    /// report lists it.
    pub owed: Option<String>,
    /// A note for the docs.
    pub note: Option<String>,
    /// Missing rules: `false` keeps the rule's functions Missing even when
    /// only Replaced code reaches them, for systems whose behaviour play
    /// still needs (the original's renderer drives its weather, decals and
    /// score screens).
    pub through_callers: Option<bool>,
    /// Rules: `true` keeps the rule's class for the functions a reference
    /// trace compiles only as the original's oracle, to check a library
    /// against it (the collision BSP walk, which Jolt does).
    pub over_traces: Option<bool>,
}

impl Entry {
    /// How specific a rule is (more wins).
    fn specificity(&self) -> usize {
        // (a directory counts its steps, so `sound/sound_import` outranks
        // `sound`; a file or a prefix outranks any directory)
        const STEP: usize = 64;
        self.directory
            .as_deref()
            .map_or(0, |d| d.split('/').count())
            + STEP * usize::from(self.file.is_some())
            + STEP * usize::from(self.prefix.is_some())
    }

    /// Whether a rule selects `name` of `file`.
    fn selects(&self, name: &str, file: &str) -> bool {
        self.directory.as_deref().is_none_or(|d| {
            file.strip_prefix(d)
                .is_some_and(|rest| rest.starts_with('/'))
        }) && self.file.as_deref().is_none_or(|f| f == file)
            && self.prefix.as_deref().is_none_or(|p| name.starts_with(p))
    }
}

/// `engine/census/census.toml`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// The rules, in order.
    #[serde(default)]
    pub rule: Vec<Entry>,
    /// The entries for single functions.
    #[serde(default)]
    pub function: BTreeMap<String, Entry>,
}

/// Where a function's class comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum From {
    /// Its own entry.
    Entry,
    /// A rule (its index, from 0).
    Rule(usize),
    /// A reference trace, which makes it Ported.
    Trace,
    /// game.wasm, which compiles its file and makes it Compiled.
    Game,
    /// Nothing.
    Nothing,
}

impl Manifest {
    /// The manifest of `text`; `file` names it in errors.
    pub fn parse(file: &str, text: &str) -> Result<Self, Error> {
        toml::from_str(text).map_err(|error| Error::Malformed {
            file: file.to_owned(),
            line: error.span().map_or(0, |span| {
                text.get(..span.start).unwrap_or("").lines().count()
            }),
            why: error.message().to_owned(),
        })
    }

    /// The entry that classes `name` of `file`, and where it comes from.
    pub fn lookup(&self, name: &str, file: &str) -> (Option<&Entry>, From) {
        if let Some(entry) = self.function.get(name) {
            return (Some(entry), From::Entry);
        }
        let mut best: Option<(usize, usize)> = None;
        for (index, rule) in self.rule.iter().enumerate() {
            if rule.selects(name, file) && best.is_none_or(|(s, _)| rule.specificity() >= s) {
                best = Some((rule.specificity(), index));
            }
        }
        match best {
            Some((_, index)) => (self.rule.get(index), From::Rule(index)),
            None => (None, From::Nothing),
        }
    }
}

/// What the engine holds that evidence names: its files, the crates it
/// depends on and the identifiers its code uses.
#[derive(Debug, Clone, Default)]
pub struct Engine {
    /// The engine's folder (`engine/`).
    pub root: PathBuf,
    /// The crates its manifests name, with `-` as `_`.
    pub dependencies: BTreeSet<String>,
    /// Every identifier of its crates' `src/` and the app's Sources.
    pub identifiers: BTreeSet<String>,
}

/// The crates every Rust program has.
const STANDARD: [&str; 3] = ["std", "core", "alloc"];

impl Engine {
    /// The engine at `root` (`engine/`).
    pub fn read(root: &Path) -> Result<Self, Error> {
        let mut dependencies = BTreeSet::new();
        let mut identifiers = BTreeSet::new();
        let crates = root.join("crates");
        let mut manifests = vec![root.join("Cargo.toml")];
        let mut sources = Vec::new();
        if crates.is_dir() {
            manifests.extend(walk(&crates, &["toml"])?);
            sources.extend(walk(&crates, &["rs"])?.into_iter().filter(|path| {
                path.strip_prefix(&crates)
                    .ok()
                    .and_then(|place| place.iter().nth(1))
                    .is_some_and(|folder| folder == "src")
            }));
        }
        let app = root.join("app/Sources");
        if app.is_dir() {
            sources.extend(walk(&app, &["swift"])?);
        }
        for manifest in manifests.iter().filter(|path| path.is_file()) {
            for line in read_text(manifest)?.lines() {
                let key = line.split('=').next().unwrap_or("").trim();
                if line.contains('=')
                    && !key.is_empty()
                    && key
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
                {
                    dependencies.insert(key.replace('-', "_"));
                }
            }
        }
        for source in &sources {
            let text = read_text(source)?;
            identifiers.extend(
                text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                    .filter(|word| word.bytes().next().is_some_and(|b| !b.is_ascii_digit()))
                    .map(str::to_owned),
            );
        }
        Ok(Self {
            root: root.to_owned(),
            dependencies,
            identifiers,
        })
    }

    /// Whether `place` (`sim/src/units.rs:12`, `app/Sources/...`, or the
    /// repository's `port/...` and `source/...`) exists.
    pub fn exists(&self, place: &str) -> bool {
        let path = place.split(':').next().unwrap_or(place);
        // (game.wasm's stand-ins are the port's C, or the original's own,
        // outside engine/)
        if path.starts_with("port/") || path.starts_with("source/") {
            return self
                .root
                .parent()
                .is_some_and(|repository| repository.join(path).exists());
        }
        match path.strip_prefix("app/") {
            Some(_) => self.root.join(path).exists(),
            None => self.root.join("crates").join(path).exists(),
        }
    }

    /// What is wrong with `primitive` (`libm::sinf`, `Vec`), if anything.
    fn primitive(&self, primitive: &str) -> Option<String> {
        let path: Vec<&str> = primitive.split("::").collect();
        let first = path.first().copied().unwrap_or("");
        let last = path.last().copied().unwrap_or("");
        if path.len() > 1 && !STANDARD.contains(&first) && !self.dependencies.contains(first) {
            return Some(format!("{first} is not a dependency of the engine"));
        }
        if STANDARD.contains(&first) && matches!(last, "HashMap" | "HashSet") {
            return Some(format!(
                "{primitive} keeps no fixed order (the simulation's maps iterate in a fixed order)"
            ));
        }
        if !self.identifiers.contains(last) {
            return Some(format!("the engine does not use {primitive}"));
        }
        None
    }
}

/// The functions the reference traces compile, and the script of each
/// (`engine_item_reference`).
pub type Traced = BTreeMap<String, String>;

/// What a function's class still lacks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Lacking {
    /// The evidence an `owed` may stand for.
    pub owed: Vec<String>,
    /// What is wrong whatever is owed.
    pub wrong: Vec<String>,
}

/// What `entry`, classing `name` as `class`, lacks: `traced` are the
/// traced functions, `cited` the places that cite it.
pub fn lacking(
    name: &str,
    class: Class,
    entry: &Entry,
    traced: &Traced,
    cited: &[String],
    engine: &Engine,
) -> Lacking {
    let mut lacks = Lacking::default();
    let exists = |place: &str, lacks: &mut Lacking| {
        if !engine.exists(place) {
            lacks.wrong.push(format!("{place} does not exist"));
        }
    };
    // (the evidence a Ported or Matched function, or a Partial one's done
    // part, shows: a trace, a reason with a traced caller, or a check)
    let ported = |lacks: &mut Lacking| -> bool {
        if let Some(script) = traced.get(name) {
            if entry.why.is_some() {
                lacks
                    .wrong
                    .push(format!("{name} is traced by {script}: its reason is stale"));
            }
            return true;
        }
        if let Some(through) = &entry.through {
            if !traced.contains_key(through) {
                lacks.wrong.push(format!("{through} is not traced"));
            }
        }
        entry.why.is_some() && entry.through.is_some()
    };
    let matched = |lacks: &mut Lacking| -> bool {
        if let Some(place) = &entry.engine {
            exists(place, lacks);
        } else if cited.is_empty() {
            lacks
                .wrong
                .push("names no engine code that does its job".to_owned());
        }
        if let Some(check) = &entry.check {
            exists(check, lacks);
            if entry.against == Some(Against::Trace) && !check.contains("_reference") {
                lacks
                    .wrong
                    .push(format!("{check} is not a reference trace"));
            }
        }
        entry.check.is_some() && entry.against.is_some()
    };
    match class {
        Class::Ported => {
            if !ported(&mut lacks) {
                lacks
                    .owed
                    .push("Ported has no trace, and no reason with a traced caller".to_owned());
            }
        }
        Class::Matched => {
            if !matched(&mut lacks) {
                lacks
                    .owed
                    .push("Matched has no check against the original".to_owned());
            }
        }
        Class::Native => match &entry.primitive {
            Some(primitive) => lacks.wrong.extend(engine.primitive(primitive)),
            None => lacks.owed.push("Native names no primitive".to_owned()),
        },
        Class::Partial => {
            if entry.leaves_out.is_empty() {
                lacks
                    .wrong
                    .push("Partial lists nothing it leaves out".to_owned());
            }
            let done = if entry.check.is_some() || entry.engine.is_some() {
                matched(&mut lacks)
            } else {
                ported(&mut lacks)
            };
            if !done {
                lacks
                    .owed
                    .push("Partial has no evidence for the part that is done".to_owned());
            }
        }
        Class::Replaced => {
            if entry.because.is_none() {
                lacks.wrong.push("Replaced gives no reason".to_owned());
            }
            if entry.because == Some(Because::Library) && entry.lives_on.is_none() {
                lacks
                    .wrong
                    .push("a library stands in but nothing says what the player sees".to_owned());
            }
            match &entry.stands_in {
                Some(place) => exists(place, &mut lacks),
                None => lacks
                    .wrong
                    .push("Replaced names nothing that stands in for it".to_owned()),
            }
        }
        Class::Compiled | Class::Missing => {}
    }
    lacks
}

/// What game.wasm compiles: `port/linux/port.json`'s game, every C file
/// under its root (`source`) but those it excludes (a folder ends in `/`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Game {
    /// The excluded files and folders, from `source/`.
    pub exclude: Vec<String>,
    /// Whether the repository has a port.json; without one nothing is
    /// compiled.
    pub found: bool,
}

/// `port/linux/port.json`, from the repository.
pub const PORT: &str = "port/linux/port.json";

#[derive(Deserialize)]
struct Port {
    game: PortGame,
}

#[derive(Deserialize)]
struct PortGame {
    root: String,
    #[serde(default)]
    exclude: Vec<String>,
}

impl Game {
    /// The game of the repository at `root`.
    pub fn read(root: &Path) -> Result<Self, Error> {
        let path = root.join(PORT);
        if !path.is_file() {
            return Ok(Self::default());
        }
        let port: Port =
            serde_json::from_str(&read_text(&path)?).map_err(|error| Error::Malformed {
                file: PORT.to_owned(),
                line: error.line(),
                why: error.to_string(),
            })?;
        if port.game.root != "source" {
            return Err(Error::Malformed {
                file: PORT.to_owned(),
                line: 0,
                why: format!("the game's root is {}, not source", port.game.root),
            });
        }
        let exclude = port
            .game
            .exclude
            .iter()
            .map(|place| place.strip_prefix("source/").unwrap_or(place).to_owned())
            .collect();
        Ok(Self {
            exclude,
            found: true,
        })
    }

    /// Whether game.wasm compiles `file` (of `source/`).
    pub fn compiles(&self, file: &str) -> bool {
        self.found
            && !self
                .exclude
                .iter()
                .any(|place| match place.strip_suffix('/') {
                    Some(folder) => file
                        .strip_prefix(folder)
                        .is_some_and(|rest| rest.starts_with('/')),
                    None => file == place,
                })
    }
}
