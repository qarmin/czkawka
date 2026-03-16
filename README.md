<div align="center"><img src="https://github.com/user-attachments/assets/f5e4b290-d001-4cf4-9f52-dab65a30e441" alt="krokiet_logo" width="600" /></div>
     
**Krokiet** ((IPA: [ˈkrɔcɛt]), "croquette" in Polish) new generation GUI frontend, simple, multiplatform, fast and free app to remove unnecessary files from your computer.


<div align="center"><img src="https://user-images.githubusercontent.com/41945903/102616149-66490400-4137-11eb-9cd6-813b2b070834.png" alt="czkawka_logo" width="600" /></div>

**Czkawka** (_tch•kav•ka_ (IPA: [ˈʧ̑kafka]), "hiccup" in Polish) older gtk4 GUI frontend, superseded by Krokiet, but still receiving bugfix updates.

## Features

- **Written in memory-safe Rust** - almost 100% unsafe code free
- **Amazingly fast** - due multithreading and efficient algorithms
- **Free, Open Source without any ads**
- **Multiplatform** - runs on Linux, Windows, macOS, FreeBSD, x86, ARM, RISC-V and even Android
- **Cache support** - second and further scans should be much faster than the first one
- **Easy to run, easy to compile** - minimal runtime and build dependencies, portable version available
- **CLI frontend** - for easy automation
- **GUI frontend** - uses Slint or GTK 4 frameworks
- **Core library** - allows to reuse functionality in other apps
- **Android app** - experimental touch-friendly frontend for Android devices
- **No spying** - Czkawka does not have access to the Internet, nor does it collect any user information or statistics
- **Multilingual** - support multiple languages like Polish, English or Italian
- **Multiple tools to use**:
    - **Duplicates** - Finds duplicates based on file name, size or hash
    - **Empty Folders** - Finds empty folders with the help of an advanced algorithm
    - **Big Files** - Finds the provided number of the biggest files in given location
    - **Empty Files** - Looks for empty files across the drive
    - **Temporary Files** - Finds temporary files
    - **Similar Images** - Finds images which are not exactly the same (different resolution, watermarks)
    - **Similar Videos** - Looks for visually similar videos
    - **Same Music** - Searches for similar music by tags or by reading content and comparing it
    - **Invalid Symbolic Links** - Shows symbolic links which point to non-existent files/directories
    - **Broken Files** - Finds files that are invalid or corrupted
    - **Bad Extensions** - Lists files whose content not match with their extension
    - **Exif Remover** - Removes Exif metadata from various file types
    - **Video Optimizer** - Crops from static parts and converts videos to more efficient formats
    - **Bad Names** - Finds files with names that may be not wanted (e.g., containing special characters)

![Krokiet](https://github.com/user-attachments/assets/3cc7ec6a-3d6a-42cb-9d33-4b0f0c547af6)

![Czkawka](https://github.com/user-attachments/assets/b0409515-1bec-4e13-8fac-7bdfa15f5848)

Changelog about each version can be found in [CHANGELOG.md](Changelog.md).

New releases can be found in [Github releases](https://github.com/qarmin/czkawka/releases) and nightly builds also in [Nightly releases](https://github.com/qarmin/czkawka/releases/tag/Nightly)

You can read more about the 11.0.0 release, its new features, and the issues that were fixed in the following articles:
- English article – https://medium.com/@qarmin/czkawka-krokiet-11-0-0f6cea385934
- Polish article – https://medium.com/@qarmin/czkawka-krokiet-11-0-c95ee35eccc2

## Usage, installation, compilation, requirements, license

Each tool uses different technologies, so you can find instructions for each of them in the appropriate file:

- [Krokiet GUI (Slint frontend)](krokiet/README.md)</br>
- [Czkawka GUI (GTK frontend)](czkawka_gui/README.md)</br>
- [Czkawka CLI](czkawka_cli/README.md)</br>
- [Czkawka Core](czkawka_core/README.md)</br>
- [Cedinia](cedinia/README.md)</br>

## Comparison to other tools

In this comparison remember, that even if app have same features they may work different(e.g. one app may have more
options to choose than other).

|                           |   Krokiet   |     Czkawka      | FSlint |     DupeGuru      |  Bleachbit  |
|:-------------------------:|:-----------:|:----------------:|:------:|:-----------------:|:-----------:|
|         Language          |    Rust     |       Rust       | Python |   Python/Obj-C    |   Python    |
|  Framework base language  |    Rust     |        C         |   C    | C/C++/Obj-C/Swift |      C      |
|         Framework         |    Slint    |      GTK 4       | PyGTK2 | Qt 5 (PyQt)/Cocoa |   PyGTK3    |
|            OS             | Lin,Mac,Win |   Lin,Mac,Win    |  Lin   |    Lin,Mac,Win    | Lin,Mac,Win |
|     Duplicate finder      |      ✔      |        ✔         |   ✔    |         ✔         |             |
|        Empty files        |      ✔      |        ✔         |   ✔    |                   |             |
|       Empty folders       |      ✔      |        ✔         |   ✔    |                   |             |
|      Temporary files      |      ✔      |        ✔         |   ✔    |                   |      ✔      |
|         Big files         |      ✔      |        ✔         |        |                   |             |
|      Similar images       |      ✔      |        ✔         |        |         ✔         |             |
|      Similar videos       |      ✔      |        ✔         |        |                   |             |
|  Music duplicates(tags)   |      ✔      |        ✔         |        |         ✔         |             |
| Music duplicates(content) |      ✔      |        ✔         |        |                   |             |
|     Invalid symlinks      |      ✔      |        ✔         |   ✔    |                   |             |
|       Broken files        |      ✔      |        ✔         |        |                   |             |
| Invalid names/extensions  |      ✔      |        ✔         |   ✔    |                   |             |
|       Exif cleaner        |      ✔      |                  |        |                   |             |
|      Video optimizer      |      ✔      |                  |        |                   |             |
|         Bad Names         |      ✔      |                  |        |                   |             |
|      Names conflict       |             |                  |   ✔    |                   |             |
|    Installed packages     |             |                  |   ✔    |                   |             |
|          Bad ID           |             |                  |   ✔    |                   |             |
|   Non stripped binaries   |             |                  |   ✔    |                   |             |
|   Redundant whitespace    |             |                  |   ✔    |                   |             |
|     Overwriting files     |             |                  |   ✔    |                   |      ✔      |
|     Portable version      |      ✔      |        ✔         |        |                   |      ✔      |
|    Multiple languages     |      ✔      |        ✔         |   ✔    |         ✔         |      ✔      |
|       Cache support       |      ✔      |        ✔         |        |         ✔         |             |
|   In active development   |     Yes     | Yes<sup>**</sup> |   No   |  No<sup>*</sup>   |     Yes     |

<p><sup>*</sup> Few small commits added recently and last version released in 2023</p> 
<p><sup>**</sup> Czkawka GTK is in maintenance mode receiving only bugfixes</p>

## Other apps

There are many similar applications to Czkawka on the Internet, which do some things better and some things worse:

### GUI

- [DupeGuru](https://github.com/arsenetar/dupeguru) - Many options to customize; great photo compare tool
- [FSlint](https://github.com/pixelb/fslint) - A little outdated, but still have some tools not available in Czkawka
- [AntiDupl.NET](https://github.com/ermig1979/AntiDupl) - Shows a lot of metadata of compared images
- [Video Duplicate Finder](https://github.com/0x90d/videoduplicatefinder) - Finds similar videos(surprising, isn't it)

### CLI

Due to limited time, the biggest emphasis is on the GUI version so if you are looking for really good and feature-packed
console apps, then take a look at these:

- [Fclones](https://github.com/pkolaczk/fclones) - One of the fastest tools to find duplicates; it is written also in
  Rust
- [Rmlint](https://github.com/sahib/rmlint) - Nice console interface and also is feature packed
- [RdFind](https://github.com/pauldreik/rdfind) - Fast, but written in C++ ¯\\\_(ツ)\_/¯


## Projects using Czkawka

Czkawka exposes its common functionality through a crate called **`czkawka_core`**, which can be reused by other projects.

It is written in Rust and is used by all Czkawka frontends (`czkawka_gui`, `czkawka_cli`, `krokiet`, `cedinia`).

It is also used by external projects, such as:

- **Czkawka Tauri** - https://github.com/shixinhuang99/czkawka-tauri - A Tauri-based GUI frontend for Czkawka.
- **page-dewarp** – https://github.com/lmmx/page-dewarp - A library for dewarping document images using a cubic sheet model.

Bindings are also available for:

- **Python** – https://pypi.org/project/czkawka/

Some projects work as wrappers around `czkawka_cli`. Without directly depending on `czkawka_core`, they allow simple scanning and retrieving results in JSON format:

- **Schluckauf** – https://github.com/fadykuzman/schluckauf

## Thanks

Big thanks to Pádraig Brady, creator of fantastic FSlint, because without his work I wouldn't create this tool.

Thanks also to all the people who contributed to the project in every possible way

Also, I really appreciate work of people that create crates on which Czkawka is based and for that I try to report bugs to make it even better.

## How to help?

- **Creating issues** - Mainly related to bugs, oddly behaving functionality, etc. As you can see from the issue tracker, there are plenty of ideas for new features, but most of them are either difficult to implement or not aligned with the vision of the project, which evolves slightly over time.
- **Creating pull requests** - Bug fixes are of course very welcome. Regarding new features, it is best to consult with me before implementing them to confirm they align with the project vision.
- **Updating translations** - The project uses the Crowdin platform, where translations can be created and updated. In the case of a new release and missing translations, I use machine translation, which is often inaccurate, so updating translations is highly appreciated.
- **Creating packages for various platforms** - Due to the difficulties related to adding and maintaining support for each new platform, such as learning package formats like deb or rpm, creating installers and packages, I decided to mainly focus on providing prebuilt binaries. However, having the project available in distribution repositories or in projects such as Chocolatey, Homebrew or Winget would be beneficial for users who prefer centralized repositories.
- **Creating articles, videos, tutorials, etc.** - Any material that helps people better understand this program and its capabilities is welcome.
- **Recommending it to friends, family, coworkers, etc.** - This is probably the simplest way to help the project become even more popular, which gives me motivation to continue developing the program. Here are a few example ways to naturally mention this program in a regular conversation:

**S** - Someone  
**Y** - You  

### Situation 1:

- **S** - Hey Anon, I have a lot of junk on my disk, what should I do?
- **Y** - Download Krokiet/Czkawka. They are completely free and works on almost every system.
- **S** - Thanks man!

### Situation 2:

- **S** - I am so thirsty...
- **Y** - Have you heard about Krokiet/Czkawka?
- **S** - Wait, what?
- **Y** - Krokiet and Czkawka, in case you did not know, let you clean unnecessary files from your disk. They are completely free...
- **S** - That is nice, but I am thirsty...
- **Y** - ...they work on Windows, Linux and macOS, and some people even port them to FreeBSD and Android...


## AI Policy
The vast majority of the code in this project was written by me(qarmin), without using AI. However, as AI tools have improved and can significantly simplify development and reduce boilerplate, I see no reason to forbid their use.

That said, every pull request, whether created with AI or not, must meet proper quality standards. The author must be able to clearly explain what the code does, without relying on AI for that explanation. I manually review every PR and test each change, so the risk of incorrect code slipping through is low. Still, to avoid wasting time, please refrain from submitting AI Slop PRs.

## Officially Supported Projects
Only this repository, [prebuild-binaries](https://github.com/qarmin/czkawka/releases), projects on [crates.io](https://crates.io/crates/czkawka_gui) and [flathub](https://flathub.org/apps/com.github.qarmin.czkawka) are directly maintained by me.  

Czkawka does not have an official website, so do not trust any sites that claim to be the official one.  

If you use packages from unofficial sources, make sure they are safe.

## License

The entire code in this repository is licensed under the [MIT](https://mit-license.org/) license.

All images and audio files are licensed under the [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) license.

The Czkawka GTK GUI and CLI applications are licensed under the [MIT](https://mit-license.org/) license, while the Krokiet/Cedinia(due Slint license requirements) are licensed under the [GPL-3.0-only](https://www.gnu.org/licenses/gpl-3.0.en.html) license.

## Donations

If you are using the app, I would appreciate a donation for its further development, which can be
done [here](https://github.com/sponsors/qarmin).

