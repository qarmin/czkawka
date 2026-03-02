#!/usr/bin/env python3
"""
gen_android_icons.py – Generate cedinia Android icon files from an SVG.

Reads:
  cedinia/icons/logo.svg          (or a path you pass as the first argument)

Writes:
  cedinia/res/mipmap-mdpi/ic_launcher.png        48 × 48
  cedinia/res/mipmap-hdpi/ic_launcher.png        72 × 72
  cedinia/res/mipmap-xhdpi/ic_launcher.png       96 × 96
  cedinia/res/mipmap-xxhdpi/ic_launcher.png     144 × 144
  cedinia/res/mipmap-xxxhdpi/ic_launcher.png    192 × 192

  cedinia/res/drawable-nodpi/ic_launcher_fg_src.png   432 × 432
  cedinia/res/drawable/ic_launcher_foreground.xml     (bitmap reference)

The existing ic_launcher_background.xml and mipmap-anydpi-v26/ic_launcher.xml
are NOT modified (they already reference the correct resource names).

Requires one of:
  cairosvg   – pip install cairosvg
  inkscape   – system package / https://inkscape.org

Usage:
  python misc/gen_android_icons.py
  python misc/gen_android_icons.py path/to/custom.svg
"""

import subprocess
import sys
from pathlib import Path

# Legacy launcher icon: density → size in pixels
MIPMAP_SIZES: dict[str, int] = {
    "mdpi":     48,
    "hdpi":     72,
    "xhdpi":    96,
    "xxhdpi":  144,
    "xxxhdpi": 192,
}

# Adaptive icon foreground PNG: 108 dp × 4 (xxxhdpi scale) = 432 px
FOREGROUND_SIZE = 432

# Content of the bitmap drawable that wraps our foreground PNG
FOREGROUND_XML = """\
<?xml version="1.0" encoding="utf-8"?>
<bitmap xmlns:android="http://schemas.android.com/apk/res/android"
    android:src="@drawable/ic_launcher_fg_src"
    android:gravity="center" />
"""

# ── renderers ────────────────────────────────────────────────────────────────

def render_cairosvg(svg: Path, out: Path, size: int) -> None:
    import cairosvg  # type: ignore[import]
    out.parent.mkdir(parents=True, exist_ok=True)
    cairosvg.svg2png(url=str(svg), write_to=str(out), output_width=size, output_height=size)


def render_inkscape(svg: Path, out: Path, size: int) -> None:
    out.parent.mkdir(parents=True, exist_ok=True)
    # Inkscape 1.x uses --export-filename; 0.9x used --export-png.
    result = subprocess.run(
        [
            "inkscape", str(svg),
            f"--export-filename={out}",
            f"--export-width={size}",
            f"--export-height={size}",
        ],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        # Fallback: try the old --export-png flag (Inkscape 0.9x)
        result2 = subprocess.run(
            [
                "inkscape", str(svg),
                f"--export-png={out}",
                f"--export-width={size}",
                f"--export-height={size}",
            ],
            capture_output=True,
            text=True,
        )
        if result2.returncode != 0:
            raise RuntimeError(
                f"inkscape failed (exit {result.returncode}):\n{result.stderr}\n{result2.stderr}"
            )


# ── main ─────────────────────────────────────────────────────────────────────

def main() -> None:
    script_dir = Path(__file__).resolve().parent
    project_root = script_dir.parent  # misc/../ = czkawka/

    if len(sys.argv) >= 2:
        svg = Path(sys.argv[1]).resolve()
    else:
        svg = project_root / "cedinia" / "icons" / "logo.svg"

    if not svg.exists():
        print(f"error: SVG not found: {svg}", file=sys.stderr)
        print("Usage: python misc/gen_android_icons.py [path/to/logo.svg]", file=sys.stderr)
        sys.exit(1)

    res_dir = project_root / "cedinia" / "res"

    # Pick renderer
    try:
        import cairosvg  # type: ignore[import]  # noqa: F401
        render = render_cairosvg
        print("renderer: cairosvg")
    except ImportError:
        render = render_inkscape
        print("renderer: inkscape  (cairosvg not found – install with: pip install cairosvg)")

    print(f"source  : {svg}")
    print()

    # Legacy mipmap PNGs
    for density, size in MIPMAP_SIZES.items():
        out = res_dir / f"mipmap-{density}" / "ic_launcher.png"
        render(svg, out, size)
        print(f"  {str(out.relative_to(project_root)): <60}  {size}×{size} px")

    print()

    # Adaptive icon foreground PNG (drawable-nodpi so the system uses it unscaled)
    fg_png = res_dir / "drawable-nodpi" / "ic_launcher_fg_src.png"
    render(svg, fg_png, FOREGROUND_SIZE)
    print(f"  {str(fg_png.relative_to(project_root)): <60}  {FOREGROUND_SIZE}×{FOREGROUND_SIZE} px")

    # Adaptive icon foreground XML (bitmap wrapper)
    fg_xml = res_dir / "drawable" / "ic_launcher_foreground.xml"
    fg_xml.parent.mkdir(parents=True, exist_ok=True)
    fg_xml.write_text(FOREGROUND_XML, encoding="utf-8")
    print(f"  {str(fg_xml.relative_to(project_root)): <60}  (bitmap drawable)")

    print()
    print("Done.")
    print()
    print("Note: ic_launcher_background.xml and mipmap-anydpi-v26/ic_launcher.xml")
    print("      are NOT modified – they already reference the correct resource names.")


if __name__ == "__main__":
    main()
