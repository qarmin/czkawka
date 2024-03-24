#!/bin/bash
# Get the directory of the current script (works even if the script is called from another location)
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PARENT_DIR="$(dirname "$DIR")"

BUNDLE_NAME="Czkawka.app"
BUNDLE_PATH="$DIR/../target/release/$BUNDLE_NAME"
BINARY_NAME="czkawka_gui"

# Step 1: Build the project
cargo build --release --manifest-path="$DIR/../Cargo.toml" --bin $BINARY_NAME

# Step 2: Create the application bundle structure
mkdir -p "$BUNDLE_PATH/Contents/MacOS"
mkdir -p "$BUNDLE_PATH/Contents/Resources"

# Step 3: Copy the binary
cp "$PARENT_DIR/target/release/$BINARY_NAME" "$BUNDLE_PATH/Contents/MacOS/"

# Step 4: Copy the icon and rename as .icns (macOS icon format)
# Replace 'myicon.icns' with the actual name of your .icns file
ICON_NAME="Czkawka.icns"
cp "$PARENT_DIR/data/icons/com.github.qarmin.czkawka.svg" "$BUNDLE_PATH/Contents/Resources/$ICON_NAME"

# Step 5: Create the Info.plist file
cat <<EOF >"$BUNDLE_PATH/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$BINARY_NAME</string>
    <key>CFBundleIconFile</key>
    <string>$ICON_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.github.qarmin.czkawka</string>
    <key>CFBundleName</key>
    <string>Czkawka</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
</dict>
</plist>
EOF

echo "Application bundle created at: $BUNDLE_PATH"