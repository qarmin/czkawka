from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox,
    QCheckBox, QStyle
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QFont

from ..localizer import tr


class DeleteDialog(QDialog):
    """Confirmation dialog for deleting files."""

    def __init__(self, count: int, move_to_trash: bool = True, parent=None):
        super().__init__(parent)
        self.setWindowTitle(tr("delete-dialog-title"))
        self.setMinimumWidth(400)
        self._move_to_trash = move_to_trash

        layout = QVBoxLayout(self)

        # Warning icon from system theme
        icon_label = QLabel()
        icon = self.style().standardIcon(QStyle.StandardPixmap.SP_MessageBoxWarning)
        icon_label.setPixmap(icon.pixmap(48, 48))
        icon_label.setAlignment(Qt.AlignCenter)
        layout.addWidget(icon_label)

        msg = QLabel(tr("delete-dialog-message", count=count))
        msg_font = QFont()
        msg_font.setPointSize(11)
        msg.setFont(msg_font)
        msg.setAlignment(Qt.AlignCenter)
        msg.setWordWrap(True)
        layout.addWidget(msg)

        # Move to trash checkbox
        self._trash_cb = QCheckBox(tr("delete-dialog-trash"))
        self._trash_cb.setChecked(move_to_trash)
        layout.addWidget(self._trash_cb)

        # Dry run checkbox
        self._dry_run_cb = QCheckBox(tr("delete-dialog-dry-run"))
        layout.addWidget(self._dry_run_cb)

        # Buttons
        buttons = QDialogButtonBox(
            QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        )
        buttons.button(QDialogButtonBox.Ok).setText(tr("delete-dialog-confirm"))
        buttons.accepted.connect(self.accept)
        buttons.rejected.connect(self.reject)
        layout.addWidget(buttons)

    @property
    def move_to_trash(self) -> bool:
        return self._trash_cb.isChecked()

    @property
    def dry_run(self) -> bool:
        return self._dry_run_cb.isChecked()
