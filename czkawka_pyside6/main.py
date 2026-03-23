#!/usr/bin/env python3
"""
Czkawka PySide6 - A PySide6/Qt interface for czkawka file cleanup tool.

This application provides a graphical interface to czkawka, using the
czkawka_cli binary as its backend for all scanning operations.

Usage:
    python main.py
    # or
    python -m czkawka_pyside6.main

Requirements:
    - PySide6 >= 6.6.0
    - czkawka_cli binary in PATH (or configured in settings)
    - Optional: send2trash (for trash support)
    - Optional: Pillow (for EXIF cleaning)
"""

import sys
import os


def main():
    # Set environment for better HiDPI support
    os.environ.setdefault("QT_AUTO_SCREEN_SCALE_FACTOR", "1")

    from PySide6.QtWidgets import QApplication
    from PySide6.QtCore import Qt
    from PySide6.QtGui import QFont

    app = QApplication(sys.argv)
    app.setApplicationName("Czkawka")
    app.setApplicationVersion("11.0.1")
    app.setOrganizationName("czkawka")
    app.setDesktopFileName("com.github.qarmin.czkawka")

    # Set application icon
    from app.icons import app_icon
    icon = app_icon()
    if not icon.isNull():
        app.setWindowIcon(icon)

    # Set default font
    font = QFont()
    font.setPointSize(10)
    app.setFont(font)

    # Import and create main window
    from app.main_window import MainWindow
    window = MainWindow()
    window.show()

    sys.exit(app.exec())


if __name__ == "__main__":
    main()
