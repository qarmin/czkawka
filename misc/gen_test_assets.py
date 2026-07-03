#!/usr/bin/env python3
"""
Generate tiny test image assets embedded in czkawka_core for runtime feature probing.

Run via: uv run --with pillow --with pillow-heif misc/gen_test_assets.py

Outputs:
  czkawka_core/src/common/test_assets/test_3x3.png        (baseline, always present)
  czkawka_core/src/common/test_assets/test_3x3.avif       (libavif runtime probe)
  czkawka_core/src/common/test_assets/test_3x3.heic       (heif HEVC runtime probe)
  czkawka_core/src/common/test_assets/test_heif_av1.avif  (heif AV1 runtime probe, via ffmpeg)
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

try:
    from PIL import Image  # type: ignore[import-not-found]
except ImportError:
    sys.exit("pillow not installed - run: uv run --with pillow misc/gen_test_assets.py")

REPO_ROOT = Path(__file__).parent.parent
OUT_DIR = REPO_ROOT / "czkawka_core" / "src" / "common" / "test_assets"
OUT_DIR.mkdir(parents=True, exist_ok=True)

img = Image.new("RGB", (3, 3), color=(128, 64, 192))

# PNG - baseline, always available
png_path = OUT_DIR / "test_3x3.png"
img.save(png_path)
print(f"PNG:  {png_path.stat().st_size} bytes")

# AVIF - requires Pillow built with AVIF support (Pillow >= 9.1 + libavif)
try:
    avif_path = OUT_DIR / "test_3x3.avif"
    img.save(avif_path)
    print(f"AVIF: {avif_path.stat().st_size} bytes")
except Exception as exc:
    print(f"AVIF: FAILED ({exc})")

# HEIC (HEVC) - requires pillow-heif (bundles x265 encoder)
try:
    from pillow_heif import register_heif_opener  # type: ignore[import-not-found]

    register_heif_opener()
    heic_path = OUT_DIR / "test_3x3.heic"
    img.save(heic_path)
    print(f"HEIC: {heic_path.stat().st_size} bytes")
except ImportError:
    print("HEIC: SKIPPED (install pillow-heif: uv run --with pillow --with pillow-heif misc/gen_test_assets.py)")
except Exception as exc:
    print(f"HEIC: FAILED ({exc})")

# HEIF AV1 - ffmpeg with libaom-av1 encoder; used to probe libheif's AV1 decode path
av1_path = OUT_DIR / "test_heif_av1.avif"
try:
    result = subprocess.run(
        [
            "ffmpeg",
            "-y",
            "-f",
            "lavfi",
            "-i",
            "color=c=0x406080:s=3x3:d=1",
            "-vframes",
            "1",
            "-c:v",
            "libaom-av1",
            "-still-picture",
            "1",
            str(av1_path),
        ],
        capture_output=True,
        check=True,
    )
    print(f"HEIF AV1: {av1_path.stat().st_size} bytes")
except FileNotFoundError:
    print("HEIF AV1: SKIPPED (ffmpeg not found)")
except subprocess.CalledProcessError as exc:
    print(f"HEIF AV1: FAILED - {exc.stderr.decode(errors='replace').splitlines()[-1]}")

print(f"\nAssets written to {OUT_DIR}")
