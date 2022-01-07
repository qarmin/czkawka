# Compiling Czkawka from sources

## Requirements

If you only want the terminal version without a GUI, just skip all the packages with `gtk` in their names.

FFmpeg is not included here, because is not needed to build because it is dynamically loaded.


| Program | Min  | What for                                                                      |
|---------|------|-------------------------------------------------------------------------------|
| Rust    | 1.53 | Czkawka, aims to support the latest available version of Rust on Ubuntu 20.04 |
| GTK     | 3.24 | Only for the `GTK` backend                                                    |

#### Debian / Ubuntu
```shell
sudo apt install -y curl  # Needed by Rust update tool
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo apt install -y libgtk-3-dev
```

#### Fedora / CentOS / Rocky Linux
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo yum install gtk3-devel glib2-devel
```

#### macOS
You need to install Rust via Homebrew and GTK Libraries
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rustup
rustup-init
brew install gtk+3 adwaita-icon-theme librsvg
```

### Windows

*Will be available in the future*

For Linux-to-Windows cross-building instruction look at the CI.

<!-- First you need to install Visual C++ components from Visual Studio installer - https://visualstudio.microsoft.com/downloads/
Next install Rust from site https://rustup.rs/
After that the latest GTK 3 runtime must be installed from https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases
-->

### Docker

```shell
docker build ./misc/docker/ --tag cargo-gtk
```

## Compilation

Czkawka can be installed with Debug or Release build.  
With Debug build additional checks e.g. for variables overflow are available but depends of the usage it works very slow so should be using only to develop this app.    
Compilation with `--release` flag will optimize binaries, so they can be used with good performance(official binaries are build with this flag)


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
For now, finding broken audio files is temporary disabled by default, because app crashes when audio libraries are not found on the computer.  
I'm waiting for ability to disable audio playback feature, so after that I will be able to re-enable by default this feature (https://github.com/RustAudio/rodio/issues/349)

To enable checking for broken audio files, just add ` --all-features`
```
cargo run --all-features --bin czkawka_cli -- broken  -d /home/rafal/ -f "results.txt"
```
