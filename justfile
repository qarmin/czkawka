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

fix:
    ruff format --line-length 120 --no-cache
    mypy . --strict
    python3 misc/delete_unused_krokiet_slint_imports.py
    python3 misc/find_unused_fluent_translations.py czkawka_gui
    python3 misc/find_unused_fluent_translations.py krokiet
    python3 misc/find_unused_fluent_translations.py czkawka_core
    python3 misc/find_unused_slint_translations.py krokiet

    cargo +nightly fmt
    cargo clippy --fix --allow-dirty --allow-staged --all-features --all-targets
    cargo +nightly fmt
    cargo fmt

fix_nightly:
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

setup_verify:
    cargo install cargo-llvm-lines
    cargo install cargo-bloat

debug_verify:
    cargo llvm-lines -p krokiet --bin krokiet | head -40
    cargo llvm-lines -p czkawka_gui --bin czkawka_gui | head -40
    cargo llvm-lines -p czkawka_cli --bin czkawka_cli | head -40

    cargo bloat --release --bin czkawka_cli -n 30
    cargo bloat --release --bin czkawka_gui -n 30
    cargo bloat --release --bin krokiet -n 30

bloat:
    cargo bloat --release --crates --bin czkawka_cli
    cargo bloat --release --crates --bin czkawka_gui
    cargo bloat --release --crates --bin krokiet

check_compilations:
    git checkout Cargo.toml
    #cargo install --path misc/test_compilation_speed_size
    test_compilation_speed_size misc/test_compilation_speed_size/krokiet.json
    python3 misc/test_compilation_speed_size/generate_md_and_plots.py

tags:
    tags=($(git tag --sort=version:refname | grep -v Nightly)); for ((i=0; i<${#tags[@]}-1; i++)); do from=${tags[$i]}; to=${tags[$i+1]}; echo "$from -> $to : $(git diff --shortstat "$from" "$to")"; done; last=${tags[-1]}; echo "$last -> master : $(git diff --shortstat "$last" master)"
