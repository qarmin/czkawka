# Compiling Czkawka from sources

## Requirements

If you only want the terminal version without a GUI, just skip all the packages with `gtk` in their names.

FFmpeg is not included here because it is not needed to build - it is dynamically loaded.

Support for heif images is optional and require to install libheif library.


| Program | Min  | What for                                                                      |
|---------|------|-------------------------------------------------------------------------------|
| Rust    | 1.60 | Czkawka, aims to support the latest available version of Rust on Ubuntu 22.04 |
| GTK     | 4.6  | Only for the `GTK` backend                                                    |

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
You need to install Rust via Homebrew, GTK Libraries and optionally heif library(to have support for heic files, which are quite popular on Mac)
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rustup
rustup-init
brew install gtk4 adwaita-icon-theme librsvg libheif
```

### Windows

*Will be available in the future*

For Linux-to-Windows cross-building instruction look at the CI.

<!-- First you need to install Visual C++ components from Visual Studio installer - https://visualstudio.microsoft.com/downloads/
Next install Rust from site https://rustup.rs/
After that the latest GTK 4 runtime must be installed from(not available yet for GTK 4) https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases
-->

### Docker

```shell
docker build ./misc/docker/ --tag cargo-gtk
```

## Compilation

Czkawka can be installed with Debug or Release build.  
With Debug build additional checks, e.g., variables overflow, are available, but depending on the usage it works very slow, so it should be used only for development purposes.    
Compilation with `--release` flag will optimize binaries, so they can be used with good performance (official binaries are built with this flag)


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
