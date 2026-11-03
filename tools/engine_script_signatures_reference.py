"""The game's script functions, as its table of them declares them, taken
from the sources and compiled, to record each function's name, the type it
returns and its parameters' types, as the native engine's registry must
give them.

    python3 tools/engine_script_signatures_reference.py          # writes the reference
    python3 tools/engine_script_signatures_reference.py --check  # fails if it is stale

The table is source/hs/hs.c's `hs_function_table`, and each entry is the
`<name>_definition` it points at, found by name and compiled for this
machine with source/hs/hs.h's types. A definition with more than one
parameter keeps its first parameter's type in the definition itself and the
rest in the array after it (laid out together on the Xbox, not on this
machine), so each is read from where its definition puts it. The parse and
evaluate procedures are stand-ins: only the signatures are recorded.

The trace is written to tools/traces/script_signatures_reference.txt, which
tools/engine_wasm_traces.py checks game.wasm against.
"""

import argparse
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from harness import enum_with, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "script_signatures_reference.txt"

FAKE_WORLD = r"""
#include <stddef.h>
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;

typedef boolean (*hs_function_parse_proc)(short function_index, long expression_index);
typedef void (*hs_evaluate_procedure)(short function_index, long thread_index, boolean initialize);
"""


def table_entries(hs):
    """The table's entries in order: each definition's name, and whether it
    is a definition with parameters (`&<name>.definition`)."""
    start = hs.index("struct hs_function_table_storage hs_function_table=")
    start = hs.index("{", hs.index("{", start) + 1) + 1
    end = hs.index("\t},", start)
    entries = re.findall(r"&(\w+)(\.definition)?\s*,", hs[start:end])
    return [(name, bool(parameters)) for name, parameters in entries]


def definition(hs, name):
    """A definition's whole declaration, its structure's name, and its parse
    and evaluate procedures' names."""
    match = re.search(r"^static struct (\w+) const " + re.escape(name) + r"\s*=\s*$", hs, re.M)
    if not match:
        raise LookupError(f"definition not found in the sources: {name}")
    end = hs.index("\n};", match.end()) + 3
    text = hs[match.start() : end]
    lines = [line.strip().rstrip(",") for line in text.splitlines()]
    named = next(at for at, line in enumerate(lines) if line.startswith('"'))
    parse, evaluate = lines[named + 1], lines[named + 2]
    for procedure in (parse, evaluate):
        if not re.fullmatch(r"\w+", procedure):
            raise LookupError(f"{name}'s procedures are not where they are expected: {procedure}")
    return text, match.group(1), parse, evaluate


def source_file():
    """The game's table and the definitions it points at, with a driver
    that prints them, as one C file."""
    hs_h = read("source/hs/hs.h")
    hs = read("source/hs/hs.c")
    entries = table_entries(hs)
    definitions = [definition(hs, name) for name, _ in entries]
    kinds = sorted({kind for _, kind, _, _ in definitions if kind != "hs_function_definition"})
    parses = sorted({parse for _, _, parse, _ in definitions} - {"NULL"})
    evaluates = sorted({evaluate for _, _, _, evaluate in definitions} - {"NULL"})
    driver = ["int main(void)", "{"]
    for (name, with_parameters), (_, kind, _, _) in zip(entries, definitions):
        if with_parameters != (kind != "hs_function_definition"):
            raise LookupError(f"the table points at {name} unlike its kind, {kind}")
        whole = f"(&{name}.definition)" if with_parameters else f"(&{name})"
        driver.append(f'\tprintf("native %s %d %d", {whole}->name, {whole}->return_type, {whole}->parameter_count);')
        driver.append(f"\tif ({whole}->parameter_count > 0)")
        driver.append(f'\t\tprintf(" %d", {whole}->parameter_types[0]);')
        if with_parameters:
            count = int(re.search(r"(\d+)_parameters?$", kind).group(1))
            for at in range(1, count):
                driver.append(f"\tif ({whole}->parameter_count > {at})")
                driver.append(f'\t\tprintf(" %d", {name}.parameter_types[{at - 1}]);')
        driver.append('\tprintf("\\n");')
    driver += ["\treturn 0;", "}"]
    return "\n".join(
        [
            FAKE_WORLD,
            enum_with(hs_h, "_hs_type_void"),
            structure(hs_h, "hs_function_definition"),
            *[structure(hs, kind) for kind in kinds],
            *[
                f"static boolean {parse}(short function_index, long expression_index) {{ return FALSE; }}"
                for parse in parses
            ],
            *[
                f"static void {evaluate}(short function_index, long thread_index, boolean initialize) {{}}"
                for evaluate in evaluates
            ],
            *[text for text, _, _, _ in definitions],
            *driver,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="script-signatures-reference-") as work:
        source = Path(work) / "script_signatures_reference.c"
        executable = Path(work) / "script_signatures_reference"
        source.write_text("#define FALSE 0\n" + source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    if "assert" in output:
        raise SystemExit("the game's code asserted")
    return (
        "# The game's script functions (tools/engine_script_signatures_reference.py).\n"
        "# native <name> <the type it returns> <its parameters' count> <their types>"
        " (types numbered as source/hs/hs.h's enum hs_type numbers them)\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_script_signatures_reference.py")
        print("the script signatures reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
