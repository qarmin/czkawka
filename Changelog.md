## Version 1.1 - 10.10.2020r
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
