![com github qarmin czkawka](https://user-images.githubusercontent.com/41945903/102616149-66490400-4137-11eb-9cd6-813b2b070834.png)

**Czkawka** (_tch•kav•ka_ (IPA: [ˈʧ̑kafka]), "hiccup" in Polish) is a simple, fast and free app to remove unnecessary files from your computer.

## Features
- Written in memory-safe Rust
- Amazingly fast - due to using more or less advanced algorithms and multithreading
- Free, Open Source without ads
- Multiplatform - works on Linux, Windows, macOS, FreeBSD and many more
- Cache support - second and further scans should be much faster than the first one
- CLI frontend - for easy automation
- GUI frontend - uses GTK 4 framework and looks similar to FSlint
- No spying - Czkawka does not have access to the Internet, nor does it collect any user information or statistics
- Multilingual - support multiple languages like Polish, English or Italian
- Multiple tools to use:
  - Duplicates - Finds duplicates based on file name, size or hash
  - Empty Folders - Finds empty folders with the help of an advanced algorithm
  - Big Files - Finds the provided number of the biggest files in given location
  - Empty Files - Looks for empty files across the drive
  - Temporary Files - Finds temporary files
  - Similar Images - Finds images which are not exactly the same (different resolution, watermarks)
  - Similar Videos - Looks for visually similar videos
  - Same Music - Searches for similar music by tags or by reading content and comparing it
  - Invalid Symbolic Links - Shows symbolic links which point to non-existent files/directories
  - Broken Files - Finds files that are invalid or corrupted
  - Bad Extensions - Lists files whose content not match with their extension

![Czkawka](https://user-images.githubusercontent.com/41945903/145280350-506f7e94-4db0-4de7-a68d-6e7c26bbd2bf.gif)

## Usage, installation, compilation, requirements, license
Each tool uses different technologies, so you can find instructions for each of them in the appropriate file:
- [Czkawka GUI (GTK frontend)](czkawka_gui/README.md)</br>
- [Czkawka CLI](czkawka_cli/README.md)</br>
- [Czkawka Core](czkawka_core/README.md)</br>
- [Krokiet GUI (Slint frontend)](krokiet/README.md)</br>

## Benchmarks

Since Czkawka is written in Rust and it aims to be a faster alternative to FSlint or DupeGuru which are written in Python, we need to compare the speed of these tools.

I tested it on a 256 GB SSD and an i7-4770 CPU.

I prepared a disk and performed a test without any folder exceptions and with disabled ignoring of hard links. The disk contained 363 215 files, took 221,8 GB and had 62093 duplicate files in 31790 groups which occupied 4,1 GB.

I set the minimal file size to check to 1KB on all programs.

| App                         | Executing Time |
|:----------------------------|:--------------:|
| FSlint 2.4.7 (First Run)    |      86s       |
| FSlint 2.4.7 (Second Run)   |      43s       |
| Czkawka 3.0.0 (First Run)   |       8s       |
| Czkawka 3.0.0 (Second Run)  |       7s       |
| DupeGuru 4.1.1 (First Run)  |      22s       |
| DupeGuru 4.1.1 (Second Run) |      21s       |

I used Mprof for checking memory usage of FSlint and DupeGuru, and Heaptrack for Czkawka.

| App             | Idle Ram | Max Operational Ram Usage | Stabilized after search |
|:----------------|:--------:|:-------------------------:|:-----------------------:|
| FSlint 2.4.7    |  62 MB   |          164 MB           |         158 MB          |
| Dupeguru 4.1.1  |  90 MB   |          170 MB           |         166 MB          |
| Czkawka 3.0.0   |  12 MB   |          122 MB           |          60 MB          |


In Dupeguru, I enabled checking images with different dimensions to match Czkawka behavior.
Both apps use a caching mechanism, so the second scan is really fast.

Similar images which check 10949 files that occupied 6.6 GB

| App                         | Scan time |
|:----------------------------|:---------:|
| Czkawka 3.0.0 (First Run)   |   276s    |
| Czkawka 3.0.0 (Second Run)  |    1s     |
| DupeGuru 4.1.1 (First Run)  |   539s    |
| DupeGuru 4.1.1 (Second Run) |    1s     |

Similar images which check 349 image files that occupied 1.7 GB

| App                         | Scan time  |
|:----------------------------|:----------:|
| Czkawka 3.0.0 (First Run)   |    54s     |
| Czkawka 3.0.0 (Second Run)  |     1s     |
| DupeGuru 4.1.1 (First Run)  |    55s     |
| DupeGuru 4.1.1 (Second Run) |     1s     |

## Comparison to other tools

Bleachbit is a master at finding and removing temporary files, while Czkawka only finds the most basic ones. So these two apps shouldn't be compared directly or be considered as an alternative to one another.

In this comparison remember, that even if app have same features they may work different(e.g. one app may have more options to choose than other). 

|                          |   Czkawka   |   Krokiet   | FSlint |      DupeGuru      |  Bleachbit  |
|:------------------------:|:-----------:|:-----------:|:------:|:------------------:|:-----------:|
|         Language         |    Rust     |    Rust     | Python |    Python/Obj-C    |   Python    |
| Framework base language  |      C      |    Rust     |   C    | C/C++/Obj-C/Swift  |      C      |
|        Framework         |    GTK 4    |    Slint    | PyGTK2 | Qt 5 (PyQt)/Cocoa  |   PyGTK3    |
|            OS            | Lin,Mac,Win | Lin,Mac,Win |  Lin   |    Lin,Mac,Win     | Lin,Mac,Win |
|     Duplicate finder     |      ✔      |      ✔      |   ✔    |         ✔          |             |
|       Empty files        |      ✔      |      ✔      |   ✔    |                    |             |
|      Empty folders       |      ✔      |      ✔      |   ✔    |                    |             |
|     Temporary files      |      ✔      |      ✔      |   ✔    |                    |      ✔      |
|        Big files         |      ✔      |      ✔      |        |                    |             |
|      Similar images      |      ✔      |      ✔      |        |         ✔          |             |
|      Similar videos      |      ✔      |      ✔      |        |                    |             |
|  Music duplicates(tags)  |      ✔      |      ✔      |        |         ✔          |             |
|     Invalid symlinks     |      ✔      |      ✔      |   ✔    |                    |             |
|       Broken files       |      ✔      |      ✔      |        |                    |             |
|      Names conflict      |      ✔      |      ✔      |   ✔    |                    |             |
| Invalid names/extensions |      ✔      |      ✔      |   ✔    |                    |             |
|    Installed packages    |             |             |   ✔    |                    |             |
|          Bad ID          |             |             |   ✔    |                    |             |
|  Non stripped binaries   |             |             |   ✔    |                    |             |
|   Redundant whitespace   |             |             |   ✔    |                    |             |
|    Overwriting files     |             |             |   ✔    |                    |      ✔      |
|    Multiple languages    |      ✔      |             |   ✔    |         ✔          |      ✔      |
|      Cache support       |      ✔      |      ✔      |        |         ✔          |             |
|  In active development   |     Yes     |             |   No   |        Yes         |     Yes     |

## Other apps
There are many similar applications to Czkawka on the Internet, which do some things better and some things worse:  
### GUI
- [DupeGuru](https://github.com/arsenetar/dupeguru) - Many options to customize; great photo compare tool
- [FSlint](https://github.com/pixelb/fslint) - A little outdated, but still have some tools not available in Czkawka
- [AntiDupl.NET](https://github.com/ermig1979/AntiDupl) - Shows a lot of metadata of compared images
- [Video Duplicate Finder](https://github.com/0x90d/videoduplicatefinder) - Finds similar videos(surprising, isn't it), supports video thumbnails
### CLI
Due to limited time, the biggest emphasis is on the GUI version so if you are looking for really good and feature-packed console apps, then take a look at these:
- [Fclones](https://github.com/pkolaczk/fclones) - One of the fastest tools to find duplicates; it is written also in Rust
- [Rmlint](https://github.com/sahib/rmlint) - Nice console interface and also is feature packed
- [RdFind](https://github.com/pauldreik/rdfind) - Fast, but written in C++ ¯\\\_(ツ)\_/¯

## Contributions
Contributions to this repository are welcome.

You can help by creating:
- Bug reports - memory leaks, unexpected behavior, crashes
- Feature proposals - proposal to change/add/delete some features
- Pull Requests - implementing a new feature yourself or fixing bugs.
  If the change is bigger, then it's a good idea to open a new issue to discuss changes, but issues with label `PR welcome` are already checked and accepted.
- Documentation - There is an [instruction](instructions/Instruction.md) which you can improve.
- Translations - Instruction how to translate files is available [here](instructions/Translations.md)
- External contributions - App use big number of external libraries like [lofty](https://github.com/Serial-ATA/lofty-rs), [image-rs](https://github.com/image-rs/image) or [symphonia](https://github.com/pdeljanov/Symphonia) so improving this libraries will automatically improve Czkawka

You can also help by doing other things:
- Creating text articles - [LinuxUprising](https://www.linuxuprising.com/2021/03/find-and-remove-duplicate-files-similar.html) or [Ubunlog](https://ubunlog.com/en/czkawka-finds-and-removes-empty-and-broken-duplicate-files/)
- Adding Czkawka to repositories - [Alpine Linux](https://pkgs.alpinelinux.org/packages?name=czkawka&branch=edge) or [NixOS](https://github.com/NixOS/nixpkgs/pull/116441) or [OpenMandriva](https://github.com/OpenMandrivaAssociation/czkawka)
- Creating videos - [First Video](https://www.youtube.com/watch?v=CWlRiTD4vDc) or [Spanish Tutorial](https://www.youtube.com/watch?v=V9x-pHJRmKY)
- Recommending it to others

## Thanks

Big thanks to Pádraig Brady, creator of fantastic FSlint, because without his work I wouldn't create this tool.

Thanks also to all the people who create patches for this program, make it available on other systems, create videos, articles about it etc.

Also, I really appreciate work of people that create crates on which Czkawka is based and for that I try to report bugs to make it even better.

## Donations
If you are using the app, I would appreciate a donation for its further development, which can be done [here](https://github.com/sponsors/qarmin).
