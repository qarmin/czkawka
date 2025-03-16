build_all:
    cargo build --release
    cargo build
    cargo clippy
    cargo test

itests:
    [ ! -f TestFiles.zip ] && wget https://github.com/qarmin/czkawka/releases/download/6.0.0/TestFiles.zip || true
    cd ci_tester;cargo build --release;cd ..
    cargo build --release --bin czkawka_cli

    ci_tester/target/release/ci_tester target/release/czkawka_cli

## run

czkawka:
    cargo run --bin czkawka_gui
czkawka_r:
    cargo run --bin czkawka_gui --release

krokiet:
    cargo run --bin krokiet
krokiet_r:
    cargo run --bin krokiet --release
krokiet_dark:
    SLINT_STYLE=fluent-dark cargo run --bin krokiet

cli +args:
    cargo run --bin czkawka_cli -- {{args}}
cli_r +args:
    cargo run --bin czkawka_cli --release -- {{args}}
cli_help:
    cargo run --bin czkawka_cli -- --help

## Other

bench:
    cd czkawka_core && cargo bench
    xdg-open target/criterion/report/index.html

bench_clean:
    rm -rf target/criterion

build:
    cargo build --bin czkawka_gui

check:
    cargo check --bin czkawka_gui

check_all:
    cargo check

checkc:
    CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly check -Zcodegen-backend

build_krokiet:
    cargo build --bin krokiet

build_czkawka:
    cargo build --bin czkawka_gui

upgrade:
    cargo +nightly -Z unstable-options update --breaking
    cargo update

fix:
    cargo +nightly fmt
    cargo clippy --fix --allow-dirty --allow-staged -- -Wclippy::bool_to_int_with_if -Wclippy::expl_impl_clone_on_copy -Wclippy::explicit_into_iter_loop -Wclippy::explicit_iter_loop -Wclippy::filter_map_next -Wclippy::flat_map_option -Wclippy::float_cmp -Wclippy::from_iter_instead_of_collect -Wclippy::ignored_unit_patterns -Wclippy::implicit_clone -Wclippy::index_refutable_slice -Wclippy::invalid_upcast_comparisons -Wclippy::iter_filter_is_ok -Wclippy::iter_filter_is_some -Wclippy::large_stack_arrays -Wclippy::large_types_passed_by_value -Wclippy::macro_use_imports -Wclippy::manual_assert -Wclippy::manual_instant_elapsed  -Wclippy::manual_is_power_of_two  -Wclippy::manual_is_variant_and  -Wclippy::manual_let_else  -Wclippy::manual_ok_or  -Wclippy::map_unwrap_or  -Wclippy::match_bool  -Wclippy::match_on_vec_items  -Wclippy::match_same_arms  -Wclippy::match_wildcard_for_single_variants   -Wclippy::mut_mut  -Wclippy::needless_bitwise_bool  -Wclippy::needless_continue  -Wclippy::needless_for_each  -Wclippy::needless_pass_by_value  -Wclippy::option_as_ref_cloned  -Wclippy::range_minus_one  -Wclippy::range_plus_one  -Wclippy::redundant_else  -Wclippy::ref_binding_to_reference  -Wclippy::ref_option  -Wclippy::ref_option_ref  -Wclippy::same_functions_in_if_condition  -Wclippy::semicolon_if_nothing_returned  -Wclippy::stable_sort_primitive  -Wclippy::str_split_at_newline  -Wclippy::string_add_assign  -Wclippy::uninlined_format_args  -Wclippy::unnecessary_box_returns  -Wclippy::unnecessary_join   -Wclippy::unnecessary_wraps  -Wclippy::unnested_or_patterns  -Wclippy::used_underscore_binding  -Wclippy::used_underscore_items   -Aclippy::match_same_arms                         -Wclippy::branches_sharing_code -Wclippy::collection_is_never_read -Wclippy::debug_assert_with_mut_call -Wclippy::equatable_if_let -Wclippy::fallible_impl_from -Wclippy::iter_on_empty_collections -Wclippy::iter_on_single_items -Wclippy::needless_collect -Wclippy::needless_pass_by_ref_mut -Wclippy::nonstandard_macro_braces -Wclippy::path_buf_push_overwrite -Wclippy::redundant_clone -Wclippy::set_contains_or_insert -Wclippy::suspicious_operation_groupings -Wclippy::trait_duplication_in_bounds -Wclippy::trivial_regex -Wclippy::type_repetition_in_bounds -Wclippy::unused_rounding -Wclippy::use_self -Wclippy::useless_let_if_seq -Wclippy::while_float
    cargo +nightly fmt
    #    -Wclippy::missing_panics_doc
    #    -Wclippy::option_if_let_else
    #    -Wclippy::tuple_array_conversions

fix_nightly:
    cargo +nightly fmt
    cargo +nightly clippy --fix --allow-dirty --allow-staged -- -Wclippy::bool_to_int_with_if -Wclippy::expl_impl_clone_on_copy -Wclippy::explicit_into_iter_loop -Wclippy::explicit_iter_loop -Wclippy::filter_map_next -Wclippy::flat_map_option -Wclippy::float_cmp -Wclippy::from_iter_instead_of_collect -Wclippy::ignored_unit_patterns -Wclippy::implicit_clone -Wclippy::index_refutable_slice -Wclippy::invalid_upcast_comparisons -Wclippy::iter_filter_is_ok -Wclippy::iter_filter_is_some -Wclippy::large_stack_arrays -Wclippy::large_types_passed_by_value -Wclippy::macro_use_imports -Wclippy::manual_assert -Wclippy::manual_instant_elapsed  -Wclippy::manual_is_power_of_two  -Wclippy::manual_is_variant_and  -Wclippy::manual_let_else  -Wclippy::manual_ok_or  -Wclippy::map_unwrap_or  -Wclippy::match_bool  -Wclippy::match_on_vec_items  -Wclippy::match_same_arms  -Wclippy::match_wildcard_for_single_variants   -Wclippy::mut_mut  -Wclippy::needless_bitwise_bool  -Wclippy::needless_continue  -Wclippy::needless_for_each  -Wclippy::needless_pass_by_value  -Wclippy::option_as_ref_cloned  -Wclippy::range_minus_one  -Wclippy::range_plus_one  -Wclippy::redundant_else  -Wclippy::ref_binding_to_reference  -Wclippy::ref_option  -Wclippy::ref_option_ref  -Wclippy::same_functions_in_if_condition  -Wclippy::semicolon_if_nothing_returned  -Wclippy::stable_sort_primitive  -Wclippy::str_split_at_newline  -Wclippy::string_add_assign  -Wclippy::uninlined_format_args  -Wclippy::unnecessary_box_returns  -Wclippy::unnecessary_join   -Wclippy::unnecessary_wraps  -Wclippy::unnested_or_patterns  -Wclippy::used_underscore_binding  -Wclippy::used_underscore_items   -Aclippy::match_same_arms                -Wclippy::branches_sharing_code -Wclippy::collection_is_never_read -Wclippy::debug_assert_with_mut_call -Wclippy::equatable_if_let -Wclippy::fallible_impl_from -Wclippy::iter_on_empty_collections -Wclippy::iter_on_single_items -Wclippy::needless_collect -Wclippy::needless_pass_by_ref_mut -Wclippy::nonstandard_macro_braces -Wclippy::path_buf_push_overwrite -Wclippy::redundant_clone -Wclippy::set_contains_or_insert -Wclippy::suspicious_operation_groupings -Wclippy::trait_duplication_in_bounds -Wclippy::trivial_regex -Wclippy::type_repetition_in_bounds -Wclippy::unused_rounding -Wclippy::use_self -Wclippy::useless_let_if_seq -Wclippy::while_float
    cargo +nightly fmt
    #    -Wclippy::missing_panics_doc
    #    -Wclippy::option_if_let_else
    #    -Wclippy::tuple_array_conversions

test_resize arg:
    cd misc/test_image_perf; cargo build --release; sudo ./target/release/test_image_perf "{{arg}}"
    cd misc/test_image_perf; cargo build --release --features fast_image_resize; sudo ./target/release/test_image_perf "{{arg}}"

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
    sudo echo "AA" # Needed later by hyperfine
    #hyperfine --prepare "sudo sh -c 'sync; echo 3 > /proc/sys/vm/drop_caches'; rm cache_duplicates_Blake3_70.bin || true" 'benchmarks/czkawka_cli_fastest dup -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_v4 dup -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_normal dup -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_old image -d "{{ media }}" > /dev/null'
    hyperfine --prepare "sudo sh -c 'sync; echo 3 > /proc/sys/vm/drop_caches'; rm /home/rafal/.cache/czkawka/cache_similar_images_16_Gradient_Nearest_80.bin || true" 'benchmarks/czkawka_cli_fastest image -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_v4 image -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_normal image -d "{{ media }}" -W -N -M -H' 'benchmarks/czkawka_cli_old image -d "{{ media }}" > /dev/null'

