"""Puts the results of misc/bench_disk_usage_matrix.sh side by side, as markdown tables."""

import argparse
import re
from pathlib import Path

RESULT_LINE = re.compile(r"^RESULT (\S+) (\S+) ([\d.]+) ([\d.]+)$")
INFO_LINE = re.compile(r"^INFO (.*)$")
BASELINE = "release"
STAGE_LABELS = {
    "layout": "layout (build_layout)",
    "render": "picture (render_chart)",
    "layout_and_render": "layout + picture (build_and_render)",
    "hit_test_per_call_us": "hit_test per call [us]",
}


def read_results(path: Path) -> tuple[dict[tuple[str, str], float], list[str]]:
    medians: dict[tuple[str, str], float] = {}
    info: list[str] = []
    for line in path.read_text().splitlines():
        if result := RESULT_LINE.match(line):
            mode, stage, median, _fastest = result.groups()
            medians[(mode, stage)] = float(median)
        elif details := INFO_LINE.match(line):
            info.append(details.group(1))
    return medians, info


def format_table(header: list[str], rows: list[list[str]]) -> str:
    lines = ["| " + " | ".join(header) + " |", "|" + "|".join(["---"] + ["---:"] * (len(header) - 1)) + "|"]
    lines.extend("| " + " | ".join(row) + " |" for row in rows)
    return "\n".join(lines)


def value_text(value: float | None, unit: str) -> str:
    return "-" if value is None else f"{value:,.1f}{unit}" if value >= 10 else f"{value:,.2f}{unit}"


def build_tables(results: dict[str, dict[tuple[str, str], float]], configs: list[str]) -> str:
    keys = sorted(
        {key for medians in results.values() for key in medians},
        key=lambda key: (key[0], list(STAGE_LABELS).index(key[1]) if key[1] in STAGE_LABELS else 99),
    )
    absolute = [
        [f"{mode} {STAGE_LABELS.get(stage, stage)}"]
        + [value_text(results[config].get((mode, stage)), "") for config in configs]
        for mode, stage in keys
    ]
    out = ["Median of the measured runs, milliseconds:", "", format_table(["stage"] + configs, absolute)]
    if BASELINE in results:
        relative = []
        for mode, stage in keys:
            base = results[BASELINE].get((mode, stage))
            relative.append(
                [f"{mode} {STAGE_LABELS.get(stage, stage)}"]
                + [
                    f"{results[config][(mode, stage)] / base:.2f}x"
                    if base and (mode, stage) in results[config]
                    else "-"
                    for config in configs
                ]
            )
        out += [
            "",
            f"Time relative to `{BASELINE}` (below 1.00x is faster):",
            "",
            format_table(["stage"] + configs, relative),
        ]
    return "\n".join(out)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("results_dir", type=Path, help="directory with one <config>.txt file per configuration")
    parser.add_argument("--configs", help="comma separated configurations, in the order of the columns")
    arguments = parser.parse_args()

    available = {path.stem: path for path in arguments.results_dir.glob("*.txt")}
    configs = [
        name for name in (arguments.configs.split(",") if arguments.configs else list(available)) if name in available
    ]
    results: dict[str, dict[tuple[str, str], float]] = {}
    for config in configs:
        medians, info = read_results(available[config])
        if medians:
            results[config] = medians
            print(f"{config}: {'; '.join(line for line in info if line.startswith(('run ', 'build ')))}")
    if results:
        print()
        print(build_tables(results, [config for config in configs if config in results]))
    else:
        print("No results found")


if __name__ == "__main__":
    main()
