## Version 2.1.0 - 31.12.2020r
- Hide preview when deleting images or symlinking it - [#167](https://github.com/qarmin/czkawka/pull/167)
- Add manual adding of directories - [#165](https://github.com/qarmin/czkawka/pull/165), [#168](https://github.com/qarmin/czkawka/pull/168)
- Add resizable top panel - [#164](https://github.com/qarmin/czkawka/pull/164)
- Add support for delete button - [#159](https://github.com/qarmin/czkawka/pull/159)
- Allow to select multiple entries in File Chooser - [#154](https://github.com/qarmin/czkawka/pull/154)
- Add cache support for similar images - [#139](https://github.com/qarmin/czkawka/pull/139)
- Add selecting images with it's size - [#138](https://github.com/qarmin/czkawka/pull/138)
- Modernize popovers code and simplify later changes - [#137](https://github.com/qarmin/czkawka/pull/137)

## Version 2.0.0 - 23.12.2020r
- Add Snap support - [ee3d4](https://github.com/qarmin/czkawka/commit/ee3d450552cd0c37a114b05c557ff9381ef92466)
- Select longer names by default - [#113](https://github.com/qarmin/czkawka/pull/113)
- Add setting for deletion confirmation dialog - [#114](https://github.com/qarmin/czkawka/pull/114)
- Add button to hide/show text view errors - [#115](https://github.com/qarmin/czkawka/pull/115)
- Remove console window in Windows - [#116](https://github.com/qarmin/czkawka/pull/116)
- Add custom selection/unselection - [#117](https://github.com/qarmin/czkawka/pull/117)
- Add Image preview to similar images - [#118](https://github.com/qarmin/czkawka/pull/118)
- Remove orbtk frontend - [#119](https://github.com/qarmin/czkawka/pull/119)
- Update Icon - [#120](https://github.com/qarmin/czkawka/pull/120)
- Add setting button to disable/enable previews(enabled by default) - [#121](https://github.com/qarmin/czkawka/pull/121)
- Add button to enable/disable in settings text view errors - [#122](https://github.com/qarmin/czkawka/pull/122)
- Add support for symbolic links - [#123](https://github.com/qarmin/czkawka/pull/123)
- Add support for checking for invalid symlinks - [#124](https://github.com/qarmin/czkawka/pull/124)
- Add new windows dark theme - [#125](https://github.com/qarmin/czkawka/pull/125)
- Fix appimage crash by adding PNG version of icon - [#126](https://github.com/qarmin/czkawka/pull/126)
- Split symlink path to two path and file name - [#127](https://github.com/qarmin/czkawka/pull/127)
- Add option to open folders by double right click - [#128](https://github.com/qarmin/czkawka/pull/128)
- Add minimal similarity level - [#129](https://github.com/qarmin/czkawka/pull/129)
- Show errors in image previewer when failed to generate it - [#130](https://github.com/qarmin/czkawka/pull/130)
- Added instruction - [58e6221a](https://github.com/qarmin/czkawka/commit/58e6221a0e02d17d07c71152f56b948f616751a8), [598aec345e](https://github.com/qarmin/czkawka/commit/598aec345e9f5ac199fc3d642c0699d5228100a6), [afaa402b](https://github.com/qarmin/czkawka/commit/afaa402b31526aa8e6b47f3670bc62b26ad9f60f)

## Version 1.5.1 - 08.12.2020r
- Fix errors in progress bar caused by dividing by 0  - [#109](https://github.com/qarmin/czkawka/pull/109)
- Add option to save file, store settings and load them - [#108](https://github.com/qarmin/czkawka/pull/108)
- Center dialog to current window - [a04](https://github.com/qarmin/czkawka/commit/a047380dbe8aa4d04f9c482364469e21d231fab2)

## Version 1.5.0 - 02.12.2020r
- Added progress bar - [#106](https://github.com/qarmin/czkawka/pull/106)
- Removed unused buttons - [#107](https://github.com/qarmin/czkawka/pull/107)

## Version 1.4.0 - 09.11.2020r
- Multithreading Support to most modules - [#98](https://github.com/qarmin/czkawka/pull/98) [#99](https://github.com/qarmin/czkawka/pull/99) [#100](https://github.com/qarmin/czkawka/pull/100) [#101](https://github.com/qarmin/czkawka/pull/101)
- Simplify GUI code [#96](https://github.com/qarmin/czkawka/pull/96)
- Group similar images - [#97](https://github.com/qarmin/czkawka/pull/97)
- Add select buttons to each type of mode - [#102](https://github.com/qarmin/czkawka/pull/102)
- Fix GUI behavior in GUI when deleting similar image - [#103](https://github.com/qarmin/czkawka/pull/103)
- Add new similarity level - [#104](https://github.com/qarmin/czkawka/pull/104)

## Version 1.3.0 - 02.11.2020r
- Appimage support - [#77](https://github.com/qarmin/czkawka/pull/77)
- Removed warnings about non existend excluded directories - [#79](https://github.com/qarmin/czkawka/pull/79)
- Updated README - [8ec](https://github.com/qarmin/czkawka/commit/8ecde0fc9adb3e6cedf432c4ba749e698b645a7a)
- Added pre hash support(speedup for searching big duplicates) - [#83](https://github.com/qarmin/czkawka/pull/83)
- Support for searching duplicates by file name - [#84](https://github.com/qarmin/czkawka/pull/84)
- Added support for checking for zeroed file - [#88](https://github.com/qarmin/czkawka/pull/88)
- Refactored GUI code to faster and safer changing/adding code - [#89](https://github.com/qarmin/czkawka/pull/89)
- Added some missing options to CLI in some modes - [#90](https://github.com/qarmin/czkawka/pull/90)
- Implemented finding duplicates by music tags - [#95](https://github.com/qarmin/czkawka/pull/95)

## Version 1.2.1 - 17.10.2020r
- Make image similarity search significantly faster. [#72](https://github.com/qarmin/czkawka/pull/72)
- Improve similar images GUI a little and add sorting to Similarity Enum [#73](https://github.com/qarmin/czkawka/pull/73)
- Improve deleting files in Similar files in GUI [#75](https://github.com/qarmin/czkawka/pull/75)

## Version 1.2.0 - 15.10.2020r
- Replace String with PathBuf for paths [#59](https://github.com/qarmin/czkawka/pull/59)
- Add test suite to PR [#65](https://github.com/qarmin/czkawka/pull/65)
- Support for finding similar images to CLI [#66](https://github.com/qarmin/czkawka/pull/66)
- Fix grammar-related errors and Ponglish expressions [#62](https://github.com/qarmin/czkawka/pull/62), [#63](https://github.com/qarmin/czkawka/pull/63)
- Don't delete by default files in duplicate finder in CLI - [23f203](https://github.com/qarmin/czkawka/commit/23f203a061e254275c95ca23ca4f1a78bd941f02)
- Support for finding similar images to GUI [#69](https://github.com/qarmin/czkawka/pull/69)
- Add support for opening files/folders from GUI with double-click [#70](https://github.com/qarmin/czkawka/pull/70)

## Version 1.1.0 - 10.10.2020r
- Windows support [#58](https://github.com/qarmin/czkawka/pull/58)
- Improve code quality/Simplify codebase [#52](https://github.com/qarmin/czkawka/pull/52)
- Fixed skipping some correct results in specific situations [#52](https://github.com/qarmin/czkawka/pull/52#discussion_r502613895)
- Added support for searching in other thread [#51](https://github.com/qarmin/czkawka/pull/51)
- Divide CI across files [#48](https://github.com/qarmin/czkawka/pull/48)
- Added ability to stop task from GUI [#55](https://github.com/qarmin/czkawka/pull/55)
- Fixed removing directories which contains only empty directories from GUI [#57](https://github.com/qarmin/czkawka/pull/57)

## Version 1.0.1 - 06.10.2020r
- Replaced default argument parser with StructOpt [#37](https://github.com/qarmin/czkawka/pull/37)
- Added all(except MacOS GTK build) builds to CI where can be freely downloaded [#41](https://github.com/qarmin/czkawka/pull/41) [#39](https://github.com/qarmin/czkawka/pull/39)
- App can be downloaded also from Arch AUR and Cargo [#36](https://github.com/qarmin/czkawka/pull/36)
- Fixed crash with invalid file modification date [#33](https://github.com/qarmin/czkawka/issues/33)
- Upper tabs can hide and show when this is necessary [#38](https://github.com/qarmin/czkawka/pull/38)
- Fixed crash when file/folder name have non Unicode character [#44](https://github.com/qarmin/czkawka/issues/44)
- Added support for finding similar pictures in GUI [#69](https://github.com/qarmin/czkawka/issues/69)

## Version 1.0.0 - 02.10.2020r
- Added confirmation button to delete button
- Updated Readme
- Tested a lot app, so I think that it version 1.0.0 can be freely released

## Version 0.1.4 - 01.10.2020r
- Fixes -f default argument
- Added save button to GUI
- Cleaned a little code
- Deleting files and folders i GUI
- Support for all notebooks items in GUI
- Support for deleting and adding directories to search and to exclude in GUI
- Support for light themes in GUI
- Changed SystemTime to u64 from EPOCH_TIME
- Selective selecting of rows duplicate finder in GUI
- Changed minimum version of GTK to 3.22
- Added save system to GUI
- Added Big, Temporary and Empty folders finder to GUI

## Version 0.1.3 - 27.09.2020r
- Big code refactoring - now is a lot of easier create new modules and maintain old ones
- Added finding empty files
- Added new option to find duplicates by checking hash max 1MB of file
- Added support for finding temporary folder finder
- Improved README
- Simplify CLI help and improve it

## Version 0.1.2 - 26.09.2020r
- Add basic search empty folders in GTK GUI
- Remember place where button are placed
- Read and parse more values from GUI
- Print errors/warnings/messages to text field in GUI
- Add upper notebook with included, excluded directories, items and extensions
- Improve a little GUI
- Add version argument which print version e.g. `czkawka_gui --version`
- Simple Empty folder support in GUI
- The biggest files support in CLI

## Version 0.1.1 - 20.09.2020r
- Added images to readme
- Better GTK buttons and glade file
- Basic search in GTK
- Cleaned core from println
- Core functions doesn't use now process::exit(everything is done with help of messages/errors/warnings)
- Added support for non recursive search
- Improved finding number and size of duplicated files
- Saving results to file
- Print how much data was read by duplicate finder(debug only)
- Added Github CI
- Only debug build prints debug information's
- Clean code
- Add basic idea config to misc folder

## Version 0.1.0 - 07.09.2020r
- Initial Version
- Duplicate file finder
- Empty folder finder
- Very WIP Orbtk GUI frontend
- Basic GTK Frontend(without any logic)
- CLI

## Initial commit - 26.08.2020r
