# Czkawka GUI
Czkawka GUI is a graphical user interface for Czkawka Core written with GTK 4.

## Requirements
Requirements depends on platform that you are using:
### Linux
#### Prebuild binaries
  Ubuntu - `sudo apt install libgtk-4 libheif ffmpeg -y`
#### Snap - 
  none - all needed libraries are bundled in snap - except ffmpeg https://github.com/snapcrafters/ffmpeg/issues/73
#### Flatpak
  none - all needed libraries are bundled
### Mac
```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install gtk4 adwaita-icon-theme ffmpeg librsvg libheif
```

### Windows
All needed libraries should be bundled in zip(except ffmpeg which you need download and unpack to location with `czkawka_gui.exe` - https://ffmpeg.org/download.html#build-windows)


|  Program  |  Minimal version  |
|:---------:|:-----------------:|
|   Rust    |      1.72.1       |
|    GTK    |        4.6        |

Prebuild binaries - https://github.com/qarmin/czkawka/releases/ </br>
Snap package - https://snapcraft.io/czkawka </br>
Flatpak package - https://flathub.org/apps/com.github.qarmin.czkawka </br>

## Compilation
Compilation of gui is harder that compilation cli or core, because uses gtk4 which is written in C and also requires a lot build and runtime dependencies.

### Linux (Ubuntu, but on other OS should work similar)
```shell
sudo apt install libgtk-4-dev libheif-dev -y
cargo run --release --bin czkawka_gui
```
### Mac
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rustup gtk4 adwaita-icon-theme ffmpeg librsvg libheif pkg-config
rustup-init
cargo run --release --bin czkawka_gui
```
### Windows
Currently, it is not possible to compile app natively on Windows, but is possible to cross-compile it from Linux.</br>
You can check for CI for instructions how to cross-compile app(uses prebuilt docker image) - [CI Instructions](../.github/workflows/windows.yml)

## Limitations
Not all available features in core are available in GUI and also there are limitations between platforms:
- Snap versions not allows to use similar videos feature
- Windows version not supports heif and webp files
- Prebuild binaries for mac arm not exists

## License
Code is distributed under MIT license.

Icon was created by [jannuary](https://github.com/jannuary) and licensed CC-BY-4.0.

Windows dark theme is used from project [WhiteSur](https://github.com/slypy/whitesur-gtk4-theme) with MIT license.

Some icons were taken from [ReShot](https://www.reshot.com) site and are licensed under Reshot Free License.

The program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"