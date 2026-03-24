"""Tests for Qt widget components."""
import pytest
import sys
import os

os.environ["QT_QPA_PLATFORM"] = "offscreen"
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from PySide6.QtWidgets import QApplication
from PySide6.QtCore import Qt


@pytest.fixture(scope="session")
def qapp():
    app = QApplication.instance()
    if app is None:
        app = QApplication(sys.argv)
        app.setApplicationName("Czkawka")
        app.setOrganizationName("czkawka")
    return app


from app.models import ActiveTab, ResultEntry, SelectMode, ScanProgress, TAB_COLUMNS
from app.results_view import ResultsView
from app.left_panel import LeftPanel
from app.action_buttons import ActionButtons
from app.progress_widget import ProgressWidget


@pytest.fixture
def results_view(qapp):
    rv = ResultsView()
    rv.set_active_tab(ActiveTab.DUPLICATE_FILES)
    return rv


@pytest.fixture
def sample_grouped_results():
    return [
        ResultEntry(values={"__header": "Group 1 (2 files)"}, header_row=True, group_id=0),
        ResultEntry(values={"File Name": "a.txt", "Path": "/home", "Size": "1.0 KB",
                            "__full_path": "/home/a.txt", "__size_bytes": 1024,
                            "__modified_date_ts": 1000}, group_id=0),
        ResultEntry(values={"File Name": "b.txt", "Path": "/tmp", "Size": "2.0 KB",
                            "__full_path": "/tmp/b.txt", "__size_bytes": 2048,
                            "__modified_date_ts": 2000}, group_id=0),
        ResultEntry(values={"__header": "Group 2 (2 files)"}, header_row=True, group_id=1),
        ResultEntry(values={"File Name": "c.txt", "Path": "/var", "Size": "500 B",
                            "__full_path": "/var/c.txt", "__size_bytes": 500,
                            "__modified_date_ts": 500}, group_id=1),
        ResultEntry(values={"File Name": "d.txt", "Path": "/opt", "Size": "500 B",
                            "__full_path": "/opt/d.txt", "__size_bytes": 500,
                            "__modified_date_ts": 3000}, group_id=1),
    ]


@pytest.fixture
def sample_flat_results():
    return [
        ResultEntry(values={"File Name": "empty1.txt", "Path": "/a",
                            "__full_path": "/a/empty1.txt", "__size_bytes": 0,
                            "__modified_date_ts": 1000}),
        ResultEntry(values={"File Name": "empty2.txt", "Path": "/b",
                            "__full_path": "/b/empty2.txt", "__size_bytes": 0,
                            "__modified_date_ts": 2000}),
        ResultEntry(values={"File Name": "empty3.txt", "Path": "/c",
                            "__full_path": "/c/empty3.txt", "__size_bytes": 0,
                            "__modified_date_ts": 3000}),
    ]


class TestResultsView:
    def test_set_active_tab(self, results_view):
        for tab in list(ActiveTab)[:14]:
            results_view.set_active_tab(tab)
            cols = TAB_COLUMNS.get(tab, [])
            assert results_view._tree.columnCount() == len(cols)

    def test_set_grouped_results(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        assert results_view._tree.topLevelItemCount() == 6
        # Header should be spanned
        header_item = results_view._tree.topLevelItem(0)
        assert header_item.isFirstColumnSpanned()

    def test_set_flat_results(self, results_view, sample_flat_results):
        results_view.set_active_tab(ActiveTab.EMPTY_FILES)
        results_view.set_results(sample_flat_results)
        assert results_view._tree.topLevelItemCount() == 3

    def test_select_all(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        results_view.apply_selection(SelectMode.SELECT_ALL)
        checked = results_view.get_checked_entries()
        assert len(checked) == 4  # 4 non-header entries

    def test_unselect_all(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        results_view.apply_selection(SelectMode.SELECT_ALL)
        results_view.apply_selection(SelectMode.UNSELECT_ALL)
        checked = results_view.get_checked_entries()
        assert len(checked) == 0

    def test_invert_selection(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        results_view.apply_selection(SelectMode.SELECT_ALL)
        results_view.apply_selection(SelectMode.INVERT_SELECTION)
        checked = results_view.get_checked_entries()
        assert len(checked) == 0

    def test_select_biggest(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        results_view.apply_selection(SelectMode.SELECT_BIGGEST_SIZE)
        checked = results_view.get_checked_entries()
        # Should select all except the biggest in each group
        # Group 0: b.txt (2048) is biggest -> a.txt selected
        # Group 1: same size -> first kept, second selected
        assert len(checked) == 2

    def test_select_newest(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        results_view.apply_selection(SelectMode.SELECT_NEWEST)
        checked = results_view.get_checked_entries()
        assert len(checked) == 2

    def test_get_all_entries(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        all_entries = results_view.get_all_entries()
        assert len(all_entries) == 4  # Excludes headers

    def test_clear(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        results_view.clear()
        assert results_view._tree.topLevelItemCount() == 0
        assert results_view.get_all_entries() == []

    def test_sort_by_column(self, results_view, sample_flat_results):
        results_view.set_active_tab(ActiveTab.EMPTY_FILES)
        results_view.set_results(sample_flat_results)
        results_view.sort_by_column(1, ascending=True)  # Sort by File Name
        # Should not crash
        assert results_view._tree.topLevelItemCount() == 3

    def test_filter(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        results_view._apply_filter("a.txt")
        # Only a.txt should be visible
        visible = sum(1 for i in range(results_view._tree.topLevelItemCount())
                      if not results_view._tree.topLevelItem(i).isHidden())
        assert visible >= 1  # At least the matching item

    def test_filter_clear(self, results_view, sample_grouped_results):
        results_view.set_results(sample_grouped_results)
        results_view._apply_filter("a.txt")
        results_view._apply_filter("")  # Clear filter
        visible = sum(1 for i in range(results_view._tree.topLevelItemCount())
                      if not results_view._tree.topLevelItem(i).isHidden())
        assert visible == 6  # All visible again


class TestLeftPanel:
    def test_creation(self, qapp):
        panel = LeftPanel()
        assert panel._tool_list.count() == 14

    def test_set_active_tab(self, qapp):
        panel = LeftPanel()
        panel.set_active_tab(ActiveTab.SIMILAR_IMAGES)
        assert panel.get_active_tab() == ActiveTab.SIMILAR_IMAGES

    def test_all_tabs_selectable(self, qapp):
        panel = LeftPanel()
        for i, tab in enumerate(LeftPanel.TOOL_TABS):
            panel.set_active_tab(tab)
            assert panel.get_active_tab() == tab


class TestActionButtons:
    def test_creation(self, qapp):
        ab = ActionButtons()
        assert ab._scan_btn is not None
        assert ab._stop_btn is not None
        assert ab._delete_btn is not None

    def test_scanning_state(self, qapp):
        ab = ActionButtons()
        ab.set_scanning(True)
        assert ab._scan_btn.isHidden() is True
        assert ab._stop_btn.isHidden() is False
        ab.set_scanning(False)
        assert ab._scan_btn.isHidden() is False
        assert ab._stop_btn.isHidden() is True

    def test_tab_visibility(self, qapp):
        ab = ActionButtons()
        ab.set_active_tab(ActiveTab.DUPLICATE_FILES)
        assert ab._hardlink_btn.isHidden() is False
        assert ab._rename_btn.isHidden() is True

        ab.set_active_tab(ActiveTab.BAD_EXTENSIONS)
        assert ab._hardlink_btn.isHidden() is True
        assert ab._rename_btn.isHidden() is False

        ab.set_active_tab(ActiveTab.EXIF_REMOVER)
        assert ab._clean_exif_btn.isHidden() is False

    def test_load_button_exists(self, qapp):
        ab = ActionButtons()
        assert hasattr(ab, "_load_btn")


class TestProgressWidget:
    def test_creation(self, qapp):
        pw = ProgressWidget()
        assert pw.isVisible() is False

    def test_start_stop(self, qapp):
        pw = ProgressWidget()
        pw.start(ActiveTab.DUPLICATE_FILES)
        assert pw.isVisible() is True
        pw.stop()
        # After stop, still visible briefly (auto-hide timer)
        assert pw._stage_label.text() == "Scan complete"

    def test_update_progress_with_entries(self, qapp):
        pw = ProgressWidget()
        pw.start(ActiveTab.DUPLICATE_FILES)
        pw.update_progress(ScanProgress(
            stage_name="Calculating hashes",
            current_stage_idx=5,
            max_stage_idx=6,
            entries_checked=500,
            entries_to_check=1000,
            bytes_checked=50000000,
            bytes_to_check=100000000,
        ))
        assert "6" in pw._stage_label.text()  # [6/7] in title
        assert pw._stage_bar.maximum() == 100
        assert pw._stage_bar.value() == 50
        assert pw._overall_bar.value() > 0

    def test_update_progress_collecting(self, qapp):
        pw = ProgressWidget()
        pw.start(ActiveTab.DUPLICATE_FILES)
        pw.update_progress(ScanProgress(
            stage_name="Collecting files",
            current_stage_idx=0,
            max_stage_idx=6,
            entries_checked=50000,
            entries_to_check=0,
        ))
        assert "Collecting" in pw._stage_label.text()

    def test_format_time(self, qapp):
        assert ProgressWidget._format_time(5) == "5s"
        assert ProgressWidget._format_time(65) == "1m 5s"
        assert ProgressWidget._format_time(3665) == "1h 1m"

    def test_format_size(self, qapp):
        assert ProgressWidget._format_size(0) == "0 B"
        assert ProgressWidget._format_size(1024) == "1.0 KB"
        assert ProgressWidget._format_size(1048576) == "1.0 MB"
