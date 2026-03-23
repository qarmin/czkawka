from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox, QFrame
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QPixmap, QFont

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
        title_font = QFont()
        title_font.setPointSize(22)
        title_font.setBold(True)
        title.setFont(title_font)
        title.setAlignment(Qt.AlignCenter)
        layout.addWidget(title)

        subtitle = QLabel("PySide6 / Qt 6 Edition")
        sub_font = QFont()
        sub_font.setPointSize(11)
        subtitle.setFont(sub_font)
        subtitle.setAlignment(Qt.AlignCenter)
        layout.addWidget(subtitle)

        version = QLabel("Version 11.0.1")
        version.setAlignment(Qt.AlignCenter)
        version.setEnabled(False)
        layout.addWidget(version)

        # Separator
        sep = QFrame()
        sep.setFrameShape(QFrame.HLine)
        sep.setFrameShadow(QFrame.Sunken)
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
        layout.addWidget(desc)

        layout.addStretch()

        buttons = QDialogButtonBox(QDialogButtonBox.Ok)
        buttons.accepted.connect(self.accept)
        layout.addWidget(buttons)
