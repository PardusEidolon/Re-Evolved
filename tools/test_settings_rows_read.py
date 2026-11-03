"""No dummy switch: every row of a settings screen the menus reach names a
setting the game reads, in game.wasm (the C code's config_* calls take the
key as a string, kept with its NUL) or in the runtime (display.rs, input.rs
and the rest read theirs by name). A row nothing reads would be a switch
that does nothing. Network Setup's screens are not reached: the app has no
network (engine/crates/runtime/src/net.rs), and menu_tags.c hides its row."""

import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))

import port_settings  # noqa: E402

WASM = ROOT / "build/engine/wasm/game.wasm"
# (the screens below Network Setup's hidden row)
UNREACHED = "network_setup"


def runtime_source():
    return "".join(path.read_text() for path in sorted((ROOT / "engine/crates/runtime/src").rglob("*.rs")))


def rows():
    return [
        (name, row[1])
        for name, screen in port_settings.SCREENS.items()
        if not name.startswith(UNREACHED)
        for row in screen["rows"]
    ]


@pytest.mark.parametrize("screen,key", rows())
def test_the_row_names_a_setting_the_game_reads(screen, key):
    if not WASM.is_file():
        pytest.skip("needs build/engine/wasm/game.wasm")
    read = (key + "\0").encode() in WASM.read_bytes() or f'"{key}"' in runtime_source()
    assert read, f"{screen}'s {key} is read by nothing: a switch that does nothing"


def test_the_network_row_is_hidden():
    tags = (ROOT / "port/linux/game/menu_tags.c").read_text()
    hidden = tags.split("static boolean settings_item_hidden", 1)[1].split("\n}\n", 1)[0]
    assert "SETTINGS_NETWORK_ITEM" in hidden, "menu_tags.c's settings_item_hidden leaves Network Setup's row"


def test_no_row_is_for_network_games_alone():
    # (read by network_voice.c, which has no network to talk over)
    online = {"audio.voice_chat", "audio.voice_volume"}
    assert not online & {key for _, key in rows()}
