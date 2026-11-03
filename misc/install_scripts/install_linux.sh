#!/usr/bin/env bash
set -euo pipefail

if [[ "$EUID" -ne 0 ]]; then
    echo "Please run this script as root (sudo)."
    exit 1
fi

# Detect distro
if command -v apt &>/dev/null; then
    DISTRO="debian"
elif command -v dnf &>/dev/null; then
    DISTRO="fedora"
elif command -v pacman &>/dev/null; then
    DISTRO="arch"
elif command -v zypper &>/dev/null; then
    DISTRO="opensuse"
else
    echo "Unsupported distro. Install manually: ffmpeg."
    exit 1
fi

# Packages per distro
case "$DISTRO" in
    debian)
        BASE="ffmpeg"
        INSTALL_CMD="apt install -y"
        UPDATE_CMD="apt update"
        ;;
    fedora)
        BASE="ffmpeg"
        INSTALL_CMD="dnf install -y"
        UPDATE_CMD="dnf check-update || true"
        ;;
    arch)
        BASE="ffmpeg"
        INSTALL_CMD="pacman -S --noconfirm"
        UPDATE_CMD="pacman -Sy"
        ;;
    opensuse)
        BASE="ffmpeg-4"
        INSTALL_CMD="zypper install -y"
        UPDATE_CMD="zypper refresh"
        ;;
esac

echo "Detected distro: $DISTRO"
echo ""
echo "The following packages will be installed:"
echo "  Base:     $BASE"
echo ""
read -r -p "Do you want to continue? [Y/n] " answer
if [[ "$answer" =~ ^[Nn]$ ]]; then
    echo "Aborted."
    exit 0
fi

$UPDATE_CMD

$INSTALL_CMD $BASE

echo ""
echo "Successfully installed all dependencies."


