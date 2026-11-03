#!/usr/bin/env python3
"""The game's settings, for the runtime's config_* imports.

The native ports keep every setting in one table (port/linux/src/
port_config.c: its name, type, default as the file writes it, the HALO_*
variable that overrides it and how). game.wasm asks the platform for them
by name, so the runtime answers from the same table, which this script
copies into engine/crates/runtime/src/settings.tsv, one setting a line:

    name <tab> type <tab> default <tab> variable <tab> style

where a string's default is without its quotes (the table's defaults have no
escapes), the variable is empty when there is none, and the style is
"value", "set_is_true" or "set_is_false". --check fails when the file is
stale.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import List, Tuple

ROOT = Path(__file__).resolve().parent.parent
TABLE = Path("port/linux/src/port_config.c")
SETTINGS = Path("engine/crates/runtime/src/settings.tsv")

# the macros the table's defaults use, as the desktop builds define them (C
# literals of the file's text)
MACROS = {"CROUCH_KEYS": r'"\"Left Ctrl, C\""'}

ROW = re.compile(
    r'\{\s*"(?P<name>[^"]+)",\s*_config_(?P<type>\w+),\s*'
    r'(?P<default>"(?:[^"\\]|\\.)*"|\w+),\s*'
    r'(?P<variable>NULL|"[^"]*"),\s*_environment_(?P<style>\w+),'
)


def c_string(literal: str) -> str:
    """a C string literal's text (the table's use only \\" escapes)"""
    if not (literal.startswith('"') and literal.endswith('"')):
        raise ValueError(f"not a string literal: {literal}")
    return literal[1:-1].replace('\\"', '"')


def settings(source: str) -> List[Tuple[str, str, str, str, str]]:
    """the table's settings, in its order"""
    start = source.index("config_settings[] =")
    end = source.index("};", start)
    rows = []
    for match in ROW.finditer(source[start:end]):
        default = c_string(MACROS.get(match["default"], match["default"]))
        if match["type"] == "string":
            default = c_string(default)
        variable = "" if match["variable"] == "NULL" else c_string(match["variable"])
        rows.append((match["name"], match["type"], default, variable, match["style"]))
    if not rows:
        raise ValueError("no settings in the table")
    return rows


def text() -> str:
    rows = settings((ROOT / TABLE).read_text())
    return "".join("\t".join(row) + "\n" for row in rows)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--check", action="store_true", help="fail when the settings file is stale")
    args = parser.parse_args()
    wanted = text()
    path = ROOT / SETTINGS
    if args.check:
        if not path.exists() or path.read_text() != wanted:
            sys.exit(f"engine_settings: {SETTINGS} is stale (run tools/engine_settings.py)")
        return
    path.write_text(wanted)
    print(path)


if __name__ == "__main__":
    main()
