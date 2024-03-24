#!/bin/bash

# Define bundle ID
BUNDLE_ID="com.github.qarmin.czkawka"

# Get the directory of the current script (works even if the script is called from another location)
DIR="$(dirname "$0")"
PARENT_DIR="$(dirname "$DIR")"
BUNDLE_NAME="Czkawka.app"
BUNDLE_PATH="$PARENT_DIR/target/release/$BUNDLE_NAME"

BINARY_NAME="czkawka_gui"
# Extract version from Cargo.toml
VERSION=$(grep '^version = ' "$PARENT_DIR/$BINARY_NAME/Cargo.toml" | head -n 1 | cut -d '"' -f 2)

# Step 1: Build the project
cargo build --release --manifest-path="$DIR/../Cargo.toml" --bin $BINARY_NAME

# Step 2: Create the application bundle structure
mkdir -p "$BUNDLE_PATH/Contents/MacOS"
mkdir -p "$BUNDLE_PATH/Contents/Resources"

# Step 3: Copy the binary
cp "$PARENT_DIR/target/release/$BINARY_NAME" "$BUNDLE_PATH/Contents/MacOS/"

# Step 4: Generate the icon from SVG and copy it to the bundle
SVG_PATH="$PARENT_DIR/data/icons/${BUNDLE_ID}.svg"
ICONSET_DIR="${PARENT_DIR}/data/icons/${BUNDLE_ID}.iconset"

# Create iconset directory
mkdir -p "$ICONSET_DIR"

# Generate icon sizes and populate the iconset
ICON_SIZES="16 32 64 128 256 512"
SRC_ICON="$SVG_PATH"

for SIZE in $ICON_SIZES; do
    rsvg-convert -w $SIZE -h $SIZE $SRC_ICON -o "${ICONSET_DIR}/icon_${SIZE}x${SIZE}.png"
    if [ $SIZE -ne 512 ]; then
        SIZE_2X=$((SIZE*2))
        rsvg-convert -w $SIZE_2X -h $SIZE_2X $SRC_ICON -o "${ICONSET_DIR}/icon_${SIZE}x${SIZE}@2x.png"
    fi
done

# Convert the iconset to an icns file
iconutil -c icns "$ICONSET_DIR" -o "$BUNDLE_PATH/Contents/Resources/${BUNDLE_ID}.icns"

# Clean up the iconset directory
rm -rf "$ICONSET_DIR"

# Step 5: Create the Info.plist file
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
