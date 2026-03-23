from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QPushButton, QDialogButtonBox
)
from PySide6.QtCore import Signal

from ..models import SelectMode


class SelectDialog(QDialog):
    """Dialog for selecting/deselecting results."""
    mode_selected = Signal(object)  # SelectMode

    MODES = [
        (SelectMode.SELECT_ALL, "Select All"),
        (SelectMode.UNSELECT_ALL, "Unselect All"),
        (SelectMode.INVERT_SELECTION, "Invert Selection"),
        (SelectMode.SELECT_BIGGEST_SIZE, "Select Biggest (by Size)"),
        (SelectMode.SELECT_SMALLEST_SIZE, "Select Smallest (by Size)"),
        (SelectMode.SELECT_NEWEST, "Select Newest"),
        (SelectMode.SELECT_OLDEST, "Select Oldest"),
        (SelectMode.SELECT_SHORTEST_PATH, "Select Shortest Path"),
        (SelectMode.SELECT_LONGEST_PATH, "Select Longest Path"),
    ]

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setWindowTitle("Select Results")
        self.setMinimumWidth(300)

        layout = QVBoxLayout(self)

        label = QLabel("Choose selection mode:")
        label.setStyleSheet("font-size: 13px; padding: 4px;")
        layout.addWidget(label)

        for mode, name in self.MODES:
            btn = QPushButton(name)
            btn.clicked.connect(lambda checked, m=mode: self._select(m))
            layout.addWidget(btn)

        # Cancel
        cancel = QPushButton("Cancel")
        cancel.clicked.connect(self.reject)
        layout.addWidget(cancel)

    def _select(self, mode: SelectMode):
        self.mode_selected.emit(mode)
        self.accept()
