from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QTreeWidget, QTreeWidgetItem, QHeaderView,
    QAbstractItemView, QMenu, QLabel, QHBoxLayout
)
from PySide6.QtCore import Signal, Qt
from PySide6.QtGui import QColor, QBrush, QFont, QAction

from .models import (
    ActiveTab, ResultEntry, TAB_COLUMNS, GROUPED_TABS, SelectMode
)


class ResultsView(QWidget):
    """Results display with tree view for grouped results and table for flat results."""

    selection_changed = Signal(int)  # number of selected items
    item_activated = Signal(object)  # ResultEntry
    context_menu_requested = Signal(object, object)  # QPoint, ResultEntry

    # Colors
    HEADER_BG = QColor(60, 60, 80)
    HEADER_FG = QColor(220, 220, 255)
    SELECTED_BG = QColor(40, 80, 40)

    def __init__(self, parent=None):
        super().__init__(parent)
        self._active_tab = ActiveTab.DUPLICATE_FILES
        self._results: list[ResultEntry] = []
        self._setup_ui()

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)

        # Summary bar
        summary_layout = QHBoxLayout()
        self._summary_label = QLabel("No results")
        self._summary_label.setStyleSheet("padding: 4px;")
        summary_layout.addWidget(self._summary_label)
        self._selection_label = QLabel("")
        self._selection_label.setAlignment(Qt.AlignRight)
        self._selection_label.setStyleSheet("padding: 4px; color: #aaa;")
        summary_layout.addWidget(self._selection_label)
        layout.addLayout(summary_layout)

        # Tree widget for results
        self._tree = QTreeWidget()
        self._tree.setSelectionMode(QAbstractItemView.ExtendedSelection)
        self._tree.setRootIsDecorated(False)
        self._tree.setAlternatingRowColors(True)
        self._tree.setContextMenuPolicy(Qt.CustomContextMenu)
        self._tree.customContextMenuRequested.connect(self._on_context_menu)
        self._tree.itemChanged.connect(self._on_item_changed)
        self._tree.itemDoubleClicked.connect(self._on_item_double_clicked)
        layout.addWidget(self._tree)

    def set_active_tab(self, tab: ActiveTab):
        self._active_tab = tab
        columns = TAB_COLUMNS.get(tab, ["Selection", "File Name", "Path"])
        self._tree.setHeaderLabels(columns)
        header = self._tree.header()
        for i in range(len(columns)):
            if columns[i] == "Path":
                header.setSectionResizeMode(i, QHeaderView.Stretch)
            else:
                header.setSectionResizeMode(i, QHeaderView.ResizeToContents)

    def set_results(self, results: list[ResultEntry]):
        self._results = results
        self._tree.blockSignals(True)
        self._tree.clear()

        columns = TAB_COLUMNS.get(self._active_tab, ["Selection", "File Name", "Path"])

        for entry in results:
            if entry.header_row:
                item = QTreeWidgetItem()
                header_text = entry.values.get("__header", "Group")
                item.setText(0, header_text)
                item.setFirstColumnSpanned(True)
                font = QFont()
                font.setBold(True)
                item.setFont(0, font)
                for col in range(len(columns)):
                    item.setBackground(col, QBrush(self.HEADER_BG))
                    item.setForeground(col, QBrush(self.HEADER_FG))
                item.setFlags(item.flags() & ~Qt.ItemIsUserCheckable)
                item.setData(0, Qt.UserRole, entry)
                self._tree.addTopLevelItem(item)
            else:
                item = QTreeWidgetItem()
                # First column is checkbox (Selection)
                item.setFlags(item.flags() | Qt.ItemIsUserCheckable)
                item.setCheckState(0, Qt.Checked if entry.checked else Qt.Unchecked)

                for col_idx, col_name in enumerate(columns):
                    if col_idx == 0:  # Selection column
                        continue
                    value = entry.values.get(col_name, "")
                    item.setText(col_idx, str(value))

                item.setData(0, Qt.UserRole, entry)
                self._tree.addTopLevelItem(item)

        self._tree.blockSignals(False)
        self._update_summary()

    def _on_item_changed(self, item, column):
        if column == 0:
            entry = item.data(0, Qt.UserRole)
            if entry and not entry.header_row:
                entry.checked = item.checkState(0) == Qt.Checked
                self._update_selection_count()

    def _on_item_double_clicked(self, item, column):
        entry = item.data(0, Qt.UserRole)
        if entry and not entry.header_row:
            self.item_activated.emit(entry)

    def _on_context_menu(self, pos):
        item = self._tree.itemAt(pos)
        if item:
            entry = item.data(0, Qt.UserRole)
            if entry and not entry.header_row:
                menu = QMenu(self)
                open_action = QAction("Open File", self)
                open_action.triggered.connect(lambda: self._open_file(entry))
                menu.addAction(open_action)

                open_dir_action = QAction("Open Containing Folder", self)
                open_dir_action.triggered.connect(lambda: self._open_folder(entry))
                menu.addAction(open_dir_action)

                menu.addSeparator()

                select_action = QAction("Select", self)
                select_action.triggered.connect(lambda: self._set_check(item, True))
                menu.addAction(select_action)

                deselect_action = QAction("Deselect", self)
                deselect_action.triggered.connect(lambda: self._set_check(item, False))
                menu.addAction(deselect_action)

                menu.exec_(self._tree.viewport().mapToGlobal(pos))

    def _open_file(self, entry: ResultEntry):
        import subprocess, sys
        path = entry.values.get("__full_path", "")
        if path:
            if sys.platform == "linux":
                subprocess.Popen(["xdg-open", path])
            elif sys.platform == "darwin":
                subprocess.Popen(["open", path])
            else:
                subprocess.Popen(["start", path], shell=True)

    def _open_folder(self, entry: ResultEntry):
        import subprocess, sys
        from pathlib import Path
        path = entry.values.get("__full_path", "")
        if path:
            folder = str(Path(path).parent)
            if sys.platform == "linux":
                subprocess.Popen(["xdg-open", folder])
            elif sys.platform == "darwin":
                subprocess.Popen(["open", folder])
            else:
                subprocess.Popen(["explorer", folder], shell=True)

    def _set_check(self, item, checked):
        item.setCheckState(0, Qt.Checked if checked else Qt.Unchecked)

    def _update_summary(self):
        total = sum(1 for r in self._results if not r.header_row)
        groups = sum(1 for r in self._results if r.header_row)
        if self._active_tab in GROUPED_TABS and groups > 0:
            self._summary_label.setText(f"Found {total} files in {groups} groups")
        elif total > 0:
            self._summary_label.setText(f"Found {total} entries")
        else:
            self._summary_label.setText("No results")
        self._update_selection_count()

    def _update_selection_count(self):
        selected = sum(1 for r in self._results if r.checked and not r.header_row)
        total = sum(1 for r in self._results if not r.header_row)
        if selected > 0:
            self._selection_label.setText(f"Selected: {selected}/{total}")
        else:
            self._selection_label.setText("")
        self.selection_changed.emit(selected)

    def apply_selection(self, mode: SelectMode):
        self._tree.blockSignals(True)

        if mode == SelectMode.SELECT_ALL:
            self._select_all(True)
        elif mode == SelectMode.UNSELECT_ALL:
            self._select_all(False)
        elif mode == SelectMode.INVERT_SELECTION:
            self._invert_selection()
        elif mode in (SelectMode.SELECT_BIGGEST_SIZE, SelectMode.SELECT_SMALLEST_SIZE,
                      SelectMode.SELECT_NEWEST, SelectMode.SELECT_OLDEST,
                      SelectMode.SELECT_BIGGEST_RESOLUTION, SelectMode.SELECT_SMALLEST_RESOLUTION,
                      SelectMode.SELECT_SHORTEST_PATH, SelectMode.SELECT_LONGEST_PATH):
            self._select_by_group_criteria(mode)

        self._tree.blockSignals(False)
        self._update_selection_count()

    def _select_all(self, checked: bool):
        for i in range(self._tree.topLevelItemCount()):
            item = self._tree.topLevelItem(i)
            entry = item.data(0, Qt.UserRole)
            if entry and not entry.header_row:
                entry.checked = checked
                item.setCheckState(0, Qt.Checked if checked else Qt.Unchecked)

    def _invert_selection(self):
        for i in range(self._tree.topLevelItemCount()):
            item = self._tree.topLevelItem(i)
            entry = item.data(0, Qt.UserRole)
            if entry and not entry.header_row:
                entry.checked = not entry.checked
                item.setCheckState(0, Qt.Checked if entry.checked else Qt.Unchecked)

    def _select_by_group_criteria(self, mode: SelectMode):
        # First unselect all
        self._select_all(False)

        if self._active_tab not in GROUPED_TABS:
            return

        # Group entries by group_id
        groups: dict[int, list[tuple[int, ResultEntry]]] = {}
        for i in range(self._tree.topLevelItemCount()):
            item = self._tree.topLevelItem(i)
            entry = item.data(0, Qt.UserRole)
            if entry and not entry.header_row:
                groups.setdefault(entry.group_id, []).append((i, entry))

        for group_id, items in groups.items():
            if len(items) <= 1:
                continue

            best_idx = 0
            if mode == SelectMode.SELECT_BIGGEST_SIZE:
                best_idx = max(range(len(items)), key=lambda j: items[j][1].values.get("__size_bytes", 0))
            elif mode == SelectMode.SELECT_SMALLEST_SIZE:
                best_idx = min(range(len(items)), key=lambda j: items[j][1].values.get("__size_bytes", 0))
            elif mode == SelectMode.SELECT_NEWEST:
                best_idx = max(range(len(items)), key=lambda j: items[j][1].values.get("__modified_date_ts", 0))
            elif mode == SelectMode.SELECT_OLDEST:
                best_idx = min(range(len(items)), key=lambda j: items[j][1].values.get("__modified_date_ts", 0))
            elif mode == SelectMode.SELECT_SHORTEST_PATH:
                best_idx = min(range(len(items)), key=lambda j: len(items[j][1].values.get("__full_path", "")))
            elif mode == SelectMode.SELECT_LONGEST_PATH:
                best_idx = max(range(len(items)), key=lambda j: len(items[j][1].values.get("__full_path", "")))

            # Select all EXCEPT the best (the one to keep)
            for j, (tree_idx, entry) in enumerate(items):
                if j != best_idx:
                    entry.checked = True
                    self._tree.topLevelItem(tree_idx).setCheckState(0, Qt.Checked)

    def sort_by_column(self, column: int, ascending: bool = True):
        order = Qt.AscendingOrder if ascending else Qt.DescendingOrder
        self._tree.sortItems(column, order)

    def get_checked_entries(self) -> list[ResultEntry]:
        return [r for r in self._results if r.checked and not r.header_row]

    def get_all_entries(self) -> list[ResultEntry]:
        return [r for r in self._results if not r.header_row]

    def clear(self):
        self._results = []
        self._tree.clear()
        self._summary_label.setText("No results")
        self._selection_label.setText("")
