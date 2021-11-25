# Installation
## Requirements
### Linux
If you use Snap, Flatpak or Appimage, you need to only install ffmpeg if you want to use Similar Videos tool.

For Czkawka GUI you are required to have at least `GTK 3.22` and also `Alsa` installed (for finding broken music files, but it is disabled by default).  
`FFmpeg` in Similar Videos is non required dependency - app will work, but this tool will throw errors, so I recommend to install it.  
It should be installed by default on all the most popular distros.
#### Ubuntu/Debian/Linux Mint
```
sudo apt install libgtk-3-dev ffmpeg
```
#### Fedora/Rocky Linux
```
sudo yum install gtk3-devel glib2-devel
sudo dnf -y install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
sudo dnf -y install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm
sudo dnf -y install ffmpeg
```
#### Void Linux (CLI only)
```
sudo xbps-install gcc pkg-config alsa-lib-devel ffpmeg
```

### macOS
Currently, you need to manually install `GTK 3` libraries, `FFmpeg` and the Adwaita theme, because they are dynamically loaded from the OS.  
One very straight-forward way to do this is by using [Homebrew](https://brew.sh/).  
Installation in the terminal:
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install gtk+3 adwaita-icon-theme ffmpeg librsvg
```
After that, go to the location where you downloaded Czkawka and add the `executable` permission to this file.
```shell
chmod +x mac_czkawka_gui
```
At the end execute it:
```shell
./mac_czkawka_gui
```

**Warning**  
If you use want to use app on ARM machine e.g. Mac M1, you can't use prebuilt binaries, because they were compiled for x86_64 architecture. You need to compile Czkawka manually to get proper binaries.

### Windows
By default, all needed libraries are bundled with the app, inside `windows_czkawka_gui.zip`, but if you compile the app or just move `czkawka_gui.exe`, then you will need to install the `GTK 3`
runtime from [**here**](https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases).

FFmpeg to be able to use Similar Videos, you can download and install from this [**link**](https://ffmpeg.org/).

## Installation
### Precompiled binaries
Ready-to-go executables for Linux, Windows and macOS are available [**here**](https://github.com/qarmin/czkawka/releases/).  
If the app does not run when clicking the launcher, run it through a terminal.  
You don't need to have any additional libraries for CLI Czkawka.

### Nightly Builds
Artifacts from each commit can be downloaded [**here**](https://github.com/qarmin/czkawka/actions)

### Appimage
Appimage files are available in release page - [**GitHub releases**](https://github.com/qarmin/czkawka/releases/)  
This version is bundled with its own theme.  
There is also minimal appimage which use system theme.

### Cargo
The easiest method to install Czkawka is using the `cargo` command. To compile it, you need to get all the
requirements from the [compilation section](Compilation.md).
```
cargo install czkawka_gui
cargo install czkawka_cli
```
You can update the package with the same command.

### Snap
```
sudo snap install czkawka
```
By default, Snap can only access the files in your home directory. You have to allow czkawka access to all the drives:

```
sudo snap connect czkawka:removable-media
```

The Snap store entry can be found [**here**](https://snapcraft.io/czkawka).

Fresh builds are available in edge branch, but they may be a little unstable, although that happens very rarely
because I don't push untested code.

### Flatpak
```
flatpak install flathub com.github.qarmin.czkawka
```
Flathub page with Czkawka can be found [**here**](https://flathub.org/apps/details/com.github.qarmin.czkawka)

#
#

**Unofficial packages, which may not always provide the latest version of Czkawka.**

### PPA - Debian/Ubuntu (unofficial)
```
sudo add-apt-repository ppa:xtradeb/apps
sudo apt-get update
sudo apt-get install czkawka
```

alternatively you can use instruction from this [xtradeb site](https://xtradeb.net/wiki/how-to-install-applications-from-this-web-site/)

### AUR - Arch Linux Package (unofficial)
Czkawka is also available in Arch Linux's AUR from which it can be easily installed.
```
yay -Syu czkawka-gui
yay -Syu czkawka-cli
```
or
```
yay -Syu czkawka-gui-bin
yay -Syu czkawka-cli-bin
```

Package info's - https://aur.archlinux.org/packages/?O=0&SeB=nd&K=czkawka&outdated=&SB=n&SO=a&PP=50&do_Search=Go

### Docker image (unofficial)
Czkawka docker image is available [**here**](https://github.com/jlesage/docker-czkawka)

### Chocolatey (unofficial)
Windows Chocolatey binaries are available [**here**](https://community.chocolatey.org/packages/czkawka)