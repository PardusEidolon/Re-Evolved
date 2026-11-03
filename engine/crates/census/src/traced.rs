//! The reference traces: the `tools/engine_*_reference.py` scripts, which
//! compile functions of `source/` with clang against a small fake world
//! and record what they do (`tools/traces/<name>_reference.txt`).
//!
//! The scripts stay in Python, since they drive clang. The census asks
//! each for the C it compiles (its `source_file()`), takes the functions
//! that C defines, and keeps them in `engine/census/traced.tsv`, so that
//! the census's own check needs neither Python nor clang. A script's C
//! also holds its fake world, whose stand-ins can share a name with the
//! original's functions, so a function is traced only when the script's
//! body refers to the same names, in the same order, as one of `source/`'s
//! definitions of it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::c::{definitions, Kind};
use crate::classes::Traced;
use crate::{read_text, walk, Error};

/// The Python that prints each script's C, after a `### <script>` line.
const SOURCES: &str = "
import importlib.util, pathlib, sys
tools = pathlib.Path(sys.argv[1])
sys.path.insert(0, str(tools))
for path in sorted(tools.glob('engine_*_reference.py')):
    spec = importlib.util.spec_from_file_location(path.stem, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    print('### ' + path.stem)
    print(module.source_file())
";

/// The reference scripts of the repository at `root`, in order.
pub fn scripts(root: &Path) -> Result<Vec<PathBuf>, Error> {
    let tools = root.join("tools");
    if !tools.is_dir() {
        return Ok(Vec::new());
    }
    Ok(walk(&tools, &["py"])?
        .into_iter()
        .filter(|path| {
            path.parent() == Some(tools.as_path())
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| {
                        name.starts_with("engine_") && name.ends_with("_reference.py")
                    })
        })
        .collect())
}

/// The name of a script's trace file (`item_reference.txt` for
/// `engine_item_reference.py`).
pub fn trace_file(script: &Path) -> Option<String> {
    let stem = script.file_stem()?.to_str()?;
    Some(format!("{}.txt", stem.strip_prefix("engine_")?))
}

/// The original's function bodies: for each name, what each of its
/// definitions refers to, in order.
pub type Bodies = BTreeMap<String, Vec<Vec<String>>>;

/// The function bodies of the `.c` files under `source`, and of its
/// headers' inline helpers (`real_math.h`'s, which the traces compile).
pub fn bodies(source: &Path) -> Result<Bodies, Error> {
    let mut found = Bodies::new();
    for path in walk(source, &["c", "h"])? {
        for (name, body) in bodies_of(&read_text(&path)?) {
            found.entry(name).or_default().extend(body);
        }
    }
    Ok(found)
}

/// The function bodies of one C file.
pub fn bodies_of(c: &str) -> Bodies {
    let mut found = Bodies::new();
    for definition in definitions(c) {
        if definition.kind == Kind::Function {
            found
                .entry(definition.name)
                .or_default()
                .push(definition.refers_to);
        }
    }
    found
}

/// The traced functions of the scripts' C, from [`SOURCES`]'s output: the
/// ones whose body is one of `original`'s.
pub fn read_sources(text: &str, original: &Bodies) -> Traced {
    let mut traced = Traced::new();
    let mut sections: Vec<(&str, String)> = Vec::new();
    for line in text.lines() {
        match line.strip_prefix("### ") {
            Some(script) => sections.push((script, String::new())),
            None => {
                if let Some((_, c)) = sections.last_mut() {
                    c.push_str(line);
                    c.push('\n');
                }
            }
        }
    }
    for (script, c) in sections {
        for definition in definitions(&c) {
            let copied = original
                .get(&definition.name)
                .is_some_and(|bodies| bodies.contains(&definition.refers_to));
            if definition.kind == Kind::Function && copied {
                traced
                    .entry(definition.name)
                    .or_insert_with(|| script.to_owned());
            }
        }
    }
    traced
}

/// `traced.tsv`'s text for `traced`.
pub fn write_traced(traced: &Traced) -> String {
    traced
        .iter()
        .map(|(name, script)| format!("{name}\t{script}\n"))
        .collect()
}

/// The traced functions, asked of the scripts at `root` (Python runs
/// them).
pub fn generate(root: &Path) -> Result<Traced, Error> {
    let output = Command::new("python3")
        .arg("-c")
        .arg(SOURCES)
        .arg(root.join("tools"))
        .output()
        .map_err(|source| Error::Io {
            path: PathBuf::from("python3"),
            source,
        })?;
    if !output.status.success() {
        return Err(Error::Problems(vec![format!(
            "the reference scripts' C could not be read: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )]));
    }
    Ok(read_sources(
        &String::from_utf8_lossy(&output.stdout),
        &bodies(&root.join("source"))?,
    ))
}

/// The scripts whose trace is stale (each script's own `--check`).
pub fn stale(root: &Path) -> Result<Vec<String>, Error> {
    let mut found = Vec::new();
    for script in scripts(root)? {
        let output = Command::new("python3")
            .arg(&script)
            .arg("--check")
            .current_dir(root)
            .output()
            .map_err(|source| Error::Io {
                path: script.clone(),
                source,
            })?;
        if !output.status.success() {
            let why = if output.stderr.is_empty() {
                output.stdout
            } else {
                output.stderr
            };
            found.push(format!(
                "{}: {}",
                script.display(),
                String::from_utf8_lossy(&why).trim()
            ));
        }
    }
    Ok(found)
}
