"""The map audit's driver (tools/engine_audit.py): which scripts are a
map's cutscenes and how each is played, and how two recorded runs are
compared (frames by region, the sound's loudness and bands, the script
events in order, frame times), with thresholds taken from two stock runs.
Pure functions only: the runs themselves need the user's maps and are
made by `engine_audit.py run`."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))

import engine_audit as audit  # noqa: E402
from frame_compare import Image  # noqa: E402


def test_cutscenes_are_called_or_woken_and_startup_scripts_are_left_to_the_run() -> None:
    listing = "\n".join(
        [
            "cutscene_intro\tstatic",
            "cutscene_one_rescued_a\tdormant",
            "cutscene_insertion\tstartup",
            "cinematic_skip_start\tstatic",
            "cinematic_skip_stop\tstatic",
            "cinematic_finale\tstatic",
            "music_loop\tcontinuous",
            "cutscene_watch\tcontinuous",
            "x20_stub\tstub",
        ]
    )
    assert audit.cutscenes(listing) == [
        "cutscene_intro",
        "wake cutscene_one_rescued_a",
        "cinematic_finale",
    ]


def test_a_tsv_is_read_as_rows_by_its_header() -> None:
    rows = audit.parse_tsv("tick\tevent\tscript\ttype\n0\tstart\t12\t0\n64\tend\t-1\t2\n")
    assert rows == [
        {"tick": "0", "event": "start", "script": "12", "type": "0"},
        {"tick": "64", "event": "end", "script": "-1", "type": "2"},
    ]
    assert audit.parse_tsv("") == []


def test_region_differences_find_the_quarter_that_changed() -> None:
    black = Image(8, 8, bytes(8 * 8 * 4))
    changed = bytearray(black.pixels)
    # the top-left 2x2 region of a 4x4 grid turns white
    for y in range(2):
        for x in range(2):
            changed[(y * 8 + x) * 4 : (y * 8 + x) * 4 + 3] = b"\xff\xff\xff"
    regions = audit.region_differences(black, Image(8, 8, bytes(changed)), grid=4)
    assert len(regions) == 16
    assert regions[0] == 255.0
    assert all(value == 0.0 for value in regions[1:])
    assert audit.region_differences(black, black) == [0.0] * 16


def test_sound_is_compared_second_by_second_above_the_floor() -> None:
    header = "second\tdbfs\tb63\tb125\n"
    a = audit.parse_tsv(header + "0\t-30.00\t-40\t-41\n1\t-120.00\t-120\t-120\n2\t-20\t-30\t-31\n")
    b = audit.parse_tsv(header + "0\t-31.50\t-40\t-45\n1\t-120.00\t-120\t-120\n")
    assert audit.sound_difference(a, b) == {"seconds": 2, "dbfs": 1.5, "band": 4.0, "dbfs_at": 0, "band_at": 0}
    assert audit.sound_difference([], b) == {"seconds": 0, "dbfs": 0.0, "band": 0.0, "dbfs_at": None, "band_at": None}


def test_script_events_compare_by_order_and_by_how_far_they_moved() -> None:
    a = audit.parse_tsv("tick\tevent\tscript\ttype\n0\tstart\t12\t0\n64\tstart\t-1\t2\n542\tend\t-1\t2\n")
    same_later = audit.parse_tsv("tick\tevent\tscript\ttype\n0\tstart\t12\t0\n70\tstart\t-1\t2\n540\tend\t-1\t2\n")
    assert audit.event_difference(a, same_later) == {"missing": 0, "extra": 0, "shift": 6}
    lost = audit.parse_tsv("tick\tevent\tscript\ttype\n0\tstart\t12\t0\n")
    assert audit.event_difference(a, lost) == {"missing": 2, "extra": 0, "shift": 0}
    assert audit.event_difference(lost, a)["extra"] == 2


def test_two_runs_part_at_the_first_tick_their_state_differs() -> None:
    header = "tick\tstate\tobjects@81a00000\tactor@81a10000\n"
    a = audit.parse_tsv(header + "0\taa\t01\t02\n1\tab\t01\t03\n2\tac\t04\t05\n")
    b = audit.parse_tsv(header + "0\taa\t01\t02\n1\tbb\t01\t09\n2\tbc\t07\t05\n")
    assert audit.state_difference(a, b) == {"parted": 1, "arrays": ["actor@81a10000"], "ticks": 2}
    # (the same, as far as the shorter run went)
    assert audit.state_difference(a, a[:2]) == {"parted": None, "arrays": [], "ticks": 2}
    # (the whole differs where no array does: the state outside them)
    c = audit.parse_tsv(header + "0\tff\t01\t02\n")
    assert audit.state_difference(a, c) == {"parted": 0, "arrays": [], "ticks": 1}
    assert audit.state_difference([], b) == {"parted": None, "arrays": [], "ticks": 0}


def test_frame_times_skip_the_load_and_give_percentiles() -> None:
    times = [1693.6] + [10.0] * 98 + [30.0]
    stats = audit.frame_stats(times, skip=1)
    assert stats["frames"] == 99
    assert stats["p50"] == 10.0
    assert stats["p99"] == 30.0
    assert audit.frame_stats([], skip=1) == {"frames": 0, "p50": 0.0, "p99": 0.0, "max": 0.0}


def test_thresholds_are_the_stock_runs_own_spread_with_a_floor() -> None:
    noise = {"region": 0.4, "dbfs": 0.0, "band": 2.0, "shift": 3, "missing": 0, "extra": 0}
    limits = audit.thresholds(noise)
    assert limits["region"] == audit.FLOORS["region"]
    assert limits["band"] == 4.0
    assert limits["shift"] == 6
    assert limits["missing"] == 0


def test_findings_name_each_measure_over_its_threshold() -> None:
    limits = {"region": 8.0, "dbfs": 1.0, "band": 3.0, "shift": 30, "missing": 0, "extra": 0}
    metrics = {"region": 12.5, "dbfs": 0.5, "band": 3.0, "shift": 31, "missing": 1, "extra": 0}
    found = audit.findings("a30", "shipped", metrics, limits)
    assert [finding["measure"] for finding in found] == ["region", "shift", "missing"]
    assert found[0] == {
        "map": "a30",
        "variant": "shipped",
        "measure": "region",
        "value": 12.5,
        "limit": 8.0,
        "at": "",
    }
    # a frame's finding names the frame (its tick) and the region of the 4x4 grid
    metrics["worst"] = {"frame": "tick_03000.png", "region": 5}
    assert audit.findings("a30", "shipped", metrics, limits)[0]["at"] == "tick 3000, region 5"
    # the sound's, the second it was worst at
    metrics.update(dbfs=2.0, dbfs_at=41)
    assert audit.findings("a30", "shipped", metrics, limits)[1]["at"] == "second 41"


def test_every_run_steps_its_clock_a_frame_at_a_time_so_runs_repeat() -> None:
    command = audit.audit_command(Path("out"), "a10", "stock", "wake cutscene_x", 150)
    assert command[1:7] == ["--audit", "out", "--map", "a10", "--seconds", "150"]
    settings = [command[index + 1] for index, word in enumerate(command) if word == "--setting"]
    assert settings == ["debug.frame_clock=true", *audit.STOCK]
    assert command[-2:] == ["--at", f"{audit.CUTSCENE_TICK}:wake cutscene_x"]
    shipped = audit.audit_command(Path("out"), "a10", "shipped", None, 40)
    assert "--at" not in shipped
    assert [word for word in shipped if word.startswith("debug.")] == ["debug.frame_clock=true"]


def test_a_run_played_again_starts_from_an_empty_folder(tmp_path: Path) -> None:
    # (a frame kept at another tick by the last run would be compared as if
    # this run had drawn it)
    folder = tmp_path / "stock" / "a30"
    (folder / "frames").mkdir(parents=True)
    (folder / "frames" / "tick_03001.png").write_bytes(b"old")
    (folder / "summary.json").write_text("{}")
    audit.empty_folder(folder)
    assert folder.is_dir()
    assert list(folder.iterdir()) == []
