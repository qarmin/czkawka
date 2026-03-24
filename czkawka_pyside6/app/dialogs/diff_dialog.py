import os
from pathlib import Path

from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QHBoxLayout, QLabel, QDialogButtonBox,
    QFrame, QSizePolicy, QScrollArea, QWidget
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QPixmap, QFont


class DiffDialog(QDialog):
    """Side-by-side comparison of two files from a duplicate group."""

    def __init__(self, entry1, entry2, parent=None):
        super().__init__(parent)
        self.setWindowTitle("File Comparison")
        self.setMinimumSize(700, 500)

        layout = QVBoxLayout(self)

        # Side-by-side layout
        compare_layout = QHBoxLayout()

        compare_layout.addWidget(self._create_file_panel(entry1))

        # Separator
        sep = QFrame()
        sep.setFrameShape(QFrame.VLine)
        sep.setFrameShadow(QFrame.Sunken)
        compare_layout.addWidget(sep)

        compare_layout.addWidget(self._create_file_panel(entry2))

        layout.addLayout(compare_layout)

        # Difference summary
        diff_label = QLabel(self._compute_diff_summary(entry1, entry2))
        diff_label.setAlignment(Qt.AlignCenter)
        diff_label.setWordWrap(True)
        layout.addWidget(diff_label)

        buttons = QDialogButtonBox(QDialogButtonBox.Ok)
        buttons.accepted.connect(self.accept)
        layout.addWidget(buttons)

    def _create_file_panel(self, entry) -> QWidget:
        panel = QWidget()
        layout = QVBoxLayout(panel)

        path = entry.values.get("__full_path", "")
        p = Path(path)

        # File name
        name = QLabel(p.name)
        name_font = QFont()
        name_font.setBold(True)
        name.setFont(name_font)
        name.setAlignment(Qt.AlignCenter)
        name.setWordWrap(True)
        layout.addWidget(name)

        # Image preview if applicable
        if p.suffix.lower() in ('.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp'):
            img_label = QLabel()
            pixmap = QPixmap(path)
            if not pixmap.isNull():
                scaled = pixmap.scaledToWidth(300, Qt.SmoothTransformation)
                img_label.setPixmap(scaled)
                img_label.setAlignment(Qt.AlignCenter)
                layout.addWidget(img_label)

        # File details
        details = [
            f"Path: {p.parent}",
            f"Size: {entry.values.get('Size', 'N/A')}",
            f"Modified: {entry.values.get('Modification Date', 'N/A')}",
        ]
        if entry.values.get("Hash"):
            details.append(f"Hash: {entry.values.get('Hash', '')[:16]}...")

        for d in details:
            lbl = QLabel(d)
            lbl.setWordWrap(True)
            layout.addWidget(lbl)

        layout.addStretch()
        return panel

    def _compute_diff_summary(self, entry1, entry2) -> str:
        diffs = []
        s1 = entry1.values.get("__size_bytes", 0)
        s2 = entry2.values.get("__size_bytes", 0)
        if s1 != s2:
            diffs.append(f"Size differs: {entry1.values.get('Size', '')} vs {entry2.values.get('Size', '')}")

        t1 = entry1.values.get("__modified_date_ts", 0)
        t2 = entry2.values.get("__modified_date_ts", 0)
        if t1 != t2:
            diffs.append(f"Modified date differs: {entry1.values.get('Modification Date', '')} vs {entry2.values.get('Modification Date', '')}")

        p1 = entry1.values.get("__full_path", "")
        p2 = entry2.values.get("__full_path", "")
        if Path(p1).parent != Path(p2).parent:
            diffs.append("Files are in different directories")

        if not diffs:
            return "Files are identical in size and modification date"
        return " | ".join(diffs)
