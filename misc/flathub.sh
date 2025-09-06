#!/bin/bash
pip3 install aiohttp toml # Or sudo apt install python3-aiohttp python3-toml python3-tomlkit
wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
mkdir flatpak
python3 flatpak-cargo-generator.py ./Cargo.lock -o flatpak/cargo-sources.json
rm flatpak-cargo-generator.py
