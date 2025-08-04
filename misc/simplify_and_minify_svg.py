import subprocess
from pathlib import Path
import sys

# Required apps:
# - inkscape
# - svgcleaner - https://crates.io/crates/svgcleaner

FAKE_RUN = False


def run_cmd(cmd: list[str], cwd: Path | None = None) -> bool:
    print(f"Running: {' '.join(cmd)}")
    if FAKE_RUN:
        return True
    result = subprocess.run(cmd, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.returncode != 0:
        print(f"Error: {result.stderr}")
    return result.returncode == 0


def simplify_svg(svg_path: Path) -> None:
    run_cmd(
        ["inkscape", str(svg_path), "--export-type=svg", "--export-text-to-path", "--export-filename=" + str(svg_path)]
    )

    cleaned_path = svg_path.with_suffix(".cleaned.svg")
    if run_cmd(["svgcleaner", str(svg_path), str(cleaned_path)]):
        if not FAKE_RUN:
            svg_path.unlink()
            cleaned_path.rename(svg_path)
    elif cleaned_path.exists():
        if not FAKE_RUN:
            cleaned_path.unlink()


def main() -> None:
    # First arg is folder to search for svg files
    if len(sys.argv) < 2:
        print("Usage: python simplify_and_minify_svg.py <folder>")
        return
    svg_folder = Path(sys.argv[1])
    svg_files = list(svg_folder.glob("**/*.svg"))
    if not svg_files:
        print(f"No SVG files found in the specified directory: {svg_folder}")
        return

    for svg_file in svg_files:
        simplify_svg(svg_file)


if __name__ == "__main__":
    main()
