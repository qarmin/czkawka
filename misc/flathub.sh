#!/bin/bash
pip3 install aiohttp toml
wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
python3 flatpak-cargo-generator.py ./Cargo.lock -o flatpak/cargo-sources.json
rm flatpak-cargo-generator.py
