"""The new engine's --headless mode: the app runs with no window, its
control socket made for the user alone, runs a command on the game's
console, draws a frame offscreen into a PNG when asked, and quits when
told to. Run inside nix develop with assets/maps/bloodgulch.map."""

from __future__ import annotations

import json
import os
import shutil
import socket
import stat
import subprocess
import tempfile
import time
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
MAP = ROOT / "assets" / "maps" / "bloodgulch.map"
OUT = ROOT / "build" / "engine"
# how long the app may take to load the map and open its socket
READY = 120


def socket_line(log: Path, process: subprocess.Popen) -> str:
    """The app's "control socket: <path>" line from its standard error,
    which goes to a file so that nothing blocks on a pipe; fails when the
    app exits first or the line does not come in time."""
    deadline = time.monotonic() + READY
    while time.monotonic() < deadline:
        for line in log.read_text(errors="replace").splitlines():
            if "control socket: " in line:
                return line
        if process.poll() is not None:
            pytest.fail(f"the app exited ({process.returncode}) before its socket: {log.read_text()}")
        time.sleep(0.2)
    pytest.fail(f"no control socket in {READY} s: {log.read_text()}")


@pytest.fixture(scope="module")
def executable() -> Path:
    if shutil.which("cargo") is None or not Path("/usr/bin/xcrun").exists():
        pytest.skip("needs nix develop's cargo and Xcode's Swift")
    if not MAP.exists():
        pytest.skip("no assets/maps/bloodgulch.map")
    subprocess.run(["python3", str(ROOT / "tools" / "engine_build.py"), "--out", str(OUT)], check=True)
    return OUT / "OpenCE Engine.app" / "Contents" / "MacOS" / "OpenCEEngine"


class Connection:
    def __init__(self, path: str) -> None:
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.settimeout(60)
        self.socket.connect(path)
        self.lines = self.socket.makefile("r", encoding="utf-8")

    def send(self, request: object) -> dict:
        self.socket.sendall((json.dumps(request) + "\n").encode())
        return json.loads(self.lines.readline())

    def ask(self, command: str) -> dict:
        """Runs a command on the game's console, waiting for the console to
        listen, which it does once the game has started."""
        deadline = time.monotonic() + READY
        while True:
            answer = self.send(command)
            if answer.get("ok") or "not listening" not in answer.get("error", ""):
                return answer
            if time.monotonic() > deadline:
                return answer
            time.sleep(0.5)


def test_headless_answers_and_draws(executable: Path) -> None:
    folder = Path(tempfile.mkdtemp(prefix="oce"))
    path = folder / "run" / "c.sock"
    cache = OUT / "cache"
    cache.mkdir(parents=True, exist_ok=True)
    log = folder / "app.log"
    process = subprocess.Popen(
        [
            str(executable),
            "--headless",
            "--socket",
            str(path),
            "--map",
            str(MAP),
            "--cache",
            str(cache),
            "--tick-rate",
            "60",
        ],
        stdout=subprocess.DEVNULL,
        stderr=log.open("w"),
    )
    try:
        line = socket_line(log, process)
        assert line.endswith(str(path)), line
        assert stat.S_IMODE(os.stat(path).st_mode) == 0o600
        assert stat.S_IMODE(os.stat(path.parent).st_mode) == 0o700

        connection = Connection(str(path))
        answer = connection.ask("game_speed")
        assert answer["ok"] and "text" in answer, answer
        png = folder / "frame.png"
        assert connection.send({"request": "frame", "path": str(png)})["ok"]
        deadline = time.monotonic() + 30
        while not (png.exists() and png.stat().st_size > 24) and time.monotonic() < deadline:
            time.sleep(0.1)
        assert png.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n"
        assert connection.send({"request": "quit"})["ok"]
        assert process.wait(timeout=30) == 0
        assert not path.exists()
    finally:
        if process.poll() is None:
            process.kill()
            process.wait()
        shutil.rmtree(folder, ignore_errors=True)
