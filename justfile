set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

build_all: && fix
    cargo build --release
    cargo build
    cargo clippy
    cargo test

itests:
    [ ! -f TestFiles.zip ] && wget https://github.com/qarmin/czkawka/releases/download/6.0.0/TestFiles.zip || true
    cd ci_tester && cargo build --release && cd ..
    cargo build --release --bin czkawka_cli

    RUST_BACKTRACE=1 ci_tester/target/release/ci_tester target/release/czkawka_cli

## run

run +args:
    cargo run --bin {{args}}

runr +args:
    cargo run --profile fast_release --bin {{args}}

runc +args:
    CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly run -Zcodegen-backend --bin {{args}}

runs +args:
    export RUST_BACKTRACE=1 # or full depending on project
    export ASAN_SYMBOLIZER_PATH=$(which llvm-symbolizer) # Version depends on your system
    export ASAN_OPTIONS="symbolize=1:detect_leaks=0" # Leak check is disabled, because is slow and ususally not needed
    ASAN_OPTIONS="symbolize=1:detect_leaks=0" RUSTFLAGS="-Zsanitizer=address" cargo +nightly run --target x86_64-unknown-linux-gnu --bin {{args}}

val bin:
    valgrind --leak-check=full --show-leak-kinds=definite --track-origins=yes target/debug/{{bin}}

valr bin:
    valgrind --leak-check=full --show-leak-kinds=definite --track-origins=yes target/release/{{bin}}


#  echo '-1' | sudo tee /proc/sys/kernel/perf_event_paranoid
samply bin:
    cargo build --bin {{bin}}
    samply record target/debug/{{bin}}

samplyrd bin:
    cargo build --bin {{bin}} --profile rdebug
    samply record target/rdebug/{{bin}}

## Other

setup_sanitizer:
    rustup install nightly
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
    rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu


bench:
    cd czkawka_core && cargo bench
    xdg-open target/criterion/report/index.html

bench_clean:
    rm -rf target/criterion

upgrade:
    cargo +nightly -Z unstable-options update --breaking
    cargo update

clip:
    cargo clippy --fix --allow-dirty --allow-staged --all-features --all-targets
    cargo clippy --fix --allow-dirty --allow-staged --no-default-features --features winit_software --all-targets

fix:
    ruff format --line-length 120 --no-cache
    mypy misc --strict
    python3 misc/delete_unused_krokiet_slint_imports.py
    python3 misc/find_unused_fluent_translations.py czkawka_gui
    python3 misc/find_unused_fluent_translations.py krokiet
    python3 misc/find_unused_fluent_translations.py czkawka_core
    python3 misc/find_unused_slint_translations.py krokiet
    python3 misc/find_unused_callbacks.py krokiet
    # python3 misc/find_unused_settings_properties.py

    cargo +nightly fmt
    cargo clippy --fix --allow-dirty --allow-staged --all-features --all-targets
    cargo +nightly fmt
    cargo fmt

fixn:
    cargo +nightly fmt
    cargo +nightly clippy --fix --allow-dirty --allow-staged --all-features --all-targets
    cargo +nightly fmt

test_resize arg:
    cd misc/test_image_perf; cargo build --release; sudo ./target/release/test_image_perf "{{arg}}"

# Not works, due of edition 2024 and workspaces
unused_features:
    unused-features analyze
    unused-features build-report --input krokiet/report.json
    unused-features build-report --input czkawka_cli/report.json
    unused-features build-report --input czkawka_core/report.json
    unused-features build-report --input czkawka_gui/report.json
    xdg-open krokiet/report.html
    xdg-open czkawka_cli/report.html
    xdg-open czkawka_core/report.html
    xdg-open czkawka_gui/report.html

prepare_binaries:
    mkdir -p benchmarks
    wget https://github.com/qarmin/czkawka/releases/download/Nightly/linux_czkawka_cli -O benchmarks/czkawka_cli_normal
    cd czkawka_cli; cargo build --release; cd ..; cp target/release/czkawka_cli benchmarks/czkawka_cli_v4
    cd czkawka_cli; cargo build --profile fastest; cd ..; cp target/fastest/czkawka_cli benchmarks/czkawka_cli_fastest

benchmark media:
    # benchmarks/czkawka_cli_old dup -d /media/rafal/Kotyk
    # benchmarks/czkawka_cli_fastest dup -d /media/rafal/Kotyk -W -N -M -H
    sudo echo "AA" # Ability to run as root is needed later by hyperfine
    #hyperfine --prepare "sudo sh -c 'sync; echo 3 > /proc/sys/vm/drop_caches'; rm cache_duplicates_Blake3_70.bin || true" 'benchmarks/czkawka_cli_fastest dup -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_v4 dup -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_normal dup -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_old image -d "{{ media }}" > /dev/null'
    hyperfine --prepare "sudo sh -c 'sync; echo 3 > /proc/sys/vm/drop_caches'; rm /home/rafal/.cache/czkawka/cache_similar_images_16_Gradient_Nearest_80.bin || true" 'benchmarks/czkawka_cli_fastest image -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_v4 image -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_normal image -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_old image -d "{{ media }}" > /dev/null'


check_compilations:
    git checkout Cargo.toml
    #cargo install --path misc/test_compilation_speed_size
    test_compilation_speed_size misc/test_compilation_speed_size/krokiet.json
    python3 misc/test_compilation_speed_size/generate_md_and_plots.py

tags:
    tags=($(git tag --sort=version:refname | grep -v Nightly)); for ((i=0; i<${#tags[@]}-1; i++)); do from=${tags[$i]}; to=${tags[$i+1]}; echo "$from -> $to : $(git diff --shortstat "$from" "$to")"; done; last=${tags[-1]}; echo "$last -> master : $(git diff --shortstat "$last" master)"

install:
    cargo install --path czkawka_cli --locked
    cargo install --path krokiet --locked
    cargo install --path czkawka_gui --locked


prepare_translations_deps:
    @command -v uv >/dev/null 2>&1 || curl -LsSf https://astral.sh/uv/install.sh | sh
    @command -v ollama >/dev/null 2>&1 || curl -fsSL https://ollama.com/install.sh | sh
    cd misc/ai_translate; uv sync
    # qwen2.5:7b - fast, but quite bad quality
    # qwen2.5:32b - very slow, still not good quality
    # zongwei/gemma3-translator:4b - not so fast, but looks quite good
    # translategemma:12b - probably very slow - using only for small tasks
    export OLLAMA_VULKAN=1; export HSA_OVERRIDE_GFX_VERSION=10.3.0; ollama pull translategemma:12b

translate:
    cd misc/ai_translate; uv run translate.py ../../czkawka_gui/i18n
    cd misc/ai_translate; uv run translate.py ../../czkawka_core/i18n
    cd misc/ai_translate; uv run translate.py ../../krokiet/i18n
    just pack_translations

validate_translations *args: # Available --fix argument, which removes invalid translations
    cd misc/ai_translate; uv run validate_translations.py ../../czkawka_gui/i18n {{args}}
    cd misc/ai_translate; uv run validate_translations.py ../../czkawka_core/i18n {{args}}
    cd misc/ai_translate; uv run validate_translations.py ../../krokiet/i18n {{args}}

# Crowdin allows to import zip file with structured translations
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
    done
    cd /tmp/czkawka_i18n && zip -r - i18n > "{{justfile_directory()}}/i18n_translations.zip"
    rm -rf /tmp/czkawka_i18n
    @echo "Created i18n_translations.zip with all translations (excluding English)"

unpack_translations path_to_file:
    @echo "Unpacking translations from {{path_to_file}}..."
    mkdir -p /tmp/czkawka_unpack
    unzip -q "{{path_to_file}}" -d /tmp/czkawka_unpack
    for lang_dir in /tmp/czkawka_unpack/*/; do \
        lang_code=$(basename "$lang_dir"); \
        [ -f "$lang_dir/czkawka_gui.ftl" ] && mkdir -p "czkawka_gui/i18n/$lang_code" && cp "$lang_dir/czkawka_gui.ftl" "czkawka_gui/i18n/$lang_code/" && echo "Copied czkawka_gui.ftl to czkawka_gui/i18n/$lang_code/" || true; \
        [ -f "$lang_dir/czkawka_core.ftl" ] && mkdir -p "czkawka_core/i18n/$lang_code" && cp "$lang_dir/czkawka_core.ftl" "czkawka_core/i18n/$lang_code/" && echo "Copied czkawka_core.ftl to czkawka_core/i18n/$lang_code/" || true; \
        [ -f "$lang_dir/krokiet.ftl" ] && mkdir -p "krokiet/i18n/$lang_code" && cp "$lang_dir/krokiet.ftl" "krokiet/i18n/$lang_code/" && echo "Copied krokiet.ftl to krokiet/i18n/$lang_code/" || true; \
    done
    rm -rf /tmp/czkawka_unpack
    @echo "Translations unpacked successfully"

cache:
    xdg-open ~/.cache/czkawka

configc:
    xdg-open ~/.config/czkawka

configk:
    xdg-open ~/.config/krokiet


##################### DEBUG SIZE, PERFORMANCE AND OTHERS #####################
setup_verify_tools:
    rustup component add llvm-tools-preview
    cargo install cargo-llvm-lines cargo-bloat cargo-deps flamegraph measureme
    cargo install --git https://github.com/rust-lang/measureme crox flamegraph summarize

# Prints lines of certain functions in binary
llvm_lines:
    cargo llvm-lines -p krokiet --bin krokiet | head -40
    cargo llvm-lines -p czkawka_gui --bin czkawka_gui | head -40
    cargo llvm-lines -p czkawka_cli --bin czkawka_cli | head -40

# Prints size of functions in binary
bloat_by_function:
    cargo bloat --release --bin czkawka_cli -n 30
    cargo bloat --release --bin czkawka_gui -n 30
    cargo bloat --release --bin krokiet -n 30

# Prints size of crates in binary
bloat_by_crate:
    cargo bloat --release --crates --bin czkawka_cli
    cargo bloat --release --crates --bin czkawka_gui
    cargo bloat --release --crates --bin krokiet

# Draws dependency graphs of certain binaries(like regex, image, etc)
dependencies_graph:
    cd czkawka_core;cargo deps --all-deps | dot -Tpng > deps.png;cd ..
    cd czkawka_cli;cargo deps --all-deps | dot -Tpng > deps.png;cd ..
    cd czkawka_gui;cargo deps --all-deps | dot -Tpng > deps.png;cd ..
    cd krokiet;cargo deps --all-deps | dot -Tpng > deps.png;cd ..

# Shows llvm compilation data summary
profiling profile='debug' mode='build':
    if [ "{{profile}}" = "release" ]; then release_flag="--release"; else release_flag=""; fi; \
    cargo clean; \
    for crate in czkawka_core czkawka_gui czkawka_cli krokiet; do \
        cd "$crate"; \
        rm ../*.mm_profdata || true; \
        rm *.mm_profdata || true; \
        RUSTFLAGS="-Zself-profile" cargo +nightly rustc $release_flag; \
        summarize summarize ../*.mm_profdata || true; \
        cd ..; \
    done

# Timings of crates compilation
timings profile='debug' mode='build':
    if [ "{{profile}}" = "release" ]; then release_flag="--release"; else release_flag=""; fi; \
    cargo clean; \
    for crate in czkawka_core czkawka_gui czkawka_cli krokiet; do \
        cd "$crate"; \
        rm ../target/cargo-timings/*.html || true; \
        cargo "{{mode}}" $release_flag --timings; \
        cp "$(find ../target/cargo-timings -maxdepth 1 -name '*.html' -print -quit)" "../$crate.html" || true; \
        cd ..; \
    done
    #cargo clean
#    cd czkawka_core; RUSTFLAGS="-Ztime" cargo +nightly rustc; cd ..;
#    cargo clean
#    cd czkawka_gui; RUSTFLAGS="-Ztime" cargo +nightly rustc; cd ..;
#    cargo clean
#    cd czkawka_cli; RUSTFLAGS="-Ztime" cargo +nightly rustc; cd ..;
#    cargo clean
#    cd krokiet; RUSTFLAGS="-Ztime" cargo +nightly rustc; cd ..;

# Per crate compilation times and ram usage
# This is very verbose, so probably not really useful
time_passes:
    cargo clean
    cd czkawka_core; RUSTFLAGS="-Ztime-passes" cargo +nightly rustc; cd ..;
    cargo clean
    cd czkawka_gui; RUSTFLAGS="-Ztime-passes" cargo +nightly rustc; cd ..;
    cargo clean
    cd czkawka_cli; RUSTFLAGS="-Ztime-passes" cargo +nightly rustc; cd ..;
    cargo clean
    cd krokiet; RUSTFLAGS="-Ztime-passes" cargo +nightly rustc; cd ..;
