## Version 8.0.0 - ?

### Core

- Removed some unnecessary panics
- Simplified usage of structures when sending/receiving progress information
- Added Median hash algorithm
- Fixed compilation with Rust >=1.80
- Extracted tool input parameters, that helped to find not used parameters
- Added new mod to find similar music only in groups with similar title tag

### Krokiet

- Fixed invalid default hash size in similar images
- Fixed and added more input parameters to the application

### GTK GUI

- Fixed and added more input parameters to the application

### CLI

- Fixed and added more input parameters to the application

## Version 7.0.0 - 19.02.2024r

### BREAKING CHANGES

- Reducing size of cache files, made old cache files incompatible with new version
- `-C` in CLI now saves as compact json

### GTK GUI

- Added drag&drop support for included/excluded folders - [#1106](https://github.com/qarmin/czkawka/pull/1106)
- Added information where are saved scan results - [#1102](https://github.com/qarmin/czkawka/pull/1102)

### CLI

- Providing full static rust binary
  with [Eyra](https://github.com/sunfishcode/eyra) - [#1102](https://github.com/qarmin/czkawka/pull/1102)
- Fixed duplicated `-c` argument, now saving as compact json is handled via
  `-C` - [#1153](https://github.com/qarmin/czkawka/pull/1153)
- Added scan progress bar - [#1183](https://github.com/qarmin/czkawka/pull/1183)
- Clean and safe cancelling of scan - [#1183](https://github.com/qarmin/czkawka/pull/1183)
- Unification of CLI arguments - [#1183](https://github.com/qarmin/czkawka/pull/1183)
- Hardlink support for similar images/videos - [#1201](https://github.com/qarmin/czkawka/pull/1201)

### Krokiet GUI

- Initial release of new gui - [#1102](https://github.com/qarmin/czkawka/pull/1102)

### Core

- Using normal crossbeam channels instead of asyncio tokio
  channel - [#1102](https://github.com/qarmin/czkawka/pull/1102)
- Fixed tool type when using progress of empty directories - [#1102](https://github.com/qarmin/czkawka/pull/1102)
- Fixed missing json support when saving size and name duplicate
  results - [#1102](https://github.com/qarmin/czkawka/pull/1102)
- Fix cross-compiled debug windows build - [#1102](https://github.com/qarmin/czkawka/pull/1102)
- Added bigger stack size by default(fixes stack overflow in some musl
  apps) - [#1102](https://github.com/qarmin/czkawka/pull/1102)
- Added optional libraw dependency(better single-core performance and support more raw
  files) - [#1102](https://github.com/qarmin/czkawka/pull/1102)
- Speedup checking for wildcards and fix invalid recognizing long excluded
  items - [#1152](https://github.com/qarmin/czkawka/pull/1152)
- Big speedup when searching for empty folders(especially with multithreading + cached FS
  schema)  - [#1152](https://github.com/qarmin/czkawka/pull/1152)
- Collecting files for scan can be a lot of faster due lazy file metadata
  gathering - [#1152](https://github.com/qarmin/czkawka/pull/1152)
- Fixed recognizing not accessible folders as non-empty - [#1152](https://github.com/qarmin/czkawka/pull/1152)
- Unifying code for collecting files to scan - [#1159](https://github.com/qarmin/czkawka/pull/1159)
- Decrease memory usage when collecting files by removing unused fields in custom file entries
  structs - [#1159](https://github.com/qarmin/czkawka/pull/1159)
- Decrease a little size of cache by few percents and improve loading/saving
  speed - [#1159](https://github.com/qarmin/czkawka/pull/1159)
- Added ability to remove from scan files with excluded
  extensions - [#1184](https://github.com/qarmin/czkawka/pull/1102)
- Fixed not showing in similar images results, files with same hashes when using reference
  folders - [#1184](https://github.com/qarmin/czkawka/pull/1102)
- Optimize release binaries with LTO(~25/50% smaller, ~5/10%
  faster) - [#1184](https://github.com/qarmin/czkawka/pull/1102)

## Version 6.1.0 - 15.10.2023r

- BREAKING CHANGE - Changed cache saving method, deduplicated, optimized and simplified procedure(all files needs to be
  hashed
  again) - [#1072](https://github.com/qarmin/czkawka/pull/1072), [#1086](https://github.com/qarmin/czkawka/pull/1086)
- Remove up to 340ms of delay when waiting for results - [#1070](https://github.com/qarmin/czkawka/pull/1070)
- Added logger with useful info when debugging app (level can be adjusted via e.g. `RUST_LOG=debug`
  env) - [#1072](https://github.com/qarmin/czkawka/pull/1072), [#1070](https://github.com/qarmin/czkawka/pull/1070)
- Core code
  cleanup - [#1072](https://github.com/qarmin/czkawka/pull/1072), [#1070](https://github.com/qarmin/czkawka/pull/1070), [#1082](https://github.com/qarmin/czkawka/pull/1082)
- Updated list of bad extensions and support for finding invalid jar
  files - [#1070](https://github.com/qarmin/czkawka/pull/1070)
- More default excluded items on Windows(like pagefile) - [#1074](https://github.com/qarmin/czkawka/pull/1074)
- Unified printing/saving method to files/terminal and fixed some
  differences/bugs - [#1082](https://github.com/qarmin/czkawka/pull/1082)
- Uses fun_time library to print how much functions take time - [#1082](https://github.com/qarmin/czkawka/pull/1082)
- Added exporting results into json file format - [#1083](https://github.com/qarmin/czkawka/pull/1083)
- Added new test/regression suite for CI - [#1083](https://github.com/qarmin/czkawka/pull/1083)
- Added ability to use relative paths - [#1083](https://github.com/qarmin/czkawka/pull/1083)
- Allowed removing similar images/videos/music from cli - [#1087](https://github.com/qarmin/czkawka/pull/1087)
- Added info about saving/loading items to cache in duplicate and music
  mode - [#1091](https://github.com/qarmin/czkawka/pull/1091)
- Fixed number of files to check in duplicate mode - [#1091](https://github.com/qarmin/czkawka/pull/1091)
- Added support for qoi image format(without preview
  yet) - [e92a](https://github.com/qarmin/czkawka/commit/e92a8a65de9bd1250be482dbce06959125554849)
- Fixed stability problem, that could remove invalid file in CLI - [#1083](https://github.com/qarmin/czkawka/pull/1083)
- Fix Windows gui crashes by using gtk 4.6 instead 4.8 or 4.10 - [#992](https://github.com/qarmin/czkawka/pull/992)
- Fixed printing info about duplicated music files - [#1016](https://github.com/qarmin/czkawka/pull/1016)
- Fixed printing info about duplicated video files - [#1017](https://github.com/qarmin/czkawka/pull/1017)

## Version 6.0.0 - 11.06.2023r

- Add finding similar audio files by content - [#970](https://github.com/qarmin/czkawka/pull/970)
- Allow to find duplicates by name/size at once - [#956](https://github.com/qarmin/czkawka/pull/956)
- Fix, simplify and speed up finding similar images - [#983](https://github.com/qarmin/czkawka/pull/956)
- Fixed bug when cache for music tags not worked - [#970](https://github.com/qarmin/czkawka/pull/970)
- Allow to set number of threads from CLI - [#972](https://github.com/qarmin/czkawka/pull/972)
- Fix problem with invalid item sorting in bad extensions mode - [#972](https://github.com/qarmin/czkawka/pull/972)
- Big refactor/cleaning of
  code - [#956](https://github.com/qarmin/czkawka/pull/956)/[#970](https://github.com/qarmin/czkawka/pull/970)/[#972](https://github.com/qarmin/czkawka/pull/972)
- Use builtin gtk webp loader for previews - [#923](https://github.com/qarmin/czkawka/pull/923)
- Fixed docker build - [#947](https://github.com/qarmin/czkawka/pull/947)
- Restore snap builds broken since GTk 4 port - [#965](https://github.com/qarmin/czkawka/pull/947)
- Instruction how to build native ARM64 binaries on
  Mac - [#945](https://github.com/qarmin/czkawka/pull/945)/[#971](https://github.com/qarmin/czkawka/pull/971)

## Version 5.1.0 - 19.02.2023r

- Added sort button - [#894](https://github.com/qarmin/czkawka/pull/894)
- Allow to set number of thread used to scan - [#839](https://github.com/qarmin/czkawka/pull/839)
- Faster similar images comparing with reference folders - [#826](https://github.com/qarmin/czkawka/pull/826)
- Update to clap 4 - [#878](https://github.com/qarmin/czkawka/pull/878)
- Use FileChooserNative instead FileChooserDialog - [#894](https://github.com/qarmin/czkawka/pull/894)
- Fix invalid music tags in music files when using reference
  folders - [#894](https://github.com/qarmin/czkawka/pull/894)
- Updated pdf dependency(a lot of less amount of broken pdf false
  positives) - [#894](https://github.com/qarmin/czkawka/pull/894)
- Changed strange PDF error message - "Try at" - [#894](https://github.com/qarmin/czkawka/pull/894)
- Treat extensions Mp4 and m4v as identical - [#834](https://github.com/qarmin/czkawka/pull/834)
- Improve thumbnail quality - [#895](https://github.com/qarmin/czkawka/pull/895)
- Verify if hardlinking works, and if not, disable button with proper
  message - [#881](https://github.com/qarmin/czkawka/pull/881)
- Apply some pydantic clippy lints on project - [#901](https://github.com/qarmin/czkawka/pull/901)

## Version 5.0.2 - 30.08.2022r

- Fixed problem with missing some similar images when using similarity >
  0 - [#799](https://github.com/qarmin/czkawka/pull/799)
- Prebuilt Linux binaries are compiled without heif
  support - [24b](https://github.com/qarmin/czkawka/commit/24b64a32c65904c506b54270f0977ccbe5098cc8)
- Similar videos stops to proceed video after certain amount of time(fixes
  freezes)  - [#815](https://github.com/qarmin/czkawka/pull/815)
- Add --version argument for czkawka_cli - [#806](https://github.com/qarmin/czkawka/pull/806)
- Rewrite a little nonsense message about minimal file size - [#807](https://github.com/qarmin/czkawka/pull/807)

## Version 5.0.1 - 03.08.2022r

- Fixed problem with removing ending slash with empty disk window
  path - [975](https://github.com/qarmin/czkawka/commit/97563a7b2a70fb5fcf6463f28069e6ea3b0ff5c2)
- Added to CLI bad extensions mode - [#795](https://github.com/qarmin/czkawka/pull/795)
- Restore default sorting method in CLI where finding biggest
  files - [5d7](https://github.com/qarmin/czkawka/commit/5d79dc7ccfee6d5426e37c4e6a860fa555c5927a)
- Added tests to CI - [#791](https://github.com/qarmin/czkawka/pull/791)
- Show error message when all directories are set as reference
  folders - [#795](https://github.com/qarmin/czkawka/pull/795)
- Added more info about new requirements on Linux - [#795](https://github.com/qarmin/czkawka/pull/795)

## Version 5.0.0 - 28.07.2022r

- GUI ported to use GTK 4 - [#466](https://github.com/qarmin/czkawka/pull/466)
- Use multithreading and improved algorithm to compare image hashes - [#762](https://github.com/qarmin/czkawka/pull/762)
- Resize preview with window - [#466](https://github.com/qarmin/czkawka/pull/466)
- Fix removing only one item from list view - [#466](https://github.com/qarmin/czkawka/pull/466)
- Fix showing help command in duplicate CLI mode - [#720](https://github.com/qarmin/czkawka/pull/720)
- Fix freeze when not choosing any tag in similar music mode - [#732](https://github.com/qarmin/czkawka/pull/732)
- Fix preview of files with non-lowercase extensions - [#694](https://github.com/qarmin/czkawka/pull/694)
- Read more tags from music files - [#705](https://github.com/qarmin/czkawka/pull/705)
- Improve checking for invalid
  extensions - [#705](https://github.com/qarmin/czkawka/pull/705), [#747](https://github.com/qarmin/czkawka/pull/747), [#749](https://github.com/qarmin/czkawka/pull/749)
- Support for finding invalid PDF files - [#705](https://github.com/qarmin/czkawka/pull/705)
- Re-enable checking for broken music files(`libasound.so.2` no longer
  needed) - [#705](https://github.com/qarmin/czkawka/pull/705)
- Fix disabled ui when using invalid settings in similar music - [#740](https://github.com/qarmin/czkawka/pull/740)
- Speedup searching for invalid extensions - [#740](https://github.com/qarmin/czkawka/pull/740)
- Support for finding the smallest files - [#741](https://github.com/qarmin/czkawka/pull/741)
- Improved Windows CI - [#749](https://github.com/qarmin/czkawka/pull/749)
- Ability to check for broken files by types - [#749](https://github.com/qarmin/czkawka/pull/749)
- Add heif and Webp files support - [#750](https://github.com/qarmin/czkawka/pull/750)
- Use in CLI Clap library instead StructOpt - [#759](https://github.com/qarmin/czkawka/pull/759)
- Multiple directories can be added via Manual Add button - [#782](https://github.com/qarmin/czkawka/pull/782)
- Option to exclude files from other filesystems in GUI(Linux) - [#776](https://github.com/qarmin/czkawka/pull/776)

## Version 4.1.0 - 24.04.2022r

- New mode - finding files whose content not match with their
  extension - [#678](https://github.com/qarmin/czkawka/pull/678)
- Builtin icons - no more invalid, theme/OS dependent icons - [#659](https://github.com/qarmin/czkawka/pull/659)
- Big(usually 2x) speedup of showing previews of images(both previews in scan and compare
  window) - [#660](https://github.com/qarmin/czkawka/pull/660)
- Fix selecting records by custom selection popup - [#632](https://github.com/qarmin/czkawka/pull/632)
- Support more tags when comparing music files - [#590](https://github.com/qarmin/czkawka/pull/590)
- Fix not proper selecting path - [#656](https://github.com/qarmin/czkawka/pull/656)
- No more popups during scan for similar videos on Windows - [#656](https://github.com/qarmin/czkawka/pull/656) -
  external
  change [4056](https://github.com/Farmadupe/ffmpeg_cmdline_utils/commit/405687514f9d9e8984cbe2547c53e85b71e08b27)
- Custom selecting is now case-insensitive by default - [#657](https://github.com/qarmin/czkawka/pull/657)
- Better approximate comparison of tags - [#641](https://github.com/qarmin/czkawka/pull/641)
- Fix search problem due accumulated stop events - [#623](https://github.com/qarmin/czkawka/pull/623)
- Option to ignore other filesystems in Unix OS(for now only in
  CLI) - [#673](https://github.com/qarmin/czkawka/pull/673)
- Fix file hardlinking on Windows - [#668](https://github.com/qarmin/czkawka/pull/668)
- Support for case-insensitive name grouping of files - [#669](https://github.com/qarmin/czkawka/pull/669)
- Directories for search GUI can be passed by CLI - [#677](https://github.com/qarmin/czkawka/pull/677)
- Prevent from getting non respond app notification from display
  servers - [#625](https://github.com/qarmin/czkawka/pull/625)

## Version 4.0.0 - 20.01.2022r

- Multithreading support for collecting files to check(2/3x speedup on 4 thread processor and
  SSD) - [#502](https://github.com/qarmin/czkawka/pull/502), [#504](https://github.com/qarmin/czkawka/pull/504)
- Add multiple translations - Polish, Italian, French, German,
  Russian ... - [#469](https://github.com/qarmin/czkawka/pull/469), [#508](https://github.com/qarmin/czkawka/pull/508), [5be](https://github.com/qarmin/czkawka/commit/5be801e76395855f07ab1da43cdbb8bd0b843834)
- Add support for finding similar videos - [#460](https://github.com/qarmin/czkawka/pull/460)
- GUI code refactoring and search code
  unification - [#462](https://github.com/qarmin/czkawka/pull/462), [#531](https://github.com/qarmin/czkawka/pull/531)
- Fixed crash when trying to hard/symlink 0 files - [#462](https://github.com/qarmin/czkawka/pull/462)
- GTK 4 compatibility improvements for future change of
  toolkit - [#467](https://github.com/qarmin/czkawka/pull/467), [#468](https://github.com/qarmin/czkawka/pull/468), [#473](https://github.com/qarmin/czkawka/pull/473), [#474](https://github.com/qarmin/czkawka/pull/474), [#503](https://github.com/qarmin/czkawka/pull/503), [#505](https://github.com/qarmin/czkawka/pull/505)
- Change minimal supported OS to Ubuntu 20.04(needed by GTK) - [#468](https://github.com/qarmin/czkawka/pull/468)
- Increased performance by avoiding creating unnecessary image
  previews - [#468](https://github.com/qarmin/czkawka/pull/468)
- Improved performance due caching hash of broken/not supported
  images/videos = [#471](https://github.com/qarmin/czkawka/pull/471)
- Option to not remove cache from non-existent files(e.g. from unplugged
  pendrive) - [#472](https://github.com/qarmin/czkawka/pull/472)
- Add multiple tooltips with helpful messages - [#472](https://github.com/qarmin/czkawka/pull/472)
- Allow caching prehash - [#477](https://github.com/qarmin/czkawka/pull/477)
- Improve custom selecting of records(allows to use Rust regex) - [#489](https://github.com/qarmin/czkawka/pull/478)
- Remove support for finding zeroed files - [#461](https://github.com/qarmin/czkawka/pull/461)
- Remove HashMB mode - [#476](https://github.com/qarmin/czkawka/pull/476)
- Approximate comparison of music - [#483](https://github.com/qarmin/czkawka/pull/483)
- Enable column sorting for simple treeview - [#487](https://github.com/qarmin/czkawka/pull/487)
- Allow hiding upper panel - [#491](https://github.com/qarmin/czkawka/pull/491)
- Make UI take less space - [#500](https://github.com/qarmin/czkawka/pull/500)
- Add support for raw images(NEF, CR2, KDC...) - [#532](https://github.com/qarmin/czkawka/pull/532)
- Image compare performance and usability
  improvements - [#529](https://github.com/qarmin/czkawka/pull/529), [#528](https://github.com/qarmin/czkawka/pull/528), [#530](https://github.com/qarmin/czkawka/pull/530), [#525](https://github.com/qarmin/czkawka/pull/525)
- Reorganize(unify) saving/loading data from file - [#524](https://github.com/qarmin/czkawka/pull/524)
- Add "reference folders" -  [#516](https://github.com/qarmin/czkawka/pull/516)
- Add cache for similar music files - [#558](https://github.com/qarmin/czkawka/pull/558)

## Version 3.3.1 - 22.11.2021r

- Fix crash when moving buttons [#457](https://github.com/qarmin/czkawka/pull/457)
- Hide move button at start [c9ca230](https://github.com/qarmin/czkawka/commit/c9ca230dfd05e2166b2d68683b091cfd45037edd)

## Version 3.3.0 - 20.11.2021r

- Select files by pressing space key [#415](https://github.com/qarmin/czkawka/pull/415)
- Add additional info to printed errors [#446](https://github.com/qarmin/czkawka/pull/446)
- Add support for multiple image filters, hashes and sizes in similar images
  tool [#447](https://github.com/qarmin/czkawka/pull/447), [#448](https://github.com/qarmin/czkawka/pull/448)
- Button to move files/folders to provided location [#449](https://github.com/qarmin/czkawka/pull/449)
- Add non-clickable button to fix white theme [#450](https://github.com/qarmin/czkawka/pull/450)
- Fixed freeze when opening in same thread file/folder [#448](https://github.com/qarmin/czkawka/pull/448)
- Tool to check performance of different image filters and hash types and
  sizes [#447](https://github.com/qarmin/czkawka/pull/447)
- Add scheduled CI and pin it to support Rust
  1.53.0 [7bb](https://github.com/qarmin/czkawka/commit/7bbdf742739a513b80d0cc06ba61dfafec976b23), [#431](https://github.com/qarmin/czkawka/pull/431)
- Update snap file to use builtin rust plugin and update gnome
  extension [8f2](https://github.com/qarmin/czkawka/commit/8f232285e5c34bee6d5da8e1453d7f40a0ffd08d)
- Disable from checking in similar images `webp`, `gif`, `bmp`, `ico` extension which caused
  crashes [#445](https://github.com/qarmin/czkawka/pull/446), [49e](https://github.com/qarmin/czkawka/commit/49effca169adb57b33f666757966d43b244319cc)

## Version 3.2.0 - 07.08.2021r

- Use checkbox instead selection to select files [#392](https://github.com/qarmin/czkawka/pull/392)
- Re-enable hardlink on windows - [#410](https://github.com/qarmin/czkawka/pull/410)
- Fix symlink and hardlink creating - [#409](https://github.com/qarmin/czkawka/pull/409)
- Add image preview to duplicate finder [#408](https://github.com/qarmin/czkawka/pull/408)
- Add setting maximum file size [#407](https://github.com/qarmin/czkawka/pull/407)
- Add new grouping algorithm to similar images [#405](https://github.com/qarmin/czkawka/pull/405)
- Update to Rust 1.54 [#400](https://github.com/qarmin/czkawka/pull/400)
- Add webp support to similar images [#396](https://github.com/qarmin/czkawka/pull/396)
- Use GtkScale instead radio buttons for similarity [#397](https://github.com/qarmin/czkawka/pull/397)
- Update all
  dependencies [#405](https://github.com/qarmin/czkawka/pull/405), [#395](https://github.com/qarmin/czkawka/pull/395)
- Split UI into multiple files [#391](https://github.com/qarmin/czkawka/pull/391)
- Update to gtk-rs 0.14 [#383](https://github.com/qarmin/czkawka/pull/383)
- Fix bug with moving windows [#361](https://github.com/qarmin/czkawka/pull/361)
- Generate Minimal Appimage [#339](https://github.com/qarmin/czkawka/pull/339)

## Version 3.1.0 - 09.05.2021r

- Clean README, by moving instructions to different
  files - [9aea6e9b](https://github.com/qarmin/czkawka/commit/9aea6e9b1ef5ac1e56ccd008e7456b80401179d0)
- Fix excluded items on Windows - [#324](https://github.com/qarmin/czkawka/pull/324)
- Center windows and add missing settings icon - [#323](https://github.com/qarmin/czkawka/pull/323)
- Sort cache - [#322](https://github.com/qarmin/czkawka/pull/322)
- Add desktop file to
  Snap - [018d5bebb](https://github.com/qarmin/czkawka/commit/018d5bebb0b297ba35529b03b8e2e68eb0a9b474), [ade2a756e2](https://github.com/qarmin/czkawka/commit/ade2a756e29c5ce5739d6268fcab7e76f59ed5f6)
- Customize minimum file size of cached records - [#321](https://github.com/qarmin/czkawka/pull/321)
- Update benchmarks - [2044b9185](https://github.com/qarmin/czkawka/commit/2044b91852fea89dfaf10dc1ab79c1d00e9e0c12)
- Rearrange
  Instruction - [8e7ac4a2d7f5b0](https://github.com/qarmin/czkawka/commit/8e7ac4a2d7f5b0beba2552581fb3a0d19c2efeb5)
- Add info that Czkawka and Bleachbit are not alternatives to each
  other - [30602a486](https://github.com/qarmin/czkawka/commit/30602a486f6ade6f9b7b91a73708225b4f4c2a7d)
- Fix crashes with too small message queue - [#316](https://github.com/qarmin/czkawka/pull/316)
- Fix a little unsorted results - [#304](https://github.com/qarmin/czkawka/pull/304)
- Fix Appimage(external bug) - [#299](https://github.com/qarmin/czkawka/issues/299)
- Fix error with saving results of name duplicates - [#307](https://github.com/qarmin/czkawka/pull/307)
- Update to Rust 1.5.1 - [#302](https://github.com/qarmin/czkawka/pull/302)

## Version 3.0.0 - 11.03.2021r

- Option to not ignore hardlinks - [#273](https://github.com/qarmin/czkawka/pull/273)
- Hardlink support for GUI - [#276](https://github.com/qarmin/czkawka/pull/276)
- New settings window - [#262](https://github.com/qarmin/czkawka/pull/262)
- Unify file removing - [#278](https://github.com/qarmin/czkawka/pull/278)
- Dryrun in duplicates CLI - [#277](https://github.com/qarmin/czkawka/pull/277)
- Option to turn off cache - [#263](https://github.com/qarmin/czkawka/pull/263)
- Update Image dependency and fix
  crashes - [#270](https://github.com/qarmin/czkawka/pull/270), [e3aca69](https://github.com/qarmin/czkawka/commit/e3aca69499966499413e4b7cd4d1037bec6a5d68)
- Add confirmation dialog when trying to remove all files in group - [#281](https://github.com/qarmin/czkawka/pull/281)
- Add confirmation dialog when removing files with delete key - [#282](https://github.com/qarmin/czkawka/pull/282)
- Open file when clicking at the Enter button - [#285](https://github.com/qarmin/czkawka/pull/285)
- Allow to put files to trash instead fully remove them - [#284](https://github.com/qarmin/czkawka/pull/284)

## Version 2.4.0 - 22.02.2021r

- Add about dialog - [#226](https://github.com/qarmin/czkawka/pull/226)
- Remove checking for ico in similar images - [#227](https://github.com/qarmin/czkawka/pull/227)
- Change progress dialog to progress window - [#229](https://github.com/qarmin/czkawka/pull/229)
- Restore snap
  confinement - [#218](https://github.com/qarmin/czkawka/pull/218), [8dcb718](https://github.com/qarmin/czkawka/commit/8dcb7188434e1c1728368642e17ccec29a4b372d)
- Add support for CRC32 and XXH3 hash - [#243](https://github.com/qarmin/czkawka/pull/243)
- Add delete method to replace duplicate files with hard links - [#236](https://github.com/qarmin/czkawka/pull/236)
- Add checking for broken music opt-in - [#249](https://github.com/qarmin/czkawka/pull/249)
- Allow to save to file similar images
  results - [10156ccfd3](https://github.com/qarmin/czkawka/commit/10156ccfd3ba880d26d4bbad1e025b0050d7753b)
- Keep original file if replacing duplicate with hardlink fails - [#256](https://github.com/qarmin/czkawka/pull/256)
- Fix Windows theme - [#265](https://github.com/qarmin/czkawka/pull/265)
- Windows taskbar progress support - [#264](https://github.com/qarmin/czkawka/pull/264)
- Ignore duplicates if those are hard links - [#234](https://github.com/qarmin/czkawka/pull/234)
- Support the hash type parameter in the CLI - [#267](https://github.com/qarmin/czkawka/pull/267)
- Use one implementation for all hash calculations - [#268](https://github.com/qarmin/czkawka/pull/268)
- Disable for now broken tga and gif files - [#270](https://github.com/qarmin/czkawka/pull/270)

## Version 2.3.2 - 21.01.2021r

- Add support for moving selection by keyboard to update similar image
  preview [#223](https://github.com/qarmin/czkawka/pull/223)

This version is only needed to test flatpak build

## Version 2.3.1 - 20.01.2021r

- Added flatpak support - [#203](https://github.com/qarmin/czkawka/pull/203)
- Spell fixes - [#222](https://github.com/qarmin/czkawka/pull/222), [#219](https://github.com/qarmin/czkawka/pull/219)

## Version 2.3.0 - 15.01.2021r

- Add cache for duplicate finder - [#205](https://github.com/qarmin/czkawka/pull/205)
- Add cache for broken files - [#204](https://github.com/qarmin/czkawka/pull/204)
- Decrease ram usage - [#212](https://github.com/qarmin/czkawka/pull/212)
- Add support for finding broken zip and audio files - [#210](https://github.com/qarmin/czkawka/pull/210)
- Sort Results by path where it is possible - [#211](https://github.com/qarmin/czkawka/pull/211)
- Add missing popover info for invalid symlinks - [#209](https://github.com/qarmin/czkawka/pull/209)
- Use the oldest available OS in Linux and Mac CI and the newest on
  Windows - [#206](https://github.com/qarmin/czkawka/pull/206)
- Add broken files support - [#202](https://github.com/qarmin/czkawka/pull/202)
- Remove save workaround and fix crashes when loading/saving cache - [#200](https://github.com/qarmin/czkawka/pull/200)
- Fix error when closing dialog progress by X - [#199](https://github.com/qarmin/czkawka/pull/199)

## Version 2.2.0 - 11.01.2021r

- Adds Mac GUI - [#160](https://github.com/qarmin/czkawka/pull/160)
- Use master gtk plugin again - [#179](https://github.com/qarmin/czkawka/pull/179)
- Only show preview when 1 image is selected - [#183](https://github.com/qarmin/czkawka/pull/183)
- Add buffered write/read - [#186](https://github.com/qarmin/czkawka/pull/186)
- Fix included/excluded files which contains commas - [#195](https://github.com/qarmin/czkawka/pull/195)
- Move image cache to cache from config dir - [#197](https://github.com/qarmin/czkawka/pull/197)
- Reorganize GUI Code(no visible
  changes) - [#184](https://github.com/qarmin/czkawka/pull/184), [#184](https://github.com/qarmin/czkawka/pull/184), [#189](https://github.com/qarmin/czkawka/pull/189), [#190](https://github.com/qarmin/czkawka/pull/190), [#194](https://github.com/qarmin/czkawka/pull/194)

## Version 2.1.0 - 31.12.2020r

- Hide preview when deleting images or symlinking it - [#167](https://github.com/qarmin/czkawka/pull/167)
- Add manual adding of
  directories - [#165](https://github.com/qarmin/czkawka/pull/165), [#168](https://github.com/qarmin/czkawka/pull/168)
- Add resizable top panel - [#164](https://github.com/qarmin/czkawka/pull/164)
- Add support for delete button - [#159](https://github.com/qarmin/czkawka/pull/159)
- Allow to select multiple entries in File Chooser - [#154](https://github.com/qarmin/czkawka/pull/154)
- Add cache support for similar images - [#139](https://github.com/qarmin/czkawka/pull/139)
- Add selecting images with its size - [#138](https://github.com/qarmin/czkawka/pull/138)
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
- Added
  instruction - [58e6221a](https://github.com/qarmin/czkawka/commit/58e6221a0e02d17d07c71152f56b948f616751a8), [598aec345e](https://github.com/qarmin/czkawka/commit/598aec345e9f5ac199fc3d642c0699d5228100a6), [afaa402b](https://github.com/qarmin/czkawka/commit/afaa402b31526aa8e6b47f3670bc62b26ad9f60f)

## Version 1.5.1 - 08.12.2020r

- Fix errors in progress bar caused by dividing by 0 - [#109](https://github.com/qarmin/czkawka/pull/109)
- Add option to save file, store settings and load them - [#108](https://github.com/qarmin/czkawka/pull/108)
- Center dialog to current
  window - [a04](https://github.com/qarmin/czkawka/commit/a047380dbe8aa4d04f9c482364469e21d231fab2)

## Version 1.5.0 - 02.12.2020r

- Added progress bar - [#106](https://github.com/qarmin/czkawka/pull/106)
- Removed unused buttons - [#107](https://github.com/qarmin/czkawka/pull/107)

## Version 1.4.0 - 09.11.2020r

- Multithreading Support to most
  modules - [#98](https://github.com/qarmin/czkawka/pull/98) [#99](https://github.com/qarmin/czkawka/pull/99) [#100](https://github.com/qarmin/czkawka/pull/100) [#101](https://github.com/qarmin/czkawka/pull/101)
- Simplify GUI code [#96](https://github.com/qarmin/czkawka/pull/96)
- Group similar images - [#97](https://github.com/qarmin/czkawka/pull/97)
- Add select buttons to each type of mode - [#102](https://github.com/qarmin/czkawka/pull/102)
- Fix GUI behavior in GUI when deleting similar image - [#103](https://github.com/qarmin/czkawka/pull/103)
- Add new similarity level - [#104](https://github.com/qarmin/czkawka/pull/104)

## Version 1.3.0 - 02.11.2020r

- Appimage support - [#77](https://github.com/qarmin/czkawka/pull/77)
- Removed warnings about non-existed excluded directories - [#79](https://github.com/qarmin/czkawka/pull/79)
- Updated README - [8ec](https://github.com/qarmin/czkawka/commit/8ecde0fc9adb3e6cedf432c4ba749e698b645a7a)
- Added pre hash support(speedup for searching big duplicates) - [#83](https://github.com/qarmin/czkawka/pull/83)
- Support for searching duplicates by file name - [#84](https://github.com/qarmin/czkawka/pull/84)
- Added support for checking for zeroed file - [#88](https://github.com/qarmin/czkawka/pull/88)
- Refactored GUI code to faster and safer changing/adding code - [#89](https://github.com/qarmin/czkawka/pull/89)
- Added some missing options to CLI in some modes - [#90](https://github.com/qarmin/czkawka/pull/90)
- Implemented finding duplicates by music tags - [#95](https://github.com/qarmin/czkawka/pull/95)

## Version 1.2.1 - 17.10.2020r

- Make image similarity search significantly faster. [#72](https://github.com/qarmin/czkawka/pull/72)
- Improve similar images GUI a little and add sorting to Similarity
  Enum [#73](https://github.com/qarmin/czkawka/pull/73)
- Improve deleting files in Similar files in GUI [#75](https://github.com/qarmin/czkawka/pull/75)

## Version 1.2.0 - 15.10.2020r

- Replace String with PathBuf for paths [#59](https://github.com/qarmin/czkawka/pull/59)
- Add test suite to PR [#65](https://github.com/qarmin/czkawka/pull/65)
- Support for finding similar images to CLI [#66](https://github.com/qarmin/czkawka/pull/66)
- Fix grammar-related errors and Ponglish
  expressions [#62](https://github.com/qarmin/czkawka/pull/62), [#63](https://github.com/qarmin/czkawka/pull/63)
- Don't delete by default files in duplicate finder in
  CLI - [23f203](https://github.com/qarmin/czkawka/commit/23f203a061e254275c95ca23ca4f1a78bd941f02)
- Support for finding similar images to GUI [#69](https://github.com/qarmin/czkawka/pull/69)
- Add support for opening files/folders from GUI with double-click [#70](https://github.com/qarmin/czkawka/pull/70)

## Version 1.1.0 - 10.10.2020r

- Windows support [#58](https://github.com/qarmin/czkawka/pull/58)
- Improve code quality/Simplify codebase [#52](https://github.com/qarmin/czkawka/pull/52)
- Fixed skipping some correct results in specific
  situations [#52](https://github.com/qarmin/czkawka/pull/52#discussion_r502613895)
- Added support for searching in other thread [#51](https://github.com/qarmin/czkawka/pull/51)
- Divide CI across files [#48](https://github.com/qarmin/czkawka/pull/48)
- Added ability to stop task from GUI [#55](https://github.com/qarmin/czkawka/pull/55)
- Fixed removing directories which contains only empty directories from
  GUI [#57](https://github.com/qarmin/czkawka/pull/57)

## Version 1.0.1 - 06.10.2020r

- Replaced default argument parser with StructOpt [#37](https://github.com/qarmin/czkawka/pull/37)
- Added all(except macOS GTK build) builds to CI where can be freely
  downloaded [#41](https://github.com/qarmin/czkawka/pull/41) [#39](https://github.com/qarmin/czkawka/pull/39)
- App can be downloaded also from Arch AUR and Cargo [#36](https://github.com/qarmin/czkawka/pull/36)
- Fixed crash with invalid file modification date [#33](https://github.com/qarmin/czkawka/issues/33)
- Upper tabs can hide and show when this is necessary [#38](https://github.com/qarmin/czkawka/pull/38)
- Fixed crash when file/folder name have non Unicode character [#44](https://github.com/qarmin/czkawka/issues/44)
- Added support for finding similar pictures in GUI [#69](https://github.com/qarmin/czkawka/issues/69)

## Version 1.0.0 - 02.10.2020r

- Added confirmation dialog to delete button
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
- Added support for non-recursive search
- Improved finding number and size of duplicated files
- Saving results to file
- Print how much data was read by duplicate finder(debug only)
- Added GitHub CI
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
