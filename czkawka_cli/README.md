# Czkawka CLI

CLI frontend, allows to use Czkawka from terminal.

## Requirements

Precompiled binaries should work without any additional dependencies with Linux(Ubuntu 20.04+), Windows(10+) and macOS(
10.15+).

If you decide to compile the app, you probably will be able to run it on even older versions of OS, like Ubuntu 16.04 or
Windows 7.

On linux it is even possible with eyra to avoid entirely libc and using fully static rust binary.

If you want to use similar videos tool, you need to install ffmpeg(runtime dependency).  
If you want to use heif/libraw/libavif(build/runtime dependency) you need to install required packages(may require
bigger os version than czkawka).

- mac - `brew install ffmpeg libraw libheif libavif` - https://formulae.brew.sh/formula/ffmpeg
- linux - `sudo apt install ffmpeg libraw-dev libheif-dev libavif-dev libdav1d-dev`
- windows - `choco install ffmpeg` - or if not working, download from https://ffmpeg.org/download.html#build-windows and
  unpack to location with `czkawka_cli.exe`, heif and libraw are not supported on windows

## Compilation

For compilation, you need to have installed Rust via rustup - https://rustup.rs/ and compile it e.g. via

```shell
cargo run --release --bin czkawka_cli
```

you can enable additional features via

```shell
cargo run --release --bin czkawka_cli --features "heif,libraw,libavif"
```

on linux to build fully static binary with eyra you need to use (this is only for crazy people, so just use command
above if you don't know what you are doing)

```shell
rustup default nightly-2024-09-25 # or any newer nightly that works fine with eyra
cd czkawka_cli
cargo add eyra --rename=std
echo 'fn main() { println!("cargo:rustc-link-arg=-nostartfiles"); }' > build.rs
cd ..
cargo build --release --bin czkawka_cli
```

## Limitations

Not all available features in core are available in CLI.

List of not available features:

- Ability to use/choose referenced directories

## LICENSE

MIT