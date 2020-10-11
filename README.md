# Czkawka
Czkawka is a simple, fast and easy to use alternative to FSlint, written in Rust.  
This is my first ever project in Rust so probably a lot of things are not being written in the most optimal way.

![Czkawka](https://user-images.githubusercontent.com/41945903/94850792-c200cc80-0427-11eb-99a7-23ab9cf39556.gif)

## Why?
There's a lot of tools to find duplicates, empty folders, temporary files etc. on the Internet, but in most cases these are only available as CLI, which is hard to use by users.

GUI FSlint allows to select different files and folders easily, but is based on old and unsupported Python 2 and GTK 2.

Other tools are mostly written in C/C++ for high performance but still need to be tested a lot for memory leaks, invalid memory reads/writes and double frees.

But the most important thing for me was to learn Rust and create a program useful for the open source community.

## Features
- Written in fast and memory safe Rust
- CLI frontend, very fast and powerful with rich help
- GUI GTK frontend - uses modern GTK 3 and looks similar to FSlint
- Light/Dark theme match the appearance of the system
- GUI Orbtk frontend(Very early WIP) - alternative GUI with reduced functionality
- Saving results to a file - allows reading entries found by the tool easily
- Rich search option - allows setting absolute included and excluded directories, set of allowed file extensions or excluded items with * wildcard
- Clean Glade file in which UI can be easily modernized
- Multiple tools to use:
  - Duplicates - Finds duplicates basing on size(fast), hash(accurate), first 1MB of hash(moderate)
  - Empty Folders - Finds empty folders with the help of advanced algorithm
  - Big Files - Finds provided number of the biggest files in given location
  - Empty Files - Looks for empty files across disk
  - Temporary Files - Allows finding temporary files

## Usage and requirements
### Requirements
For normal use of the program, the only requirement is having GTK 3.22+.  
For CLI, Orbtk on all OS and GTK GUI on Windows, there are no special requirements.

Precompiled binaries are here - https://github.com/qarmin/czkawka/releases/

You can also browse application workflows with every single commit here - https://github.com/qarmin/czkawka/actions

If the app is not launching when clicking at a launcher, run it through a terminal.
### Cargo
You can install Czkawka quickly from Cargo by typing `cargo install czkawka_gui`

### Snap,  Flatpak, Appimage
Still WIP, but looking for help

### AUR - Arch Linux Package
Czkawka is also available in Arch Linux AUR from which it can be simply downloaded and installed on the system.
```
yay -Syu czkawka-git
```

## Compilation
### Requirements
Rust 1.46 - probably lower also works fine(1.40 is needed by GTK)  
GTK 3.22 - for GTK backend


For now only Linux(and maybe also macOS) is supported

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
- Run alternative Orbtk GUI(Still WIP, currently stopped due https://github.com/intellij-rust/intellij-rust/issues/5943)

```
cargo run --bin czkawka_gui_orbtk
```
![GUI Orbtk](https://user-images.githubusercontent.com/41945903/92405241-7b27fb80-f135-11ea-9fc4-5ebc2b76b011.png)
- Run CLI(this will print help with a lot of examples)
```
cargo run --bin czkawka_cli
```
![CLI](https://user-images.githubusercontent.com/41945903/93716816-0bbcfd80-fb72-11ea-8d31-4c87cc2abe6d.png)

## Speed
Since Czkawka is written in Rust and aims to be a faster alternative for FSlint written in Python, we need to compare speed of these two tools.

I prepared a directory and performed a check without any folder exceptions(I removed all directories from FSlint and Czkawka from other tabs than Include Directory) which contained 176 056 files and 22194 folders and 88436 duplicated files in 52330 groups which took 6,2 GB.

The first run reads the file entry and caches it so this step is mostly limited by disk performance. In the second run the cache helps so searching is sometimes faster(with a small amount of duplicates it's even 10x faster).

Duplicate Checker(Version 0.1.4)

| App| Executing Time |
|:----------:|:-------------:|
| FSlint (First Run)| 284s |
| FSlint (Second Run)| 247s |
| Czkawka GUI Release(First Run) | 118s |
| Czkawka GUI Release(Second Run) | 120s |

I used Mprof for FSlint and Heaptrack for Czkawka

| App| Idle Ram | Max Operational Ram Usage | Stabilized after search usage |
|:----------:|:-------------:|:-------------:|:-------------:|
| FSlint | 55 MB | 160 MB | 150 MB |
| Czkawka GTK GUI Release | 8 MB | 76 MB | 75 MB |


Differences should be more visible when using slower CPU or faster disk.

## Comparsion with FSLint

|  | Czkawka | FSlint |
|:----------:|:-------------:|:-----:|
| Language | Rust| Python | 
| Framework | GTK 3 (Gtk-rs)| GTK 2 (PyGTK) |
| Ram Usage | Low | Medium |
| Duplicate finder | X | X |
| Empty files | X | X |
| Empty folders | X | X |
| Temporary files | X | X |
| Big files | X |   |
| Installed packages |  | X |
| Invalid names |   | X |
| Names conflict |   | X |
| Invalid symlinks |   | X |
| Bad ID |   | X |
| Non stripped binaries |   | X |
| Redundant whitespace |  | X |
| Project Activity | High | Very Low | 

## Contributions
Contributions to this repository are welcome.  

You can help by creating:
- Bug report - memory leaks, unexpected behavior, crashes
- Feature proposals - proposal to change/add/delete some features
- Pull Requests - implementing a new feature yourself or fixing bugs, but code quality is also important. If a change is bigger, then it's a good idea to open a new issue to discuss changes.

Code should be clean and well formatted (Clippy and fmt are required in each PR).

Code should also be easy to read so please use the simplest possible language without magic numbers and variables with strange names; try to write unit tests if possible.

## Name
Czkawka is a Polish word which means _hiccup_.  
I chose this name because I wanted to hear people speaking other languages pronounce it.  
This name is not as bad as it seems, because I was also thinking about using words like _żółć_, _gżegżółka_ or _żołądź_, but I gave up on these ideas due to them having Polish characters, which would be a huge difficulty in searching for a project.

## License
Code is distributed under MIT license.

Program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"
