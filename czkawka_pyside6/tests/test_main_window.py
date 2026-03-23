"""Integration tests for MainWindow."""
import pytest
import sys
import os

os.environ["QT_QPA_PLATFORM"] = "offscreen"
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from PySide6.QtWidgets import QApplication


@pytest.fixture(scope="session")
def qapp():
    app = QApplication.instance()
    if app is None:
        app = QApplication(sys.argv)
        app.setApplicationName("Czkawka")
        app.setOrganizationName("czkawka")
    return app


from app.main_window import MainWindow
from app.models import ActiveTab, ResultEntry


@pytest.fixture
def window(qapp):
    return MainWindow()


class TestMainWindow:
    def test_creation(self, window):
        assert window.windowTitle() == "Czkawka - PySide6 Edition"
        assert window.minimumWidth() == 900

    def test_all_tabs(self, window):
        for tab in list(ActiveTab)[:14]:
            window._on_tab_changed(tab)
        assert True  # No crash

    def test_initial_state(self, window):
        assert window._state.scanning is False
        assert window._state.active_tab == ActiveTab.DUPLICATE_FILES

    def test_has_shortcuts(self, window):
        assert hasattr(window, '_setup_shortcuts')

    def test_has_system_tray(self, window):
        assert hasattr(window, '_tray')

    def test_has_scan_history(self, window):
        assert hasattr(window, '_scan_history')

    def test_has_scan_queue(self, window):
        assert hasattr(window, '_scan_queue')

    def test_bottom_panel_accepts_drops(self, window):
        assert window._bottom_panel.acceptDrops() is True

    def test_results_view_has_filter(self, window):
        assert hasattr(window._results_view, '_filter_edit')

    def test_action_buttons_load(self, window):
        assert hasattr(window._action_buttons, '_load_btn')

    def test_window_icon(self, window):
        assert not window.windowIcon().isNull()

    def test_tab_change_updates_buttons(self, window):
        window._on_tab_changed(ActiveTab.EXIF_REMOVER)
        assert not window._action_buttons._clean_exif_btn.isHidden()
        window._on_tab_changed(ActiveTab.DUPLICATE_FILES)
        assert not window._action_buttons._hardlink_btn.isHidden()

    def test_set_results_and_clear(self, window):
        results = [
            ResultEntry(values={"File Name": "test.txt", "Path": "/tmp",
                                "__full_path": "/tmp/test.txt", "__size_bytes": 100,
                                "__modified_date_ts": 1000}),
        ]
        window._results_view.set_active_tab(ActiveTab.EMPTY_FILES)
        window._results_view.set_results(results)
        assert window._results_view._tree.topLevelItemCount() == 1
        window._results_view.clear()
        assert window._results_view._tree.topLevelItemCount() == 0
