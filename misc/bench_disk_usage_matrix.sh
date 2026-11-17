#!/usr/bin/env bash
# Runs krokiet/benches/disk_usage_render.rs (time of the disk usage layout and of drawing the chart) in many build
# configurations and prints the results side by side. It is slow and wants a quiet machine: every configuration is built
# from scratch in a directory of its own (several GB each), and the measurements use all the threads.
#
#   bash misc/bench_disk_usage_matrix.sh [--dry-run] [--configs a,b,c] [--runs N] [--files N | --path DIR] [--size WxH] [--out DIR]
#
# Configurations: debug (profile dev, the way `just run` builds), release_debug (profile rdebug), release, lto_fat (release
# with lto = "fat"), v1 v2 v3 v4 (release with -C target-cpu=x86-64, x86-64-v2, ...; v1 is what release already is, so it
# also shows how much two runs of the same thing differ). Configurations the processor cannot run are skipped.
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ALL_CONFIGS=(debug release_debug release lto_fat v1 v2 v3 v4)
CONFIGS=("${ALL_CONFIGS[@]}")
OUT=$ROOT/tmp/bench_matrix
RUNS=7
FILES=1000000
BENCH_SOURCE_PATH=""
SIZE=1600x900
DRY_RUN=0

usage() { sed -n '2,12p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; }

while [ $# -gt 0 ]; do
    case $1 in
        --dry-run) DRY_RUN=1 ;;
        --configs) IFS=, read -r -a CONFIGS <<<"$2"; shift ;;
        --runs) RUNS=$2; shift ;;
        --files) FILES=$2; shift ;;
        --path) BENCH_SOURCE_PATH=$2; shift ;;
        --size) SIZE=$2; shift ;;
        --out) OUT=$2; shift ;;
        -h | --help) usage; exit 0 ;;
        *) echo "unknown argument: $1" >&2; usage >&2; exit 1 ;;
    esac
    shift
done

cpu_flags() { grep -m1 '^flags' /proc/cpuinfo 2>/dev/null || true; }

# Whether the processor has what the microarchitecture level needs, the lists are the ones of the x86-64 psABI
supports_level() {
    local flags
    flags=" $(cpu_flags) "
    local required
    case $1 in
        v2) required="cx16 lahf_lm popcnt sse4_1 sse4_2 ssse3" ;;
        v3) required="avx avx2 bmi1 bmi2 f16c fma abm movbe xsave" ;;
        v4) required="avx512f avx512bw avx512cd avx512dq avx512vl" ;;
        *) return 0 ;;
    esac
    local flag
    for flag in $required; do
        [[ $flags == *" $flag "* ]] || return 1
    done
}

# Sets PROFILE, TARGET_NAME, CARGO_ENV (extra environment of cargo) for a configuration
select_config() {
    PROFILE=release
    TARGET_NAME=shared
    CARGO_ENV=()
    case $1 in
        debug) PROFILE=dev ;;
        release_debug) PROFILE=rdebug ;;
        release) ;;
        lto_fat) TARGET_NAME=lto_fat; CARGO_ENV=(CARGO_PROFILE_RELEASE_LTO=fat) ;;
        v1) TARGET_NAME=v1; CARGO_ENV=(RUSTFLAGS="-C target-cpu=x86-64") ;;
        v2) TARGET_NAME=v2; CARGO_ENV=(RUSTFLAGS="-C target-cpu=x86-64-v2") ;;
        v3) TARGET_NAME=v3; CARGO_ENV=(RUSTFLAGS="-C target-cpu=x86-64-v3") ;;
        v4) TARGET_NAME=v4; CARGO_ENV=(RUSTFLAGS="-C target-cpu=x86-64-v4") ;;
        *) echo "unknown configuration: $1 (known: ${ALL_CONFIGS[*]})" >&2; exit 1 ;;
    esac
}

run_config() {
    local config=$1
    select_config "$config"
    local bench_env=(BENCH_RUNS="$RUNS" BENCH_WIDTH="${SIZE%x*}" BENCH_HEIGHT="${SIZE#*x}")
    if [ -n "$BENCH_SOURCE_PATH" ]; then
        bench_env+=(BENCH_PATH="$BENCH_SOURCE_PATH")
    else
        bench_env+=(BENCH_FILES="$FILES")
    fi
    local command=(env "${CARGO_ENV[@]}" "${bench_env[@]}" CARGO_TARGET_DIR="$OUT/target_$TARGET_NAME"
        cargo bench -p krokiet --bench disk_usage_render --profile "$PROFILE")
    echo "== $config: ${command[*]}"
    [ "$DRY_RUN" = 1 ] && return 0
    if "${command[@]}" >"$OUT/$config.txt" 2>"$OUT/$config.build.log"; then
        grep -E '^(INFO|RESULT) (run|build|treemap|sunburst)' "$OUT/$config.txt" | grep -E 'RESULT .* (layout|render|layout_and_render) ' || true
    else
        echo "   FAILED, see $OUT/$config.build.log" >&2
        rm -f "$OUT/$config.txt"
    fi
}

mkdir -p "$OUT"
cd "$ROOT"
echo "Machine: $(grep -m1 'model name' /proc/cpuinfo 2>/dev/null || echo unknown), $(nproc) threads"
echo "Governor: $(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null || echo unknown) (performance gives the steadiest numbers)"
echo "Results in $OUT"

RAN=()
for config in "${CONFIGS[@]}"; do
    case $config in
        v2 | v3 | v4)
            if [ "$DRY_RUN" = 0 ] && ! supports_level "$config"; then
                echo "== $config: skipped, this processor does not support x86-64-$config"
                continue
            fi
            ;;
    esac
    run_config "$config"
    RAN+=("$config")
done

if [ "$DRY_RUN" = 0 ]; then
    (IFS=,; python3 -I "$ROOT/misc/bench_disk_usage_table.py" "$OUT" --configs "${RAN[*]}") | tee "$OUT/table.md"
fi
