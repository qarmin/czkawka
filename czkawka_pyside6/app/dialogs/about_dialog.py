from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QPixmap

from ..icons import app_logo_path


class AboutDialog(QDialog):
    """About dialog showing application information."""

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setWindowTitle("About Czkawka PySide6")
        self.setMinimumWidth(480)
        self.setMinimumHeight(420)

        layout = QVBoxLayout(self)
        layout.setSpacing(6)

        # Logo
        logo_path = app_logo_path()
        if logo_path:
            logo_label = QLabel()
            pixmap = QPixmap(logo_path)
            scaled = pixmap.scaledToHeight(100, Qt.SmoothTransformation)
            logo_label.setPixmap(scaled)
            logo_label.setAlignment(Qt.AlignCenter)
            layout.addWidget(logo_label)

        title = QLabel("Czkawka")
        title.setStyleSheet("font-size: 28px; font-weight: bold; padding: 4px;")
        title.setAlignment(Qt.AlignCenter)
        layout.addWidget(title)

        subtitle = QLabel("PySide6 / Qt6 Edition")
        subtitle.setStyleSheet("font-size: 14px; color: #6fbf73; padding: 2px;")
        subtitle.setAlignment(Qt.AlignCenter)
        layout.addWidget(subtitle)

        version = QLabel("Version 11.0.1")
        version.setStyleSheet("font-size: 11px; color: #888; padding: 2px;")
        version.setAlignment(Qt.AlignCenter)
        layout.addWidget(version)

        # Separator
        sep = QLabel()
        sep.setFixedHeight(1)
        sep.setStyleSheet("background-color: #444; margin: 8px 40px;")
        layout.addWidget(sep)

        desc = QLabel(
            "Czkawka (tch-kav-ka) is a simple, fast and free app to remove\n"
            "unnecessary files from your computer.\n\n"
            "This PySide6/Qt interface uses the czkawka_cli backend\n"
            "for all scanning and file operations.\n\n"
            "Features:\n"
            "  - Find duplicate files (by hash, name, or size)\n"
            "  - Find empty files and folders\n"
            "  - Find similar images, videos, and music\n"
            "  - Find broken files and invalid symlinks\n"
            "  - Find files with bad extensions or names\n"
            "  - Remove EXIF metadata from images\n"
            "  - Optimize and crop videos\n\n"
            "Licensed under MIT License\n"
            "https://github.com/qarmin/czkawka"
        )
        desc.setWordWrap(True)
        desc.setAlignment(Qt.AlignCenter)
        desc.setStyleSheet("padding: 6px 20px; line-height: 1.4;")
        layout.addWidget(desc)

        layout.addStretch()

        buttons = QDialogButtonBox(QDialogButtonBox.Ok)
        buttons.accepted.connect(self.accept)
        layout.addWidget(buttons)
