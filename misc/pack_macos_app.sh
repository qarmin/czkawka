#!/bin/sh
# Packages a macOS GUI Mach-O into a double-clickable .app and zips it.
#
# Usage: pack_macos_app.sh <binary> <output_zip> [app_name] [bundle_id]
#   binary      - compiled GUI binary
#   output_zip  - zip to create (overwritten). Contains <app_name>.app.
#   app_name    - Finder name, default Krokiet. A name starting with Czkawka
#                 stores the binary as czkawka_gui; anything else as krokiet.
#   bundle_id   - CFBundleIdentifier, default pl.Qarmin.Krokiet
#
# The bundle is ad-hoc signed. A download is still unidentified to Gatekeeper;
# the first launch needs the Finder context menu "Open".
# The launcher prepends Homebrew to PATH so double-click can see ffmpeg.
set -eu

if [ "$#" -lt 2 ] || [ "$#" -gt 4 ]; then
    echo "usage: pack_macos_app.sh <binary> <output_zip> [app_name] [bundle_id]" >&2
    exit 1
fi

BINARY="$1"
OUTPUT_ZIP="$2"
APP_NAME="${3:-Krokiet}"
BUNDLE_ID="${4:-pl.Qarmin.Krokiet}"

case "$APP_NAME" in
    Czkawka*) EXEC_NAME="czkawka_gui" ;;
    *) EXEC_NAME="krokiet" ;;
esac

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
ICON_SVG="$REPO_ROOT/krokiet/icons/krokiet_logo_flag.svg"
VERSION=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$REPO_ROOT/krokiet/Cargo.toml" | head -n 1)
if [ -z "$VERSION" ]; then
    VERSION="0.0.0"
fi

WORKDIR=$(mktemp -d)
# shellcheck disable=SC2064
trap "rm -rf '$WORKDIR'" EXIT

APP="$WORKDIR/$APP_NAME.app"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

cp "$BINARY" "$APP/Contents/MacOS/$EXEC_NAME.bin"
chmod +x "$APP/Contents/MacOS/$EXEC_NAME.bin"
# cp keeps the quarantine xattr of a downloaded binary, which makes Gatekeeper
# reject the bundle even after it is signed.
xattr -c "$APP/Contents/MacOS/$EXEC_NAME.bin" 2>/dev/null || true

# Launch Services starts the app with a minimal PATH, so Homebrew ffmpeg
# would be invisible. The shell inherits the user's PATH when they run the
# naked binary from a terminal.
cat > "$APP/Contents/MacOS/$EXEC_NAME" <<EOF
#!/bin/sh
DIR=\$(CDPATH= cd -- "\$(dirname "\$0")" && pwd)
export PATH="/opt/homebrew/bin:/usr/local/bin:\${PATH:-/usr/bin:/bin:/usr/sbin:/sbin}"
exec "\$DIR/$EXEC_NAME.bin" "\$@"
EOF
chmod +x "$APP/Contents/MacOS/$EXEC_NAME"

cat > "$APP/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>
  <string>${APP_NAME}</string>
  <key>CFBundleDisplayName</key>
  <string>${APP_NAME}</string>
  <key>CFBundleIdentifier</key>
  <string>${BUNDLE_ID}</string>
  <key>CFBundleExecutable</key>
  <string>${EXEC_NAME}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleVersion</key>
  <string>${VERSION}</string>
  <key>CFBundleShortVersionString</key>
  <string>${VERSION}</string>
  <key>CFBundleIconFile</key>
  <string>AppIcon</string>
  <key>LSMinimumSystemVersion</key>
  <string>10.15</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
EOF

render_icon_png() {
    dest="$1"
    if command -v rsvg-convert >/dev/null 2>&1; then
        rsvg-convert -w 1024 -h 1024 "$ICON_SVG" -o "$dest"
        return
    fi
    thumb_dir=$(mktemp -d)
    qlmanage -t -s 1024 -o "$thumb_dir" "$ICON_SVG" >/dev/null
    rendered=$(find "$thumb_dir" -name '*.png' -print | head -n 1)
    if [ -z "$rendered" ]; then
        echo "failed to rasterize $ICON_SVG" >&2
        exit 1
    fi
    cp "$rendered" "$dest"
    rm -rf "$thumb_dir"
}

ICONSET="$WORKDIR/AppIcon.iconset"
mkdir -p "$ICONSET"
MASTER="$WORKDIR/icon_1024.png"
render_icon_png "$MASTER"

# iconutil requires this exact size set.
for spec in \
    "16:icon_16x16.png" \
    "32:icon_16x16@2x.png" \
    "32:icon_32x32.png" \
    "64:icon_32x32@2x.png" \
    "128:icon_128x128.png" \
    "256:icon_128x128@2x.png" \
    "256:icon_256x256.png" \
    "512:icon_256x256@2x.png" \
    "512:icon_512x512.png" \
    "1024:icon_512x512@2x.png"
do
    size=${spec%%:*}
    name=${spec#*:}
    sips -z "$size" "$size" "$MASTER" --out "$ICONSET/$name" >/dev/null
done

iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns"
plutil -lint "$APP/Contents/Info.plist" >/dev/null

# Ad-hoc signature. No Apple Developer certificate.
# Sign the Mach-O first; the bundle executable is a shell launcher.
codesign --force --sign - "$APP/Contents/MacOS/$EXEC_NAME.bin"
codesign --force --sign - "$APP"
codesign --verify --strict "$APP"

rm -f "$OUTPUT_ZIP"
ditto -c -k --keepParent "$APP" "$OUTPUT_ZIP"
