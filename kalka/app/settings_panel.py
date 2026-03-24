from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QLabel, QCheckBox,
    QLineEdit, QSpinBox, QGroupBox, QFormLayout, QScrollArea,
    QPushButton, QListWidget, QFileDialog, QDoubleSpinBox,
    QTabWidget, QSizePolicy
)
from PySide6.QtCore import Qt, Signal

from .models import AppSettings
from .localizer import tr


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
        title = QLabel(tr("settings-title"))
        font = title.font()
        font.setBold(True)
        font.setPointSize(font.pointSize() + 2)
        title.setFont(font)
        header.addWidget(title)
        header.addStretch()
        close_btn = QPushButton(tr("settings-close"))
        close_btn.clicked.connect(self.close_requested.emit)
        header.addWidget(close_btn)
        main_layout.addLayout(header)

        # Tabs
        tabs = QTabWidget()

        # General tab
        tabs.addTab(self._create_general_tab(), tr("settings-tab-general"))
        # Directories tab
        tabs.addTab(self._create_directories_tab(), tr("settings-tab-directories"))
        # Filters tab
        tabs.addTab(self._create_filters_tab(), tr("settings-tab-filters"))
        # Preview tab
        tabs.addTab(self._create_preview_tab(), tr("settings-tab-preview"))

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
        browse_btn = QPushButton(tr("settings-browse"))
        browse_btn.clicked.connect(self._browse_cli)
        cli_layout.addWidget(browse_btn)
        layout.addRow(tr("settings-cli-path"), cli_layout)

        # Thread number
        self._threads = QSpinBox()
        self._threads.setRange(0, 64)
        self._threads.setValue(self._settings.thread_number)
        self._threads.setSpecialValueText(tr("settings-thread-auto"))
        self._threads.valueChanged.connect(
            lambda v: setattr(self._settings, 'thread_number', v)
        )
        layout.addRow(tr("settings-thread-count"), self._threads)

        # Recursive search
        recursive = QCheckBox(tr("settings-recursive"))
        recursive.setChecked(self._settings.recursive_search)
        recursive.toggled.connect(
            lambda v: setattr(self._settings, 'recursive_search', v)
        )
        layout.addRow(recursive)

        # Use cache
        cache = QCheckBox(tr("settings-use-cache"))
        cache.setChecked(self._settings.use_cache)
        cache.toggled.connect(
            lambda v: setattr(self._settings, 'use_cache', v)
        )
        layout.addRow(cache)

        # Move to trash
        trash = QCheckBox(tr("settings-move-to-trash"))
        trash.setChecked(self._settings.move_to_trash)
        trash.toggled.connect(
            lambda v: setattr(self._settings, 'move_to_trash', v)
        )
        layout.addRow(trash)

        # Hide hard links
        hardlinks = QCheckBox(tr("settings-hide-hard-links"))
        hardlinks.setChecked(self._settings.hide_hard_links)
        hardlinks.toggled.connect(
            lambda v: setattr(self._settings, 'hide_hard_links', v)
        )
        layout.addRow(hardlinks)

        # Low priority scanning
        low_priority = QCheckBox("Low priority scanning (nice/ionice)")
        low_priority.setChecked(self._settings.low_priority_scan)
        low_priority.setToolTip("Run scans with idle CPU and I/O priority so they don't slow down other applications")
        low_priority.toggled.connect(
            lambda v: setattr(self._settings, 'low_priority_scan', v)
        )
        layout.addRow(low_priority)

        # Save as JSON
        save_json = QCheckBox(tr("settings-save-as-json"))
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
        inc_group = QGroupBox(tr("settings-included-dirs"))
        inc_layout = QVBoxLayout(inc_group)

        self._inc_list = QListWidget()
        for path in self._settings.included_paths:
            self._inc_list.addItem(path)
        inc_layout.addWidget(self._inc_list)

        inc_btns = QHBoxLayout()
        add_inc = QPushButton(tr("settings-add"))
        add_inc.clicked.connect(self._add_included)
        inc_btns.addWidget(add_inc)
        rem_inc = QPushButton(tr("settings-remove"))
        rem_inc.clicked.connect(self._remove_included)
        inc_btns.addWidget(rem_inc)
        inc_btns.addStretch()
        inc_layout.addLayout(inc_btns)
        layout.addWidget(inc_group)

        # Excluded paths
        exc_group = QGroupBox(tr("settings-excluded-dirs"))
        exc_layout = QVBoxLayout(exc_group)

        self._exc_list = QListWidget()
        for path in self._settings.excluded_paths:
            self._exc_list.addItem(path)
        exc_layout.addWidget(self._exc_list)

        exc_btns = QHBoxLayout()
        add_exc = QPushButton(tr("settings-add"))
        add_exc.clicked.connect(self._add_excluded)
        exc_btns.addWidget(add_exc)
        rem_exc = QPushButton(tr("settings-remove"))
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
        self._excluded_items.setPlaceholderText(tr("settings-excluded-items-hint"))
        self._excluded_items.textChanged.connect(
            lambda t: setattr(self._settings, 'excluded_items', t)
        )
        layout.addRow(tr("settings-excluded-items"), self._excluded_items)

        # Allowed extensions
        self._allowed_ext = QLineEdit(self._settings.allowed_extensions)
        self._allowed_ext.setPlaceholderText(tr("settings-allowed-extensions-hint"))
        self._allowed_ext.textChanged.connect(
            lambda t: setattr(self._settings, 'allowed_extensions', t)
        )
        layout.addRow(tr("settings-allowed-extensions"), self._allowed_ext)

        # Excluded extensions
        self._excluded_ext = QLineEdit(self._settings.excluded_extensions)
        self._excluded_ext.setPlaceholderText(tr("settings-excluded-extensions-hint"))
        self._excluded_ext.textChanged.connect(
            lambda t: setattr(self._settings, 'excluded_extensions', t)
        )
        layout.addRow(tr("settings-excluded-extensions"), self._excluded_ext)

        # Min file size
        self._min_size = QLineEdit(self._settings.minimum_file_size)
        self._min_size.setPlaceholderText(tr("settings-min-file-size-hint"))
        self._min_size.textChanged.connect(
            lambda t: setattr(self._settings, 'minimum_file_size', t)
        )
        layout.addRow(tr("settings-min-file-size"), self._min_size)

        # Max file size
        self._max_size = QLineEdit(self._settings.maximum_file_size)
        self._max_size.setPlaceholderText(tr("settings-max-file-size-hint"))
        self._max_size.textChanged.connect(
            lambda t: setattr(self._settings, 'maximum_file_size', t)
        )
        layout.addRow(tr("settings-max-file-size"), self._max_size)

        return widget

    def _create_preview_tab(self) -> QWidget:
        widget = QWidget()
        layout = QVBoxLayout(widget)

        preview = QCheckBox(tr("settings-show-image-preview"))
        preview.setChecked(self._settings.show_image_preview)
        preview.toggled.connect(
            lambda v: setattr(self._settings, 'show_image_preview', v)
        )
        layout.addWidget(preview)

        layout.addStretch()
        return widget

    def _browse_cli(self):
        path, _ = QFileDialog.getOpenFileName(
            self, tr("settings-select-cli-binary"), "",
            "Executables (*);;All Files (*)"
        )
        if path:
            self._cli_path.setText(path)

    def _add_included(self):
        path = QFileDialog.getExistingDirectory(self, tr("settings-select-dir-include"))
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
        path = QFileDialog.getExistingDirectory(self, tr("settings-select-dir-exclude"))
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
