# Czkawka Core

Core scanning library shared by Czkawka GUI, Czkawka CLI, Krokiet, and Cedinia.

## Overview

`czkawka_core` implements all the scanning tools (duplicate files, empty folders, similar images, similar videos, broken files, and more). It has no UI dependency and is designed to be embedded in any frontend.

## Requirements

The base build has no mandatory native dependencies – it is pure Rust.

Optional features require additional native libraries:

| Feature   | Library               | Purpose                  |
|-----------|-----------------------|--------------------------|
| `heif`    | `libheif`             | HEIF/HEIC image support  |
| `libraw`  | `libraw`              | RAW camera image support |
| `libavif` | `libavif`, `libdav1d` | AVIF image support       |

The `similar_videos` tool requires **ffmpeg** at runtime (not a build dependency).

### Linux (Ubuntu / Debian)

```shell
# Runtime: similar videos
sudo apt install ffmpeg 

# Optional build + runtime: extra image formats
sudo apt install libheif-dev libraw-dev libavif-dev libdav1d-dev
```

### macOS

```shell
brew install ffmpeg libraw libheif libavif dav1d
```

### Windows

- ffmpeg: `choco install ffmpeg` or download from [ffmpeg.org](https://ffmpeg.org/download.html#build-windows) and place `ffmpeg.exe` in your `PATH`.
- `heif` and `libraw` features are very hard to set up on Windows and are not available in prebuild binaries(there are some unofficial builds, that enables this features)

## Compilation

```shell
# Base (no optional features)
cargo build --release -p czkawka_core

# With all optional image format features
cargo build --release -p czkawka_core --features "heif,libraw,libavif"
```

## License

MIT
