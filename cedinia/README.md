<div align="center"><img src="https://github.com/user-attachments/assets/ed6dfeea-a984-49e8-a621-8d6ae521c760" alt="cedinia_logo" width="600" /></div>

Cedinia is a new Android touch friendly GUI frontend for Czkawka Core, built with Slint.

Supports the same scanning tools as Krokiet (duplicates, similar images and videos, empty files and folders, large files, invalid extensions, duplicate music, and more). The only missing features are similar video detection based on frames, detection of corrupted video files, and video optimization, all of which require FFmpeg.

The name refers to the Battle of Cedynia in 972, a victory that was significant for the early Polish state.

<div align="center"><img src="https://github.com/user-attachments/assets/d1e486a2-1d11-4df8-9fff-0e0af2d003da" alt="cedinia_logo" width="1000" /></div>

## Installation

The easiest way to install it (at least for now) is to download the latest release APK from the releases page: https://github.com/qarmin/czkawka/releases

Alternatively, you can download the latest nightly build here: https://github.com/qarmin/czkawka/releases/download/Nightly/cedinia.apk

There is currently no distribution via F-Droid, Google Play Store, or any other app store, but I am open to suggestions and contributions in this area.

Other OS support, such as iOS, is not planned at this time (although it is likely possible to port it manually)

## Compilation / Setup

The build process is relatively complex and documentation is limited. You can refer to the Android GitHub workflow for more details, but complete and up-to-date instructions are still missing.

## Known issues and missing features

- Keyboard support currently has multiple issues due to bugs in Slint
- Only portrait mode is supported with specific aspect ratios, which may cause issues on some devices, especially tablets  

## AI usage

Because this project touches parts of Android I am not deeply familiar with, I used AI assistance during development, mainly for the bridge code between this application and Android via `jni-rs`. The code was carefully tested.

## License

The code is licensed under the MIT license, however the overall project is distributed under GPL-3.0 due to Slint licensing restrictions.

All icons and images are licensed under the [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) license.