from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox,
    QCheckBox, QHBoxLayout
)
from PySide6.QtCore import Qt


class DeleteDialog(QDialog):
    """Confirmation dialog for deleting files."""

    def __init__(self, count: int, move_to_trash: bool = True, parent=None):
        super().__init__(parent)
        self.setWindowTitle("Delete Files")
        self.setMinimumWidth(400)
        self._move_to_trash = move_to_trash

        layout = QVBoxLayout(self)

        # Warning
        icon_label = QLabel()
        icon_label.setStyleSheet("font-size: 36px;")
        icon_label.setText("Warning")
        icon_label.setAlignment(Qt.AlignCenter)
        layout.addWidget(icon_label)

        msg = QLabel(f"Are you sure you want to delete {count} selected file(s)?")
        msg.setStyleSheet("font-size: 14px; padding: 10px;")
        msg.setAlignment(Qt.AlignCenter)
        msg.setWordWrap(True)
        layout.addWidget(msg)

        # Move to trash checkbox
        self._trash_cb = QCheckBox("Move to trash instead of permanent delete")
        self._trash_cb.setChecked(move_to_trash)
        layout.addWidget(self._trash_cb)

        # Buttons
        buttons = QDialogButtonBox(
            QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        )
        buttons.button(QDialogButtonBox.Ok).setText("Delete")
        buttons.button(QDialogButtonBox.Ok).setStyleSheet(
            "background-color: #8a2222; color: white; padding: 6px 20px;"
        )
        buttons.accepted.connect(self.accept)
        buttons.rejected.connect(self.reject)
        layout.addWidget(buttons)

    @property
    def move_to_trash(self) -> bool:
        return self._trash_cb.isChecked()
