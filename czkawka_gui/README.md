# Czkawka GUI

Czkawka GUI is a graphical user interface for Czkawka Core written with GTK 4.

![Screenshot from 2023-11-26 12-43-32](https://github.com/qarmin/czkawka/assets/41945903/722ed490-0be1-4dac-bcfc-182a4d0787dc)

## Requirements

Requirements depend on your platform.

Prebuilt binaries are available here: https://github.com/qarmin/czkawka/releases

### Linux

#### Prebuild binaries

Ubuntu: `sudo apt install libgtk-4 libheif libraw ffmpeg -y`
For

#### Snap

None—All needed libraries are bundled [except FFmpeg](https://github.com/snapcrafters/ffmpeg/issues/73) - https://snapcraft.io/czkawka

#### Flatpak

None—All needed libraries are bundled - https://flathub.org/apps/com.github.qarmin.czkawka

### macOS

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install gtk4 adwaita-icon-theme ffmpeg librsvg libheif libraw
```

### Windows

All needed libraries should be bundled in zip (except FFmpeg, for which you must download and unpack to the location
with `czkawka_gui.exe` - https://ffmpeg.org/download.html#build-windows)

You can also install the app via [msys2](https://www.msys2.org/#installation) (WebP and HEIF should work here). Czkawka
package - https://packages.msys2.org/base/mingw-w64-czkawka)

```
pacman -S mingw-w64-x86_64-czkawka-gui
```

And you can create a shortcut to `C:\msys64\mingw64\bin\czkawka_gui.exe`

## Compilation

Compiling the GUI is harder than compiling the CLI or core because it uses GTK 4, written in C. It also requires a
lot of build and runtime dependencies.

### Requirements

| Program | Minimal version |
|:-------:|:---------------:|
|  Rust   |     1.75.0      |
|   GTK   |       4.6       |

### Linux (Ubuntu, but should be similar on other OSes)

```shell
sudo apt install libgtk-4-dev libheif-dev libraw-dev -y
cargo run --release --bin czkawka_gui
# Or with support for heif and libraw
cargo run --release --bin czkawka_gui --features "heif,libraw"
```

### macOS

```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rustup gtk4 adwaita-icon-theme ffmpeg librsvg libheif libraw pkg-config
rustup-init
cargo run --release --bin czkawka_gui
# Or with support for heif and libraw
cargo run --release --bin czkawka_gui --features "heif,libraw"
```

### Windows

Currently, there are no instructions on how to compile the app on Windows natively.</br>44
You can check for CI for instructions how to cross-compile the app from Linux to Windows (uses prebuilt Docker
image) - [CI Instructions](../.github/workflows/windows.yml)</br>
There is a mingw recipe which you can try to convert for your
purposes - https://github.com/msys2/MINGW-packages/blob/master/mingw-w64-czkawka/PKGBUILD

## Limitations

This is not a list of features, but a list of limitations:

- Snap version doesn't allow using the similar videos feature
- Windows version doesn't support HEIF and WebP files with prebuilt binaries
- Prebuilt binaries for Mac ARM don't exist
- On Windows, text may appear tiny on high-resolution displays; a solution is to manually change DPI scaling for
  this app:
  - [Recommended
    fix](https://github.com/qarmin/czkawka/issues/787#issuecomment-1292253437) (modify gtk.css)
  - Or [this workaround](https://github.com/qarmin/czkawka/issues/863#issuecomment-1416761308) (modify windows DPI
    settings for this app, that works too, but the text is a bit blurry)

## License

Code is distributed under MIT license.

Icon was created by [jannuary](https://github.com/jannuary) and licensed CC-BY-4.0.

Windows dark theme is used from project [WhiteSur](https://github.com/slypy/whitesur-gtk4-theme) with MIT license.

Some icons were taken from [Reshot](https://www.reshot.com) site and are licensed under Reshot Free License.

The program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"

## Name

Czkawka is a Polish word that means _hiccup_.

I chose this name because I wanted to hear people speaking other languages pronounce it, so feel free to spell it the
way you want.

This name isn't as bad as it seems. I was also thinking about using words like _żółć_, _gżegżółka_ or _żołądź_, but
I gave up on these ideas because they contained Polish characters, which would cause difficulty in searching for the
project.

At the beginning of the program creation, if the response concerning the name was unanimously negative, I prepared
myself for a possible name change of the program, and the opinions were extremely mixed.
