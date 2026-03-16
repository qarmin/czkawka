![czkawka_logo](https://user-images.githubusercontent.com/41945903/102616149-66490400-4137-11eb-9cd6-813b2b070834.png)

Czkawka GUI is a graphical user interface for Czkawka Core, built with GTK 4.

![Screenshot from 2023-11-26 12-43-32](https://github.com/qarmin/czkawka/assets/41945903/722ed490-0be1-4dac-bcfc-182a4d0787dc)

## Maintenance Mode

Czkawka GTK is currently in maintenance mode.  
This means that new features will be kept to an absolute minimum, and only critical bugs will be fixed.  Compatibility updates with the Czkawka core package will still be provided to ensure that the application continues to compile correctly.  
Active development is now focused on the Krokiet GUI.

## Requirements

Requirements depend on your platform.

Prebuilt binaries are available here: https://github.com/qarmin/czkawka/releases/

Additional features such as HEIF, libraw, and libavif require extra libraries to be installed, which may increase the number of dependencies.

### Linux

#### Prebuilt binaries / Self-compiled

Ubuntu:  
`sudo apt install libgtk-4-bin libheif1 libraw-bin ffmpeg -y`

### Mac

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install gtk4 ffmpeg librsvg libheif libraw dav1d
```

### Windows

#### Prebuilt binaries
All required libraries are bundled in the zip (except ffmpeg, which you can install manually and place `ffmpeg.exe` in a directory included in your system PATH).

## Installation

### Prebuilt binaries (All OS)
After installing the required dependencies, download the prebuilt binaries for your platform from the [releases page](https://github.com/qarmin/czkawka/releases).

### Linux

#### Flatpak
```
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install flathub com.github.qarmin.czkawka
```

#### Debian package (Unofficial) 
Requires Debian 13 (or derivatives) or later.
```
sudo apt install czkawka_gui
```

#### PPA (Unofficial) - Debian-based distributions (Ubuntu, Linux Mint, etc.)
```
sudo add-apt-repository ppa:xtradeb/apps
sudo apt update
sudo apt install czkawka
```
[PPA page](https://launchpad.net/~xtradeb/+archive/ubuntu/apps)

### Mac

#### Homebrew (Unofficial)
```
brew install czkawka
```
[Formula page](https://formulae.brew.sh/formula/czkawka)

### Windows

#### MSYS2 (Unofficial)
```
pacman -S mingw-w64-x86_64-czkawka-gui
```
[Package link](https://packages.msys2.org/base/mingw-w64-czkawka)

The file should be installed to `C:\msys64\mingw64\bin\czkawka_gui.exe` and can be run from there.  
This version is likely the most feature-complete on Windows, as it is compiled with optional features enabled.

## Compilation

Compiling the GUI is more complex than compiling the CLI, core, or Krokiet, because it uses GTK4 (written in C) and requires many build and runtime dependencies.

### Requirements

| Program | Minimal version |
|:-------:|:---------------:|
|  Rust   |     1.92.0      | 
|   GTK   |       4.6       |

The Rust version corresponds to the latest rustc available in Debian Sid: https://packages.debian.org/sid/rustc

### Linux (Ubuntu; similar steps apply to other distributions)

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
brew install rustup gtk4 adwaita-icon-theme ffmpeg librsvg libheif libraw dav1d pkg-config
rustup-init
cargo run --release --bin czkawka_gui
# Or with support for heif, libraw, libavif
cargo run --release --bin czkawka_gui --features "heif,libraw,libavif"
```

### Windows

Currently, there are no instructions for compiling the app natively on Windows.</br>
You can check the CI for instructions on how to cross-compile the app from Linux to Windows (using a prebuilt Docker image): [CI Instructions](../.github/workflows/windows.yml)</br>
There is also a mingw recipe you can try to adapt for your needs: https://github.com/msys2/MINGW-packages/blob/master/mingw-w64-czkawka/PKGBUILD

## Limitations

Not all features and components are implemented here. The main limitations are:

- The Windows version does not support HEIF and WebP files with prebuilt binaries (the MSYS2 version supports them).
- On Windows, text may appear very small on high-resolution displays. You can manually change DPI scaling for this app:
    - [Recommended fix](https://github.com/qarmin/czkawka/issues/787#issuecomment-1292253437) (modify gtk.css)
    - [Alternative workaround](https://github.com/qarmin/czkawka/issues/863#issuecomment-1416761308) (modify Windows DPI settings for this app; this works too, but the text may be a bit blurry).

## License

The code is distributed under the MIT license.

The icon was created by [jannuary](https://github.com/jannuary) and is licensed under CC-BY-4.0.

The Windows dark theme is from the [WhiteSur](https://github.com/slypy/whitesur-gtk4-theme) project, licensed under MIT.

The program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"

## Name

Czkawka is a Polish word meaning _hiccup_.

I chose this name because I wanted to hear people speaking other languages pronounce it, so feel free to say it however you like.

This name is not as difficult as it seems; I also considered words like _żółć_, _gżegżółka_, or _żołądź_, but decided against them because they contain Polish characters, which would make searching for the project harder.

At the beginning of the project, if the response to the name was unanimously negative, I was prepared to change it, but the opinions were extremely mixed.
