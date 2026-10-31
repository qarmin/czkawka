set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set export := true

# Android related commands require these env variables to be set in your shell:
# export ANDROID_HOME=~/android-sdk
# export ANDROID_NDK_HOME=~/android-sdk/ndk/26.x.x

adb := "adb"
apk_package := "io.github.qarmin.cedinia"
apk_activity := "android.app.NativeActivity"

[group('test')]
build_all: && fix
    cargo build --release
    cargo build
    cargo clippy
    cargo test

[group('test')]
itests:
    [ ! -f TestFiles.zip ] && wget https://github.com/qarmin/czkawka/releases/download/6.0.0/TestFiles.zip || true
    cd ci_tester && cargo build --release && cd ..
    cargo build --release --bin czkawka_cli

    RUST_BACKTRACE=1 ci_tester/target/release/ci_tester target/release/czkawka_cli

## run

[group('run')]
run +args:
    cargo run --bin {{args}}

[group('run')]
runr +args:
    cargo run --profile fast_release --bin {{args}}

[group('run')]
runc +args:
    CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly run -Zcodegen-backend --bin {{args}}

# needs: nightly + rust-src + llvm-tools-preview (see `doctor`/`setup`)
[group('run')]
runs +args:
    export RUST_BACKTRACE=1 # or full depending on project
    export ASAN_SYMBOLIZER_PATH=$(which llvm-symbolizer) # Version depends on your system
    export ASAN_OPTIONS="symbolize=1:detect_leaks=0" # Leak check is disabled, because is slow and ususally not needed
    ASAN_OPTIONS="symbolize=1:detect_leaks=0" RUSTFLAGS="-Zsanitizer=address" cargo +nightly run --target x86_64-unknown-linux-gnu --bin {{args}}

[group('run')]
val bin:
    valgrind --leak-check=full --show-leak-kinds=definite --track-origins=yes target/debug/{{bin}}

[group('run')]
valr bin:
    valgrind --leak-check=full --show-leak-kinds=definite --track-origins=yes target/release/{{bin}}


# echo '-1' | sudo tee /proc/sys/kernel/perf_event_paranoid
# // Or permanently
# echo "kernel.perf_event_paranoid = -1" | sudo tee /etc/sysctl.d/99-perf.conf
# sudo sysctl --system

[group('profiling')]
samply bin *args:
    cargo build --bin {{bin}}
    samply record target/debug/{{bin}} {{args}}

[group('profiling')]
samplyrd bin *args:
    cargo build --bin {{bin}} --profile rdebug
    samply record target/rdebug/{{bin}} {{args}}

## setup
# doctor = read-only check that what the rest of this justfile needs is actually
# installed; exits non-zero if something required is missing. Never calls sudo:
# it prints the install command and lets the human decide whether to run it.
# setup = installs whatever `doctor` flags as missing, but only Rust-toolchain-level
# and cargo-installable things; system packages are still print-only.

# read-only health check
[group('setup')]
doctor:
    #!/usr/bin/env bash
    set +e
    fail=0
    check() {
        local name="$1"; shift
        if "$@" >/dev/null 2>&1; then
            printf "  [OK]      %s\n" "$name"
        else
            printf "  [MISSING] %s\n" "$name"
            fail=$((fail + 1))
        fi
    }
    info() {
        local name="$1"; shift
        if "$@" >/dev/null 2>&1; then
            printf "  [OK]      %s\n" "$name"
        else
            printf "  [-]       %s (optional)\n" "$name"
        fi
    }

    echo "Rust toolchain:"
    check "rustc"                      rustc --version
    check "cargo"                      cargo --version
    check "nightly toolchain"          rustup run nightly rustc --version
    check "rustfmt (nightly)"          rustup run nightly cargo fmt --version
    check "clippy"                     cargo clippy --version
    check "cargo-edit (cargo upgrade)" cargo upgrade --version
    check "uv (fix, translations)"     uv --version

    echo ""
    echo "Optional tools (only needed for specific recipes):"
    info "nightly: rust-src (runs)"             bash -c "rustup component list --toolchain nightly --installed | grep -q rust-src"
    info "nightly: llvm-tools-preview (runs)"   bash -c "rustup component list --toolchain nightly --installed | grep -q llvm-tools"
    info "llvm-symbolizer (runs)"               bash -c "command -v llvm-symbolizer"
    info "valgrind (val/valr)"                  valgrind --version
    info "samply (samply/samplyrd)"             samply --version
    info "heaptrack (heaptrack/test_heaptrack)" heaptrack --version
    info "perf (hotspot profiling)"             perf --version
    info "hyperfine (benchmark)"                hyperfine --version
    info "cargo-llvm-lines"                     cargo llvm-lines --version
    info "cargo-bloat"                          cargo bloat --version
    info "cargo-deps"                           bash -c "command -v cargo-deps"
    info "graphviz dot (dependencies_graph)"    dot -V
    info "flamegraph"                           bash -c "command -v flamegraph"
    info "measureme (profiling/timings)"        bash -c "command -v summarize"
    info "ollama (translate deps)"              ollama --version
    info "wget (itests/prepare_binaries)"       wget --version
    info "adb (android)"                        adb --version
    info "gradle (android)"                     gradle --version
    info "keytool (gen_keystores)"              bash -c "command -v keytool"

    echo ""
    if [ "$fail" -eq 0 ]; then
        echo "All required tools present."
    else
        echo "$fail required tool(s) missing - run 'just setup' to install what's installable."
        exit 1
    fi

# installs whatever `doctor` flagged as missing (Rust-toolchain-level and cargo-installable only)
[group('setup')]
setup: setup_sanitizer setup_verify_tools
    #!/usr/bin/env bash
    set -e
    rustup component add clippy
    rustup component add rustfmt --toolchain nightly
    command -v cargo-upgrade >/dev/null 2>&1 || cargo install cargo-edit --locked
    command -v uv >/dev/null 2>&1 || curl -LsSf https://astral.sh/uv/install.sh | sh
    command -v samply >/dev/null 2>&1 || cargo install samply --locked

    echo "System packages not installed here (no sudo) - install manually if missing:"
    command -v valgrind  >/dev/null 2>&1 || echo "  sudo apt install valgrind"
    command -v heaptrack >/dev/null 2>&1 || echo "  sudo apt install heaptrack"
    command -v perf      >/dev/null 2>&1 || echo "  sudo apt install linux-tools-common linux-tools-generic"
    command -v hyperfine >/dev/null 2>&1 || echo "  sudo apt install hyperfine"
    command -v dot       >/dev/null 2>&1 || echo "  sudo apt install graphviz"
    command -v wget      >/dev/null 2>&1 || echo "  sudo apt install wget"
    command -v ollama    >/dev/null 2>&1 || echo "  curl -fsSL https://ollama.com/install.sh | sh"

    echo "Done - run 'just doctor' to verify."

[group('setup')]
setup_sanitizer:
    rustup install nightly
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
    rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu

[group('setup')]
setup_sanitizer_android:
    rustup install nightly
    rustup target add aarch64-linux-android --toolchain nightly
    rustup component add llvm-tools-preview --toolchain nightly


[group('benchmark')]
bench:
    cd czkawka_core && cargo bench
    xdg-open target/criterion/report/index.html

[group('benchmark')]
bench_clean:
    rm -rf target/criterion

# Bumps version requirements in Cargo.toml, including incompatible ones (needs cargo-edit, see `doctor`)
[group('install')]
upgrade:
    cargo upgrade -i
    cargo update

[group('fix')]
clip:
    cargo clippy --fix --allow-dirty --allow-staged --all-features --all-targets
    cargo clippy --fix --allow-dirty --allow-staged --no-default-features --features winit_software --all-targets

[group('fix')]
fix:
    grep -rl --null -F --include='*.rs' --include='*.slint' --include='*.md' --include='*.ftl' --exclude='AGENTS.md' --exclude='justfile' --exclude-dir='.git' --exclude-dir='target' -e '─' -e '–' -e '—' -e '…' . | xargs -0 -r perl -CSD -i -pe 's/[\x{2500}\x{2013}\x{2014}]/-/g; s/\x{2026}\.?/.../g' || true
    cp misc/pyproject.toml .
    uv sync

    uv run ruff format --line-length 120 --no-cache
    uv run mypy misc --strict

    bash misc/run_checks.sh

    cargo +nightly fmt
    cargo clippy --fix --allow-dirty --allow-staged --all-features --all-targets
    cargo +nightly fmt
    # stable rustfmt warns that the nightly-only import options in .rustfmt.toml aren't
    # available on this channel - harmless noise on every run, so it's silenced here.
    cargo fmt 2>/dev/null

[group('fix')]
fixn:
    cargo +nightly fmt
    cargo +nightly clippy --fix --allow-dirty --allow-staged --all-features --all-targets
    cargo +nightly fmt

[group('benchmark')]
test_resize arg:
    cd misc/test_image_perf; cargo build --release; sudo ./target/release/test_image_perf "{{arg}}"

# Run all normal (non-ignored) tests across the workspace
[group('test')]
test:
    cargo test --workspace

# Run only #[ignore]-marked tests (fuzzers, stress tests, slow tests)
[group('test')]
ignored:
    cargo test --workspace -- --ignored

# Not works, due of edition 2024 and workspaces
[group('profiling')]
unused_features:
    unused-features analyze
    unused-features build-report --input krokiet/report.json
    unused-features build-report --input czkawka_cli/report.json
    unused-features build-report --input czkawka_core/report.json
    xdg-open krokiet/report.html
    xdg-open czkawka_cli/report.html
    xdg-open czkawka_core/report.html

##################### VERSION #####################

# Bump version in all crates, UI files and release metadata, e.g. `just change-version 12.0.1`
[group('release')]
change-version version:
    uv run misc/change_version.py {{version}}

##################### LICENSES #####################

# Generate third-party license file for Cedinia (parses `cargo metadata` JSON directly, no extra tool needed).
[group('release')]
gen_cedinia_licenses:
    cargo metadata --format-version 1 \
        | python3 misc/gen_cedinia_licenses.py > cedinia/THIRD_PARTY_LICENSES.txt
    @echo "Generated cedinia/THIRD_PARTY_LICENSES.txt ($(wc -l < cedinia/THIRD_PARTY_LICENSES.txt) lines)"

##################### ANDROID #####################

keystore_dir := "cedinia/android/keystore"

[group('android')]
gen_keystores:
    #!/usr/bin/env bash
    set -euo pipefail
    PASS=$(cat keystore_pass)
    mkdir -p {{keystore_dir}}
    [ -f {{keystore_dir}}/debug.keystore ] || keytool -genkey -v \
        -keystore {{keystore_dir}}/debug.keystore \
        -alias debug -keyalg RSA -keysize 2048 -validity 10000 \
        -storepass "$PASS" -keypass "$PASS" \
        -dname "CN=Debug, OU=Debug, O=Debug, L=Debug, S=Debug, C=US" \
        -noprompt
    [ -f {{keystore_dir}}/rdebug.keystore ] || keytool -genkey -v \
        -keystore {{keystore_dir}}/rdebug.keystore \
        -alias rdebug -keyalg RSA -keysize 2048 -validity 10000 \
        -storepass "$PASS" -keypass "$PASS" \
        -dname "CN=Release, OU=Release, O=Release, L=Release, S=Release, C=US" \
        -noprompt
    [ -f {{keystore_dir}}/release.keystore ] || keytool -genkey -v \
        -keystore {{keystore_dir}}/release.keystore \
        -alias release -keyalg RSA -keysize 2048 -validity 10000 \
        -storepass "$PASS" -keypass "$PASS" \
        -dname "CN=Release, OU=Release, O=Release, L=Release, S=Release, C=US" \
        -noprompt

[group('android')]
android_build:
    #!/usr/bin/env bash
    set -euo pipefail
    PASS=$(cat keystore_pass)
    sed -i "s|TO_REPLACE_KEYSTORE_PASSWORD|$PASS|g" cedinia/Cargo.toml
    trap 'sed -i "s|$PASS|TO_REPLACE_KEYSTORE_PASSWORD|g" cedinia/Cargo.toml' EXIT

    cargo apk build -p cedinia --lib

[group('android')]
android_build_release:
    #!/usr/bin/env bash
    set -euo pipefail
    PASS=$(cat keystore_pass)
    sed -i "s|TO_REPLACE_KEYSTORE_PASSWORD|$PASS|g" cedinia/Cargo.toml
    trap 'sed -i "s|$PASS|TO_REPLACE_KEYSTORE_PASSWORD|g" cedinia/Cargo.toml' EXIT

    cargo apk build -p cedinia --lib --release

[group('android')]
android_install:
    {{adb}} install -r target/debug/apk/cedinia.apk

[group('android')]
android_install_release:
    {{adb}} install -r target/release/apk/cedinia.apk

[group('android')]
android_run:
    {{adb}} shell am start -n {{apk_package}}/{{apk_activity}}

[group('android')]
android_log:
    {{adb}} logcat -s RustStdoutStderr:V *:S

[group('android')]
android_logc:
    {{adb}} logcat | grep edinia

[group('android')]
android_devices:
    {{adb}} devices -l

[group('android')]
android: android_build android_install android_run

[group('android')]
androidr: android_build_release android_install_release android_run

# Install and launch a prebuilt APK (e.g. downloaded from CI) without building.
[group('android')]
android_manual apk:
    {{adb}} install -r "{{apk}}"
    {{adb}} shell am start -n {{apk_package}}/{{apk_activity}}

# Build, install and launch a debug APK with AddressSanitizer for testing the
# JNI bridge (file_picker_android.rs) on a real arm64 device/emulator.
#
# Requires: nightly toolchain, ANDROID_NDK_HOME, a connected arm64 device.
# How it works:
#   1. Compile libcedinia.so for aarch64-linux-android with -Zsanitizer=address
#      (nightly), linking through the NDK clang wrapper. The cdylib leaves the
#      __asan_* symbols undefined - they are resolved at load by wrap.sh.
#   2. Stage libcedinia.so + the NDK ASan runtime into Gradle jniLibs and drop
#      wrap.sh into resources/lib/<abi>/ (the only supported ASan injection on
#      Android). The debug overlay manifest forces extractNativeLibs=true.
#   3. gradle assembleDebug packages a debuggable APK; install + launch.
# After launch it auto-streams logcat (heap-buffer-overflow / use-after-free
# reports + the Cedinia/crash tags); press Ctrl-C to stop.
#
# Pass `smoke` to enable a deliberate startup heap-buffer-overflow that ASan must
# report - use it once to confirm ASan is really active:  just android_asan smoke
[group('android')]
android_asan smoke="":
    #!/usr/bin/env bash
    set -euo pipefail
    : "${ANDROID_NDK_HOME:?Set ANDROID_NDK_HOME to your NDK (e.g. \$ANDROID_HOME/ndk/27.2.12479018)}"
    ROOT="$(pwd)"
    TRIPLE=aarch64-linux-android
    API=26
    ABI=arm64-v8a

    HOSTTAG=$(ls "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt" | head -n1)
    TOOLBIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$HOSTTAG/bin"

    GRADLE_APP="$ROOT/cedinia/android/app"
    JNI="$GRADLE_APP/src/main/jniLibs/$ABI"
    RES="$GRADLE_APP/src/main/resources/lib/$ABI"
    # Keep release builds clean: never leave the ASan runtime / wrap.sh staged.
    cleanup() { rm -rf "$GRADLE_APP/src/main/jniLibs" "$GRADLE_APP/src/main/resources/lib"; }
    trap cleanup EXIT
    cleanup

    # 1. Instrumented .so (build.rs still embeds the Java DEX for the android target).
    export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$TOOLBIN/${TRIPLE}${API}-clang"
    export CC_aarch64_linux_android="$TOOLBIN/${TRIPLE}${API}-clang"
    export CXX_aarch64_linux_android="$TOOLBIN/${TRIPLE}${API}-clang++"
    export AR_aarch64_linux_android="$TOOLBIN/llvm-ar"
    export RUSTFLAGS="-Zsanitizer=address -Cforce-frame-pointers=yes -Cdebuginfo=2"
    cargo +nightly build -p cedinia --lib --target "$TRIPLE"

    # 2. Stage native libs + ASan runtime + wrap.sh for Gradle.
    mkdir -p "$JNI" "$RES"
    cp "$ROOT/target/$TRIPLE/debug/libcedinia.so" "$JNI/"
    ASAN_RT=$(find "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$HOSTTAG/lib/clang" \
        -name "libclang_rt.asan-aarch64-android.so" | head -n1)
    [ -n "$ASAN_RT" ] || { echo "ASan runtime not found in NDK"; exit 1; }
    cp "$ASAN_RT" "$JNI/"
    cp "$ROOT/cedinia/android/asan_wrap.sh" "$RES/wrap.sh"
    if [ "{{smoke}}" = "smoke" ]; then
        # Inject the smoke-test env so android_main triggers a heap-buffer-overflow.
        sed -i 's|^exec "\$@"|export CEDINIA_ASAN_SMOKETEST=1\nexec "$@"|' "$RES/wrap.sh"
        echo "Smoke test ENABLED: the app will deliberately heap-overflow at startup."
    fi
    chmod +x "$RES/wrap.sh"

    # 3. Debuggable APK -> install -> launch -> auto-capture logcat.
    cd "$ROOT/cedinia/android"
    gradle assembleDebug
    {{adb}} install -r app/build/outputs/apk/debug/app-debug.apk
    {{adb}} logcat -c || true
    {{adb}} shell am start -n {{apk_package}}/{{apk_activity}}
    echo "ASan build launched - streaming logcat (Ctrl-C to stop)."
    echo "Look for lines starting with '==... ERROR: AddressSanitizer:'."
    # Symbolize backtraces to function + file:line via the unstripped local .so.
    # The on-device ASan symbolizer only resolves names, not source lines; ndk-stack
    # reads DWARF from target/.../debug/libcedinia.so (same code offsets as the APK).
    # Raw stream is also teed to a file so nothing is lost if ndk-stack filters it.
    # Not exec: keep the EXIT trap so the gradle staging is cleaned after Ctrl-C.
    SYMDIR="$ROOT/target/$TRIPLE/debug"
    NDK_STACK="$ANDROID_NDK_HOME/ndk-stack"
    if [ -x "$NDK_STACK" ]; then
        {{adb}} logcat -v threadtime | tee "$ROOT/target/cedinia-asan-logcat.txt" | "$NDK_STACK" -sym "$SYMDIR" || true
    else
        echo "ndk-stack not found at $NDK_STACK - raw logcat (offsets only); symbolize later with: just android_symbolize <dump>"
        {{adb}} logcat -v threadtime | tee "$ROOT/target/cedinia-asan-logcat.txt" || true
    fi

# Symbolize a saved logcat / tombstone dump: offsets -> function + file:line, using
# the unstripped local libcedinia.so (build it first with `just android_asan`).
# Usage: just android_symbolize target/cedinia-asan-logcat.txt
[group('android')]
android_symbolize dump:
    "$ANDROID_NDK_HOME/ndk-stack" -sym target/aarch64-linux-android/debug -dump {{dump}}

# Build a signed release AAB suitable for Google Play Store upload.
# Requires gradle 8.9+ in PATH (e.g. sdk install gradle 8.9 via sdkman).
# The libcedinia.so is compiled by cargo-apk and the DEX is already embedded
# in the .so via include_bytes! – no separate Java compilation is needed.
[group('android')]
android_build_aab:
    #!/usr/bin/env bash
    set -euo pipefail
    ROOT="$(pwd)"
    PASS=$(cat keystore_pass)
    CARGO_TOML="$ROOT/cedinia/Cargo.toml"
    sed -i "s|TO_REPLACE_KEYSTORE_PASSWORD|$PASS|g" "$CARGO_TOML"
    trap 'sed -i "s|$PASS|TO_REPLACE_KEYSTORE_PASSWORD|g" "$CARGO_TOML"' EXIT
    export KEYSTORE_PASSWORD="$PASS"
    export KEY_PASSWORD="$PASS"

    rm -rf "$ROOT/cedinia/android/app/src/main/jniLibs"
    rm -rf "$ROOT/cedinia/android/app/src/main/resources/lib"
    rm -f "$ROOT/cedinia.aab"
    rm -rf "$ROOT/cedinia/android/.gradle"
    rm -rf "$ROOT/cedinia/android/app/build"
    rm -rf "$ROOT/cedinia/android/build"

    cargo apk build -p cedinia --lib --release
    mkdir -p "$ROOT/cedinia/android/app/src/main/jniLibs/arm64-v8a"
    cp "$ROOT/target/aarch64-linux-android/release/libcedinia.so" "$ROOT/cedinia/android/app/src/main/jniLibs/arm64-v8a/"
    cd "$ROOT/cedinia/android" && gradle bundleRelease
    cp "$ROOT/cedinia/android/app/build/outputs/bundle/release/app-release.aab" "$ROOT/cedinia.aab"
    echo "AAB built: $ROOT/cedinia.aab"

##################### BENCHMARKS #####################

[group('benchmark')]
prepare_binaries:
    mkdir -p benchmarks
    wget https://github.com/qarmin/czkawka/releases/download/Nightly/linux_czkawka_cli -O benchmarks/czkawka_cli_normal
    cd czkawka_cli; cargo build --release; cd ..; cp target/release/czkawka_cli benchmarks/czkawka_cli_v4
    cd czkawka_cli; cargo build --profile fastest; cd ..; cp target/fastest/czkawka_cli benchmarks/czkawka_cli_fastest

[group('benchmark')]
benchmark media:
    # benchmarks/czkawka_cli_old dup -d /media/rafal/Kotyk
    # benchmarks/czkawka_cli_fastest dup -d /media/rafal/Kotyk -W -N -M -H
    sudo echo "AA" # Ability to run as root is needed later by hyperfine
    #hyperfine --prepare "sudo sh -c 'sync; echo 3 > /proc/sys/vm/drop_caches'; rm cache_duplicates_Blake3_70.bin || true" 'benchmarks/czkawka_cli_fastest dup -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_v4 dup -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_normal dup -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_old image -d "{{ media }}" > /dev/null'
    hyperfine --prepare "sudo sh -c 'sync; echo 3 > /proc/sys/vm/drop_caches'; rm /home/rafal/.cache/czkawka/cache_similar_images_16_Gradient_Nearest_80.bin || true" 'benchmarks/czkawka_cli_fastest image -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_v4 image -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_normal image -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_old image -d "{{ media }}" > /dev/null'


[group('benchmark')]
check_compilations:
    git checkout Cargo.toml
    #cargo install --path misc/test_compilation_speed_size
    test_compilation_speed_size misc/test_compilation_speed_size/krokiet.json
    python3 misc/test_compilation_speed_size/generate_md_and_plots.py

[group('misc')]
tags:
    tags=($(git tag --sort=version:refname | grep -v Nightly)); for ((i=0; i<${#tags[@]}-1; i++)); do from=${tags[$i]}; to=${tags[$i+1]}; echo "$from -> $to : $(git diff --shortstat "$from" "$to")"; done; last=${tags[-1]}; echo "$last -> master : $(git diff --shortstat "$last" master)"

[group('install')]
install:
    cargo install --path czkawka_cli --locked
    cargo install --path krokiet --locked

##################### TRANSLATIONS #####################

[group('translations')]
prepare_translations_deps:
    @command -v uv >/dev/null 2>&1 || curl -LsSf https://astral.sh/uv/install.sh | sh
    @command -v ollama >/dev/null 2>&1 || curl -fsSL https://ollama.com/install.sh | sh
    uv sync
    # qwen2.5:7b - fast, but quite bad quality
    # qwen2.5:32b - very slow, still not good quality
    # zongwei/gemma3-translator:4b - not so fast, but looks quite good
    # translategemma:12b - probably very slow - using only for small tasks
    export OLLAMA_VULKAN=1; export HSA_OVERRIDE_GFX_VERSION=10.3.0; ollama pull translategemma:12b

[group('translations')]
translate:
    uv run misc/ai_translate/translate.py czkawka_core/i18n
    uv run misc/ai_translate/translate.py krokiet/i18n
    uv run misc/ai_translate/translate.py cedinia/i18n

[group('translations')]
validate_translations *args: # Available --fix argument, which removes invalid translations
    uv run misc/ai_translate/validate_translations.py czkawka_core/i18n {{args}}
    uv run misc/ai_translate/validate_translations.py krokiet/i18n {{args}}
    uv run misc/ai_translate/validate_translations.py cedinia/i18n {{args}}

# Crowdin allows to import zip file with structured translations
[group('translations')]
pack_translations:
    rm -f i18n_translations.zip
    mkdir -p /tmp/czkawka_i18n
    for lang in czkawka_gui/i18n/*/; do \
        lang_code=$(basename "$lang"); \
        [ "$lang_code" = "en" ] && continue; \
        mkdir -p "/tmp/czkawka_i18n/i18n/$lang_code"; \
        [ -f "czkawka_gui/i18n/$lang_code/czkawka_gui.ftl" ] && cp "czkawka_gui/i18n/$lang_code/czkawka_gui.ftl" "/tmp/czkawka_i18n/i18n/$lang_code/" || true; \
        [ -f "czkawka_core/i18n/$lang_code/czkawka_core.ftl" ] && cp "czkawka_core/i18n/$lang_code/czkawka_core.ftl" "/tmp/czkawka_i18n/i18n/$lang_code/" || true; \
        [ -f "krokiet/i18n/$lang_code/krokiet.ftl" ] && cp "krokiet/i18n/$lang_code/krokiet.ftl" "/tmp/czkawka_i18n/i18n/$lang_code/" || true; \
        [ -f "cedinia/i18n/$lang_code/cedinia.ftl" ] && cp "cedinia/i18n/$lang_code/cedinia.ftl" "/tmp/czkawka_i18n/i18n/$lang_code/" || true; \
    done
    cd /tmp/czkawka_i18n && zip -r - i18n > "{{justfile_directory()}}/i18n_translations.zip"
    rm -rf /tmp/czkawka_i18n
    @echo "Created i18n_translations.zip with all translations (excluding English)"

[group('translations')]
unpack_translations path_to_file:
    @echo "Unpacking translations from {{path_to_file}}..."
    mkdir -p /tmp/czkawka_unpack
    unzip -q "{{path_to_file}}" -d /tmp/czkawka_unpack
    for lang_dir in /tmp/czkawka_unpack/*/; do \
        lang_code=$(basename "$lang_dir"); \
        [ -f "$lang_dir/czkawka_gui.ftl" ] && mkdir -p "czkawka_gui/i18n/$lang_code" && cp "$lang_dir/czkawka_gui.ftl" "czkawka_gui/i18n/$lang_code/" && echo "Copied czkawka_gui.ftl to czkawka_gui/i18n/$lang_code/" || true; \
        [ -f "$lang_dir/czkawka_core.ftl" ] && mkdir -p "czkawka_core/i18n/$lang_code" && cp "$lang_dir/czkawka_core.ftl" "czkawka_core/i18n/$lang_code/" && echo "Copied czkawka_core.ftl to czkawka_core/i18n/$lang_code/" || true; \
        [ -f "$lang_dir/krokiet.ftl" ] && mkdir -p "krokiet/i18n/$lang_code" && cp "$lang_dir/krokiet.ftl" "krokiet/i18n/$lang_code/" && echo "Copied krokiet.ftl to krokiet/i18n/$lang_code/" || true; \
        [ -f "$lang_dir/cedinia.ftl" ] && mkdir -p "cedinia/i18n/$lang_code" && cp "$lang_dir/cedinia.ftl" "cedinia/i18n/$lang_code/" && echo "Copied cedinia.ftl to cedinia/i18n/$lang_code/" || true; \
    done
    rm -rf /tmp/czkawka_unpack
    @echo "Translations unpacked successfully"

[group('misc')]
cache:
    xdg-open ~/.cache/czkawka

[group('misc')]
configc:
    xdg-open ~/.config/czkawka

[group('misc')]
configk:
    xdg-open ~/.config/krokiet


##################### DEBUG SIZE, PERFORMANCE AND OTHERS #####################

[group('setup')]
setup_verify_tools:
    rustup component add llvm-tools-preview
    cargo install cargo-llvm-lines cargo-bloat cargo-deps flamegraph
    cargo install --git https://github.com/rust-lang/measureme crox flamegraph summarize

# Prints lines of certain functions in binary
[group('profiling')]
llvm_lines:
    cargo llvm-lines -p krokiet --bin krokiet | head -40
    cargo llvm-lines -p czkawka_cli --bin czkawka_cli | head -40

# Prints size of functions in binary
[group('profiling')]
bloat_by_function:
    cargo bloat --release --bin czkawka_cli -n 30
    cargo bloat --release --bin krokiet -n 30

# Prints size of crates in binary
[group('profiling')]
bloat_by_crate:
    cargo bloat --release --crates --bin czkawka_cli
    cargo bloat --release --crates --bin krokiet

# Draws dependency graphs of certain binaries(like regex, image, etc)
[group('profiling')]
dependencies_graph:
    cd czkawka_core;cargo deps --all-deps | dot -Tpng > deps.png;cd ..
    cd czkawka_cli;cargo deps --all-deps | dot -Tpng > deps.png;cd ..
    cd krokiet;cargo deps --all-deps | dot -Tpng > deps.png;cd ..

# Shows llvm compilation data summary
[group('profiling')]
profiling profile='debug' mode='build':
    if [ "{{profile}}" = "release" ]; then release_flag="--release"; else release_flag=""; fi; \
    cargo clean; \
    for crate in czkawka_core czkawka_cli krokiet; do \
        cd "$crate"; \
        rm ../*.mm_profdata || true; \
        rm *.mm_profdata || true; \
        RUSTFLAGS="-Zself-profile" cargo +nightly rustc $release_flag; \
        summarize summarize ../*.mm_profdata || true; \
        cd ..; \
    done

# Timings of crates compilation
[group('profiling')]
timings profile='debug' mode='build':
    if [ "{{profile}}" = "release" ]; then release_flag="--release"; else release_flag=""; fi; \
    cargo clean; \
    for crate in czkawka_core czkawka_cli krokiet; do \
        cd "$crate"; \
        rm ../target/cargo-timings/*.html || true; \
        cargo "{{mode}}" $release_flag --timings; \
        cp "$(find ../target/cargo-timings -maxdepth 1 -name '*.html' -print -quit)" "../$crate.html" || true; \
        cd ..; \
    done
    #cargo clean
#    cd czkawka_core; RUSTFLAGS="-Ztime" cargo +nightly rustc; cd ..;
#    cargo clean
#    cd czkawka_cli; RUSTFLAGS="-Ztime" cargo +nightly rustc; cd ..;
#    cargo clean
#    cd krokiet; RUSTFLAGS="-Ztime" cargo +nightly rustc; cd ..;

# Per crate compilation times and ram usage
# This is very verbose, so probably not really useful
[group('profiling')]
time_passes:
    cargo clean
    cd czkawka_core; RUSTFLAGS="-Ztime-passes" cargo +nightly rustc; cd ..;
    cargo clean
    cd czkawka_cli; RUSTFLAGS="-Ztime-passes" cargo +nightly rustc; cd ..;
    cargo clean
    cd krokiet; RUSTFLAGS="-Ztime-passes" cargo +nightly rustc; cd ..;

[group('profiling')]
test_heaptrack:
    cd test && cargo build --profile rdebug
    heaptrack test/target/rdebug/TEST
    heaptrack_gui --appimage-extract-and-run "$(ls -t *.zst | head -n1)"

[group('profiling')]
heaptrack bin:
    cargo build --profile rdebug --bin {{bin}}
    heaptrack target/rdebug/{{bin}}
    heaptrack_gui --appimage-extract-and-run "$(ls -t *.zst | head -n1)"

[group('misc')]
diffs VERSION:
	mkdir -p DIFFS
	git diff --name-only {{VERSION}} HEAD | while read file; do \
		mkdir -p "DIFFS/$(dirname "$file")"; \
		git diff {{VERSION}} HEAD -- "$file" > "DIFFS/$file"; \
	done
