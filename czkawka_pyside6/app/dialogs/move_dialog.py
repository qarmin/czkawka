from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox,
    QCheckBox, QLineEdit, QHBoxLayout, QPushButton,
    QFileDialog, QFormLayout
)
from PySide6.QtCore import Qt


class MoveDialog(QDialog):
    """Dialog for moving/copying files to a destination."""

    def __init__(self, count: int, parent=None):
        super().__init__(parent)
        self.setWindowTitle("Move/Copy Files")
        self.setMinimumWidth(500)

        layout = QVBoxLayout(self)

        msg = QLabel(f"Move or copy {count} selected file(s) to:")
        layout.addWidget(msg)

        # Destination path
        dest_layout = QHBoxLayout()
        self._dest_edit = QLineEdit()
        self._dest_edit.setPlaceholderText("Select destination folder...")
        dest_layout.addWidget(self._dest_edit)

        browse_btn = QPushButton("Browse")
        browse_btn.clicked.connect(self._browse)
        dest_layout.addWidget(browse_btn)
        layout.addLayout(dest_layout)

        # Options
        self._preserve_structure = QCheckBox("Preserve folder structure")
        layout.addWidget(self._preserve_structure)

        self._copy_mode = QCheckBox("Copy instead of move")
        layout.addWidget(self._copy_mode)

        self._dry_run = QCheckBox("Dry run (preview only, no files will be moved)")
        layout.addWidget(self._dry_run)

        # Buttons
        buttons = QDialogButtonBox(
            QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        )
        buttons.button(QDialogButtonBox.Ok).setText("Move")
        buttons.accepted.connect(self.accept)
        buttons.rejected.connect(self.reject)
        layout.addWidget(buttons)

    def _browse(self):
        path = QFileDialog.getExistingDirectory(self, "Select Destination")
        if path:
            self._dest_edit.setText(path)

    @property
    def destination(self) -> str:
        return self._dest_edit.text()

    @property
    def preserve_structure(self) -> bool:
        return self._preserve_structure.isChecked()

    @property
    def copy_mode(self) -> bool:
        return self._copy_mode.isChecked()

    @property
    def dry_run(self) -> bool:
        return self._dry_run.isChecked()
