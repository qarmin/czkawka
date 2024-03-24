#!/bin/bash

# Navigate to the script's directory
cd "$(dirname "$0")"

# Define the base directory relative to this script
BASE_DIR="../data/icons"

# Name of the original SVG file without the extension
BASENAME="com.github.qarmin.czkawka"

# Full path to the iconset directory
ICONSET_DIR="${BASE_DIR}/${BASENAME}.iconset"

# Create the iconset directory
mkdir -p "$ICONSET_DIR"

# List of sizes to create (standard macOS icon sizes)
declare -a SIZES=("16" "32" "64" "128" "256" "512" "1024")

# Loop through the sizes and generate PNG files
for SIZE in "${SIZES[@]}"; do
    rsvg-convert -h $SIZE "${BASE_DIR}/${BASENAME}.svg" > "${ICONSET_DIR}/icon_${SIZE}x${SIZE}.png"
    # Create @2x versions for retina displays
    if [ $SIZE -ne 1024 ]; then # Skip 1024 size for @2x, as it's the max size
        let DOUBLE_SIZE=SIZE*2
        rsvg-convert -h $DOUBLE_SIZE "${BASE_DIR}/${BASENAME}.svg" > "${ICONSET_DIR}/icon_${SIZE}x${SIZE}@2x.png"
    fi
done

# Convert the iconset to an ICNS file
iconutil -c icns "$ICONSET_DIR"

# Move the ICNS file to the icons directory
mv "${ICONSET_DIR}.icns" "${BASE_DIR}/${BASENAME}.icns"

echo "ICNS file created as ${BASE_DIR}/${BASENAME}.icns"

