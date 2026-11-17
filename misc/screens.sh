#!/usr/bin/env bash
# Screenshots of every screen of Krokiet in the dark and the light theme, saved to tmp/screens/<theme>/NN_name.png,
# so a change in the UI can be checked without opening the application. Needs xvfb, xdotool, imagemagick and python3.
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
BIN=${BIN:-$ROOT/target/debug/krokiet}
OUT=$ROOT/tmp/screens
WORK=$ROOT/tmp/screens_work
FIXTURE=$WORK/fixture
WIDTH=${SCREENS_WIDTH:-1100}
HEIGHT=${SCREENS_HEIGHT:-900}
DISPLAY_NUMBER=${SCREENS_DISPLAY:-95}
export DISPLAY=:$DISPLAY_NUMBER

for tool in Xvfb xdotool import python3 convert; do
    command -v "$tool" >/dev/null || { echo "missing tool: $tool" >&2; exit 1; }
done
[ -x "$BIN" ] || { echo "no binary at $BIN, build krokiet first" >&2; exit 1; }

# Names of the tools in the order of the list on the left; every row is 25 px high
TOOLS=(duplicate_files empty_folders big_files empty_files temporary_files similar_images similar_videos music_duplicates
    invalid_symlinks broken_files bad_extensions bad_names exif_remover video_optimizer disk_usage)
FIRST_TOOL_Y=108
TOOL_ROW_HEIGHT=25
# Positions of controls that are anchored to the bottom of the window
GEAR_Y=$((HEIGHT - 274))
SCAN_Y=$((HEIGHT - 222))

make_fixture() {
    [ -d "$FIXTURE" ] && return
    mkdir -p "$FIXTURE"/{music,video,photos,docs,src,downloads,archives,empty_folder/inner}
    for i in $(seq 1 25); do truncate -s $((i * 37000 + 5000)) "$FIXTURE/music/track_$i.mp3"; done
    for i in $(seq 1 6); do truncate -s $((i * 3000000)) "$FIXTURE/video/clip_$i.mkv"; done
    for i in $(seq 1 40); do truncate -s $((i * 21000 + 1000)) "$FIXTURE/photos/photo_$i.jpg"; done
    for i in $(seq 1 30); do truncate -s $((i * 4000 + 100)) "$FIXTURE/docs/document_$i.pdf"; done
    for i in $(seq 1 60); do printf 'fn main() {}\n// %s\n' "$i" > "$FIXTURE/src/file_$i.rs"; done
    for i in $(seq 1 4); do truncate -s $((i * 9000000)) "$FIXTURE/archives/backup_$i.zip"; done
    cp "$FIXTURE/docs/document_1.pdf" "$FIXTURE/docs/copy_of_document_1.pdf"
    : > "$FIXTURE/docs/empty.txt"
    : > "$FIXTURE/downloads/video.mp4.part"
    printf 'cache' > "$FIXTURE/downloads/data.tmp"
    printf '\x89PNG\r\n\x1a\nnot really' > "$FIXTURE/downloads/picture.txt"
    printf 'x' > "$FIXTURE/docs/Report .PDF"
    head -c 3000 /dev/urandom > "$FIXTURE/archives/broken.zip"
    ln -sf /nonexistent/target "$FIXTURE/downloads/broken_link"
    convert -size 128x128 gradient:red-blue "$FIXTURE/photos/similar_a.png"
    convert "$FIXTURE/photos/similar_a.png" -blur 0x1 "$FIXTURE/photos/similar_b.png"
}

start_display() {
    Xvfb "$DISPLAY" -screen 0 "${WIDTH}x${HEIGHT}x24" >/dev/null 2>&1 &
    XVFB_PID=$!
    sleep 1
}

stop_all() {
    [ -n "${APP_PID:-}" ] && kill "$APP_PID" 2>/dev/null || true
    [ -n "${XVFB_PID:-}" ] && kill "$XVFB_PID" 2>/dev/null || true
    wait 2>/dev/null || true
    APP_PID=""
    XVFB_PID=""
}
trap stop_all EXIT

start_app() {
    HOME=$1 SLINT_BACKEND=winit-software "$BIN" >"$WORK/krokiet.log" 2>&1 &
    APP_PID=$!
    for _ in $(seq 1 40); do
        WINDOW=$(xdotool search --onlyvisible --class krokiet 2>/dev/null | head -1 || true)
        [ -n "$WINDOW" ] && break
        sleep 0.5
    done
    [ -n "${WINDOW:-}" ] || { echo "Krokiet did not show a window, see $WORK/krokiet.log" >&2; exit 1; }
    xdotool windowmove "$WINDOW" 0 0 windowsize "$WINDOW" "$WIDTH" "$HEIGHT"
    sleep 2
}

# The first start creates the default configuration, which is then given the theme and the fixture directory
prepare_home() {
    local theme=$1 home=$WORK/home_$theme
    if [ ! -d "$home/.config/krokiet" ]; then
        mkdir -p "$home"
        start_app "$home"
        stop_all
        start_display
    fi
    python3 - "$home/.config/krokiet" "$theme" "$FIXTURE" <<'PY'
import json, sys
config, theme, fixture = sys.argv[1:]
with open(f"{config}/config_general.json") as file:
    general = json.load(file)
general["dark_theme"] = theme == "dark"
with open(f"{config}/config_general.json", "w") as file:
    json.dump(general, file, indent=2)
with open(f"{config}/config_preset_0.json") as file:
    preset = json.load(file)
preset["included_paths"] = [fixture]
preset["included_paths_referenced"] = []
preset["excluded_paths"] = []
with open(f"{config}/config_preset_0.json", "w") as file:
    json.dump(preset, file, indent=2)
PY
    HOME_DIR=$home
}

click() { xdotool mousemove "$1" "$2" click "${3:-1}"; sleep "${4:-1}"; }
shot() { import -window "$WINDOW" "$OUT/$THEME/$(printf '%02d' "$INDEX")_$1.png"; INDEX=$((INDEX + 1)); }

scan_current_tool() { click 47 "$SCAN_Y" 1 "${1:-2}"; }

capture_theme() {
    THEME=$1
    INDEX=1
    mkdir -p "$OUT/$THEME"
    rm -f "$OUT/$THEME"/*.png
    start_display
    prepare_home "$THEME"
    start_app "$HOME_DIR"

    local index=0
    for tool in "${TOOLS[@]}"; do
        click 100 $((FIRST_TOOL_Y + TOOL_ROW_HEIGHT * index)) 1 0.7
        scan_current_tool $([ "$tool" = disk_usage ] && echo 4 || echo 2)
        shot "$tool"
        index=$((index + 1))
    done

    capture_disk_usage_views
    click 38 "$GEAR_Y" 1 1
    shot settings
    click 60 50 1 1
    shot about
    stop_all
}

# The disk usage tool is already open and scanned: its views, the nested borders, the menu and the settings window
capture_disk_usage_views() {
    click 100 $((FIRST_TOOL_Y + TOOL_ROW_HEIGHT * 14)) 1 1
    click 251 31 1 1.5
    shot disk_usage_rings
    click 179 31 1 1.5
    click 298 31 1 1.5
    shot disk_usage_treemap_nested_borders
    click 298 31 1 1
    xdotool mousemove 520 300
    sleep 0.3
    xdotool mousemove 523 303
    sleep 2
    shot disk_usage_treemap_hover
    click 520 300 3 1
    shot disk_usage_treemap_menu
    click 700 $((HEIGHT - 120)) 1 0.5
    click 111 "$GEAR_Y" 1 1
    shot disk_usage_settings
    # The window is centred and its OK button sits near the bottom of it; the window has no other way of closing
    click $((WIDTH / 2 - 229)) $((HEIGHT / 2 + 113)) 1 0.5
}

make_fixture
mkdir -p "$WORK"
for theme in dark light; do
    capture_theme "$theme"
done
echo "Screenshots saved to $OUT"
