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
    ruff format --line-length 120
    mypy . --strict
    python3 misc/delete_unused_krokiet_slint_imports.py
    python3 misc/find_unused_fluent_translations.py czkawka_gui
    python3 misc/find_unused_fluent_translations.py krokiet
    python3 misc/find_unused_fluent_translations.py czkawka_core
    python3 misc/find_unused_slint_translations.py krokiet

    cargo +nightly fmt
    cargo clippy --fix --allow-dirty --allow-staged -- -Wclippy::bool_to_int_with_if -Wclippy::expl_impl_clone_on_copy -Wclippy::explicit_into_iter_loop -Wclippy::explicit_iter_loop -Wclippy::filter_map_next -Wclippy::flat_map_option -Wclippy::float_cmp -Wclippy::from_iter_instead_of_collect -Wclippy::ignored_unit_patterns -Wclippy::implicit_clone -Wclippy::index_refutable_slice -Wclippy::invalid_upcast_comparisons -Wclippy::iter_filter_is_ok -Wclippy::iter_filter_is_some -Wclippy::large_stack_arrays -Wclippy::large_types_passed_by_value -Wclippy::macro_use_imports -Wclippy::manual_assert -Wclippy::manual_instant_elapsed -Wclippy::manual_is_power_of_two -Wclippy::manual_is_variant_and -Wclippy::manual_let_else -Wclippy::manual_ok_or -Wclippy::map_unwrap_or -Wclippy::match_bool -Wclippy::match_same_arms -Wclippy::match_wildcard_for_single_variants -Wclippy::mut_mut -Wclippy::needless_bitwise_bool -Wclippy::needless_continue -Wclippy::needless_for_each -Wclippy::needless_pass_by_value -Wclippy::option_as_ref_cloned -Wclippy::range_minus_one -Wclippy::range_plus_one -Wclippy::redundant_else -Wclippy::ref_binding_to_reference -Wclippy::ref_option -Wclippy::ref_option_ref -Wclippy::same_functions_in_if_condition -Wclippy::semicolon_if_nothing_returned -Wclippy::stable_sort_primitive -Wclippy::str_split_at_newline -Wclippy::string_add_assign -Wclippy::uninlined_format_args -Wclippy::unnecessary_box_returns -Wclippy::unnecessary_join -Wclippy::unnecessary_wraps -Wclippy::unnested_or_patterns -Wclippy::used_underscore_binding -Wclippy::used_underscore_items -Aclippy::match_same_arms      -Wclippy::branches_sharing_code -Wclippy::collection_is_never_read -Wclippy::debug_assert_with_mut_call -Wclippy::equatable_if_let -Wclippy::fallible_impl_from -Wclippy::iter_on_empty_collections -Wclippy::iter_on_single_items -Wclippy::needless_collect -Wclippy::needless_pass_by_ref_mut -Wclippy::nonstandard_macro_braces -Wclippy::path_buf_push_overwrite -Wclippy::redundant_clone -Wclippy::set_contains_or_insert -Wclippy::suspicious_operation_groupings -Wclippy::trait_duplication_in_bounds -Wclippy::trivial_regex -Wclippy::type_repetition_in_bounds -Wclippy::unused_rounding -Wclippy::use_self -Wclippy::useless_let_if_seq -Wclippy::while_float
    cargo +nightly fmt
    # -Wclippy::missing_panics_doc
    # -Wclippy::option_if_let_else
    # -Wclippy::tuple_array_conversions

fix_nightly:
    cargo +nightly fmt
    cargo +nightly clippy --fix --allow-dirty --allow-staged -- -Wclippy::bool_to_int_with_if -Wclippy::expl_impl_clone_on_copy -Wclippy::explicit_into_iter_loop -Wclippy::explicit_iter_loop -Wclippy::filter_map_next -Wclippy::flat_map_option -Wclippy::float_cmp -Wclippy::from_iter_instead_of_collect -Wclippy::ignored_unit_patterns -Wclippy::implicit_clone -Wclippy::index_refutable_slice -Wclippy::invalid_upcast_comparisons -Wclippy::iter_filter_is_ok -Wclippy::iter_filter_is_some -Wclippy::large_stack_arrays -Wclippy::large_types_passed_by_value -Wclippy::macro_use_imports -Wclippy::manual_assert -Wclippy::manual_instant_elapsed -Wclippy::manual_is_power_of_two -Wclippy::manual_is_variant_and -Wclippy::manual_let_else -Wclippy::manual_ok_or -Wclippy::map_unwrap_or -Wclippy::match_bool -Wclippy::match_same_arms -Wclippy::match_wildcard_for_single_variants -Wclippy::mut_mut -Wclippy::needless_bitwise_bool -Wclippy::needless_continue -Wclippy::needless_for_each -Wclippy::needless_pass_by_value -Wclippy::option_as_ref_cloned -Wclippy::range_minus_one -Wclippy::range_plus_one -Wclippy::redundant_else -Wclippy::ref_binding_to_reference -Wclippy::ref_option -Wclippy::ref_option_ref -Wclippy::same_functions_in_if_condition -Wclippy::semicolon_if_nothing_returned -Wclippy::stable_sort_primitive -Wclippy::str_split_at_newline -Wclippy::string_add_assign -Wclippy::uninlined_format_args -Wclippy::unnecessary_box_returns -Wclippy::unnecessary_join -Wclippy::unnecessary_wraps -Wclippy::unnested_or_patterns -Wclippy::used_underscore_binding -Wclippy::used_underscore_items -Aclippy::match_same_arms -Wclippy::branches_sharing_code -Wclippy::collection_is_never_read -Wclippy::debug_assert_with_mut_call -Wclippy::equatable_if_let -Wclippy::fallible_impl_from -Wclippy::iter_on_empty_collections -Wclippy::iter_on_single_items -Wclippy::needless_collect -Wclippy::needless_pass_by_ref_mut -Wclippy::nonstandard_macro_braces -Wclippy::path_buf_push_overwrite -Wclippy::redundant_clone -Wclippy::set_contains_or_insert -Wclippy::suspicious_operation_groupings -Wclippy::trait_duplication_in_bounds -Wclippy::trivial_regex -Wclippy::type_repetition_in_bounds -Wclippy::unused_rounding -Wclippy::use_self -Wclippy::useless_let_if_seq -Wclippy::while_float
    cargo +nightly fmt
    # -Wclippy::missing_panics_doc
    # -Wclippy::option_if_let_else
    # -Wclippy::tuple_array_conversions

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
