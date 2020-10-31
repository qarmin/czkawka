# Czkawka
Czkawka is a simple, fast and easy to use alternative to FSlint, written in Rust.  
This is my first ever project in Rust so many things might not be written in the most optimal way.

![Czkawka](https://user-images.githubusercontent.com/41945903/94850792-c200cc80-0427-11eb-99a7-23ab9cf39556.gif)

## Why?
There's a lot of tools for finding duplicates, empty folders, temporary files etc. on the Internet, but in most cases these are only available as CLI, which is hard to use by users.

GUI FSlint allows selecting different files and folders easily, but is based on old and unsupported Python 2 and GTK 2.

Other tools are usually written in C/C++ for high performance but still need to be tested a lot for memory leaks, invalid memory reads/writes and double frees.

But the most important thing for me was to learn Rust and create a program useful for the open source community.

## Features
- Written in memory safe Rust
- Amazingly fast - due using more or less advanced algorithms
- CLI frontend, very fast and powerful with rich help
- GUI GTK frontend - uses modern GTK 3 and looks similar to FSlint
- Light/Dark theme match the appearance of the system
- Saving results to a file - allows reading entries found by the tool easily
- Rich search option - allows setting absolute included and excluded directories, set of allowed file extensions or excluded items with * wildcard
- Clean Glade file in which UI can be easily modernized
- Multiple tools to use:
  - Duplicates - Finds duplicates basing on size(fast), hash(accurate), first 1MB of hash(moderate)
  - Empty Folders - Finds empty folders with the help of advanced algorithm
  - Big Files - Finds provided number of the biggest files in given location
  - Empty Files - Looks for empty files across disk
  - Temporary Files - Allows finding temporary files
  - Similar Files - Finds files which are not exactly the same
  - Zeroed Files - Find files which are filled with zeros(usually corrupted)

## Usage and requirements


### Precompiled binaries
For Linux of the program, the only requirement is having GTK 3.22+ installed on system.  

Precompiled binaries are available here - https://github.com/qarmin/czkawka/releases/
If the app does not run when clicking at a launcher, run it through a terminal.

### Appimage
Appimage files are available in release page, same as native binaries and minimal required version of OS is Ubuntu 18.04 - https://github.com/qarmin/czkawka/releases/

### Cargo
Easier method to install Czkawka is to use Cargo command(you must have installed GTK libraries in OS) 
```
cargo install czkawka_gui
```
You can update package by typing same command.

### Snap, Flatpak
Maybe someday

### Debian/Ubuntu repository and PPA
Tried to setup it, but for now I have problems described in this issue

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
Rust 1.46 - probably lower also works fine(1.40 is needed by GTK)  
GTK 3.22 - for GTK backend


For now only Linux (and maybe also macOS) is supported

- Install requirements for GTK
```
apt install -y libgtk-3-dev
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
![GUI GTK](https://user-images.githubusercontent.com/41945903/94850801-c5945380-0427-11eb-8d4c-af4946ab02d5.png)
- Run alternative Orbtk GUI (Still WIP, currently stopped due https://github.com/intellij-rust/intellij-rust/issues/5943)

```
cargo run --bin czkawka_gui_orbtk
```
![GUI Orbtk](https://user-images.githubusercontent.com/41945903/92405241-7b27fb80-f135-11ea-9fc4-5ebc2b76b011.png)
- Run CLI(this will print help with a lot of examples)
```
cargo run --bin czkawka_cli
```
![CLI](https://user-images.githubusercontent.com/41945903/93716816-0bbcfd80-fb72-11ea-8d31-4c87cc2abe6d.png)

## Benchmarks
Since Czkawka is written in Rust and aims to be a faster alternative to FSlint (written in Python), we need to compare the speed of these tools.

I prepared a directory and performed a test without any folder exceptions(I removed all directories from FSlint and Czkawka from other tabs than Include Directory) which contained 320004 files and 36902 folders and 108844 duplicated files in 34475 groups which took 4.53 GB.

Minimum file size to check I set to 1KB on all programs

The first run reads every file entry and saves it to cache, so this step is limited mostly by disk performance. In the second run the cache helps it, so searching is sometimes faster (with few duplicates even 10x faster).

DupeGuru after selecting files, froze at 45% for ~15 minutes, so I just kill it.

| App| Executing Time |
|:----------:|:-------------:|
| FSlint 2.4.7 (First Run)| 255s |
| FSlint 2.4.7 (Second Run)| 126s |
| Czkawka 1.3.0 (First Run) | 150s |
| Czkawka 1.3.0 (Second Run) | 107s |
| DupeGuru 4.0.4 (First Run) | - | 
| DupeGuru 4.0.4 (Second Run) | - | 


I used Mprof for checking memory usage FSlint and Dupeguru, for Czkawka I used Heaptrack.
To not get Dupeguru crash I checked smaller directory with 217986 files and 41883 folders.

| App| Idle Ram | Max Operational Ram Usage | Stabilized after search |
|:----------:|:-------------:|:-------------:|:-------------:|
| FSlint 2.4.7 | 54 MB | 120 MB | 117 MB |
| Czkawka 1.3.0 | 8 MB | 42 MB | 41 MB |
| DupeGuru 4.0.4 | 110 MB | 637 MB | 602 MB |

Similar Images which check 386 files which takes 1,9GB

| App| Scan time |
|:----------:|:-------------:|
| Czkawka 1.3.0 | 267s | 
| DupeGuru 4.0.4 | 75s | 

Similar Images which check 5018 files which takes 389MB

| App| Scan time |
|:----------:|:-------------:|
| Czkawka 1.3.0 | 45s | 
| DupeGuru 4.0.4 | 87s | 

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
| Checking files EXIF| | | X |
| Installed packages |  | X |  |
| Invalid names |   | X |  |
| Names conflict |   | X |  |
| Invalid symlinks |   | X |  |
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

The code should be clean and well formatted (Clippy and fmt are required in each PR).

The code should also be easy to read, so please use the simplest language possible without any magic numbers and variables with strange names. You should also try to write unit tests if possible.

## Name
Czkawka is a Polish word which means _hiccup_.  
I chose this name because I wanted to hear people speaking other languages pronounce it.  
This name is not as bad as it seems, because I was also thinking about using words like _żółć_, _gżegżółka_ or _żołądź_, but I gave up on these ideas because they contained Polish characters, which would cause difficulty in searching for the project.

At the beginning of the program creation, if the response concerning the name was unanimously negative, I prepared myself for a possible change of the name of the program, but the opinions were extremely mixed.

## License
Code is distributed under MIT license.

Program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"
