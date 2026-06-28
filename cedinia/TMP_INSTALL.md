
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

#  Android - AddressSanitizer build (JNI testing) 
just setup_sanitizer_android   # one-time: nightly + aarch64 target + llvm-tools
just android_asan              # build (nightly+ASan) -> install -> launch + auto-logcat
just android_asan smoke        # same, but deliberately heap-overflows at startup
                               # -> confirms ASan is active (look for AddressSanitizer)
# Requires ANDROID_NDK_HOME and an arm64 device/emulator. The instrumented
# libcedinia.so is packaged by Gradle (assembleDebug) with the NDK ASan runtime
# and a wrap.sh; only the debug build is affected, release/AAB is untouched.
# After launch it streams logcat through ndk-stack (Ctrl-C to stop), which adds
# function + file:line to backtraces from the unstripped local .so. The on-device
# ASan symbolizer only shows names; file:line comes from ndk-stack.
just android_symbolize target/cedinia-asan-logcat.txt  # re-symbolize a saved dump

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
