# Compiling Czkawka from sources

This instruction is outdated and will be removed in one of next version, please look at README.md files in each module
folder for more up to date instructions.

- [Czkawka GUI (GTK frontend)](../czkawka_gui/README.md)</br>
- [Czkawka CLI](../czkawka_cli/README.md)</br>
- [Czkawka Core](../czkawka_core/README.md)</br>
- [Krokiet GUI (Slint frontend)](../krokiet/README.md)</br>

## Requirements

If you only want the terminal version without a GUI, just skip all the packages with `gtk` in their names.

FFmpeg is not included here because it is not needed to build - it is dynamically loaded.

Support for heif images is optional and require to install libheif library.

New versions of GTK fixes some bugs, so e.g. middle button selection will work only with GTK >=4.8.

| Program | Min    | What for                                                                             |
|---------|--------|--------------------------------------------------------------------------------------|
| Rust    | 1.79.0 | The minimum version of rust does not depend on anything, so it can change frequently |
| GTK     | 4.6    | Only for the `GTK` backend                                                           |

#### Debian / Ubuntu

```shell
sudo apt install -y curl git build-essential # Needed by Rust update tool
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo apt install -y libgtk-4-dev
```

#### Fedora / CentOS / Rocky Linux

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo yum install gtk4-devel glib2-devel
```

#### macOS

You need to install Rust via Homebrew, GTK Libraries and optionally heif library(to have support for heic files, which
are quite popular on Mac)

```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rustup
rustup-init
brew install gtk4 adwaita-icon-theme librsvg libheif webp-pixbuf-loader
```

### Windows

Compiling Czkawka on Windows is possible, but due using GTK is very hard.

In CI we use cross compilation which simplify a lot of things, so for now there is no instruction how to compile native
binaries on Windows.

### Docker

```shell
docker build ./misc/docker/ --tag cargo-gtk
```

## Compilation

Czkawka can be installed with Debug or Release build.  
With Debug build additional checks, e.g., variables overflow, are available, but depending on the usage it works very
slow, so it should be used only for development purposes.    
Compilation with `--release` flag will optimize binaries, so they can be used with good performance (official binaries
are built with this flag)

- Download the source

```
git clone https://github.com/qarmin/czkawka.git
cd czkawka
```

- Compile and run GTK GUI

```
cargo run --release --bin czkawka_gui
```

- Compile and run CLI (by default this will print help with examples)

```
cargo run --release --bin czkawka_cli
```

## Compilation with Docker

```shell
docker run -t --rm --volume $PWD:/app --workdir /app cargo-gtk cargo build --release --bin czkawka_gui
```

Run the binary:

```shell
target/release/czkawka_gui
```

## Additional features

Currently, the only additional dependence is heif image support.

To enable checking for heif images, just add ` --all-features` or `--features heif`

```
cargo run --features heif --bin czkawka_cli -- image  -d /home/rafal/ -f "results.txt"
```

**Be aware, that heif support is not available on Windows, so you can't compile it with this feature, because
mingw-w64-x86_64-libheif is not available in fedora repos, which are used for cross compilation.**