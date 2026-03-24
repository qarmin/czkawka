import subprocess
from pathlib import Path

from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QLabel, QSizePolicy, QPlainTextEdit, QScrollArea
)
from PySide6.QtCore import Qt, QSize
from PySide6.QtGui import QPixmap

from .localizer import tr


# File extension sets for different preview types
IMAGE_EXTENSIONS = {
    ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp",
    ".tiff", ".tif", ".ico",
}

TEXT_EXTENSIONS = {
    ".txt", ".md", ".csv", ".json", ".xml", ".yaml", ".yml",
    ".toml", ".ini", ".cfg", ".conf", ".log", ".sh", ".bash",
    ".py", ".rs", ".js", ".ts", ".html", ".css", ".c", ".cpp",
    ".h", ".hpp", ".java", ".go", ".rb", ".php", ".sql",
}

VIDEO_EXTENSIONS = {
    ".mp4", ".mkv", ".avi", ".mov", ".wmv", ".flv", ".webm",
    ".m4v", ".mpg", ".mpeg", ".3gp", ".ogv", ".ts",
}

PDF_EXTENSIONS = {".pdf"}

MAX_TEXT_PREVIEW_BYTES = 64 * 1024  # 64 KB


class PreviewPanel(QWidget):
    """File preview panel supporting images, text, PDF, and video thumbnails."""

    SUPPORTED_EXTENSIONS = IMAGE_EXTENSIONS | TEXT_EXTENSIONS | VIDEO_EXTENSIONS | PDF_EXTENSIONS

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setMinimumWidth(200)
        self.setMaximumWidth(400)
        self._current_path = ""
        self._setup_ui()

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(4, 4, 4, 4)

        self._title = QLabel(tr("preview-title"))
        font = self._title.font()
        font.setBold(True)
        self._title.setFont(font)
        self._title.setAlignment(Qt.AlignCenter)
        layout.addWidget(self._title)

        # Image preview label
        self._image_label = QLabel()
        self._image_label.setAlignment(Qt.AlignCenter)
        self._image_label.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Expanding)
        self._image_label.setMinimumSize(QSize(180, 180))
        self._image_label.setFrameShape(QLabel.StyledPanel)
        self._image_label.setScaledContents(False)
        layout.addWidget(self._image_label)

        # Text preview area (hidden by default)
        self._text_edit = QPlainTextEdit()
        self._text_edit.setReadOnly(True)
        self._text_edit.setLineWrapMode(QPlainTextEdit.WidgetWidth)
        self._text_edit.setVisible(False)
        layout.addWidget(self._text_edit)

        # Info label
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
            self._show_image_mode()
            self._image_label.setText(tr("preview-file-not-found"))
            self._info_label.setText("")
            return

        ext = p.suffix.lower()

        if ext in IMAGE_EXTENSIONS:
            self._preview_image(p, file_path)
        elif ext in TEXT_EXTENSIONS:
            self._preview_text(p)
        elif ext in VIDEO_EXTENSIONS:
            self._preview_video(p, file_path)
        elif ext in PDF_EXTENSIONS:
            self._preview_pdf(p, file_path)
        else:
            self._show_image_mode()
            self._image_label.setText(tr("preview-not-available"))
            self._info_label.setText(p.name)

    def _preview_image(self, p: Path, file_path: str):
        self._show_image_mode()
        pixmap = QPixmap(file_path)
        if pixmap.isNull():
            self._image_label.setText(tr("preview-cannot-load"))
            self._info_label.setText(p.name)
            return

        label_size = self._image_label.size()
        scaled = pixmap.scaled(
            label_size, Qt.KeepAspectRatio, Qt.SmoothTransformation
        )
        self._image_label.setPixmap(scaled)

        size = p.stat().st_size
        self._info_label.setText(
            f"{p.name}\n{pixmap.width()}x{pixmap.height()} | {_format_size(size)}"
        )

    def _preview_text(self, p: Path):
        self._show_text_mode()
        try:
            size = p.stat().st_size
            with open(p, "r", errors="replace") as f:
                content = f.read(MAX_TEXT_PREVIEW_BYTES)
            if size > MAX_TEXT_PREVIEW_BYTES:
                content += f"\n\n... (truncated, {_format_size(size)} total)"
            self._text_edit.setPlainText(content)
            self._info_label.setText(f"{p.name} | {_format_size(size)}")
        except OSError:
            self._text_edit.setPlainText("Error reading file.")
            self._info_label.setText(p.name)

    def _preview_video(self, p: Path, file_path: str):
        """Extract a thumbnail from the video using ffmpeg."""
        self._show_image_mode()
        try:
            result = subprocess.run(
                [
                    "ffmpeg", "-y", "-i", file_path,
                    "-ss", "00:00:03", "-frames:v", "1",
                    "-f", "image2pipe", "-vcodec", "png", "-"
                ],
                capture_output=True, timeout=10
            )
            if result.returncode == 0 and result.stdout:
                pixmap = QPixmap()
                pixmap.loadFromData(result.stdout)
                if not pixmap.isNull():
                    label_size = self._image_label.size()
                    scaled = pixmap.scaled(
                        label_size, Qt.KeepAspectRatio, Qt.SmoothTransformation
                    )
                    self._image_label.setPixmap(scaled)
                    size = p.stat().st_size
                    self._info_label.setText(
                        f"{p.name}\n{pixmap.width()}x{pixmap.height()} | {_format_size(size)}"
                    )
                    return
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass

        self._image_label.setText("Video preview\n(ffmpeg not available)")
        self._info_label.setText(f"{p.name} | {_format_size(p.stat().st_size)}")

    def _preview_pdf(self, p: Path, file_path: str):
        """Try to preview first page of PDF using QPdfDocument or fallback."""
        self._show_image_mode()
        try:
            from PySide6.QtPdf import QPdfDocument
            doc = QPdfDocument(self)
            doc.load(file_path)
            if doc.pageCount() > 0:
                image = doc.render(0, QSize(380, 500))
                pixmap = QPixmap.fromImage(image)
                if not pixmap.isNull():
                    label_size = self._image_label.size()
                    scaled = pixmap.scaled(
                        label_size, Qt.KeepAspectRatio, Qt.SmoothTransformation
                    )
                    self._image_label.setPixmap(scaled)
                    size = p.stat().st_size
                    self._info_label.setText(
                        f"{p.name}\n{doc.pageCount()} pages | {_format_size(size)}"
                    )
                    doc.close()
                    return
            doc.close()
        except (ImportError, Exception):
            pass

        # Fallback: show file info
        self._image_label.setText("PDF preview\n(PySide6.QtPdf not available)")
        self._info_label.setText(f"{p.name} | {_format_size(p.stat().st_size)}")

    def _show_image_mode(self):
        self._image_label.setVisible(True)
        self._text_edit.setVisible(False)

    def _show_text_mode(self):
        self._image_label.setVisible(False)
        self._text_edit.setVisible(True)

    def clear_preview(self):
        self._current_path = ""
        self._image_label.clear()
        self._image_label.setText(tr("preview-no-preview"))
        self._text_edit.clear()
        self._text_edit.setVisible(False)
        self._image_label.setVisible(True)
        self._info_label.setText("")

    def resizeEvent(self, event):
        super().resizeEvent(event)
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
