![com github qarmin czkawka](https://user-images.githubusercontent.com/41945903/102616149-66490400-4137-11eb-9cd6-813b2b070834.png)

**Czkawka** (_tch•kav•ka_ (IPA: [ʈ͡ʂkafka]), "hiccup" in Polish) is a simple, fast and free app to remove unnecessary files from your computer.

## Features
- Written in memory safe Rust
- Amazingly fast - due to using more or less advanced algorithms and multithreading
- Free, Open Source without ads
- Multiplatform - works on Linux, Windows and macOS
- Cache support - second and further scans should be a lot faster than the first one
- CLI frontend - for easy automation
- GUI frontend - uses modern GTK 3 and looks similar to FSlint
- Rich search option - allows setting absolute included and excluded directories, set of allowed file extensions 
  or excluded items with the `*` wildcard
- No spying - Czkawka does not have access to the Internet, nor does it collect any user information or statistics
- Multiple tools to use:
  - Duplicates - Finds duplicates based on file name, size, hash, hash of just first 1 MB of a file
  - Empty Folders - Finds empty folders with the help of an advanced algorithm
  - Big Files - Finds the provided number of the biggest files in given location
  - Empty Files - Looks for empty files across the drive
  - Temporary Files - Finds temporary files
  - Similar Images - Finds images which are not exactly the same (different resolution, watermarks)
  - Zeroed Files - Finds files which are filled with zeros (usually corrupted)
  - Same Music - Searches for music with the same artist, album etc.
  - Invalid Symbolic Links - Shows symbolic links which point to non-existent files/directories
  - Broken Files - Finds files with an invalid extension or that are corrupted

<!-- The GIF thingy -->
![Czkawka](https://user-images.githubusercontent.com/41945903/104711404-9cbb7400-5721-11eb-904d-9677c189f7ab.gif)

## How do I use it?
You can find the instructions on how to use Czkawka [**here**](instructions/Instruction.md).

## Installation
Installation instructions with download links you can find [**here**](instructions/Installation.md).

## Compilation
If you want to try and develop Czkawka or just use the latest available feature, you may want to look at the [**compilation instruction**](instructions/Compilation.md).

## Benchmarks

Since Czkawka is written in Rust and it aims to be a faster alternative to FSlint or DupeGuru which are written in Python, we need to compare the speed of these tools.

I tested it on a 256 GB SSD and a i7-4770 CPU.

I prepared a disk and performed a test without any folder exceptions and with disabled ignoring of hard links which contained 363 215 files, took 221,8 GB and had 62093 duplicate files in 31790 groups which occupied 4,1 GB.

I set the minimal file size to check to 1KB on all programs.

| App                         | Executing Time |
|:---------------------------:|:--------------:|
| FSlint 2.4.7 (First Run)    | 86s            |
| FSlint 2.4.7 (Second Run)   | 43s            |
| Czkawka 3.0.0 (First Run)   | 8s             |
| Czkawka 3.0.0 (Second Run)  | 7s             |
| DupeGuru 4.1.1 (First Run)  | 22s            |
| DupeGuru 4.1.1 (Second Run) | 21s            |

I used Mprof for checking memory usage of FSlint and DupeGuru, and Heaptrack for Czkawka.

| App            | Idle Ram | Max Operational Ram Usage | Stabilized after search |
|:--------------:|:--------:|:-------------------------:|:-----------------------:|
| FSlint 2.4.7   | 62 MB    | 164 MB                    | 158 MB                  |
| Dupeguru 4.1.1 | 90 MB    | 170 MB                    | 166 MB                  |
| Czkawka 3.0.0  | 12 MB    | 122 MB                    | 60 MB                   |


In Dupeguru I enabled checking images with different dimensions to match Czkawka behavior.
Both apps use caching mechanism, so second scan is really fast.

Similar images which check 10949 files that occupied 6.6 GB

| App                         | Scan time |
|:---------------------------:|:---------:|
| Czkawka 3.0.0 (First Run)   | 276s      |
| Czkawka 3.0.0 (Second Run)  | 1s        |
| DupeGuru 4.1.1 (First Run)  | 539s      |
| DupeGuru 4.1.1 (Second Run) | 1s        |

Similar images which check 349 image files that occupied 1.7 GB

| App                         | Scan time |
|:---------------------------:|:----------|
| Czkawka 3.0.0 (First Run)   | 54s       |
| Czkawka 3.0.0 (Second Run)  | 1s        |
| DupeGuru 4.1.1 (First Run)  | 55s       |
| DupeGuru 4.1.1 (Second Run) | 1s        |

## Comparison to other tools

Bleachbit is a master at finding and removing temporary files, while Czkawka only finds the most basic ones. So these two apps shouldn't be compared directly or be considered as an alternative to one another.

|                        | Czkawka     | FSlint     | DupeGuru          | Bleachbit   |
|:----------------------:|:-----------:|:----------:|:-----------------:|:-----------:|
| Language               | Rust        | Python     | Python/Obj-C      | Python      |
| OS                     | Lin,Mac,Win | Lin        | Lin,Mac,Win       | Lin,Mac,Win |
| Framework              | GTK 3       | PyGTK2     | Qt 5 (PyQt)/Cocoa | PyGTK3      |
| Duplicate finder       | •           | •          | •                 |             |
| Empty files            | •           | •          |                   |             |
| Empty folders          | •           | •          |                   |             |
| Temporary files        | •           | •          |                   | •           |
| Big files              | •           |            |                   |             |
| Similar images         | •           |            | •                 |             |
| Zeroed Files           | •           |            |                   |             |
| Music duplicates(tags) | •           |            | •                 |             |
| Invalid symlinks       | •           | •          |                   |             |
| Broken Files           | •           |            |                   |             |
| Names conflict         | •           | •          |                   |             |
| Installed packages     |             | •          |                   |             |
| Invalid names          |             | •          |                   |             |
| Bad ID                 |             | •          |                   |             |
| Non stripped binaries  |             | •          |                   |             |
| Redundant whitespace   |             | •          |                   |             |
| Overwriting files      |             | •          |                   | •           |
| Multiple languages(po) |             | •          | •                 | •           |
| Cache support          | •           |            | •                 |             |
| In active development  | Yes         | No         | Yes               | Yes         |

## Other apps
There are many similar applications to Czkawka on the Internet, which do some things better and some things worse.
- [DupeGuru](https://github.com/arsenetar/dupeguru) - A lot of options to customize
- [FSlint](https://github.com/pixelb/fslint) - A little outdated, but still have some tools not available in Czkawka
- [Fclones](https://github.com/pkolaczk/fclones) - One of the fastest tools to find duplicates, it is written also in Rust but only in CLI

## Contributions
Contributions to this repository are welcome.

You can help by creating:
- Bug reports - memory leaks, unexpected behavior, crashes
- Feature proposals - proposal to change/add/delete some features
- Pull Requests - implementing a new feature yourself or fixing bugs.
  If the change is bigger, then it's a good idea to open a new issue to discuss changes.
- Documentation - There is an [instruction](instructions/Instruction.md) which you can improve.

You can also help by doing different things:
- Creating text articles - [LinuxUprising](https://www.linuxuprising.com/2021/03/find-and-remove-duplicate-files-similar.html) or [Ubunlog](https://ubunlog.com/en/czkawka-finds-and-removes-empty-and-broken-duplicate-files/)
- Adding Czkawka to repositories - [Alpine Linux](https://pkgs.alpinelinux.org/packages?name=czkawka&branch=edge) or [NixOS](https://github.com/NixOS/nixpkgs/pull/116441) or [OpenMandriva](https://github.com/OpenMandrivaAssociation/czkawka)
- Creating videos - [First Video](https://www.youtube.com/watch?v=CWlRiTD4vDc) or [Tutorial Spanish](https://www.youtube.com/watch?v=V9x-pHJRmKY)
- Recommending it to others

## Name
Czkawka is a Polish word which means _hiccup_.

I chose this name because I wanted to hear people speaking other languages pronounce it, so feel free to spell it the way you want.

This name is not as bad as it seems, because I was also thinking about using words like _żółć_, _gżegżółka_ or _żołądź_, 
but I gave up on these ideas because they contained Polish characters, which would cause difficulty in searching for the project.

At the beginning of the program creation, if the response concerning the name was unanimously negative, I prepared myself 
for a possible change of the name of the program, and the opinions were extremely mixed.

## License
Code is distributed under MIT license.

Icon was created by [jannuary](https://github.com/jannuary) and licensed CC-BY-4.0.

Windows dark theme is used from [AdMin repo](https://github.com/nrhodes91/AdMin) with MIT license.

The program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"

## Thanks

Big thanks to Pádraig Brady, creator of fantastic FSlint, because without his work I wouldn't create this tool.

Thanks also to all the people who create patches for this program, make it available on other systems, create videos, articles about it etc.

## Donations
If you are using the app, I would appreciate a donation for its further development, which can be done [here](https://github.com/sponsors/qarmin).
