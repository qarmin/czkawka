#!/bin/bash

# Calculate individual MD5 hashes with file names and join them with commas
MD5_DEBUG_1=$(md5sum czkawka_cli_debug_1 czkawka_gui_debug_1 krokiet_debug_1 | awk '{print $1 "(" $2 ")"}' | paste -sd ",")
MD5_DEBUG_2=$(md5sum czkawka_cli_debug_2 czkawka_gui_debug_2 krokiet_debug_2 | awk '{print $1 "(" $2 ")"}' | paste -sd ",")
MD5_RELEASE_1=$(md5sum czkawka_cli_release_1 czkawka_gui_release_1 krokiet_release_1 | awk '{print $1 "(" $2 ")"}' | paste -sd ",")
MD5_RELEASE_2=$(md5sum czkawka_cli_release_2 czkawka_gui_release_2 krokiet_release_2 | awk '{print $1 "(" $2 ")"}' | paste -sd ",")

# Print MD5 hashes
echo "MD5_DEBUG_1: $MD5_DEBUG_1"
echo "MD5_DEBUG_2: $MD5_DEBUG_2"
echo ""
echo "MD5_RELEASE_1: $MD5_RELEASE_1"
echo "MD5_RELEASE_2: $MD5_RELEASE_2"

if [ "$MD5_DEBUG_1" == "$MD5_DEBUG_2" ]; then
    echo "DEBUG files are the same"
else
    echo "DEBUG files are different"
    exit 1
fi
if [ "$MD5_RELEASE_1" == "$MD5_RELEASE_2" ]; then
    echo "RELEASE files are the same"
else
    echo "RELEASE files are different"
    exit 1
fi