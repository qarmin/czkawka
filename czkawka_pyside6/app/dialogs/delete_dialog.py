from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox,
    QCheckBox, QMessageBox
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QFont


class DeleteDialog(QDialog):
    """Confirmation dialog for deleting files."""

    def __init__(self, count: int, move_to_trash: bool = True, parent=None):
        super().__init__(parent)
        self.setWindowTitle("Delete Files")
        self.setMinimumWidth(400)
        self._move_to_trash = move_to_trash

        layout = QVBoxLayout(self)

        # Warning icon from system theme
        icon_label = QLabel()
        icon = self.style().standardIcon(self.style().SP_MessageBoxWarning)
        icon_label.setPixmap(icon.pixmap(48, 48))
        icon_label.setAlignment(Qt.AlignCenter)
        layout.addWidget(icon_label)

        msg = QLabel(f"Are you sure you want to delete {count} selected file(s)?")
        msg_font = QFont()
        msg_font.setPointSize(11)
        msg.setFont(msg_font)
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
        buttons.accepted.connect(self.accept)
        buttons.rejected.connect(self.reject)
        layout.addWidget(buttons)

    @property
    def move_to_trash(self) -> bool:
        return self._trash_cb.isChecked()
