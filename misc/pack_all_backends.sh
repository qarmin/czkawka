#!/bin/sh
# Packages an all-backends krokiet binary together with per-backend launcher
# scripts into a zip archive (shell scripts, or batch scripts for a Windows binary).
#
# Usage: pack_all_backends.sh <binary> <output_zip> [windows]
#   binary      - path to the compiled all-backends krokiet binary
#   output_zip  - path for the resulting zip file (created or overwritten)
#   windows     - optional, produce krokiet.exe with .bat launchers instead of .sh ones
set -eu

BINARY="$1"
OUTPUT_ZIP="$2"
TARGET_OS="${3:-unix}"

BACKENDS="winit-femtovg femtovg-wgpu vello winit-software"

PKGDIR=$(mktemp -d)
# shellcheck disable=SC2064
trap "rm -rf '$PKGDIR'" EXIT

if [ "$TARGET_OS" = "windows" ]; then
    cp "$BINARY" "$PKGDIR/krokiet.exe"
    for backend in $BACKENDS; do
        printf '@echo off\r\nset SLINT_BACKEND=%s\r\n"%%~dp0krokiet.exe" %%*\r\n' "$backend" > "$PKGDIR/krokiet_$(echo "$backend" | tr '-' '_').bat"
    done
else
    cp "$BINARY" "$PKGDIR/krokiet"
    chmod +x "$PKGDIR/krokiet"
    for backend in $BACKENDS; do
        printf '#!/bin/sh\nSLINT_BACKEND=%s exec "$(dirname "$0")/krokiet" "$@"\n' "$backend" > "$PKGDIR/krokiet_$(echo "$backend" | tr '-' '_').sh"
    done
    chmod +x "$PKGDIR"/*.sh
fi

OUTPUT_ZIP_ABS="$(cd "$(dirname "$OUTPUT_ZIP")" && pwd)/$(basename "$OUTPUT_ZIP")"
rm -f "$OUTPUT_ZIP_ABS"
(cd "$PKGDIR" && zip -qr "$OUTPUT_ZIP_ABS" .)
