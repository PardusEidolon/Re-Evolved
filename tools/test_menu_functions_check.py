"""The menus' functions that do nothing: those the PC version's screens run
that menu_functions.c does not answer, from the files themselves."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

sys.path.insert(0, str(ROOT / "tools"))

import menu_functions_check  # noqa: E402

DISPATCH = """
	{
		char const *name = pc_menu_function_name(function_index);

		if (!strcmp(name, "main menu quit game"))
			return quit(widget);
		else if (!strcmp(name, "ss start game"))
			return start(widget);
	}
"""


def test_a_name_the_screens_run_and_nothing_answers_is_unwired() -> None:
    used = {"main menu quit game": 2, "ss start game": 1, "gamespy select item": 4, "close all menus": 9}
    xbox = {"close all menus"}
    dispatched = menu_functions_check.dispatched(DISPATCH)
    assert dispatched == {"main menu quit game", "ss start game"}
    # (the Xbox's own run as the game has them)
    assert menu_functions_check.unwired(used, xbox, dispatched) == [("gamespy select item", 4)]


def test_a_name_written_unwired_stays_unwired_though_answered() -> None:
    used = {"unwired ss start game": 1, "ss start game": 1}
    assert menu_functions_check.unwired(used, set(), {"ss start game"}) == [("ss start game", 1)]


def test_the_names_the_screens_run_are_counted_from_their_files() -> None:
    xml = '<widget run="ss start game"><child input="gt edit list update" run="ss start game"/></widget>'
    assert menu_functions_check.used([xml]) == {"ss start game": 2, "gt edit list update": 1}


def test_the_report_in_the_repository_is_current() -> None:
    assert menu_functions_check.report() == menu_functions_check.REPORT.read_text()


def test_the_ports_first_functions_are_answered_by_number() -> None:
    source = "\tcase _pc_menu_function_setting_load:\n\t\treturn setting_load(widget);\n"
    assert menu_functions_check.dispatched(source) == {"port setting load"}


def test_a_name_is_read_as_the_file_escapes_it() -> None:
    assert menu_functions_check.used(['<on run="create&amp;edit playlist profile"/>']) == {
        "create&edit playlist profile": 1
    }


def test_every_name_left_is_on_a_screen_nothing_opens() -> None:
    text = menu_functions_check.report()
    assert "None on a screen a player reaches" in text
