# Czkawka CLI

CLI frontend. Allows using Czkawka from the terminal.

## Requirements

Precompiled binaries should work without any additional dependencies with Linux (Ubuntu 20.04+), Windows (10+), and macOS (10.15+).

If you decide to compile the app, you will probably be able to run it on even older versions of the OSes, like Ubuntu 16.04 or Windows 7.

On Linux, it is even possible with eyra to avoid libc entirely and using fully static Rust binary.

If you want to use similar videos tool, you must install FFmpeg (runtime dependency) or use HEIF/libraw (build/runtime dependency) and install the required packages:

- Mac - `brew install ffmpeg libraw libheif` - https://formulae.brew.sh/formula/ffmpeg
- Linux - `sudo apt install ffmpeg libraw-dev libheif-dev`
- Windows - `choco install ffmpeg` - Or if not working, download from https://ffmpeg.org/download.html#build-windows and unpack to location with `czkawka_cli.exe`. HEIF and libraw aren't supported on Windows.

## Compilation

For compilation, you must have installed Rust via [rustup](https://rustup.rs) and compile it, e.g., via:

```shell
cargo run --release --bin czkawka_cli
```

You can enable additional features via:

```shell
cargo run --release --bin czkawka_cli --features "heif,libraw"
```

on Linux, to build a fully static binary with eyra, you must use (this is only for crazy people, so just use the command above if you don't know what you're doing):

```shell
rustup default nightly-2024-02-06 # or any newer nightly that works fine with eyra
cd czkawka_cli
cargo add eyra --rename=std
echo 'fn main() { println!("cargo:rustc-link-arg=-nostartfiles"); }' > build.rs
cd ..
cargo build --release --bin czkawka_cli
```

## Limitations

Not all available features in core are available in CLI.

List of missing features:

- Ability to use/choose referenced directories
- See scan progress

## LICENSE

MIT