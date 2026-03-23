import os

from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QLabel, QListWidget,
    QPushButton, QFileDialog, QTextEdit, QStackedWidget,
    QTreeWidget, QTreeWidgetItem, QHeaderView
)
from PySide6.QtCore import Signal, Qt

from .models import AppSettings


class BottomPanel(QWidget):
    """Bottom panel showing directories or error messages.

    Included directories have a "Ref" checkbox — when checked, that
    directory is a reference directory: its files are kept as references
    and never selected for deletion in grouped tools (duplicates, similar
    images/videos/music).
    """
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

        # ── Included directories (with Ref checkbox) ──
        inc_widget = QWidget()
        inc_layout = QVBoxLayout(inc_widget)
        inc_layout.setContentsMargins(0, 0, 0, 0)
        inc_layout.addWidget(QLabel("Included Directories:"))

        self._inc_tree = QTreeWidget()
        self._inc_tree.setMaximumHeight(120)
        self._inc_tree.setHeaderLabels(["Ref", "Path"])
        self._inc_tree.setColumnCount(2)
        header = self._inc_tree.header()
        header.setSectionResizeMode(0, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(1, QHeaderView.Stretch)
        self._inc_tree.setRootIsDecorated(False)
        self._inc_tree.itemChanged.connect(self._on_ref_toggled)

        self._populate_included()
        inc_layout.addWidget(self._inc_tree)

        inc_btns = QHBoxLayout()
        add_btn = QPushButton("+")
        add_btn.setFixedWidth(30)
        add_btn.setToolTip("Add included directory")
        add_btn.clicked.connect(self._add_included)
        inc_btns.addWidget(add_btn)
        rem_btn = QPushButton("-")
        rem_btn.setFixedWidth(30)
        rem_btn.setToolTip("Remove selected directory")
        rem_btn.clicked.connect(self._remove_included)
        inc_btns.addWidget(rem_btn)
        inc_btns.addStretch()
        inc_layout.addLayout(inc_btns)
        dir_layout.addWidget(inc_widget)

        # ── Excluded directories ──
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
        add_exc.setToolTip("Add excluded directory")
        add_exc.clicked.connect(self._add_excluded)
        exc_btns.addWidget(add_exc)
        rem_exc = QPushButton("-")
        rem_exc.setFixedWidth(30)
        rem_exc.setToolTip("Remove selected directory")
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

    # ── Included directory helpers ────────────────────────────

    def _populate_included(self):
        """Rebuild the included paths tree from settings."""
        self._inc_tree.blockSignals(True)
        self._inc_tree.clear()
        for path in self._settings.included_paths:
            item = QTreeWidgetItem()
            item.setFlags(item.flags() | Qt.ItemIsUserCheckable)
            is_ref = path in self._settings.reference_paths
            item.setCheckState(0, Qt.Checked if is_ref else Qt.Unchecked)
            item.setText(1, path)
            item.setToolTip(0, "Check to mark as reference directory.\n"
                               "Files in reference directories are never selected for deletion.")
            self._inc_tree.addTopLevelItem(item)
        self._inc_tree.blockSignals(False)

    def _on_ref_toggled(self, item, column):
        """Handle Ref checkbox toggle."""
        if column != 0:
            return
        path = item.text(1)
        if item.checkState(0) == Qt.Checked:
            self._settings.reference_paths.add(path)
        else:
            self._settings.reference_paths.discard(path)
        self.directories_changed.emit()

    def _add_included(self):
        path = QFileDialog.getExistingDirectory(self, "Select Directory to Include")
        if path and path not in self._settings.included_paths:
            self._settings.included_paths.append(path)
            self._populate_included()
            self.directories_changed.emit()

    def _remove_included(self):
        items = self._inc_tree.selectedItems()
        if not items:
            # Try current item
            item = self._inc_tree.currentItem()
            if item:
                items = [item]
        for item in items:
            path = item.text(1)
            if path in self._settings.included_paths:
                self._settings.included_paths.remove(path)
            self._settings.reference_paths.discard(path)
        self._populate_included()
        self.directories_changed.emit()

    # ── Excluded directory helpers ────────────────────────────

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

    # ── Public API ────────────────────────────────────────────

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

    def refresh_lists(self):
        self._populate_included()
        self._exc_list.clear()
        for path in self._settings.excluded_paths:
            self._exc_list.addItem(path)

    def dragEnterEvent(self, event):
        if event.mimeData().hasUrls():
            event.acceptProposedAction()

    def dropEvent(self, event):
        for url in event.mimeData().urls():
            path = url.toLocalFile()
            if path and os.path.isdir(path):
                if path not in self._settings.included_paths:
                    self._settings.included_paths.append(path)
                    self._populate_included()
                    self.directories_changed.emit()
