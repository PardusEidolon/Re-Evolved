"""The game's settings for the runtime (tools/engine_settings.py): a table
read as the native ports write it, and the runtime's copy of the real one
kept current.

    python -m pytest -q tools/test_engine_settings.py
"""

import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from tools import engine_settings  # noqa: E402

TABLE = r'''
static const struct config_setting config_settings[] =
{
	{ "display.fullscreen", _config_boolean, "true", "HALO_FULLSCREEN", _environment_value, _platform_desktop,
		"Where display.mode is empty." },
	{ "display.mode", _config_string, "\"\"", "HALO_DISPLAY_MODE", _environment_value, _platform_desktop,
		"A comment, with \"quotes\"." },
	{ "audio.enabled", _config_boolean, "true", "HALO_NO_AUDIO", _environment_set_is_false, _platform_all,
		"" },
	{ "controls.crouch", _config_string, CROUCH_KEYS, "HALO_KEY_CROUCH", _environment_value, _platform_all,
		"" },
	{ "debug.memory_watch", _config_boolean, "true", NULL, _environment_value, _platform_android,
		"" },
	{ "audio.volume", _config_real, "1.0", "HALO_VOLUME", _environment_value, _platform_all, "" },
};
'''


def test_a_table_s_settings_are_read_with_their_defaults_and_variables():
    assert engine_settings.settings(TABLE) == [
        ("display.fullscreen", "boolean", "true", "HALO_FULLSCREEN", "value"),
        ("display.mode", "string", "", "HALO_DISPLAY_MODE", "value"),
        ("audio.enabled", "boolean", "true", "HALO_NO_AUDIO", "set_is_false"),
        ("controls.crouch", "string", "Left Ctrl, C", "HALO_KEY_CROUCH", "value"),
        ("debug.memory_watch", "boolean", "true", "", "value"),
        ("audio.volume", "real", "1.0", "HALO_VOLUME", "value"),
    ]


def test_a_table_without_settings_is_refused():
    with pytest.raises(ValueError):
        engine_settings.settings("static const struct config_setting config_settings[] =\n{\n};\n")


def test_the_runtime_s_settings_are_the_native_ports_table():
    path = ROOT / engine_settings.SETTINGS
    assert path.read_text() == engine_settings.text(), "run tools/engine_settings.py"
    names = [line.split("\t")[0] for line in path.read_text().splitlines()]
    assert len(names) == len(set(names)) > 100
