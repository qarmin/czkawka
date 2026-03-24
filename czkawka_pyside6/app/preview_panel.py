from pathlib import Path

from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QLabel, QSizePolicy
)
from PySide6.QtCore import Qt, QSize
from PySide6.QtGui import QPixmap, QImage


class PreviewPanel(QWidget):
    """Image preview panel for similar images / duplicate files."""

    SUPPORTED_EXTENSIONS = {
        ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp",
        ".tiff", ".tif", ".ico", ".svg"
    }

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setMinimumWidth(200)
        self.setMaximumWidth(400)
        self._current_path = ""
        self._setup_ui()

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(4, 4, 4, 4)

        self._title = QLabel("Preview")
        font = self._title.font()
        font.setBold(True)
        self._title.setFont(font)
        self._title.setAlignment(Qt.AlignCenter)
        layout.addWidget(self._title)

        self._image_label = QLabel()
        self._image_label.setAlignment(Qt.AlignCenter)
        self._image_label.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Expanding)
        self._image_label.setMinimumSize(QSize(180, 180))
        self._image_label.setFrameShape(QLabel.StyledPanel)
        self._image_label.setScaledContents(False)
        layout.addWidget(self._image_label)

        self._info_label = QLabel()
        self._info_label.setAlignment(Qt.AlignCenter)
        self._info_label.setWordWrap(True)
        self._info_label.setEnabled(False)
        layout.addWidget(self._info_label)

    def show_preview(self, file_path: str):
        if not file_path or file_path == self._current_path:
            return

        self._current_path = file_path
        p = Path(file_path)

        if not p.exists():
            self._image_label.setText("File not found")
            self._info_label.setText("")
            return

        if p.suffix.lower() not in self.SUPPORTED_EXTENSIONS:
            self._image_label.setText("Preview not available\nfor this file type")
            self._info_label.setText(p.name)
            return

        pixmap = QPixmap(file_path)
        if pixmap.isNull():
            self._image_label.setText("Cannot load image")
            self._info_label.setText(p.name)
            return

        # Scale to fit while keeping aspect ratio
        label_size = self._image_label.size()
        scaled = pixmap.scaled(
            label_size, Qt.KeepAspectRatio, Qt.SmoothTransformation
        )
        self._image_label.setPixmap(scaled)

        # Show info
        size = p.stat().st_size
        size_str = self._format_size(size)
        self._info_label.setText(
            f"{p.name}\n{pixmap.width()}x{pixmap.height()} | {size_str}"
        )
        self._title.setText("Preview")

    def clear_preview(self):
        self._current_path = ""
        self._image_label.clear()
        self._image_label.setText("No preview")
        self._info_label.setText("")

    def resizeEvent(self, event):
        super().resizeEvent(event)
        # Re-render if we have a current image
        if self._current_path:
            path = self._current_path
            self._current_path = ""
            self.show_preview(path)

    @staticmethod
    def _format_size(size_bytes: int) -> str:
        if size_bytes == 0:
            return "0 B"
        units = ["B", "KB", "MB", "GB"]
        i = 0
        size = float(size_bytes)
        while size >= 1024 and i < len(units) - 1:
            size /= 1024
            i += 1
        return f"{size:.1f} {units[i]}" if i > 0 else f"{int(size)} B"
