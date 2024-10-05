# Czkawka GUI

Czkawka GUI is a graphical user interface for Czkawka Core written with GTK 4.

![Screenshot from 2023-11-26 12-43-32](https://github.com/qarmin/czkawka/assets/41945903/722ed490-0be1-4dac-bcfc-182a4d0787dc)

## Requirements

Requirements depend on your platform.

Prebuilt binaries are available here - https://github.com/qarmin/czkawka/releases/

Additional features like heif, libraw, libavif require additional libraries to be installed, and may increase

### Linux

#### Prebuild binaries

Ubuntu - `sudo apt install libgtk-4-bin libheif1 libraw-bin ffmpeg -y`

#### Snap -

none - all needed libraries are bundled in
snap [except ffmpeg](https://github.com/snapcrafters/ffmpeg/issues/73)  - https://snapcraft.io/czkawka

#### Flatpak

none - all needed libraries are bundled - https://flathub.org/apps/com.github.qarmin.czkawka

### Mac

### Homebrew

Czkawka gui is available in homebrew - https://formulae.brew.sh/formula/czkawka and can be installed via

```
brew install czkawka
```

### Manual installation requirements

```

/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install gtk4 adwaita-icon-theme ffmpeg librsvg libheif libraw

```

### Windows

All needed libraries should be bundled in zip (except ffmpeg which you need download and unpack to location
with `czkawka_gui.exe` - https://ffmpeg.org/download.html#build-windows)

You can also install the app via msys2 (webp and heif should work here) - https://www.msys2.org/#installation (czkawka
package - https://packages.msys2.org/base/mingw-w64-czkawka)

```

pacman -S mingw-w64-x86_64-czkawka-gui

```

and you can create a shortcut to `C:\msys64\mingw64\bin\czkawka_gui.exe`

## Compilation

Compiling the gui is harder than compiling cli or core, because it uses gtk4 which is written in C and also requires a
lot build and runtime dependencies.

### Requirements

| Program | Minimal version |
|:-------:|:---------------:|
|  Rust   |     1.79.0      |
|   GTK   |       4.6       |

### Linux (Ubuntu, but on other OS should work similar)

```shell
sudo apt install libgtk-4-dev -y # Base
sudo apt install libgtk-4-dev libheif-dev libraw-dev libavif-dev libdav1d-dev -y # With features
cargo run --release --bin czkawka_gui
# Or with support for heif, libraw, libavif
cargo run --release --bin czkawka_gui --features "heif,libraw,libavif"
```

### Mac

```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rustup gtk4 adwaita-icon-theme ffmpeg librsvg libheif libraw pkg-config
rustup-init
cargo run --release --bin czkawka_gui
# Or with support for heif, libraw, libavif
cargo run --release --bin czkawka_gui --features "heif,libraw,libavif"
```

### Windows

Currently, there is are no instructions on how to compile the app on Windows natively.</br>
You can check for CI for instructions how to cross-compile the app from linux to windows (uses prebuilt docker
image) - [CI Instructions](../.github/workflows/windows.yml)</br>
There exists a mingw recipe which you can try to convert for your
purposes - https://github.com/msys2/MINGW-packages/blob/master/mingw-w64-czkawka/PKGBUILD

## Limitations

Not all available features and/or components implemented here, this is the list of limitations:

- Snap versions does not allow to use the similar videos feature
- Windows version does not support heif and webp files with prebuilt binaries(msys2 version support them)
- Prebuilt binaries for mac arm do not exist
- On Windows, text may appear very small on high resolution displays, a solution is to manually change DPI scaling for
  this app, see :
    - recommended
      fix: [#787#issuecomment-1292253437](https://github.com/qarmin/czkawka/issues/787#issuecomment-1292253437) (modify
      gtk.css),
    - or this
      workaround: [#867#issuecomment-1416761308](https://github.com/qarmin/czkawka/issues/863#issuecomment-1416761308) (
      modify windows DPI settings for this app (this works too but the text is a bit blurry)).

## License

Code is distributed under MIT license.

Icon was created by [jannuary](https://github.com/jannuary) and licensed CC-BY-4.0.

Windows dark theme is used from project [WhiteSur](https://github.com/slypy/whitesur-gtk4-theme) with MIT license.

Some icons were taken from [ReShot](https://www.reshot.com) site and are licensed under Reshot Free License.

The program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"

## Name

Czkawka is a Polish word which means _hiccup_.

I chose this name because I wanted to hear people speaking other languages pronounce it, so feel free to spell it the
way you want.

This name is not as bad as it seems, because I was also thinking about using words like _żółć_, _gżegżółka_ or _żołądź_,
but I gave up on these ideas because they contained Polish characters, which would cause difficulty in searching for the
project.

At the beginning of the program creation, if the response concerning the name was unanimously negative, I prepared
myself
for a possible change of the name of the program, and the opinions were extremely mixed.
