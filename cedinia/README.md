# Cedinia

Cedinia is an experimental Android touch friendly GUI frontend for Czkawka Core, built with Slint. 

**Do not use it!!!!**

The name refers to the Battle of Cedynia in 972, a victory significant to the early Polish state.

## AI usage
Because this project goes into parts of Android that I am not familiar with, I used a lot of AI assistance during development. I reviewed and guided every step myself, but AI helped speed things up.

At first, I hoped to build the app without writing any Java code. However, accessing the file system and implementing a file picker on Android requires Java. Since I do not normally work with it, AI support was necessary. Without it, I would have needed to spend many days studying the JNI Rust bindings and Android documentation before I could move forward.

I do not expect this project to become anything serious. It will most likely remain a small side experiment. If that changes, I will review all AI generated code and rewrite it where needed so it works properly and matches my standards.

## Compilation/Setup - Ubuntu only

```shell
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64   # must be JDK 17, see note above
export ANDROID_HOME=$HOME/android-sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/26.3.11579264
export ANDROID_BUILD_TOOLS_VERSION=35.0.0              # set to 35 if you use JDK 21+
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
export PATH=$PATH:$ANDROID_HOME/platform-tools

# Ubuntu / Debian - install Java (if you don't have it already)
sudo apt install openjdk-17-jdk

# Desktop (Rust) build/run
cargo build -p cedinia
cargo run -p cedinia

# Linux example – Android command-line tools setup (adjust paths to taste)
ANDROID_HOME=$HOME/android-sdk
mkdir -p $ANDROID_HOME/cmdline-tools
cd $ANDROID_HOME/cmdline-tools
# Download from https://developer.android.com/studio#command-tools, e.g. https://dl.google.com/android/repository/commandlinetools-linux-14742923_latest.zip
# unzip commandlinetools-linux-*.zip
# mv cmdline-tools latest

# Accept licences & install required packages
$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager --licenses
$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager \
    "platform-tools" \
    "platforms;android-30" \
    "platforms;android-34" \
    "build-tools;34.0.0" \
    "build-tools;35.0.0" \
    "ndk;26.3.11579264"

# Reload the shell (if you added the exports to ~/.bashrc or ~/.zshrc)
source ~/.bashrc

# Install cargo-apk or cargo-xbuild
cargo install cargo-apk
# Or for an alternative toolchain
cargo install xbuild

# Add Android Rust targets
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android

# Build the APK
# Debug APK
cargo apk build -p cedinia --lib

# Release APK (requires a signing key)
cargo apk build -p cedinia --lib --release

# Check connected devices
adb devices

# Install and launch on a connected device
adb install -r target/debug/apk/cedinia.apk
adb shell am start -n rust.cedinia/android.app.NativeActivity

# One-liner: build → install → launch
cargo apk build -p cedinia --lib && \
  adb install -r target/debug/apk/cedinia.apk && \
  adb shell am start -n rust.cedinia/android.app.NativeActivity

# Debugging: Live logs (Rust stdout/stderr, panics)
adb logcat -s RustStdoutStderr:V

# Full logcat filtered to Cedinia
adb logcat -s RustStdoutStderr:V rust.cedinia:V AndroidRuntime:E

# Crash / panic backtrace: set RUST_BACKTRACE=1 before building
RUST_BACKTRACE=1 cargo apk build -p cedinia --lib
# Then check adb logcat -s RustStdoutStderr:V

# Uninstall
adb uninstall rust.cedinia
```