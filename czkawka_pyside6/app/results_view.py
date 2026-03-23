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

    # Group header uses the system highlight color (darkened) so it works on
    # both light and dark themes.  Computed lazily on first result display.
    _header_colors_ready = False
    HEADER_BG = QColor()
    HEADER_FG = QColor()

    def __init__(self, parent=None):
        super().__init__(parent)
        self._active_tab = ActiveTab.DUPLICATE_FILES
        self._results: list[ResultEntry] = []
        self._sort_column = -1
        self._sort_order = Qt.AscendingOrder
        self._setup_ui()

    def _ensure_header_colors(self):
        """Derive group header colors from the system palette."""
        if self._header_colors_ready:
            return
        from PySide6.QtWidgets import QApplication
        from PySide6.QtGui import QPalette
        palette = QApplication.instance().palette()
        win = palette.color(QPalette.ColorRole.Window)
        hi = palette.color(QPalette.ColorRole.Highlight)
        self.HEADER_BG = QColor(
            (win.red() + hi.red()) // 2,
            (win.green() + hi.green()) // 2,
            (win.blue() + hi.blue()) // 2,
        )
        self.HEADER_FG = palette.color(QPalette.ColorRole.HighlightedText)
        self._header_colors_ready = True

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)

        # Summary bar
        summary_layout = QHBoxLayout()
        self._summary_label = QLabel("No results")
        summary_layout.addWidget(self._summary_label)
        self._selection_label = QLabel("")
        self._selection_label.setAlignment(Qt.AlignRight)
        self._selection_label.setEnabled(False)
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

        # Sortable: click header to sort
        self._tree.setSortingEnabled(False)  # We handle sorting manually
        header = self._tree.header()
        header.setSectionsClickable(True)
        header.sectionClicked.connect(self._on_header_clicked)
        # Resizable columns
        header.setSectionResizeMode(QHeaderView.Interactive)
        header.setStretchLastSection(True)

        layout.addWidget(self._tree)

    def set_active_tab(self, tab: ActiveTab):
        self._active_tab = tab
        self._sort_column = -1
        columns = TAB_COLUMNS.get(tab, ["Selection", "File Name", "Path"])
        self._tree.setColumnCount(len(columns))
        self._tree.setHeaderLabels(columns)
        header = self._tree.header()
        # All columns interactive (resizable), last one stretches
        header.setSectionResizeMode(QHeaderView.Interactive)
        header.setStretchLastSection(True)
        # Give Path columns more initial space
        for i, col in enumerate(columns):
            if col == "Path":
                header.resizeSection(i, 300)
            elif col == "Selection":
                header.resizeSection(i, 30)
            elif col in ("Size", "Hash", "Modification Date"):
                header.resizeSection(i, 140)
            elif col == "File Name":
                header.resizeSection(i, 200)

    def set_results(self, results: list[ResultEntry]):
        self._ensure_header_colors()
        self._results = results
        self._rebuild_tree()
        self._update_summary()

    def _rebuild_tree(self):
        """Rebuild tree items from self._results."""
        self._tree.blockSignals(True)
        self._tree.clear()

        columns = TAB_COLUMNS.get(self._active_tab, ["Selection", "File Name", "Path"])

        for entry in self._results:
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
                # Span header across all columns (must be called after adding to tree)
                item.setFirstColumnSpanned(True)
            else:
                item = QTreeWidgetItem()
                item.setFlags(item.flags() | Qt.ItemIsUserCheckable)
                item.setCheckState(0, Qt.Checked if entry.checked else Qt.Unchecked)

                for col_idx, col_name in enumerate(columns):
                    if col_idx == 0:
                        continue
                    value = entry.values.get(col_name, "")
                    item.setText(col_idx, str(value))

                item.setData(0, Qt.UserRole, entry)
                self._tree.addTopLevelItem(item)

        self._tree.blockSignals(False)

    # ── Sorting ──────────────────────────────────────────────

    def _on_header_clicked(self, logical_index: int):
        """Sort by column when header is clicked. Toggle ascending/descending."""
        if logical_index == 0:
            return  # Don't sort by checkbox column

        if self._sort_column == logical_index:
            # Toggle order
            self._sort_order = (
                Qt.DescendingOrder if self._sort_order == Qt.AscendingOrder
                else Qt.AscendingOrder
            )
        else:
            self._sort_column = logical_index
            self._sort_order = Qt.AscendingOrder

        columns = TAB_COLUMNS.get(self._active_tab, [])
        col_name = columns[logical_index] if logical_index < len(columns) else ""

        # Update header sort indicator
        self._tree.header().setSortIndicator(logical_index, self._sort_order)
        self._tree.header().setSortIndicatorShown(True)

        ascending = self._sort_order == Qt.AscendingOrder

        if self._active_tab in GROUPED_TABS:
            self._sort_within_groups(col_name, ascending)
        else:
            self._sort_flat(col_name, ascending)

    def _sort_key(self, entry: ResultEntry, col_name: str):
        """Return a sort key for a result entry by column name."""
        # Use numeric values for known numeric columns
        if col_name in ("Size",):
            return entry.values.get("__size_bytes", 0)
        if col_name in ("Modification Date",):
            return entry.values.get("__modified_date_ts", 0)
        if col_name in ("Similarity", "Bitrate", "Year", "Length"):
            raw = entry.values.get(col_name, "")
            try:
                return float(str(raw).replace(",", ""))
            except (ValueError, TypeError):
                return 0
        # Default: string comparison (case-insensitive)
        return str(entry.values.get(col_name, "")).lower()

    def _sort_flat(self, col_name: str, ascending: bool):
        """Sort flat (non-grouped) results."""
        self._results.sort(
            key=lambda e: self._sort_key(e, col_name),
            reverse=not ascending,
        )
        self._rebuild_tree()

    def _sort_within_groups(self, col_name: str, ascending: bool):
        """Sort entries within each group, keeping group headers in place."""
        sorted_results = []
        current_group = []
        current_header = None

        for entry in self._results:
            if entry.header_row:
                if current_header is not None:
                    current_group.sort(
                        key=lambda e: self._sort_key(e, col_name),
                        reverse=not ascending,
                    )
                    sorted_results.append(current_header)
                    sorted_results.extend(current_group)
                current_header = entry
                current_group = []
            else:
                current_group.append(entry)

        # Last group
        if current_header is not None:
            current_group.sort(
                key=lambda e: self._sort_key(e, col_name),
                reverse=not ascending,
            )
            sorted_results.append(current_header)
            sorted_results.extend(current_group)

        self._results = sorted_results
        self._rebuild_tree()

    def sort_by_column(self, column: int, ascending: bool = True):
        """Public API for sorting (used by sort dialog)."""
        self._sort_column = column
        self._sort_order = Qt.AscendingOrder if ascending else Qt.DescendingOrder
        columns = TAB_COLUMNS.get(self._active_tab, [])
        col_name = columns[column] if column < len(columns) else ""
        self._tree.header().setSortIndicator(column, self._sort_order)
        self._tree.header().setSortIndicatorShown(True)
        if self._active_tab in GROUPED_TABS:
            self._sort_within_groups(col_name, ascending)
        else:
            self._sort_flat(col_name, ascending)

    # ── Item events ──────────────────────────────────────────

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

    # ── Summary / selection ──────────────────────────────────

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
        self._select_all(False)

        if self._active_tab not in GROUPED_TABS:
            return

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

            for j, (tree_idx, entry) in enumerate(items):
                if j != best_idx:
                    entry.checked = True
                    self._tree.topLevelItem(tree_idx).setCheckState(0, Qt.Checked)

    # ── Public accessors ─────────────────────────────────────

    def get_checked_entries(self) -> list[ResultEntry]:
        return [r for r in self._results if r.checked and not r.header_row]

    def get_all_entries(self) -> list[ResultEntry]:
        return [r for r in self._results if not r.header_row]

    def clear(self):
        self._results = []
        self._tree.clear()
        self._sort_column = -1
        self._tree.header().setSortIndicatorShown(False)
        self._summary_label.setText("No results")
        self._selection_label.setText("")
