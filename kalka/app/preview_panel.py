from pathlib import Path

from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QLabel, QSizePolicy, QSplitter,
    QStackedWidget
)
from PySide6.QtCore import Qt, QSize
from PySide6.QtGui import QPixmap

from .localizer import tr


class _ImageSlot(QWidget):
    """Single image preview slot with title and info."""

    SUPPORTED_EXTENSIONS = {
        ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp",
        ".tiff", ".tif", ".ico",
    }

    def __init__(self, parent=None):
        super().__init__(parent)
        self._current_path = ""
        self._pixmap = None

        layout = QVBoxLayout(self)
        layout.setContentsMargins(2, 2, 2, 2)
        layout.setSpacing(2)

        self._image_label = QLabel()
        self._image_label.setAlignment(Qt.AlignCenter)
        self._image_label.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Expanding)
        self._image_label.setMinimumSize(QSize(100, 100))
        self._image_label.setFrameShape(QLabel.StyledPanel)
        self._image_label.setScaledContents(False)
        layout.addWidget(self._image_label)

        self._info_label = QLabel()
        self._info_label.setAlignment(Qt.AlignCenter)
        self._info_label.setWordWrap(True)
        self._info_label.setEnabled(False)
        layout.addWidget(self._info_label)

    def show_file(self, file_path: str):
        if not file_path:
            self.clear()
            return

        self._current_path = file_path
        p = Path(file_path)

        if not p.exists():
            self._pixmap = None
            self._image_label.setText(tr("preview-file-not-found"))
            self._info_label.setText("")
            return

        if p.suffix.lower() not in self.SUPPORTED_EXTENSIONS:
            self._pixmap = None
            self._image_label.setText(tr("preview-not-available"))
            self._info_label.setText(p.name)
            return

        self._pixmap = QPixmap(file_path)
        if self._pixmap.isNull():
            self._pixmap = None
            self._image_label.setText(tr("preview-cannot-load"))
            self._info_label.setText(p.name)
            return

        self._rescale()
        size = p.stat().st_size
        self._info_label.setText(
            f"{p.name}\n{self._pixmap.width()}x{self._pixmap.height()} | {_format_size(size)}"
        )

    def clear(self):
        self._current_path = ""
        self._pixmap = None
        self._image_label.clear()
        self._image_label.setText(tr("preview-no-preview"))
        self._info_label.setText("")

    def _rescale(self):
        if self._pixmap and not self._pixmap.isNull():
            label_size = self._image_label.size()
            scaled = self._pixmap.scaled(
                label_size, Qt.KeepAspectRatio, Qt.SmoothTransformation
            )
            self._image_label.setPixmap(scaled)

    def resizeEvent(self, event):
        super().resizeEvent(event)
        self._rescale()


class PreviewPanel(QWidget):
    """Image preview panel supporting single and side-by-side comparison modes."""

    SUPPORTED_EXTENSIONS = _ImageSlot.SUPPORTED_EXTENSIONS

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setMinimumWidth(200)
        self._current_path = ""
        self._setup_ui()

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(4, 4, 4, 4)
        layout.setSpacing(4)

        self._title = QLabel(tr("preview-title"))
        font = self._title.font()
        font.setBold(True)
        self._title.setFont(font)
        self._title.setAlignment(Qt.AlignCenter)
        layout.addWidget(self._title)

        # Stacked widget: page 0 = single, page 1 = comparison
        self._stack = QStackedWidget()
        layout.addWidget(self._stack)

        # Single preview mode
        self._single_slot = _ImageSlot()
        self._stack.addWidget(self._single_slot)

        # Side-by-side comparison mode
        comparison_widget = QWidget()
        comparison_layout = QVBoxLayout(comparison_widget)
        comparison_layout.setContentsMargins(0, 0, 0, 0)

        splitter = QSplitter(Qt.Horizontal)
        self._left_slot = _ImageSlot()
        self._right_slot = _ImageSlot()
        splitter.addWidget(self._left_slot)
        splitter.addWidget(self._right_slot)
        splitter.setStretchFactor(0, 1)
        splitter.setStretchFactor(1, 1)
        comparison_layout.addWidget(splitter)

        self._stack.addWidget(comparison_widget)

    def show_preview(self, file_path: str):
        """Show a single file preview."""
        if not file_path or file_path == self._current_path:
            return
        self._current_path = file_path
        self._stack.setCurrentIndex(0)
        self._title.setText(tr("preview-title"))
        self._single_slot.show_file(file_path)

    def show_comparison(self, left_path: str, right_path: str):
        """Show two files side by side for comparison."""
        self._current_path = ""
        self._stack.setCurrentIndex(1)
        self._title.setText("Comparison")
        self.setMinimumWidth(400)
        self._left_slot.show_file(left_path)
        self._right_slot.show_file(right_path)

    def clear_preview(self):
        self._current_path = ""
        self._single_slot.clear()
        self._left_slot.clear()
        self._right_slot.clear()
        self._stack.setCurrentIndex(0)
        self._title.setText(tr("preview-title"))

    def resizeEvent(self, event):
        super().resizeEvent(event)
        # Trigger rescale on the visible slot(s)
        if self._current_path:
            path = self._current_path
            self._current_path = ""
            self.show_preview(path)

    @staticmethod
    def _format_size(size_bytes: int) -> str:
        return _format_size(size_bytes)


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
