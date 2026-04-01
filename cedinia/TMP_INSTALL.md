
```shell
#  Environment variables (add to ~/.bashrc or ~/.zshrc) 
# These are required by the justfile Android commands (justfile has `set export := true`)
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
export ANDROID_HOME=$HOME/android-sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/27.2.12479018   # NDK r27, see https://developer.android.com/ndk/downloads/revision_history
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
export PATH=$PATH:$ANDROID_HOME/platform-tools

# Reload after editing ~/.bashrc
source ~/.bashrc

#  Install Java 17 (Ubuntu/Debian) 
sudo apt install openjdk-17-jdk

#  Android command-line tools setup 
# Download from https://developer.android.com/studio#command-tools
mkdir -p $ANDROID_HOME/cmdline-tools
cd $ANDROID_HOME/cmdline-tools
# unzip commandlinetools-linux-*.zip
# mv cmdline-tools latest

# Accept licences & install required packages
sdkmanager --licenses
sdkmanager \
    "platform-tools" \
    "platforms;android-35" \
    "build-tools;35.0.0" \
    "ndk;27.2.12479018"

#  Install cargo-apk 
cargo install cargo-apk

#  Add Android Rust targets 
rustup target add aarch64-linux-android armv7-linux-androideabi

#  Desktop build/run 
cargo build -p cedinia
cargo run -p cedinia

#  Android - one-time setup 
just gen_keystores          # generate debug + release keystores (run once)

#  Android - build, install & launch 
just android                # debug:   build → install → launch
just androidr               # release: build → install → launch

#  Android - individual steps 
just android_build          # build debug APK
just android_build_release  # build release APK
just android_install        # install debug APK to connected device
just android_install_release
just android_run            # launch on connected device

#  Logs & diagnostics 
just android_log            # logcat: Rust stdout/stderr only
just android_logc           # logcat: everything from Cedinia
just android_devices        # list connected ADB devices

#  Uninstall 
adb uninstall io.github.qarmin.cedinia

#  Panic backtrace 
RUST_BACKTRACE=1 just android_build
just android_log
```
