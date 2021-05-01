![com github qarmin czkawka](https://user-images.githubusercontent.com/41945903/102616149-66490400-4137-11eb-9cd6-813b2b070834.png)

**Czkawka** (_tch•kav•ka_, hiccup) is a simple, fast and easy to use app to remove unnecessary files from your computer.

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

## Installation
Installation instruction with download links you can find [**here**](instructions/Installation.md).

## Compilation
If you want try to develop Czkawka or just use the latest available feature, you may want to look at the [**compilation instruction**](instructions/Compilation.md).

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

So there is still is a lot of room for improvements.

## Comparison to other tools

|                        | Czkawka | FSlint     | DupeGuru          |
|:----------------------:|:-------:|:----------:|:-----------------:|
| Language               | Rust    | Python     | Python/Obj-C      |
| OS                     | All     | Linux only | All               |
| Framework              | GTK 3   | PyGTK      | Qt 5 (PyQt)/Cocoa |
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
- Pull Requests - implementing a new feature yourself or fixing bugs.
  If the change is bigger, then it's a good idea to open a new issue to discuss changes.
- Documentation - There is an [instruction](instructions/Instruction.md) which you can improve.

You can also help by doing different things:
- Creating text articles - [LinuxUprising](https://www.linuxuprising.com/2021/03/find-and-remove-duplicate-files-similar.html) or [Ubunlog](https://ubunlog.com/en/czkawka-finds-and-removes-empty-and-broken-duplicate-files/)
- Adding Czkawka to repositories - [Alpine Linux](https://pkgs.alpinelinux.org/packages?name=czkawka&branch=edge) or [NixOS](https://github.com/NixOS/nixpkgs/pull/116441) or [OpenMandriva](https://github.com/OpenMandrivaAssociation/czkawka)
- Creating videos - [Tutorial Spanish 1](https://www.youtube.com/watch?v=tALYBsJAYwE) or [Tutorial Spanish 2](https://www.youtube.com/watch?v=V9x-pHJRmKY)
- Recommending it to others

The code should be clean and well formatted (Clippy and fmt are required in each PR).

## Name
Czkawka is a Polish word which means _hiccup_.  

I chose this name because I wanted to hear people speaking other languages pronounce it, so feel free to spell it the way you want.

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
