from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox, QFrame
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QPixmap, QFont

from ..icons import app_logo_path
from ..localizer import tr


class AboutDialog(QDialog):
    """About dialog showing application information."""

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setWindowTitle(tr("about-title"))
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

        title = QLabel(tr("about-app-name"))
        title_font = QFont()
        title_font.setPointSize(22)
        title_font.setBold(True)
        title.setFont(title_font)
        title.setAlignment(Qt.AlignCenter)
        layout.addWidget(title)

        subtitle = QLabel(tr("about-subtitle"))
        sub_font = QFont()
        sub_font.setPointSize(11)
        subtitle.setFont(sub_font)
        subtitle.setAlignment(Qt.AlignCenter)
        layout.addWidget(subtitle)

        version = QLabel(tr("about-version"))
        version.setAlignment(Qt.AlignCenter)
        version.setEnabled(False)
        layout.addWidget(version)

        # Separator
        sep = QFrame()
        sep.setFrameShape(QFrame.HLine)
        sep.setFrameShadow(QFrame.Sunken)
        layout.addWidget(sep)

        desc = QLabel(tr("about-description"))
        desc.setWordWrap(True)
        desc.setAlignment(Qt.AlignCenter)
        layout.addWidget(desc)

        layout.addStretch()

        buttons = QDialogButtonBox(QDialogButtonBox.Ok)
        buttons.accepted.connect(self.accept)
        layout.addWidget(buttons)
