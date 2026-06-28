#!/system/bin/sh
# AddressSanitizer launcher for Cedinia debug builds.
#
# Android only injects an ASan runtime into an app through a wrap.sh placed in
# the APK at lib/<abi>/wrap.sh. It must be a debuggable build (debug APK) and
# the native libs must be extracted to disk (extractNativeLibs=true) so that
# $HERE below points at a real directory holding the .so files.
#
# Packaged by the `android_asan` just recipe into
#   cedinia/android/app/src/main/resources/lib/arm64-v8a/wrap.sh
# Do NOT commit it under release sources - it would break the release build,
# which ships no ASan runtime for the LD_PRELOAD glob to find.

HERE="$(cd "$(dirname "$0")" && pwd)"

# detect_leaks is off: LSan is noisy and slow under a GUI/JNI process and we
# care about heap-overflow / use-after-free here, not leaks.
# allow_user_segv_handler keeps Slint/Skia signal handling working.
export ASAN_OPTIONS=log_to_syslog=false,allow_user_segv_handler=1,detect_leaks=0,abort_on_error=1

ASAN_LIB=$(ls "$HERE"/libclang_rt.asan-*-android.so)
if [ -f "$HERE/libc++_shared.so" ]; then
    export LD_PRELOAD="$ASAN_LIB $HERE/libc++_shared.so"
else
    export LD_PRELOAD="$ASAN_LIB"
fi

exec "$@"
