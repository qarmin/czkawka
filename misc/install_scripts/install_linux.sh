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
    echo "Unsupported distro. Install manually: ffmpeg, libgtk-4, libheif, libraw, libavif."
    exit 1
fi

# Packages per distro
case "$DISTRO" in
    debian)
        BASE="ffmpeg libgtk-4-dev"
        OPTIONAL="libheif-dev libraw-dev libavif-dev libdav1d-dev"
        INSTALL_CMD="apt install -y"
        UPDATE_CMD="apt update"
        ;;
    fedora)
        BASE="ffmpeg gtk4-devel"
        OPTIONAL="libheif-devel LibRaw-devel libavif-devel dav1d-devel"
        INSTALL_CMD="dnf install -y"
        UPDATE_CMD="dnf check-update || true"
        ;;
    arch)
        BASE="ffmpeg gtk4"
        OPTIONAL="libheif libraw libavif dav1d"
        INSTALL_CMD="pacman -S --noconfirm"
        UPDATE_CMD="pacman -Sy"
        ;;
    opensuse)
        BASE="ffmpeg-4 gtk4-devel"
        OPTIONAL="libheif-devel libraw-devel libavif-devel dav1d-devel"
        INSTALL_CMD="zypper install -y"
        UPDATE_CMD="zypper refresh"
        ;;
esac

echo "Detected distro: $DISTRO"
echo ""
echo "The following packages will be installed:"
echo "  Base:     $BASE"
echo "  Optional: $OPTIONAL"
echo ""
read -r -p "Do you want to continue? [Y/n] " answer
if [[ "$answer" =~ ^[Nn]$ ]]; then
    echo "Aborted."
    exit 0
fi

$UPDATE_CMD

$INSTALL_CMD $BASE

$INSTALL_CMD $OPTIONAL || echo "Some optional packages unavailable on this distro version — skipping."

echo ""
echo "Successfully installed all dependencies."


