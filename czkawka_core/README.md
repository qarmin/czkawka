# Czkawka Core

Core scanning library shared by Czkawka GUI, Czkawka CLI, Krokiet, and Cedinia.

## Overview

`czkawka_core` implements all the scanning tools (duplicate files, empty folders, similar images, similar videos, broken files, and more). It has no UI dependency and is designed to be embedded in any frontend.

## Requirements

The build has no native dependencies - it is pure Rust (including HEIC/HEIF and AVIF decoding).

The `similar_videos` tool requires **ffmpeg** at runtime (not a build dependency).

### Linux (Ubuntu / Debian)

```shell
# Runtime: similar videos
sudo apt install ffmpeg
```

### macOS

```shell
brew install ffmpeg
```

### Windows

- ffmpeg: `choco install ffmpeg` or download from [ffmpeg.org](https://ffmpeg.org/download.html#build-windows) and place `ffmpeg.exe` in your `PATH`.

## Compilation

```shell
cargo build --release -p czkawka_core
```

## License

MIT
