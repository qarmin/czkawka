#!/bin/bash

# Define bundle ID and paths
BUNDLE_ID="com.github.qarmin.czkawka"
DIR="$(dirname "$0")"
PARENT_DIR="$(dirname "$DIR")"
BUNDLE_NAME="Czkawka.app"
BUNDLE_PATH="$PARENT_DIR/target/release/$BUNDLE_NAME"
BINARY_NAME="czkawka_gui"
VERSION=$(grep '^version = ' "$PARENT_DIR/$BINARY_NAME/Cargo.toml" | head -n 1 | cut -d '"' -f 2)
SVG_PATH="$PARENT_DIR/data/icons/${BUNDLE_ID}.svg"
ICONSET_DIR="${PARENT_DIR}/data/icons/${BUNDLE_ID}.iconset"
ICON_SIZES="16 32 64 128 256 512"
SRC_ICON="$SVG_PATH"

# Build the project
cargo build --release --manifest-path="$DIR/../Cargo.toml" --bin $BINARY_NAME

# Create the application bundle structure and copy the binary
mkdir -p "$BUNDLE_PATH/Contents/MacOS" "$BUNDLE_PATH/Contents/Resources"
cp "$PARENT_DIR/target/release/$BINARY_NAME" "$BUNDLE_PATH/Contents/MacOS/"

# Create iconset directory and generate icon sizes
mkdir -p "$ICONSET_DIR"
for SIZE in $ICON_SIZES; do
    rsvg-convert -w $SIZE -h $SIZE $SRC_ICON -o "${ICONSET_DIR}/icon_${SIZE}x${SIZE}.png"
    [ $SIZE -ne 512 ] && rsvg-convert -w $((SIZE*2)) -h $((SIZE*2)) $SRC_ICON -o "${ICONSET_DIR}/icon_${SIZE}x${SIZE}@2x.png"
done

# Convert the iconset to an icns file and clean up the iconset directory
iconutil -c icns "$ICONSET_DIR" -o "$BUNDLE_PATH/Contents/Resources/${BUNDLE_ID}.icns"

rm -rf "$ICONSET_DIR"

# Create the Info.plist file
cat <<EOF >"$BUNDLE_PATH/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$BINARY_NAME</string>
    <key>CFBundleIconFile</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleName</key>
    <string>Czkawka</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
</dict>
</plist>
EOF

echo "Application bundle created at: $BUNDLE_PATH"