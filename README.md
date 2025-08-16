![com github qarmin czkawka](https://user-images.githubusercontent.com/41945903/102616149-66490400-4137-11eb-9cd6-813b2b070834.png)

**Czkawka** (_tch•kav•ka_ (IPA: [ˈʧ̑kafka]), "hiccup" in Polish) is a simple, fast and free app to remove unnecessary
files from your computer.

**Krokiet** ((IPA: [ˈkrɔcɛt]), "croquet" in Polish) same as above, but uses Slint frontend.

## Features

- Written in memory-safe Rust - almost 100% unsafe code free
- Amazingly fast - due to using more or less advanced algorithms and multithreading
- Free, Open Source without ads
- Multiplatform - works on Linux, Windows, macOS, FreeBSD and many more
- Cache support - second and further scans should be much faster than the first one
- CLI frontend - for easy automation
- GUI frontend - uses GTK 4 or Slint frameworks
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

![Czkawka](https://github.com/user-attachments/assets/b0409515-1bec-4e13-8fac-7bdfa15f5848)

![Krokiet](https://github.com/user-attachments/assets/906cbbc3-f011-4306-81da-9e4e53b49a9f)

Changelog about each version can be found in [CHANGELOG.md](Changelog.md).

New releases can be found in [Github releases](https://github.com/qarmin/czkawka/releases) and nightly builds also in [Nightly releases](https://github.com/qarmin/czkawka/releases/tag/Nightly)

## Usage, installation, compilation, requirements, license

Each tool uses different technologies, so you can find instructions for each of them in the appropriate file:

- [Czkawka GUI (GTK frontend)](czkawka_gui/README.md)</br>
- [Czkawka CLI](czkawka_cli/README.md)</br>
- [Czkawka Core](czkawka_core/README.md)</br>
- [Krokiet GUI (Slint frontend)](krokiet/README.md)</br>

## Comparison to other tools

Bleachbit is a master at finding and removing temporary files, while Czkawka only finds the most basic ones. So these
two apps shouldn't be compared directly or be considered as an alternative to one another.

In this comparison remember, that even if app have same features they may work different(e.g. one app may have more
options to choose than other).

|                           |   Czkawka   |   Krokiet   | FSlint |     DupeGuru      |  Bleachbit  |
|:-------------------------:|:-----------:|:-----------:|:------:|:-----------------:|:-----------:|
|         Language          |    Rust     |    Rust     | Python |   Python/Obj-C    |   Python    |
|  Framework base language  |      C      |    Rust     |   C    | C/C++/Obj-C/Swift |      C      |
|         Framework         |    GTK 4    |    Slint    | PyGTK2 | Qt 5 (PyQt)/Cocoa |   PyGTK3    |
|            OS             | Lin,Mac,Win | Lin,Mac,Win |  Lin   |    Lin,Mac,Win    | Lin,Mac,Win |
|     Duplicate finder      |      ✔      |      ✔      |   ✔    |         ✔         |             |
|        Empty files        |      ✔      |      ✔      |   ✔    |                   |             |
|       Empty folders       |      ✔      |      ✔      |   ✔    |                   |             |
|      Temporary files      |      ✔      |      ✔      |   ✔    |                   |      ✔      |
|         Big files         |      ✔      |      ✔      |        |                   |             |
|      Similar images       |      ✔      |      ✔      |        |         ✔         |             |
|      Similar videos       |      ✔      |      ✔      |        |                   |             |
|  Music duplicates(tags)   |      ✔      |      ✔      |        |         ✔         |             |
| Music duplicates(content) |      ✔      |      ✔      |        |                   |             |
|     Invalid symlinks      |      ✔      |      ✔      |   ✔    |                   |             |
|       Broken files        |      ✔      |      ✔      |        |                   |             |
| Invalid names/extensions  |      ✔      |      ✔      |   ✔    |                   |             |
|      Names conflict       |             |             |   ✔    |                   |             |
|    Installed packages     |             |             |   ✔    |                   |             |
|          Bad ID           |             |             |   ✔    |                   |             |
|   Non stripped binaries   |             |             |   ✔    |                   |             |
|   Redundant whitespace    |             |             |   ✔    |                   |             |
|     Overwriting files     |             |             |   ✔    |                   |      ✔      |
|    Multiple languages     |      ✔      |      ✔      |   ✔    |         ✔         |      ✔      |
|       Cache support       |      ✔      |      ✔      |        |         ✔         |             |
|   In active development   |     Yes     |     Yes     |   No   |        No*        |     Yes     |

* Last commit in 2024 and last version released in 2023

## Other apps

There are many similar applications to Czkawka on the Internet, which do some things better and some things worse:

### GUI

- [DupeGuru](https://github.com/arsenetar/dupeguru) - Many options to customize; great photo compare tool
- [FSlint](https://github.com/pixelb/fslint) - A little outdated, but still have some tools not available in Czkawka
- [AntiDupl.NET](https://github.com/ermig1979/AntiDupl) - Shows a lot of metadata of compared images
- [Video Duplicate Finder](https://github.com/0x90d/videoduplicatefinder) - Finds similar videos(surprising, isn't it), supports video thumbnails

### CLI

Due to limited time, the biggest emphasis is on the GUI version so if you are looking for really good and feature-packed
console apps, then take a look at these:

- [Fclones](https://github.com/pkolaczk/fclones) - One of the fastest tools to find duplicates; it is written also in
  Rust
- [Rmlint](https://github.com/sahib/rmlint) - Nice console interface and also is feature packed
- [RdFind](https://github.com/pauldreik/rdfind) - Fast, but written in C++ ¯\\\_(ツ)\_/¯

## Thanks

Big thanks to Pádraig Brady, creator of fantastic FSlint, because without his work I wouldn't create this tool.

Thanks also to all the people who create patches for this program, make it available on other systems, create videos,
articles about it etc.

Also, I really appreciate work of people that create crates on which Czkawka is based and for that I try to report bugs
to make it even better.

## Officially Supported Projects
Only this repository, [prebuild-binaries](https://github.com/qarmin/czkawka/releases), projects on [crates.io](https://crates.io/crates/czkawka_gui) and [flathub](https://flathub.org/apps/com.github.qarmin.czkawka) are directly maintained by me.  

Czkawka does not have an official website, so do not trust any sites that claim to be the official one.  

If you use packages from unofficial sources, make sure they are safe.

## License

The entire code in this repository is licensed under the [MIT](https://mit-license.org/) license.

All images are licensed under the [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) license.

The Czkawka GTK GUI and CLI applications are licensed under the [MIT](https://mit-license.org/) license, while the Krokiet is licensed under the [GPL-3.0-only](https://www.gnu.org/licenses/gpl-3.0.en.html) license.

## Donations

If you are using the app, I would appreciate a donation for its further development, which can be
done [here](https://github.com/sponsors/qarmin).

