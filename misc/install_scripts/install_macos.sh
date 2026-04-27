#!/usr/bin/env bash
set -euo pipefail

# Check for Homebrew
if ! command -v brew &>/dev/null; then
    echo "Homebrew is not installed."
    read -r -p "Install Homebrew now? [y/N] " answer
    if [[ "$answer" =~ ^[Yy]$ ]]; then
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    else
        echo "Homebrew is required. Aborting."
        exit 1
    fi
fi

PACKAGES="ffmpeg libheif libraw libavif"

echo "The following packages will be installed via Homebrew:"
for pkg in $PACKAGES; do echo "  $pkg"; done
echo ""
read -r -p "Do you want to continue? [Y/n] " answer
if [[ "$answer" =~ ^[Nn]$ ]]; then
    echo "Aborted."
    exit 0
fi

brew install $PACKAGES

echo ""
echo "Successfully installed all dependencies."

