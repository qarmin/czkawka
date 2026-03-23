from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox
)


class RenameDialog(QDialog):
    """Confirmation dialog for renaming files (fix extensions or bad names)."""

    def __init__(self, count: int, rename_type: str = "extensions", parent=None):
        super().__init__(parent)
        self.setWindowTitle(f"Fix {rename_type.title()}")
        self.setMinimumWidth(400)

        layout = QVBoxLayout(self)

        if rename_type == "extensions":
            msg = f"Fix extensions for {count} selected file(s)?\n\n" \
                  "Files will be renamed to use their proper extensions."
        else:
            msg = f"Fix names for {count} selected file(s)?\n\n" \
                  "Files with problematic names will be renamed."

        label = QLabel(msg)
        label.setWordWrap(True)
        label.setStyleSheet("font-size: 13px; padding: 10px;")
        layout.addWidget(label)

        buttons = QDialogButtonBox(
            QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        )
        buttons.button(QDialogButtonBox.Ok).setText("Rename")
        buttons.accepted.connect(self.accept)
        buttons.rejected.connect(self.reject)
        layout.addWidget(buttons)
