![com github qarmin czkawka](https://user-images.githubusercontent.com/41945903/102616149-66490400-4137-11eb-9cd6-813b2b070834.png)

**Czkawka** is a simple, fast and easy to use app to remove unnecessary files from your computer.

## Features
- Written in memory safe Rust
- Amazingly fast - due to using more or less advanced algorithms and multithreading
- Free, Open Source without ads
- Multiplatform - works on Linux, Windows and macOS
- Cache support - second and further scans should be a lot of faster than the first
- CLI frontend - for easy automation
- GUI frontend - uses modern GTK 3 and looks similar to FSlint
- Rich search option - allows setting absolute included and excluded directories, set of allowed file extensions 
  or excluded items with the `*` wildcard
- Multiple tools to use:
  - Duplicates - Finds duplicates basing on file name, size, hash, first 1 MB of hash
  - Empty Folders - Finds empty folders with the help of an advanced algorithm
  - Big Files - Finds the provided number of the biggest files in given location
  - Empty Files - Looks for empty files across the drive
  - Temporary Files - Finds temporary files
  - Similar Images - Finds images which are not exactly the same (different resolution, watermarks)
  - Zeroed Files - Finds files which are filled with zeros (usually corrupted)
  - Same Music - Searches for music with same artist, album etc.
  - Invalid Symbolic Links - Shows symbolic links which points to non-existent files/directories
  - Broken Files - Finds files with an invalid extension or that are corrupted

<!-- The GIF thingy -->
![Czkawka](https://user-images.githubusercontent.com/41945903/104711404-9cbb7400-5721-11eb-904d-9677c189f7ab.gif)

## How do I use it?
You can find an instruction on how to use Czkawka [**here**](instructions/Instruction.md).

## Requirements
If you are using Windows or Mac binaries, there is no specific requirements.  
Same with Appimage, Flatpak and Snap on Linux (except having system 18.04+ or similar).  

Although, compiled GUI binaries on Linux or compiling it on your own OS requires you to install these packages:
### Ubuntu/Debian
```
sudo apt install libgtk-3-dev
```
### Fedora/CentOS
```
sudo yum install gtk3-devel glib2-devel
```
### Void Linux (CLI only)
```
sudo xbps-install gcc pkg-config alsa-lib-devel
```

# Installation
### Precompiled binaries
Ready-to-go executables are available [**here**](https://github.com/qarmin/czkawka/releases/).
If the app does not run when clicking at a launcher, run it through a terminal.  
You don't need to have any additional libraries for CLI Czkawka.

### GUI Requirements

##### Linux
For Czkawka GUI you are required to have at least `GTK 3.22` and also `Alsa` installed (for finding broken music
files). It should be installed by default on all the most popular distros.

##### Windows
The `czkawka_gui.exe` which is extracted from the `windows_czkawka_gui.zip` zip file needs to be in the same 
file as the rest. If you want to move and open the executable somewhere else, you need to install the `GTK 3`
runtime from [**here**](https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases).

##### macOS
Currently you need to manually install `GTK 3` libraries, because they are dynamically loaded from the OS (*we need
help in using static linking*). Installation in the terminal:
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install gtk+3
```
After that, go to the location where you installed this and add the `executable` permission.
```shell
chmod +x mac_czkawka_gui
```
Execute in the same folder with:
```shell
./mac_czkawka_gui
```

### Appimage
Appimage files are available in release page - [**GitHub releases**](https://github.com/qarmin/czkawka/releases/)
There is a problem with this currently, as it doesn't allow you to open two images/files at once.

### Cargo
The easiest method to install Czkawka is using the `cargo` command. For compiling it, you need to get all the 
requirements from the [compilation section](#Compilation).
```
cargo install czkawka_gui
cargo install czkawka_cli
```
You can update the package with the same command.


### Snap
```
sudo snap install czkawka
```
By default, Snap can only access to the files in your home directory. You have to allow czkawka to access to all the drives:

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

**Unofficial packages, which may not always provide the latest version of Czkawka.**

### PPA - Debian/Ubuntu (unofficial)
```
sudo add-apt-repository ppa:xtradeb/apps
sudo apt-get update
sudo apt-get install czkawka
```

### AUR - Arch Linux Package (unofficial)
Czkawka is also available in Arch Linux's AUR from which it can be easily installed.
```
yay -Syu czkawka-git
```
or 
```
yay -Syu czkawka-gui-bin
yay -Syu czkawka-cli-bin
```

Package info's - https://aur.archlinux.org/packages/?O=0&SeB=nd&K=czkawka&outdated=&SB=n&SO=a&PP=50&do_Search=Go

### Docker image (unofficial)
Czkawka docker image is available [**here**](https://github.com/jlesage/docker-czkawka)

### Development versions
Artifacts from each commit can be downloaded [**here**](https://github.com/qarmin/czkawka/actions)

<!-- Note the #Compilation link if you're changing this! -->  
# Compilation

The compilation section is generally not recommended, because you have multiple better sources
of this app than compiling it yourself. 

## Requirements
Program  | Min  | What for
---------|------|------------------------------------------------------------
Rust     | 1.48 | Czkawka aims to support only the latest stable Rust version  
GTK      | 3.22 | Only for the `GTK` backend

If you only want the terminal version without a GUI, just skip all lines about `gtk`.

#### Debian / Ubuntu
```shell
sudo apt install -y curl  # Needed by Rust update tool
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo apt install -y libgtk-3-dev libasound2-dev
```

#### Fedora / CentOS / Rocky Linux
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo yum install gtk3-devel glib2-devel alsa-lib-devel
```

#### macOS
You need to install Homebrew and GTK Libraries
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rust gtk+3
```

### Windows

*Will be available in the future*

<!-- First you need to install Visual C++ components from Visual Studio installer - https://visualstudio.microsoft.com/downloads/
Next install Rust from site https://rustup.rs/
After that the latest GTK 3 runtime must be installed from https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases
-->

## Compilation from source

- Download the source
```
git clone https://github.com/qarmin/czkawka.git
cd czkawka
```
- Compile and run GTK GUI
```
cargo run --bin czkawka_gui
```

For Linux-to-Windows cross-building instruction look at the CI.
![GUI](https://user-images.githubusercontent.com/41945903/103371136-fb9cae80-4ace-11eb-8d72-7b4c8ac44260.png)
- Compile and run CLI (this will print help with a lot of examples)
```
cargo run --bin czkawka_cli
```
![CLI](https://user-images.githubusercontent.com/41945903/93716816-0bbcfd80-fb72-11ea-8d31-4c87cc2abe6d.png)

### Additional features
For now, finding broken audio files is temporary disabled by default, because it crash when not found audio libraries on computer.  
I'm waiting for ability to disable audio playback feature, so after that I will be able to re-enable by default this feature (https://github.com/RustAudio/rodio/issues/349)

To enable checking for broken audio files, just add at the end ` --all-features` 
```
cargo run --all-features --bin czkawka_cli -- broken  -d /home/rafal/ -f "results.txt"
```

<!-- End of compilation section -->


## Benchmarks


Since Czkawka is written in Rust and it aims to be a faster alternative to FSlint (which is written in Python), we need
to compare the speed of these tools.

I tested it on a 256 GB SSD and a i7-4770 CPU.

I prepared a directory and performed a test without any folder exceptions (I removed all directories from FSlint and
Czkawka from other tabs than Include Directory) which contained 229 868 files, took 203.7 GB and had 13 708 duplicate
files in 9117 groups which took 7.90 GB.

Minimum file size to check I set to 1 KB on all programs.

| App                         | Executing Time |
|:---------------------------:|:--------------:|
| FSlint 2.4.7 (Second Run)   | 86s            |
| Czkawka 1.4.0 (Second Run)  | 12s            |
| DupeGuru 4.0.4 (Second Run) | 28s            |


I used Mprof for checking memory usage FSlint and DupeGuru, for Czkawka I used Heaptrack.
To not get a crash from DupeGuru I checked a smaller directory with 217 986 files and 41 883 folders.

| App            | Idle Ram | Max Operational Ram Usage | Stabilized after search |
|:--------------:|:--------:|:-------------------------:|:-----------------------:|
| FSlint 2.4.7   | 62 MB    | 84 MB                     | 84 MB                   |
| Czkawka 1.4.0  | 9 MB     | 66 MB                     | 32 MB                   |
| DupeGuru 4.0.4 | 80 MB    | 210 MB                    | 155 MB                  |

Similar images which check 332 files which took 1.7 GB

| App            | Scan time  |
|:--------------:|:----------:|
| Czkawka 1.4.0  | 58s        |
| DupeGuru 4.0.4 | 51s        |

Similar images which check 1421 image files which took 110.1 MB

| App            | Scan time |
|:--------------:|:----------|
| Czkawka 1.4.0  | 25s       |
| DupeGuru 4.0.4 | 92s       |

<!-- it's a lot of room, not a big room lol -->
So there is still is a lot of room for improvements.

## Comparison to other tools

|                        | Czkawka | FSlint     | DupeGuru          |
|:----------------------:|:-------:|:----------:|:-----------------:|
| Language               | Rust    | Python     | Python/Obj-C      |
| OS                     | All     | Linux only | All               |
| Framework              | GTK 3   | PyGTK      | Qt 5 (PyQt)/Cocoa |
| Ram Usage              | Low     | Medium     | Very High         |
| Duplicate finder       | •       | •          | •                 |
| Empty files            | •       | •          |                   |
| Empty folders          | •       | •          |                   |
| Temporary files        | •       | •          |                   |
| Big files              | •       |            |                   |
| Similar images         | •       |            | •                 |
| Zeroed Files           | •       |            |                   |
| Music duplicates(tags) | •       |            | •                 |
| Invalid symlinks       | •       | •          |                   |
| Broken Files           | •       |            |                   |
| Installed packages     |         | •          |                   |
| Invalid names          |         | •          |                   |
| Names conflict         |         | •          |                   |
| Bad ID                 |         | •          |                   |
| Non stripped binaries  |         | •          |                   |
| Redundant whitespace   |         | •          |                   |
| Multiple languages(po) |         | •          | •                 |
| Cache support          | •       |            | •                 |
| Project Activity       | High    | Very Low   | High              |

## Contributions
Contributions to this repository are welcome.  

You can help by creating a:
- Bug report - memory leaks, unexpected behavior, crashes
- Feature proposals - proposal to change/add/delete some features
- Pull Requests - implementing a new feature yourself or fixing bugs, but you have to pay attention to code quality. 
  If the change is bigger, then it's a good idea to open a new issue to discuss changes.
- Documentation - There is an [instruction](instructions/Instruction.md) which you can improve.

The code should be clean and well formatted (Clippy and fmt are required in each PR).

## Name
Czkawka is a Polish word which means _hiccup_.  

I chose this name because I wanted to hear people speaking other languages pronounce it.

This name is not as bad as it seems, because I was also thinking about using words like _żółć_, _gżegżółka_ or _żołądź_, 
but I gave up on these ideas because they contained Polish characters, which would cause difficulty in searching for the project.

At the beginning of the program creation, if the response concerning the name was unanimously negative, I prepared myself 
for a possible change of the name of the program, and the opinions were extremely mixed.

## License
Code is distributed under MIT license.

Icon is created by [jannuary](https://github.com/jannuary) and licensed CC-BY-4.0.

Windows dark theme is used from [AdMin repo](https://github.com/nrhodes91/AdMin) with MIT license.

The program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"

## Donations
If you are using the app, I would appreciate a donation for its further development, which can be done [here](https://github.com/sponsors/qarmin).
