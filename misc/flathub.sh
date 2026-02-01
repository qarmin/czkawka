#!/bin/bash
rm -rf flatpak
uv venv -p 3.11
uv pip install aiohttp toml tomlkit # Or sudo apt install python3-aiohttp python3-toml python3-tomlkit
wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
mkdir flatpak
uv run python3 flatpak-cargo-generator.py ./Cargo.lock -o flatpak/cargo-sources.json
rm flatpak-cargo-generator.py