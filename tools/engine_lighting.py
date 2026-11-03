"""The lighting sweeps: each lighting extension switched on alone over
stock, every map played by the map audit's schedule, and its frames
measured against stock's.

  run      plays each map as stock twice (stock, stock2: their spread sets
           the limits) and once with each extension switched on alone,
           into build/engine/lighting/<variant>/<map>/
  compare  each extension against stock, frame by frame and region by
           region: brightness (the mean change in luma), colour (the mean
           change in chroma), seams (pixels where the change itself jumps
           between neighbours: light that stops at a line), and shadow
           edges (stock's edges the extension keeps, and how sharp they
           stay); into sweep.json and sweep_findings.tsv

The extensions are the lighting.* settings (port/linux/src/port_config.c),
each stock by default. Everything is written under build/engine/lighting
and stays on this machine: the frames are drawn from the user's maps."""

from __future__ import annotations

import argparse
import concurrent.futures
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))

import engine_audit as audit  # noqa: E402
from frame_compare import Image, read_png  # noqa: E402

OUT = ROOT / "build" / "engine" / "lighting"
# each extension's setting and the value that switches it on
EXTENSIONS = {
    "placed_lights": "shadowed",
    "bounce": "baked",
    "atmosphere": "scattering",
    "shadows": "ray_traced",
    "reflections": "ray_traced",
}
ALL_STOCK = [f"lighting.{name}=stock" for name in EXTENSIONS]
VARIANTS = ["stock", "stock2", *EXTENSIONS]
# the measures, in the order findings are listed, and the least each
# limit may be: luma and chroma out of 255, pixels, a fraction of edges
MEASURES = ["brightness", "colour", "seams", "lost"]
FLOORS = {"brightness": 2.0, "colour": 2.0, "seams": 0, "lost": 0.05}
# a jump in the change between neighbours that counts as a seam, and a
# luma gradient that counts as an edge
SEAM_STEP = 24.0
EDGE = 32.0


def settings(variant: str) -> list[str]:
    """the settings of a run: the original's look, every extension stock
    but the variant's own"""
    lighting = [f"lighting.{name}={EXTENSIONS[name] if name == variant else 'stock'}" for name in EXTENSIONS]
    return [*audit.STOCK, *lighting]


def sweep_command(folder: Path, name: str, variant: str, seconds: int) -> list[str]:
    command = [str(audit.APP), "--audit", str(folder), "--map", name, "--seconds", str(seconds)]
    for setting in audit.EVERY_RUN + settings(variant):
        command += ["--setting", setting]
    return command


def lumas(frame: Image) -> list[float]:
    """each pixel's luma, Rec. 709 in 256ths so whole greys stay whole"""
    p = frame.pixels
    return [(54 * p[i] + 183 * p[i + 1] + 19 * p[i + 2]) / 256 for i in range(0, len(p), 4)]


def light_regions(stock: Image, variant: Image, grid: int = 4) -> list[dict]:
    """the mean change in luma (signed) and in chroma (red and blue less
    luma, unsigned) in each of grid x grid regions, from the top left"""
    if (stock.width, stock.height) != (variant.width, variant.height):
        return [{"brightness": 255.0, "colour": 255.0}] * (grid * grid)
    a, b = lumas(stock), lumas(variant)
    brightness = [0.0] * (grid * grid)
    colour = [0.0] * (grid * grid)
    counts = [0] * (grid * grid)
    for y in range(stock.height):
        row = y * grid // stock.height
        for x in range(stock.width):
            region = row * grid + x * grid // stock.width
            at = y * stock.width + x
            i = at * 4
            brightness[region] += b[at] - a[at]
            red = (variant.pixels[i] - b[at]) - (stock.pixels[i] - a[at])
            blue = (variant.pixels[i + 2] - b[at]) - (stock.pixels[i + 2] - a[at])
            colour[region] += (abs(red) + abs(blue)) / 2
            counts[region] += 1
    return [
        {"brightness": round(brightness[r] / counts[r], 3), "colour": round(colour[r] / counts[r], 3)}
        if counts[r]
        else {"brightness": 0.0, "colour": 0.0}
        for r in range(grid * grid)
    ]


def seams(stock: Image, variant: Image, step: float = SEAM_STEP) -> int:
    """the pixels where the change from stock jumps by more than `step`
    from the next pixel across or down: light that stops at a line"""
    if (stock.width, stock.height) != (variant.width, variant.height):
        return stock.width * stock.height
    width, height = stock.width, stock.height
    change = [b - a for a, b in zip(lumas(stock), lumas(variant))]
    count = 0
    for y in range(height):
        for x in range(width):
            here = change[y * width + x]
            if x + 1 < width and abs(change[y * width + x + 1] - here) > step:
                count += 1
            if y + 1 < height and abs(change[(y + 1) * width + x] - here) > step:
                count += 1
    return count


def gradients(frame: Image) -> list[float]:
    """each pixel's luma gradient, the central differences across and down
    (the frame's edge repeated)"""
    width, height = frame.width, frame.height
    luma = lumas(frame)
    out = []
    for y in range(height):
        for x in range(width):
            left, right = luma[y * width + max(x - 1, 0)], luma[y * width + min(x + 1, width - 1)]
            up, down = luma[max(y - 1, 0) * width + x], luma[min(y + 1, height - 1) * width + x]
            out.append(abs(right - left) + abs(down - up))
    return out


def shadow_edges(stock: Image, variant: Image, edge: float = EDGE) -> dict:
    """stock's edges (gradient over `edge`): how many, the share the
    variant keeps (over half `edge`), and their gradient in the variant
    over stock's (1.0 as sharp, less softer)"""
    if (stock.width, stock.height) != (variant.width, variant.height):
        return {"edges": 0, "kept": 0.0, "sharpness": 0.0}
    a, b = gradients(stock), gradients(variant)
    edges = [index for index, value in enumerate(a) if value > edge]
    if not edges:
        return {"edges": 0, "kept": 1.0, "sharpness": 1.0}
    kept = sum(1 for index in edges if b[index] > edge / 2)
    return {
        "edges": len(edges),
        "kept": round(kept / len(edges), 4),
        "sharpness": round(sum(b[index] for index in edges) / sum(a[index] for index in edges), 4),
    }


def compare(a: Path, b: Path) -> dict:
    """two runs' frames: the worst region's brightness and colour, the most
    seams, the most edges lost and the least sharpness over the frames both
    kept, and where each was worst"""
    result = {"frames": 0, "brightness": 0.0, "colour": 0.0, "seams": 0, "lost": 0.0, "sharpness": 1.0}
    worst: dict = {}
    for frame in sorted((a / "frames").glob("tick_*.png")):
        other = b / "frames" / frame.name
        if not other.is_file():
            continue
        stock, variant = read_png(frame), read_png(other)
        result["frames"] += 1
        regions = light_regions(stock, variant)
        for measure in ["brightness", "colour"]:
            values = [abs(region[measure]) for region in regions]
            if max(values) > result[measure]:
                result[measure] = max(values)
                worst[measure] = f"{frame.stem}, region {values.index(max(values))}"
        count = seams(stock, variant)
        if count > result["seams"]:
            result["seams"] = count
            worst["seams"] = frame.stem
        edges = shadow_edges(stock, variant)
        if 1.0 - edges["kept"] > result["lost"]:
            result["lost"] = round(1.0 - edges["kept"], 4)
            worst["lost"] = frame.stem
        result["sharpness"] = min(result["sharpness"], edges["sharpness"])
    return {**result, "worst": worst}


def limits(noise: dict) -> dict:
    """twice what two stock runs differ by, and no less than the floor"""
    return {measure: max(type(FLOORS[measure])(noise.get(measure, 0) * 2), FLOORS[measure]) for measure in MEASURES}


def findings(name: str, variant: str, metrics: dict, limit: dict) -> list[dict]:
    return [
        {
            "map": name,
            "variant": variant,
            "measure": measure,
            "value": metrics[measure],
            "limit": limit[measure],
            "at": metrics.get("worst", {}).get(measure, ""),
        }
        for measure in MEASURES
        if metrics[measure] > limit[measure]
    ]


def play(name: str, variant: str, seconds: int, again: bool) -> tuple[str, int]:
    folder = OUT / variant / name
    if (folder / "summary.json").is_file() and not again:
        return str(folder), 0
    folder.mkdir(parents=True, exist_ok=True)
    with open(folder / "app.log", "w") as log:
        done = subprocess.run(
            sweep_command(folder, name, variant, seconds), cwd=ROOT, stdout=log, stderr=subprocess.STDOUT
        )
    return str(folder), done.returncode


def command_run(arguments: argparse.Namespace) -> int:
    names = audit.present(arguments.maps or audit.CAMPAIGN + audit.MULTIPLAYER)
    if not audit.APP.is_file() or not names:
        print("needs the app (tools/engine_build.py) and assets/maps", file=sys.stderr)
        return 2
    jobs = [(name, variant, arguments.seconds) for name in names for variant in arguments.variants]
    failed = 0
    with concurrent.futures.ThreadPoolExecutor(arguments.jobs) as pool:
        futures = [pool.submit(play, *job, arguments.again) for job in jobs]
        for future in concurrent.futures.as_completed(futures):
            folder, code = future.result()
            failed += code != 0
            print(f"{code}\t{Path(folder).relative_to(OUT)}", flush=True)
    print(f"{len(jobs)} runs, {failed} failed")
    return 1 if failed else 0


def command_compare(_: argparse.Namespace) -> int:
    names = sorted(path.name for path in (OUT / "stock").glob("*") if (path / "summary.json").is_file())
    noise, results, found = {}, {}, []
    for name in names:
        if (OUT / "stock2" / name / "summary.json").is_file():
            noise[name] = compare(OUT / "stock" / name, OUT / "stock2" / name)
        limit = limits(noise.get(name, {}))
        for variant in EXTENSIONS:
            if not (OUT / variant / name / "summary.json").is_file():
                continue
            metrics = compare(OUT / "stock" / name, OUT / variant / name)
            log = (OUT / variant / name / "app.log").read_text(errors="replace")
            metrics["said"] = [line for line in log.splitlines() if line.startswith("opence: lighting.")]
            results.setdefault(name, {})[variant] = metrics
            found += findings(name, variant, metrics, limit)
    (OUT / "sweep.json").write_text(json.dumps({"noise": noise, "runs": results}, indent=1))
    with open(OUT / "sweep_findings.tsv", "w") as table:
        table.write("map\tvariant\tmeasure\tvalue\tlimit\tat\n")
        for finding in found:
            table.write(
                "\t".join(str(finding[key]) for key in ["map", "variant", "measure", "value", "limit", "at"]) + "\n"
            )
    runs = sum(len(variants) for variants in results.values())
    print(f"{runs} sweeps over {len(results)} maps, {len(found)} findings")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command", required=True)
    run = commands.add_parser("run")
    run.add_argument("--maps", nargs="*")
    run.add_argument("--variants", nargs="*", default=VARIANTS)
    run.add_argument("--seconds", type=int, default=25)
    run.add_argument("--jobs", type=int, default=3)
    run.add_argument("--again", action="store_true")
    run.set_defaults(handler=command_run)
    commands.add_parser("compare").set_defaults(handler=command_compare)
    arguments = parser.parse_args()
    return arguments.handler(arguments)


if __name__ == "__main__":
    sys.exit(main())
