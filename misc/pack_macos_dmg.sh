#!/bin/bash
# Wraps a krokiet binary into an ad-hoc signed Krokiet.app and packs it into a DMG with an /Applications link.
#
# Usage: pack_macos_dmg.sh <binary> <output_dmg>
#
# Only hdiutil/sips/iconutil/codesign from the base system are used, no Finder/AppleScript, so it works
# on headless CI runners and over ssh.
set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "usage: pack_macos_dmg.sh <binary> <output_dmg>" >&2
    exit 1
fi

BINARY="$1"
OUTPUT_DMG="$2"

APP_NAME="Krokiet"
EXEC_NAME="krokiet"
# Matches the folder name `directories-next` gives the config dir on macOS
BUNDLE_ID="pl.Qarmin.Krokiet"

REPO_ROOT=$(cd "$(dirname "$0")/.." && pwd)
ICON_PNG="$REPO_ROOT/krokiet/icons/krokiet_logo_flag.png"
VERSION=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$REPO_ROOT/krokiet/Cargo.toml" | head -n 1)
if [ -z "$VERSION" ]; then
    echo "failed to read version from krokiet/Cargo.toml" >&2
    exit 1
fi

WORKDIR=$(mktemp -d)
trap 'rm -rf "$WORKDIR"' EXIT

STAGING="$WORKDIR/staging"
APP="$STAGING/$APP_NAME.app"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

cp "$BINARY" "$APP/Contents/MacOS/$EXEC_NAME"
chmod +x "$APP/Contents/MacOS/$EXEC_NAME"

ICONSET="$WORKDIR/AppIcon.iconset"
mkdir -p "$ICONSET"
for size in 16 32 128 256 512; do
    sips -z "$size" "$size" "$ICON_PNG" --out "$ICONSET/icon_${size}x${size}.png" >/dev/null
    double=$((size * 2))
    sips -z "$double" "$double" "$ICON_PNG" --out "$ICONSET/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns"

cat > "$APP/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
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
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSDesktopFolderUsageDescription</key>
    <string>Krokiet scans the folders you select for duplicates and other unneeded files.</string>
    <key>NSDocumentsFolderUsageDescription</key>
    <string>Krokiet scans the folders you select for duplicates and other unneeded files.</string>
    <key>NSDownloadsFolderUsageDescription</key>
    <string>Krokiet scans the folders you select for duplicates and other unneeded files.</string>
    <key>NSRemovableVolumesUsageDescription</key>
    <string>Krokiet scans the folders you select for duplicates and other unneeded files.</string>
    <key>NSNetworkVolumesUsageDescription</key>
    <string>Krokiet scans the folders you select for duplicates and other unneeded files.</string>
</dict>
</plist>
EOF
plutil -lint "$APP/Contents/Info.plist" >/dev/null

# codesign rejects bundles with extended attributes (e.g. quarantine of a downloaded binary)
xattr -cr "$APP"
codesign --force --sign - "$APP"
codesign --verify --strict --verbose=2 "$APP"

ln -s /Applications "$STAGING/Applications"

# hdiutil create sometimes fails with "Resource busy" on GitHub macOS runners
rm -f "$OUTPUT_DMG"
for attempt in 1 2 3; do
    if hdiutil create -volname "$APP_NAME" -srcfolder "$STAGING" -fs HFS+ -format ULFO -ov "$OUTPUT_DMG"; then
        break
    fi
    if [ "$attempt" -eq 3 ]; then
        echo "hdiutil create failed 3 times" >&2
        exit 1
    fi
    sleep 5
done
hdiutil verify "$OUTPUT_DMG"
