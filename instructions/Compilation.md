# Compiling Czkawka from sources

## Requirements
Program  | Min  | What for
---------|------|------------------------------------------------------------
Rust     | 1.53 | Czkawka, aims to support the latest available version of Rust on Ubuntu 20.04
GTK      | 3.22 | Only for the `GTK` backend

If you only want the terminal version without a GUI, just skip all the packages with `gtk` in their names.

#### Debian / Ubuntu
```shell
sudo apt install -y curl  # Needed by Rust update tool
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo apt install -y libgtk-3-dev libasound2-dev # Latest is optional
```

#### Fedora / CentOS / Rocky Linux
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo yum install gtk3-devel glib2-devel alsa-lib-devel # Latest is optional
```

#### macOS
You need to install Rust via Homebrew and GTK Libraries
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rustup
rustup-init
brew install rust gtk+3
```

### Windows

*Will be available in the future*

For Linux-to-Windows cross-building instruction look at the CI.

<!-- First you need to install Visual C++ components from Visual Studio installer - https://visualstudio.microsoft.com/downloads/
Next install Rust from site https://rustup.rs/
After that the latest GTK 3 runtime must be installed from https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases
-->

## Compilation

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


## Additional features
For now, finding broken audio files is temporary disabled by default, because it crashes when audio libraries are not found on the computer.  
I'm waiting for ability to disable audio playback feature, so after that I will be able to re-enable by default this feature (https://github.com/RustAudio/rodio/issues/349)

To enable checking for broken audio files, just add ` --all-features`
```
cargo run --all-features --bin czkawka_cli -- broken  -d /home/rafal/ -f "results.txt"
```