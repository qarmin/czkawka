from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QLabel, QCheckBox,
    QLineEdit, QSpinBox, QGroupBox, QFormLayout, QScrollArea,
    QPushButton, QListWidget, QFileDialog, QDoubleSpinBox,
    QTabWidget, QSizePolicy
)
from PySide6.QtCore import Qt, Signal

from .models import AppSettings


class SettingsPanel(QWidget):
    """Global application settings panel."""
    settings_changed = Signal()
    close_requested = Signal()

    def __init__(self, settings: AppSettings, parent=None):
        super().__init__(parent)
        self._settings = settings
        self._setup_ui()

    def _setup_ui(self):
        main_layout = QVBoxLayout(self)

        # Header
        header = QHBoxLayout()
        title = QLabel("Settings")
        title.setStyleSheet("font-weight: bold; font-size: 16px; padding: 4px;")
        header.addWidget(title)
        header.addStretch()
        close_btn = QPushButton("Close")
        close_btn.clicked.connect(self.close_requested.emit)
        header.addWidget(close_btn)
        main_layout.addLayout(header)

        # Tabs
        tabs = QTabWidget()

        # General tab
        tabs.addTab(self._create_general_tab(), "General")
        # Directories tab
        tabs.addTab(self._create_directories_tab(), "Directories")
        # Filters tab
        tabs.addTab(self._create_filters_tab(), "Filters")
        # Preview tab
        tabs.addTab(self._create_preview_tab(), "Preview")

        main_layout.addWidget(tabs)

    def _create_general_tab(self) -> QWidget:
        scroll = QScrollArea()
        scroll.setWidgetResizable(True)
        widget = QWidget()
        layout = QFormLayout(widget)

        # CLI path
        cli_layout = QHBoxLayout()
        self._cli_path = QLineEdit(self._settings.czkawka_cli_path)
        self._cli_path.textChanged.connect(
            lambda t: setattr(self._settings, 'czkawka_cli_path', t)
        )
        cli_layout.addWidget(self._cli_path)
        browse_btn = QPushButton("Browse")
        browse_btn.clicked.connect(self._browse_cli)
        cli_layout.addWidget(browse_btn)
        layout.addRow("czkawka_cli Path:", cli_layout)

        # Thread number
        self._threads = QSpinBox()
        self._threads.setRange(0, 64)
        self._threads.setValue(self._settings.thread_number)
        self._threads.setSpecialValueText("Auto (all cores)")
        self._threads.valueChanged.connect(
            lambda v: setattr(self._settings, 'thread_number', v)
        )
        layout.addRow("Thread Count:", self._threads)

        # Recursive search
        recursive = QCheckBox("Recursive search")
        recursive.setChecked(self._settings.recursive_search)
        recursive.toggled.connect(
            lambda v: setattr(self._settings, 'recursive_search', v)
        )
        layout.addRow(recursive)

        # Use cache
        cache = QCheckBox("Use cache for faster rescans")
        cache.setChecked(self._settings.use_cache)
        cache.toggled.connect(
            lambda v: setattr(self._settings, 'use_cache', v)
        )
        layout.addRow(cache)

        # Move to trash
        trash = QCheckBox("Move to trash instead of permanent delete")
        trash.setChecked(self._settings.move_to_trash)
        trash.toggled.connect(
            lambda v: setattr(self._settings, 'move_to_trash', v)
        )
        layout.addRow(trash)

        # Hide hard links
        hardlinks = QCheckBox("Hide hard links")
        hardlinks.setChecked(self._settings.hide_hard_links)
        hardlinks.toggled.connect(
            lambda v: setattr(self._settings, 'hide_hard_links', v)
        )
        layout.addRow(hardlinks)

        # Save as JSON
        save_json = QCheckBox("Save results as JSON (instead of text)")
        save_json.setChecked(self._settings.save_as_json)
        save_json.toggled.connect(
            lambda v: setattr(self._settings, 'save_as_json', v)
        )
        layout.addRow(save_json)

        scroll.setWidget(widget)
        return scroll

    def _create_directories_tab(self) -> QWidget:
        widget = QWidget()
        layout = QVBoxLayout(widget)

        # Included paths
        inc_group = QGroupBox("Included Directories")
        inc_layout = QVBoxLayout(inc_group)

        self._inc_list = QListWidget()
        for path in self._settings.included_paths:
            self._inc_list.addItem(path)
        inc_layout.addWidget(self._inc_list)

        inc_btns = QHBoxLayout()
        add_inc = QPushButton("Add")
        add_inc.clicked.connect(self._add_included)
        inc_btns.addWidget(add_inc)
        rem_inc = QPushButton("Remove")
        rem_inc.clicked.connect(self._remove_included)
        inc_btns.addWidget(rem_inc)
        inc_btns.addStretch()
        inc_layout.addLayout(inc_btns)
        layout.addWidget(inc_group)

        # Excluded paths
        exc_group = QGroupBox("Excluded Directories")
        exc_layout = QVBoxLayout(exc_group)

        self._exc_list = QListWidget()
        for path in self._settings.excluded_paths:
            self._exc_list.addItem(path)
        exc_layout.addWidget(self._exc_list)

        exc_btns = QHBoxLayout()
        add_exc = QPushButton("Add")
        add_exc.clicked.connect(self._add_excluded)
        exc_btns.addWidget(add_exc)
        rem_exc = QPushButton("Remove")
        rem_exc.clicked.connect(self._remove_excluded)
        exc_btns.addWidget(rem_exc)
        exc_btns.addStretch()
        exc_layout.addLayout(exc_btns)
        layout.addWidget(exc_group)

        return widget

    def _create_filters_tab(self) -> QWidget:
        widget = QWidget()
        layout = QFormLayout(widget)

        # Excluded items
        self._excluded_items = QLineEdit(self._settings.excluded_items)
        self._excluded_items.setPlaceholderText("Wildcard patterns, comma-separated (e.g. *.tmp,cache_*)")
        self._excluded_items.textChanged.connect(
            lambda t: setattr(self._settings, 'excluded_items', t)
        )
        layout.addRow("Excluded Items:", self._excluded_items)

        # Allowed extensions
        self._allowed_ext = QLineEdit(self._settings.allowed_extensions)
        self._allowed_ext.setPlaceholderText("e.g. jpg,png,gif")
        self._allowed_ext.textChanged.connect(
            lambda t: setattr(self._settings, 'allowed_extensions', t)
        )
        layout.addRow("Allowed Extensions:", self._allowed_ext)

        # Excluded extensions
        self._excluded_ext = QLineEdit(self._settings.excluded_extensions)
        self._excluded_ext.setPlaceholderText("e.g. log,tmp")
        self._excluded_ext.textChanged.connect(
            lambda t: setattr(self._settings, 'excluded_extensions', t)
        )
        layout.addRow("Excluded Extensions:", self._excluded_ext)

        # Min file size
        self._min_size = QLineEdit(self._settings.minimum_file_size)
        self._min_size.setPlaceholderText("In bytes (e.g. 1024)")
        self._min_size.textChanged.connect(
            lambda t: setattr(self._settings, 'minimum_file_size', t)
        )
        layout.addRow("Minimum File Size:", self._min_size)

        # Max file size
        self._max_size = QLineEdit(self._settings.maximum_file_size)
        self._max_size.setPlaceholderText("In bytes (leave empty for no limit)")
        self._max_size.textChanged.connect(
            lambda t: setattr(self._settings, 'maximum_file_size', t)
        )
        layout.addRow("Maximum File Size:", self._max_size)

        return widget

    def _create_preview_tab(self) -> QWidget:
        widget = QWidget()
        layout = QVBoxLayout(widget)

        preview = QCheckBox("Show image preview")
        preview.setChecked(self._settings.show_image_preview)
        preview.toggled.connect(
            lambda v: setattr(self._settings, 'show_image_preview', v)
        )
        layout.addWidget(preview)

        layout.addStretch()
        return widget

    def _browse_cli(self):
        path, _ = QFileDialog.getOpenFileName(
            self, "Select czkawka_cli binary", "",
            "Executables (*);;All Files (*)"
        )
        if path:
            self._cli_path.setText(path)

    def _add_included(self):
        path = QFileDialog.getExistingDirectory(self, "Select Directory to Include")
        if path and path not in self._settings.included_paths:
            self._settings.included_paths.append(path)
            self._inc_list.addItem(path)
            self.settings_changed.emit()

    def _remove_included(self):
        row = self._inc_list.currentRow()
        if row >= 0:
            self._inc_list.takeItem(row)
            self._settings.included_paths.pop(row)
            self.settings_changed.emit()

    def _add_excluded(self):
        path = QFileDialog.getExistingDirectory(self, "Select Directory to Exclude")
        if path and path not in self._settings.excluded_paths:
            self._settings.excluded_paths.append(path)
            self._exc_list.addItem(path)
            self.settings_changed.emit()

    def _remove_excluded(self):
        row = self._exc_list.currentRow()
        if row >= 0:
            self._exc_list.takeItem(row)
            self._settings.excluded_paths.pop(row)
            self.settings_changed.emit()
