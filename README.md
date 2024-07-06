![com github qarmin czkawka](https://user-images.githubusercontent.com/41945903/102616149-66490400-4137-11eb-9cd6-813b2b070834.png)

**Czkawka** (_tch•kav•ka_ (IPA: [ˈʧ̑kafka]), "hiccup" in Polish) is a simple, fast, and free app to remove unnecessary
files from your computer.

**Krokiet** (IPA: [ˈkrɔcɛt]), "croquet" in Polish) is the same as above, but uses Slint frontend.

## Features

- Written in memory-safe Rust
- Amazingly fast - Due to using more or less advanced algorithms and multithreading
- Free, open-source, and without ads
- Multiplatform - Works on Linux, Windows, macOS, FreeBSD, and many more
- Cache support - Second and further scans should be much faster than the first one
- CLI frontend - For easy automation
- GUI frontend - Uses GTK 4 or Slint frameworks
- No spying - Czkawka doesn't have access to the Internet, nor does it collect any user information or statistics
- Multilingual - Supports multiple languages like Polish, English or Italian
- Multiple tools to use:
    - Duplicates - Finds duplicates based on file name, size or hash
    - Empty Folders - Finds empty folders with the help of an advanced algorithm
    - Big Files - Finds the provided number of the biggest files in given location
  - Empty Files - Finds empty files across the drive
    - Temporary Files - Finds temporary files
  - Similar Images - Finds similar images (different resolution, watermarks)
  - Similar Videos - Finds similar videos
  - Same Music - Finds similar music by tags or by reading content and comparing it
    - Invalid Symbolic Links - Shows symbolic links which point to non-existent files/directories
    - Broken Files - Finds files that are invalid or corrupted
  - Bad Extensions - Finds files which content doesn't match with their extension

![Czkawka](https://user-images.githubusercontent.com/41945903/145280350-506f7e94-4db0-4de7-a68d-6e7c26bbd2bf.gif)

![Krokiet](https://github.com/qarmin/czkawka/assets/41945903/52628a29-ecd6-4692-9ee4-8a680c889193)

Changelog and new releases can be found in [GitHub releases](https://github.com/qarmin/czkawka/releases) or
in [CHANGELOG.md](Changelog.md).

You can find more about the latest version in the [Medium article](https://medium.com/@qarmin/czkawka-7-0-a465036e8788).

## Usage, installation, compilation, requirements, license

Each tool uses different technologies, so you can find instructions for each of them in the appropriate file:

- [Czkawka GUI (GTK frontend)](czkawka_gui/README.md)</br>
- [Czkawka CLI](czkawka_cli/README.md)</br>
- [Czkawka Core](czkawka_core/README.md)</br>
- [Krokiet GUI (Slint frontend)](krokiet/README.md)</br>

## Comparison to other tools

While Czkawka only finds the most basic ones, BleachBit is a master at finding and removing temporary files, so these
two apps shouldn't be directly compared or be considered as an alternative to one another.

In this comparison, remember that even if the apps have the same features, they may work differently (e.g., one app may have more
options to choose than the other).

|                          |   Czkawka   |   Krokiet   | FSlint |     DupeGuru      |  BleachBit  |
|:------------------------:|:-----------:|:-----------:|:------:|:-----------------:|:-----------:|
|         Language         |    Rust     |    Rust     | Python |   Python/Obj-C    |   Python    |
| Framework base language  |      C      |    Rust     |   C    | C/C++/Obj-C/Swift |      C      |
|        Framework         |    GTK 4    |    Slint    | PyGTK2 | Qt 5 (PyQt)/Cocoa |   PyGTK3    |
|            OS            | Lin,Mac,Win | Lin,Mac,Win |  Lin   |    Lin,Mac,Win    | Lin,Mac,Win |
|     Duplicate finder     |      ✔      |      ✔      |   ✔    |         ✔         |             |
|       Empty files        |      ✔      |      ✔      |   ✔    |                   |             |
|      Empty folders       |      ✔      |      ✔      |   ✔    |                   |             |
|     Temporary files      |      ✔      |      ✔      |   ✔    |                   |      ✔      |
|        Big files         |      ✔      |      ✔      |        |                   |             |
|      Similar images      |      ✔      |      ✔      |        |         ✔         |             |
|      Similar videos      |      ✔      |      ✔      |        |                   |             |
| Music duplicates (tags)  |      ✔      |      ✔      |        |         ✔         |             |
|     Invalid symlinks     |      ✔      |      ✔      |   ✔    |                   |             |
|       Broken files       |      ✔      |      ✔      |        |                   |             |
|      Names conflict      |      ✔      |      ✔      |   ✔    |                   |             |
| Invalid names/extensions |      ✔      |      ✔      |   ✔    |                   |             |
|    Installed packages    |             |             |   ✔    |                   |             |
|          Bad ID          |             |             |   ✔    |                   |             |
|  Non stripped binaries   |             |             |   ✔    |                   |             |
|   Redundant whitespace   |             |             |   ✔    |                   |             |
|    Overwriting files     |             |             |   ✔    |                   |      ✔      |
|    Multiple languages    |      ✔      |             |   ✔    |         ✔         |      ✔      |
|      Cache support       |      ✔      |      ✔      |        |         ✔         |             |
|  In active development   |     Yes     |     Yes     |   No   |        Yes        |     Yes     |

## Other apps

There are many similar applications to Czkawka on the Internet, which do some things better and some things worse:

### GUI

- [DupeGuru](https://github.com/arsenetar/dupeguru) - Many options to customize; great photo compare tool
- [FSlint](https://github.com/pixelb/fslint) - A little outdated, but still has some tools not available in Czkawka
- [AntiDupl.NET](https://github.com/ermig1979/AntiDupl) - Shows a lot of metadata of compared images
- [Video Duplicate Finder](https://github.com/0x90d/videoduplicatefinder) - Finds similar videos (surprising, isn't it?),
  supports video thumbnails

### CLI

Due to limited time, the biggest emphasis is on the GUI version, so if you're looking for really good and feature-packed
console apps, then take a look at these:

- [Fclones](https://github.com/pkolaczk/fclones) - One of the fastest tools to find duplicates; it is written in Rust too
- [Rmlint](https://github.com/sahib/rmlint) - Nice console interface and is feature packed
- [RdFind](https://github.com/pauldreik/rdfind) - Fast, but written in C++ ¯\\_(ツ)\_/¯

## Contributions

Contributions to this repository are welcome.

You can help by creating:

- Bug reports - Memory leaks, unexpected behavior or crashes
- Feature proposals - Proposal to change/add/delete some features
- Pull requests - Implementing a new feature yourself or fixing bugs
  If the change is bigger, then it is a good idea to open a new issue to discuss changes, but issues with
  the label `PR welcome` are already checked and accepted
- Documentation - There is an [instruction](instructions/Instruction.md) which you can improve
- Translations - Instructions for translating files are available [here](instructions/Translations.md)
- External contributions - The app uses many external libraries
  like [lofty](https://github.com/Serial-ATA/lofty-rs), [image-rs](https://github.com/image-rs/image)
  or [symphonia](https://github.com/pdeljanov/Symphonia), so improving these libraries will automatically improve Czkawka

You can also help by doing other things:

- Creating text
  articles - [LinuxUprising](https://www.linuxuprising.com/2021/03/find-and-remove-duplicate-files-similar.html)
  or [Ubunlog](https://ubunlog.com/en/czkawka-finds-and-removes-empty-and-broken-duplicate-files/)
- Adding Czkawka to repositories - [Alpine Linux](https://pkgs.alpinelinux.org/packages?name=czkawka&branch=edge),
  [NixOS](https://github.com/NixOS/nixpkgs/pull/116441)
  or [OpenMandriva](https://github.com/OpenMandrivaAssociation/czkawka)
- Creating videos - [First Video](https://www.youtube.com/watch?v=CWlRiTD4vDc)
  or [Spanish Tutorial](https://www.youtube.com/watch?v=V9x-pHJRmKY)
- Recommending it to others

## Thanks

Big thanks to Pádraig Brady, creator of the fantastic FSlint, because without his work, I wouldn't create this tool.

Thanks too to all the people who create patches for this program, make it available on other systems, create videos,
articles about it, etc.

Also, I really appreciate the work of people that create crates on which Czkawka is based, and for that I try to report bugs
to make it even better.

## Donations

If you're using the app, I would appreciate a donation for its further development, which can be
done [here](https://github.com/sponsors/qarmin).
