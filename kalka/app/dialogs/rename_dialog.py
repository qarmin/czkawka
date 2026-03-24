from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox
)

from ..localizer import tr


class RenameDialog(QDialog):
    """Confirmation dialog for renaming files (fix extensions or bad names)."""

    def __init__(self, count: int, rename_type: str = "extensions", parent=None):
        super().__init__(parent)
        self.setWindowTitle(f"Fix {rename_type.title()}")
        self.setMinimumWidth(400)

        layout = QVBoxLayout(self)

        if rename_type == "extensions":
            msg = tr("rename-dialog-ext-message", count=count)
        else:
            msg = tr("rename-dialog-names-message", count=count)

        label = QLabel(msg)
        label.setWordWrap(True)
        label.setContentsMargins(10, 10, 10, 10)
        layout.addWidget(label)

        buttons = QDialogButtonBox(
            QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        )
        buttons.button(QDialogButtonBox.Ok).setText(tr("rename-dialog-confirm"))
        buttons.accepted.connect(self.accept)
        buttons.rejected.connect(self.reject)
        layout.addWidget(buttons)
