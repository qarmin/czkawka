#!/bin/bash

# Calculate individual MD5 hashes with file names and join them with commas
MD1=$(md5sum czkawka_cli_debug_1 czkawka_gui_debug_1 krokiet_gui_debug_1 czkawka_cli_release_1 czkawka_gui_release_1 krokiet_gui_release_1  | awk '{print $1 "(" $2 ")"}' | paste -sd ",")
MD2=$(md5sum czkawka_cli_debug_2 czkawka_gui_debug_2 krokiet_gui_debug_2 czkawka_cli_release_2 czkawka_gui_release_2 krokiet_gui_release_2 | awk '{print $1 "(" $2 ")"}' | paste -sd ",")

# Print MD5 hashes
echo "MD5 1 - $MD1"
echo "MD5 2 - $MD2"

# Compare and fail if MD1 != MD2
if [ "$MD1" != "$MD2" ]; then
  echo "MD5 hashes do not match. Failing."
  exit 1
else
  echo "MD5 hashes match. Success."
fi