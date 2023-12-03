# Czkawka GUI
Czkawka GUI is a graphical user interface for Czkawka Core written with GTK 4.

![Screenshot from 2023-11-26 12-43-32](https://github.com/qarmin/czkawka/assets/41945903/722ed490-0be1-4dac-bcfc-182a4d0787dc)

## Requirements
Requirements depends on platform that you are using.

Prebuild binareies are available here - https://github.com/qarmin/czkawka/releases/

### Linux
#### Prebuild binaries
  Ubuntu - `sudo apt install libgtk-4 libheif libraw ffmpeg -y`
#### Snap - 
  none - all needed libraries are bundled in snap [except ffmpeg](https://github.com/snapcrafters/ffmpeg/issues/73)  - https://snapcraft.io/czkawka
#### Flatpak
  none - all needed libraries are bundled - https://flathub.org/apps/com.github.qarmin.czkawka
### Mac
```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install gtk4 adwaita-icon-theme ffmpeg librsvg libheif libraw
```

### Windows
All needed libraries should be bundled in zip(except ffmpeg which you need download and unpack to location with `czkawka_gui.exe` - https://ffmpeg.org/download.html#build-windows)

You can also install app via msys2(webp and heif should work here) - https://www.msys2.org/#installation (czkawka package - https://packages.msys2.org/base/mingw-w64-czkawka)
```
pacman -S mingw-w64-x86_64-czkawka-gui
```
and you can create shortcut to `C:\msys64\mingw64\bin\czkawka_gui.exe`

## Compilation
Compilation of gui is harder than compilation cli or core, because uses gtk4 which is written in C and also requires a lot build and runtime dependencies.

### Requirements
|  Program  |  Minimal version  |
|:---------:|:-----------------:|
|   Rust    |      1.72.1       |
|    GTK    |        4.6        |

### Linux (Ubuntu, but on other OS should work similar)
```shell
sudo apt install libgtk-4-dev libheif-dev libraw-dev -y
cargo run --release --bin czkawka_gui
# Or with support for heif and libraw
cargo run --release --bin czkawka_gui --features "heif,libraw"
```
### Mac
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rustup gtk4 adwaita-icon-theme ffmpeg librsvg libheif libraw pkg-config
rustup-init
cargo run --release --bin czkawka_gui
# Or with support for heif and libraw
cargo run --release --bin czkawka_gui --features "heif,libraw"
```
### Windows
Currently, there is no instruction how to compile app on Windows natively.</br>
You can check for CI for instructions how to cross-compile app from linux to windows(uses prebuilt docker image) - [CI Instructions](../.github/workflows/windows.yml)</br>
There exists mingw recipe which you can try to convert for your purposes - https://github.com/msys2/MINGW-packages/blob/master/mingw-w64-czkawka/PKGBUILD

## Limitations
Not all available features other components implemented here, so this is list of  limitations:
- Snap versions not allows to use similar videos feature
- Windows version not supports heif and webp files with prebuild binaries
- Prebuild binaries for mac arm not exists

## License
Code is distributed under MIT license.

Icon was created by [jannuary](https://github.com/jannuary) and licensed CC-BY-4.0.

Windows dark theme is used from project [WhiteSur](https://github.com/slypy/whitesur-gtk4-theme) with MIT license.

Some icons were taken from [ReShot](https://www.reshot.com) site and are licensed under Reshot Free License.

The program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"

## Name
Czkawka is a Polish word which means _hiccup_.

I chose this name because I wanted to hear people speaking other languages pronounce it, so feel free to spell it the way you want.

This name is not as bad as it seems, because I was also thinking about using words like _żółć_, _gżegżółka_ or _żołądź_,
but I gave up on these ideas because they contained Polish characters, which would cause difficulty in searching for the project.

At the beginning of the program creation, if the response concerning the name was unanimously negative, I prepared myself
for a possible change of the name of the program, and the opinions were extremely mixed.
