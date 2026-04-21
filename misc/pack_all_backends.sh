#!/bin/sh
# Packages an all-backends krokiet binary together with per-backend launcher
# shell scripts into a zip archive.
#
# Usage: pack_all_backends.sh <binary> <output_zip>
#   binary      - path to the compiled all-backends krokiet binary
#   output_zip  - path for the resulting zip file (created or overwritten)
set -eu

BINARY="$1"
OUTPUT_ZIP="$2"

PKGDIR=$(mktemp -d)
# shellcheck disable=SC2064
trap "rm -rf '$PKGDIR'" EXIT

cp "$BINARY" "$PKGDIR/krokiet"
chmod +x "$PKGDIR/krokiet"

printf '#!/bin/sh\nSLINT_BACKEND=winit-femtovg exec "$(dirname "$0")/krokiet" "$@"\n'     > "$PKGDIR/krokiet_winit_femtovg.sh"
printf '#!/bin/sh\nSLINT_BACKEND=winit-skia-opengl exec "$(dirname "$0")/krokiet" "$@"\n' > "$PKGDIR/krokiet_winit_skia_opengl.sh"
printf '#!/bin/sh\nSLINT_BACKEND=winit-skia-vulkan exec "$(dirname "$0")/krokiet" "$@"\n' > "$PKGDIR/krokiet_winit_skia_vulkan.sh"
printf '#!/bin/sh\nSLINT_BACKEND=winit-software exec "$(dirname "$0")/krokiet" "$@"\n'    > "$PKGDIR/krokiet_winit_software.sh"
printf '#!/bin/sh\nSLINT_BACKEND=femtovg-wgpu exec "$(dirname "$0")/krokiet" "$@"\n'     > "$PKGDIR/krokiet_femtovg_wgpu.sh"
chmod +x "$PKGDIR"/*.sh

(cd "$PKGDIR" && zip -r - .) > "$OUTPUT_ZIP"

