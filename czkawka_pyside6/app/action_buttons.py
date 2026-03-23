from PySide6.QtWidgets import (
    QWidget, QHBoxLayout, QPushButton, QSizePolicy
)
from PySide6.QtCore import Signal, QSize

from .models import ActiveTab, GROUPED_TABS
from .icons import (
    icon_search, icon_stop, icon_select, icon_delete, icon_move,
    icon_save, icon_sort, icon_hardlink, icon_symlink, icon_rename,
    icon_clean, icon_optimize,
)

ICON_SIZE = QSize(18, 18)


class ActionButtons(QWidget):
    """Action buttons bar: Scan, Stop, Select, Delete, Move, Save, Sort, etc."""

    scan_clicked = Signal()
    stop_clicked = Signal()
    select_clicked = Signal()
    delete_clicked = Signal()
    move_clicked = Signal()
    save_clicked = Signal()
    load_clicked = Signal()
    sort_clicked = Signal()
    hardlink_clicked = Signal()
    symlink_clicked = Signal()
    rename_clicked = Signal()
    clean_exif_clicked = Signal()
    optimize_video_clicked = Signal()

    def __init__(self, parent=None):
        super().__init__(parent)
        self._active_tab = ActiveTab.DUPLICATE_FILES
        self._scanning = False
        self._has_results = False
        self._has_selection = False
        self._setup_ui()

    def _setup_ui(self):
        layout = QHBoxLayout(self)
        layout.setContentsMargins(4, 4, 4, 4)
        layout.setSpacing(4)

        # Scan button
        self._scan_btn = QPushButton(icon_search(18), " Scan")
        self._scan_btn.setIconSize(ICON_SIZE)
        self._scan_btn.setMinimumWidth(90)
        self._scan_btn.clicked.connect(self.scan_clicked.emit)
        layout.addWidget(self._scan_btn)

        # Stop button
        self._stop_btn = QPushButton(icon_stop(18), " Stop")
        self._stop_btn.setIconSize(ICON_SIZE)
        self._stop_btn.setMinimumWidth(80)
        self._stop_btn.clicked.connect(self.stop_clicked.emit)
        self._stop_btn.setVisible(False)
        layout.addWidget(self._stop_btn)

        # Spacer
        spacer = QWidget()
        spacer.setFixedWidth(10)
        layout.addWidget(spacer)

        # Select button
        self._select_btn = QPushButton(icon_select(18), " Select")
        self._select_btn.setIconSize(ICON_SIZE)
        self._select_btn.clicked.connect(self.select_clicked.emit)
        layout.addWidget(self._select_btn)

        # Delete button
        self._delete_btn = QPushButton(icon_delete(18), " Delete")
        self._delete_btn.setIconSize(ICON_SIZE)
        self._delete_btn.clicked.connect(self.delete_clicked.emit)
        layout.addWidget(self._delete_btn)

        # Move button
        self._move_btn = QPushButton(icon_move(18), " Move")
        self._move_btn.setIconSize(ICON_SIZE)
        self._move_btn.clicked.connect(self.move_clicked.emit)
        layout.addWidget(self._move_btn)

        # Save button
        self._save_btn = QPushButton(icon_save(18), " Save")
        self._save_btn.setIconSize(ICON_SIZE)
        self._save_btn.clicked.connect(self.save_clicked.emit)
        layout.addWidget(self._save_btn)

        # Load button
        from .icons import icon_dir
        self._load_btn = QPushButton(icon_dir(18), " Load")
        self._load_btn.setIconSize(ICON_SIZE)
        self._load_btn.setToolTip("Load previously saved results")
        self._load_btn.clicked.connect(self.load_clicked.emit)
        layout.addWidget(self._load_btn)

        # Sort button
        self._sort_btn = QPushButton(icon_sort(18), " Sort")
        self._sort_btn.setIconSize(ICON_SIZE)
        self._sort_btn.clicked.connect(self.sort_clicked.emit)
        layout.addWidget(self._sort_btn)

        # Hardlink button (grouped tools only)
        self._hardlink_btn = QPushButton(icon_hardlink(18), " Hardlink")
        self._hardlink_btn.setIconSize(ICON_SIZE)
        self._hardlink_btn.clicked.connect(self.hardlink_clicked.emit)
        layout.addWidget(self._hardlink_btn)

        # Symlink button (grouped tools only)
        self._symlink_btn = QPushButton(icon_symlink(18), " Symlink")
        self._symlink_btn.setIconSize(ICON_SIZE)
        self._symlink_btn.clicked.connect(self.symlink_clicked.emit)
        layout.addWidget(self._symlink_btn)

        # Rename button (bad extensions / bad names)
        self._rename_btn = QPushButton(icon_rename(18), " Rename")
        self._rename_btn.setIconSize(ICON_SIZE)
        self._rename_btn.clicked.connect(self.rename_clicked.emit)
        layout.addWidget(self._rename_btn)

        # Clean EXIF button
        self._clean_exif_btn = QPushButton(icon_clean(18), " Clean EXIF")
        self._clean_exif_btn.setIconSize(ICON_SIZE)
        self._clean_exif_btn.clicked.connect(self.clean_exif_clicked.emit)
        layout.addWidget(self._clean_exif_btn)

        # Optimize Video button
        self._optimize_btn = QPushButton(icon_optimize(18), " Optimize")
        self._optimize_btn.setIconSize(ICON_SIZE)
        self._optimize_btn.clicked.connect(self.optimize_video_clicked.emit)
        layout.addWidget(self._optimize_btn)

        # Stretch at the end
        layout.addStretch()

        self._update_visibility()

    def set_active_tab(self, tab: ActiveTab):
        self._active_tab = tab
        self._update_visibility()

    def set_scanning(self, scanning: bool):
        self._scanning = scanning
        self._scan_btn.setVisible(not scanning)
        self._stop_btn.setVisible(scanning)
        self._update_enabled()

    def set_has_results(self, has_results: bool):
        self._has_results = has_results
        self._update_enabled()

    def set_has_selection(self, has_selection: bool):
        self._has_selection = has_selection
        self._update_enabled()

    def _update_visibility(self):
        tab = self._active_tab
        is_grouped = tab in GROUPED_TABS

        # Always visible
        self._select_btn.setVisible(True)
        self._delete_btn.setVisible(True)
        self._move_btn.setVisible(True)
        self._save_btn.setVisible(True)
        self._sort_btn.setVisible(True)

        # Conditional buttons
        self._hardlink_btn.setVisible(is_grouped)
        self._symlink_btn.setVisible(is_grouped)
        self._rename_btn.setVisible(tab in (ActiveTab.BAD_EXTENSIONS, ActiveTab.BAD_NAMES))
        self._clean_exif_btn.setVisible(tab == ActiveTab.EXIF_REMOVER)
        self._optimize_btn.setVisible(tab == ActiveTab.VIDEO_OPTIMIZER)

        self._update_enabled()

    def _update_enabled(self):
        has_data = self._has_results and not self._scanning
        has_sel = self._has_selection and not self._scanning

        self._scan_btn.setEnabled(not self._scanning)
        self._select_btn.setEnabled(has_data)
        self._delete_btn.setEnabled(has_sel)
        self._move_btn.setEnabled(has_sel)
        self._save_btn.setEnabled(has_data)
        self._sort_btn.setEnabled(has_data)
        self._hardlink_btn.setEnabled(has_sel)
        self._symlink_btn.setEnabled(has_sel)
        self._rename_btn.setEnabled(has_sel)
        self._clean_exif_btn.setEnabled(has_sel)
        self._optimize_btn.setEnabled(has_sel)
