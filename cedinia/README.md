<div align="center"><img src="https://github.com/user-attachments/assets/ed6dfeea-a984-49e8-a621-8d6ae521c760" alt="cedinia_logo" width="600" /></div>

Cedinia is a new and experimental Android touch friendly GUI frontend for Czkawka Core, built with Slint. 

The name refers to the Battle of Cedynia in 972, a victory significant to the early Polish state.

<div align="center"><img src="https://github.com/user-attachments/assets/d1e486a2-1d11-4df8-9fff-0e0af2d003da" alt="cedinia_logo" width="1000" /></div>

## Installation
The easiest way is to install it(at least for now) is to download, the latest released APK from releases - https://github.com/qarmin/czkawka/releases

Alternativelly, you can download the latest nightly build from https://github.com/qarmin/czkawka/releases/download/Nightly/cedinia.apk

There is no F-Droid, Google Play Store or any other store release for now, but I'm open to suggestions and help with that.

## Compilation/Setup
Quite complicated and have limited docs, you can look at the github android workflow for more details, but proper fully inststuctions are still missing.

## Known issues and missing features
- Keyboard support currently has multiple issues due to bugs in Slint
- The application is not available on the Google Play Store or Android  
- Only portrait mode is supported with specific aspect ratios, so it may have issues on some devices, especially tablets.

## AI usage
Because this project goes into parts of Android that I am not familiar with, I used a lot of AI assistance during development of bridge code between this app and Android with jni-rs. I tested the code long and very carefully

## License

The code is licensed under the MIT license, but the entire project is licensed under GPL-3.0 due to Slint license restrictions.

All icons and images are licensed under the [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) license.