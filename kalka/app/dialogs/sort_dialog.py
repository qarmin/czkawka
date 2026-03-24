from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QPushButton, QComboBox,
    QCheckBox, QDialogButtonBox, QFormLayout
)
from PySide6.QtCore import Signal

from ..localizer import tr


class SortDialog(QDialog):
    """Dialog for sorting results."""
    sort_requested = Signal(int, bool)  # column_index, ascending

    def __init__(self, columns: list[str], parent=None):
        super().__init__(parent)
        self.setWindowTitle(tr("sort-dialog-title"))
        self.setMinimumWidth(300)

        layout = QFormLayout(self)

        self._column = QComboBox()
        self._column.addItems(columns)
        layout.addRow(tr("sort-by"), self._column)

        self._ascending = QCheckBox(tr("sort-ascending"))
        self._ascending.setChecked(True)
        layout.addRow(self._ascending)

        buttons = QDialogButtonBox(
            QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        )
        buttons.accepted.connect(self._on_sort)
        buttons.rejected.connect(self.reject)
        layout.addRow(buttons)

    def _on_sort(self):
        self.sort_requested.emit(
            self._column.currentIndex(),
            self._ascending.isChecked()
        )
        self.accept()
