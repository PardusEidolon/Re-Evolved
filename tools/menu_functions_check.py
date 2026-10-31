#!/usr/bin/env python3
"""The menus' functions that do nothing yet (port/assets/menus/UNWIRED.md).

The PC version's screens (port/assets/menus/ce/) run functions by name: the
Xbox's own, which the game has, and the PC version's, which
port/linux/game/menu_functions.c answers by name. A name the screens run
that is neither does nothing, and one written "unwired <name>" is skipped on
purpose. This writes the list from the files themselves; --check fails when
the file in the repository is stale.
"""

from __future__ import annotations

import argparse
import html
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parent.parent
MENUS = ROOT / "port/assets/menus"
REPORT = MENUS / "UNWIRED.md"
FUNCTIONS = ROOT / "port/linux/game/menu_functions.c"
TAGS = ROOT / "port/linux/game/menu_tags.c"
HANDLERS = ROOT / "source/interface/ui_widget_event_handler_functions.c"


def table(text: str, start: str, count: int) -> list[str]:
    """the first `count` names a C table of names gives, from `start`"""
    text = text[text.index(start) :]
    return re.findall(r'"([^"\\]*)"', text[: text.index("}")])[:count]


def xbox_names() -> set[str]:
    """the Xbox's own functions' names, as the game has them"""
    handlers = table(HANDLERS.read_text(encoding="latin-1"), '\t{\n\t\t"NULL",', 102)
    inputs = table(TAGS.read_text(encoding="latin-1"), "game_data_input_names[] =", 41)
    return set(handlers) | set(inputs)


# the screens that run these, which nothing opens: the names do nothing, and
# no player reaches them
UNREACHED = {
    "direct ip connect init": "Direct IP's screen: Direct Link is the invite link (`join_game.port`)",
    "direct ip connect update": "Direct IP's screen: Direct Link is the invite link (`join_game.port`)",
    "direct ip edit field": "Direct IP's screen: Direct Link is the invite link (`join_game.port`)",
    "gamespy update filter settings": "the browser's Filters, hidden by `gamespy screen init`",
}


def dispatched(source: str) -> set[str]:
    """the names menu_functions.c answers: by name (`!strcmp(name, "...")`),
    and its own first ones by number (`case _pc_menu_function_quit_game:`
    is "port quit game")"""
    names = set(re.findall(r'strcmp\(name, "([^"\\]*)"\)', source))
    names |= {"port " + case.replace("_", " ") for case in re.findall(r"case _pc_menu_function_(\w+):", source)}
    return names


def used(files: Iterable[str]) -> Counter:
    """each name the screens run or read, with how often"""
    names: Counter = Counter()
    for text in files:
        names.update(html.unescape(name) for name in re.findall(r'(?:run|input)="([^"]*)"', text))
    return names


def unwired(used: dict, xbox: set[str], dispatched: set[str]) -> list[tuple[str, int]]:
    """the names that do nothing, with their uses: those written unwired,
    and the PC version's that nothing answers"""
    found: Counter = Counter()
    for name, count in used.items():
        if name.startswith("unwired "):
            found[name[len("unwired ") :]] += count
        elif name not in xbox and name not in dispatched:
            found[name] += count
    return sorted(found.items())


def report() -> str:
    screens = [path.read_text() for path in sorted((MENUS / "ce").glob("*.xml"))]
    names = used(screens)
    answered = dispatched(FUNCTIONS.read_text(encoding="latin-1"))
    left = unwired(names, xbox_names(), answered)
    wired = sorted(name for name in names if name in answered)
    lines = [
        "# Menu functions that are not wired yet",
        "",
        "The PC version's menus (`ce/`) run functions of the game's by name: the Xbox's own, and the PC",
        "version's, which `port/linux/game/menu_functions.c` answers. Those below do nothing. Written by",
        "`tools/menu_functions_check.py`; `--check` fails when this file is stale.",
        "",
    ]
    missing = [(name, count) for name, count in left if name not in UNREACHED]
    if missing:
        lines += ["| Function | Uses |", "| --- | --- |"]
        lines += [f"| `{name}` | {count} |" for name, count in missing]
    else:
        lines += ["None on a screen a player reaches: every function those run is the Xbox's or answered."]
    unreached = [(name, count) for name, count in left if name in UNREACHED]
    if unreached:
        lines += ["", "On screens nothing opens:", "", "| Function | Uses | Screen |", "| --- | --- | --- |"]
        lines += [f"| `{name}` | {count} | {UNREACHED[name]} |" for name, count in unreached]
    lines += ["", f"Wired ({len(wired)}): " + ", ".join(f"`{name}`" for name in wired) + ".", ""]
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail when UNWIRED.md is stale")
    arguments = parser.parse_args()
    text = report()
    if arguments.check:
        if REPORT.read_text() != text:
            print(f"{REPORT.relative_to(ROOT)} is stale: run tools/menu_functions_check.py", file=sys.stderr)
            return 1
        return 0
    REPORT.write_text(text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
