#!/bin/bash
# Get the directory of the current script (works even if the script is called from another location)
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PARENT_DIR="$(dirname "$DIR")"
BUNDLE_NAME="Czkawka.app"
# Simplify the bundle path using cd and pwd
BUNDLE_PATH=$(cd "$DIR/../target/release" && pwd)/$BUNDLE_NAME

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

# Step 4: Create and copy the icon and rename as .icns (macOS icon format)
# Assuming the icon file is named correctly and located in the correct directory


ICON_NAME="com.github.qarmin.czkawka.icns"
cp "$PARENT_DIR/data/icons/$ICON_NAME" "$BUNDLE_PATH/Contents/Resources/"

# Step 5: Create the Info.plist file
cat <<EOF >"$BUNDLE_PATH/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$BINARY_NAME</string>
    <key>CFBundleIconFile</key>
    <string>com.github.qarmin.czkawka</string>
    <key>CFBundleIdentifier</key>
    <string>com.github.qarmin.czkawka</string>
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
