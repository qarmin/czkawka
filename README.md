![com github qarmin czkawka](https://user-images.githubusercontent.com/41945903/102616149-66490400-4137-11eb-9cd6-813b2b070834.png)

**Czkawka** is written in Rust, simple, fast and easy to use app to remove unnecessary files from your computer.

## Features
- Written in memory safe Rust
- Amazingly fast - due using more or less advanced algorithms and multithreading support
- Free, Open Source without ads
- CLI frontend, very fast to automate tasks
- GUI GTK frontend - uses modern GTK 3 and looks similar to FSlint
- Light/Dark theme match the appearance of the system(Linux only)
- Saving results to a file - allows reading entries found by the tool easily
- Rich search option - allows setting absolute included and excluded directories, set of allowed file extensions or excluded items with * wildcard
- Image previews to get quick view at the compared photos
- Multiple tools to use:
  - Duplicates - Finds duplicates basing on file name, size, hash, first 1 MB of hash
  - Empty Folders - Finds empty folders with the help of advanced algorithm
  - Big Files - Finds provided number of the biggest files in given location
  - Empty Files - Looks for empty files across disk
  - Temporary Files - Allows finding temporary files
  - Similar Images - Finds images which are not exactly the same(different resolution, watermarks)
  - Zeroed Files - Find files which are filled with zeros(usually corrupted)
  - Same Music - Search for music with same artist, album etc.
  - Invalid Symbolic Links - Shows symbolic links which points to non-existent files/directories

![Czkawka](https://user-images.githubusercontent.com/41945903/100857797-69809680-348d-11eb-8382-acdec05fd3b8.gif)

## Instruction
You can find instruction how to use Czkawka [here](instructions/Instruction.md)

## Requirements
If you are using Windows or Mac binaries, there is no specific requirements.  
Same with Appimage on Linux(except having system 18.04+ or similar).  
But compiled GUI binaries on Linux or compiling it on your own os require to install this packages:
### Ubuntu/Debian
```
sudo apt install cargo libgtk-3-dev
```
### Fedora/CentOS
```
sudo yum install gtk3-devel glib2-devel
```
### Windows(Not working yet)
First you need to install Visual C++ components from Visual Studio installer - https://visualstudio.microsoft.com/downloads/

Next install Rust from site https://rustup.rs/  

After that the latest GTK 3 runtime must be installed from https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases

## Usage
### Precompiled binaries
Precompiled binaries are available here - https://github.com/qarmin/czkawka/releases/.  
If the app does not run when clicking at a launcher, run it through a terminal.

### Appimage
Appimage files are available in release page - https://github.com/qarmin/czkawka/releases/

For now looks that there is a bug with this format, because it doesn't allow opening two images/files at once.

### Cargo
The easiest method to install Czkawka is to use Cargo command, since it basically compile an entire app, you need to install required packages from `Compilation` section
```
cargo install czkawka_gui
cargo install czkawka_cli
```
You can update package by typing same command.

### Snap
Snap also are available, but there is no access to external drives.
```
sudo snap install czkawka
```
Snap store entry - https://snapcraft.io/czkawka

Edgy builds are build for every commit, but it may be a little unstable(very rarely, because I'm not pushing untested code).

### Flatpak
Maybe someday


### Debian/Ubuntu repository and PPA
Tried to set up it, but for now I have problems described in this issue

https://salsa.debian.org/rust-team/debcargo-conf/-/issues/21


### AUR - Arch Linux Package (unofficial)
Czkawka is also available in Arch Linux's AUR from which it can be easily downloaded and installed on the system.
```
yay -Syu czkawka-git
```

This is unofficial package, so new versions will not be always available.

### Devel versions
Artifacts from each commit you can also download here - https://github.com/qarmin/czkawka/actions

## Compilation
### Requirements
Rust 1.48 - Czkawka aims to support only the latest stable Rust version  
GTK 3.22 - only for GTK backend

If you want to compile CLI frontend, then just skip lines which contains `gtk` word.
#### Debian/Ubuntu
```shell
sudo apt install -y curl  # Needed by Rust update tool
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo apt install -y libgtk-3-dev
```
#### Fedora/CentOS/Rocky Linux
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Download the latest stable Rust
sudo yum install gtk3-devel glib2-devel
```

### Compilation from source
- Download the source
```
git clone https://github.com/qarmin/czkawka.git
cd czkawka
```
- Run GTK GUI
```
cargo run --bin czkawka_gui
```
For Linux-to-Windows cross-building instruction look at the CI.
![GUI](https://user-images.githubusercontent.com/41945903/103371136-fb9cae80-4ace-11eb-8d72-7b4c8ac44260.png)
- Run CLI(this will print help with a lot of examples)
```
cargo run --bin czkawka_cli
```
![CLI](https://user-images.githubusercontent.com/41945903/93716816-0bbcfd80-fb72-11ea-8d31-4c87cc2abe6d.png)

## Benchmarks
Since Czkawka is written in Rust and aims to be a faster alternative to FSlint (written in Python), we need to compare the speed of these tools.

I tested it on SSD Disk 256 GB GoodRam and i7 4770 CPU.

I prepared a directory and performed a test without any folder exceptions(I removed all directories from FSlint and Czkawka from other tabs than Include Directory) which contained 229868 files which took 203,7 GB and 13708 duplicates files in 9117 groups which took 7.90 GB.

Minimum file size to check I set to 1 KB on all programs

| App| Executing Time |
|:----------:|:-------------:|
| FSlint 2.4.7 (Second Run)| 86s |
| Czkawka 1.4.0 (Second Run) | 12s |
| DupeGuru 4.0.4 (Second Run) | 28s |


I used Mprof for checking memory usage FSlint and Dupeguru, for Czkawka I used Heaptrack.
To not get Dupeguru crash I checked smaller directory with 217986 files and 41883 folders.

| App| Idle Ram | Max Operational Ram Usage | Stabilized after search |
|:----------:|:-------------:|:-------------:|:-------------:|
| FSlint 2.4.7 | 62 MB | 84 MB | 84 MB |
| Czkawka 1.4.0 | 9 MB | 66 MB | 32 MB |
| DupeGuru 4.0.4 | 80 MB | 210 MB | 155 MB |

Similar Images which check 332 files which takes 1,7 GB

| App| Scan time |
|:----------:|:-------------:|
| Czkawka 1.4.0 | 58s |
| DupeGuru 4.0.4 | 51s |

Similar Images which check 1421 image files which takes 110,1 MB

| App| Scan time |
|:----------:|:-------------:|
| Czkawka 1.4.0 | 25s |
| DupeGuru 4.0.4 | 92s |

So still is a big room for improvements.

## Comparsion other tools

|  | Czkawka | FSlint | DupeGuru |
|:----------:|:-------------:|:-----:|:---:|
| Language | Rust| Python | Python/Objective C |
| OS | Linux, Windows, Mac(only CLI) | Linux | Linux, Windows, Mac|
| Framework | GTK 3 (Gtk-rs)| GTK 2 (PyGTK) | Qt 5 (PyQt)/Cocoa |
| Ram Usage | Low | Medium | Very High |
| Duplicate finder | X | X | X |
| Empty files | X | X |  |
| Empty folders | X | X |  |
| Temporary files | X | X |  |
| Big files | X |   |  |
| Similar images | X |   | X |
| Zeroed Files| X | | |
| Music duplicates(tags) | X | | X |
| Invalid symlinks | X | X |  |
| Installed packages |  | X |  |
| Invalid names |   | X |  |
| Names conflict |   | X |  |
| Bad ID |   | X |  |
| Non stripped binaries |   | X |  |
| Redundant whitespace |  | X |  |
| Multiple languages(po) | | X | X |
| Project Activity | High | Very Low | High |

## Contributions
Contributions to this repository are welcome.  

You can help by creating:
- Bug report - memory leaks, unexpected behavior, crashes
- Feature proposals - proposal to change/add/delete some features
- Pull Requests - implementing a new feature yourself or fixing bugs, but you have to pay attention to code quality. If the change is bigger, then it's a good idea to open a new issue to discuss changes.
- Documentation - There is [instruction](instructions/Instruction.md) which you can improve.

The code should be clean and well formatted (Clippy and fmt are required in each PR).

## Name
Czkawka is a Polish word which means _hiccup_.  

I chose this name because I wanted to hear people speaking other languages pronounce it.

This name is not as bad as it seems, because I was also thinking about using words like _żółć_, _gżegżółka_ or _żołądź_, but I gave up on these ideas because they contained Polish characters, which would cause difficulty in searching for the project.

At the beginning of the program creation, if the response concerning the name was unanimously negative, I prepared myself for a possible change of the name of the program, but the opinions were extremely mixed.

## License
Code is distributed under MIT license.

Icon is created by [jannuary](https://github.com/jannuary) and licensed CC-BY-4.0.

Windows dark theme is used from AdMin repo - https://github.com/nrhodes91/AdMin with MIT license

Program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"
