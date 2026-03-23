from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QListWidget, QListWidgetItem, QLabel,
    QPushButton, QHBoxLayout, QSizePolicy
)
from PySide6.QtCore import Signal, Qt, QSize, QEvent
from PySide6.QtGui import QFont, QPixmap

from .models import ActiveTab, TAB_DISPLAY_NAMES, TABS_WITH_SETTINGS
from .icons import app_logo_path, icon_settings, icon_subsettings


class LeftPanel(QWidget):
    """Left sidebar for selecting scan tools."""
    tab_changed = Signal(object)  # ActiveTab
    settings_requested = Signal()
    about_requested = Signal()
    tool_settings_toggled = Signal(bool)

    # Tool tabs in display order
    TOOL_TABS = [
        ActiveTab.DUPLICATE_FILES,
        ActiveTab.EMPTY_FOLDERS,
        ActiveTab.BIG_FILES,
        ActiveTab.EMPTY_FILES,
        ActiveTab.TEMPORARY_FILES,
        ActiveTab.SIMILAR_IMAGES,
        ActiveTab.SIMILAR_VIDEOS,
        ActiveTab.SIMILAR_MUSIC,
        ActiveTab.INVALID_SYMLINKS,
        ActiveTab.BROKEN_FILES,
        ActiveTab.BAD_EXTENSIONS,
        ActiveTab.BAD_NAMES,
        ActiveTab.EXIF_REMOVER,
        ActiveTab.VIDEO_OPTIMIZER,
    ]

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setMinimumWidth(170)
        self.setMaximumWidth(230)
        self._setup_ui()

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(6, 6, 6, 6)
        layout.setSpacing(4)

        # Logo image (clickable via event filter)
        logo_path = app_logo_path()
        if logo_path:
            self._logo_label = QLabel()
            pixmap = QPixmap(logo_path)
            scaled = pixmap.scaledToHeight(70, Qt.SmoothTransformation)
            self._logo_label.setPixmap(scaled)
            self._logo_label.setAlignment(Qt.AlignCenter)
            self._logo_label.setCursor(Qt.PointingHandCursor)
            self._logo_label.setToolTip("About Czkawka")
            self._logo_label.installEventFilter(self)
            layout.addWidget(self._logo_label)
        else:
            title_label = QLabel("Czkawka")
            title_font = QFont()
            title_font.setPointSize(16)
            title_font.setBold(True)
            title_label.setFont(title_font)
            title_label.setAlignment(Qt.AlignCenter)
            title_label.setCursor(Qt.PointingHandCursor)
            title_label.installEventFilter(self)
            self._logo_label = title_label
            layout.addWidget(title_label)

        # Top buttons row with icons
        btn_row = QHBoxLayout()
        btn_row.setSpacing(4)

        self._settings_btn = QPushButton(icon_settings(20), "")
        self._settings_btn.setFixedSize(32, 32)
        self._settings_btn.setIconSize(QSize(20, 20))
        self._settings_btn.setToolTip("Application Settings")
        self._settings_btn.clicked.connect(self.settings_requested.emit)
        btn_row.addWidget(self._settings_btn)

        self._tool_settings_btn = QPushButton(icon_subsettings(20), "")
        self._tool_settings_btn.setFixedSize(32, 32)
        self._tool_settings_btn.setIconSize(QSize(20, 20))
        self._tool_settings_btn.setToolTip("Tool-specific Settings")
        self._tool_settings_btn.setCheckable(True)
        self._tool_settings_btn.clicked.connect(
            lambda checked: self.tool_settings_toggled.emit(checked)
        )
        self._tool_settings_btn.setVisible(False)
        btn_row.addWidget(self._tool_settings_btn)

        btn_row.addStretch()
        layout.addLayout(btn_row)

        # Tool list
        self._tool_list = QListWidget()
        self._tool_list.setSpacing(1)

        for tab in self.TOOL_TABS:
            item = QListWidgetItem(TAB_DISPLAY_NAMES[tab])
            item.setData(Qt.UserRole, tab)
            item.setSizeHint(item.sizeHint().__class__(item.sizeHint().width(), 30))
            self._tool_list.addItem(item)

        self._tool_list.setCurrentRow(0)
        self._tool_list.currentItemChanged.connect(self._on_item_changed)
        layout.addWidget(self._tool_list)

        # Version label
        version_label = QLabel("Czkawka PySide6 v11.0.1")
        version_label.setAlignment(Qt.AlignCenter)
        version_label.setEnabled(False)
        layout.addWidget(version_label)

    def eventFilter(self, obj, event):
        if obj is self._logo_label and event.type() == QEvent.Type.MouseButtonPress:
            self.about_requested.emit()
            return True
        return super().eventFilter(obj, event)

    def _on_item_changed(self, current, previous):
        if current:
            tab = current.data(Qt.UserRole)
            self._tool_settings_btn.setVisible(tab in TABS_WITH_SETTINGS)
            self.tab_changed.emit(tab)

    def set_active_tab(self, tab: ActiveTab):
        for i in range(self._tool_list.count()):
            item = self._tool_list.item(i)
            if item.data(Qt.UserRole) == tab:
                self._tool_list.setCurrentRow(i)
                break

    def get_active_tab(self) -> ActiveTab:
        item = self._tool_list.currentItem()
        if item:
            return item.data(Qt.UserRole)
        return ActiveTab.DUPLICATE_FILES
