import os

from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QLabel, QListWidget,
    QPushButton, QFileDialog, QTextEdit, QStackedWidget,
    QSizePolicy
)
from PySide6.QtCore import Signal, Qt

from .models import AppSettings


class BottomPanel(QWidget):
    """Bottom panel showing directories or error messages."""
    directories_changed = Signal()

    def __init__(self, settings: AppSettings, parent=None):
        super().__init__(parent)
        self._settings = settings
        self.setMaximumHeight(200)
        self.setAcceptDrops(True)
        self._setup_ui()

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(4, 2, 4, 2)

        self._stack = QStackedWidget()

        # Page 0: Directories view
        dir_widget = QWidget()
        dir_layout = QHBoxLayout(dir_widget)
        dir_layout.setContentsMargins(0, 0, 0, 0)

        # Included directories
        inc_widget = QWidget()
        inc_layout = QVBoxLayout(inc_widget)
        inc_layout.setContentsMargins(0, 0, 0, 0)
        inc_layout.addWidget(QLabel("Included Directories:"))

        self._inc_list = QListWidget()
        self._inc_list.setMaximumHeight(120)
        for path in self._settings.included_paths:
            self._inc_list.addItem(path)
        inc_layout.addWidget(self._inc_list)

        inc_btns = QHBoxLayout()
        add_btn = QPushButton("+")
        add_btn.setFixedWidth(30)
        add_btn.clicked.connect(self._add_included)
        inc_btns.addWidget(add_btn)
        rem_btn = QPushButton("-")
        rem_btn.setFixedWidth(30)
        rem_btn.clicked.connect(self._remove_included)
        inc_btns.addWidget(rem_btn)
        inc_btns.addStretch()
        inc_layout.addLayout(inc_btns)
        dir_layout.addWidget(inc_widget)

        # Excluded directories
        exc_widget = QWidget()
        exc_layout = QVBoxLayout(exc_widget)
        exc_layout.setContentsMargins(0, 0, 0, 0)
        exc_layout.addWidget(QLabel("Excluded Directories:"))

        self._exc_list = QListWidget()
        self._exc_list.setMaximumHeight(120)
        for path in self._settings.excluded_paths:
            self._exc_list.addItem(path)
        exc_layout.addWidget(self._exc_list)

        exc_btns = QHBoxLayout()
        add_exc = QPushButton("+")
        add_exc.setFixedWidth(30)
        add_exc.clicked.connect(self._add_excluded)
        exc_btns.addWidget(add_exc)
        rem_exc = QPushButton("-")
        rem_exc.setFixedWidth(30)
        rem_exc.clicked.connect(self._remove_excluded)
        exc_btns.addWidget(rem_exc)
        exc_btns.addStretch()
        exc_layout.addLayout(exc_btns)
        dir_layout.addWidget(exc_widget)

        self._stack.addWidget(dir_widget)

        # Page 1: Text errors/info
        self._text_area = QTextEdit()
        self._text_area.setReadOnly(True)
        self._text_area.setMaximumHeight(150)
        self._stack.addWidget(self._text_area)

        layout.addWidget(self._stack)

    def show_directories(self):
        self._stack.setCurrentIndex(0)
        self.setVisible(True)

    def show_text(self):
        self._stack.setCurrentIndex(1)
        self.setVisible(True)

    def hide_panel(self):
        self.setVisible(False)

    def set_text(self, text: str):
        self._text_area.setPlainText(text)

    def append_text(self, text: str):
        self._text_area.append(text)

    def _add_included(self):
        path = QFileDialog.getExistingDirectory(self, "Select Directory to Include")
        if path and path not in self._settings.included_paths:
            self._settings.included_paths.append(path)
            self._inc_list.addItem(path)
            self.directories_changed.emit()

    def _remove_included(self):
        row = self._inc_list.currentRow()
        if row >= 0:
            self._inc_list.takeItem(row)
            self._settings.included_paths.pop(row)
            self.directories_changed.emit()

    def _add_excluded(self):
        path = QFileDialog.getExistingDirectory(self, "Select Directory to Exclude")
        if path and path not in self._settings.excluded_paths:
            self._settings.excluded_paths.append(path)
            self._exc_list.addItem(path)
            self.directories_changed.emit()

    def _remove_excluded(self):
        row = self._exc_list.currentRow()
        if row >= 0:
            self._exc_list.takeItem(row)
            self._settings.excluded_paths.pop(row)
            self.directories_changed.emit()

    def dragEnterEvent(self, event):
        if event.mimeData().hasUrls():
            event.acceptProposedAction()

    def dropEvent(self, event):
        for url in event.mimeData().urls():
            path = url.toLocalFile()
            if path and os.path.isdir(path):
                if path not in self._settings.included_paths:
                    self._settings.included_paths.append(path)
                    self._inc_list.addItem(path)
                    self.directories_changed.emit()

    def refresh_lists(self):
        self._inc_list.clear()
        for path in self._settings.included_paths:
            self._inc_list.addItem(path)
        self._exc_list.clear()
        for path in self._settings.excluded_paths:
            self._exc_list.addItem(path)
