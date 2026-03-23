"""Tests for app.models — pure data, no Qt needed."""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from app.models import (
    ActiveTab, SelectMode, DeleteMethod, CheckingMethod, HashType,
    TAB_TO_CLI_COMMAND, TAB_DISPLAY_NAMES, TAB_COLUMNS,
    GROUPED_TABS, TABS_WITH_SETTINGS,
    ResultEntry, ScanProgress, ToolSettings, AppSettings,
)


class TestActiveTab:
    def test_all_14_tool_tabs(self):
        tool_tabs = [t for t in ActiveTab if t not in (ActiveTab.SETTINGS, ActiveTab.ABOUT)]
        assert len(tool_tabs) == 14

    def test_all_tabs_have_display_names(self):
        for tab in ActiveTab:
            if tab in (ActiveTab.SETTINGS, ActiveTab.ABOUT):
                continue
            assert tab in TAB_DISPLAY_NAMES, f"Missing display name for {tab}"

    def test_all_tabs_have_cli_commands(self):
        for tab in ActiveTab:
            if tab in (ActiveTab.SETTINGS, ActiveTab.ABOUT):
                continue
            assert tab in TAB_TO_CLI_COMMAND, f"Missing CLI command for {tab}"

    def test_all_tabs_have_columns(self):
        for tab in ActiveTab:
            if tab in (ActiveTab.SETTINGS, ActiveTab.ABOUT):
                continue
            assert tab in TAB_COLUMNS, f"Missing columns for {tab}"
            assert len(TAB_COLUMNS[tab]) >= 2, f"Too few columns for {tab}"

    def test_grouped_tabs_are_subset(self):
        tool_tabs = set(t for t in ActiveTab if t not in (ActiveTab.SETTINGS, ActiveTab.ABOUT))
        assert GROUPED_TABS.issubset(tool_tabs)

    def test_tabs_with_settings_are_subset(self):
        tool_tabs = set(t for t in ActiveTab if t not in (ActiveTab.SETTINGS, ActiveTab.ABOUT))
        assert TABS_WITH_SETTINGS.issubset(tool_tabs)


class TestResultEntry:
    def test_default_values(self):
        entry = ResultEntry(values={"File Name": "test.txt"})
        assert entry.checked is False
        assert entry.header_row is False
        assert entry.group_id == 0

    def test_header_row(self):
        entry = ResultEntry(values={"__header": "Group 1"}, header_row=True, group_id=5)
        assert entry.header_row is True
        assert entry.group_id == 5


class TestScanProgress:
    def test_defaults(self):
        p = ScanProgress()
        assert p.step_name == ""
        assert p.entries_checked == 0
        assert p.entries_to_check == 0
        assert p.bytes_checked == 0
        assert p.bytes_to_check == 0
        assert p.current_stage_idx == 0
        assert p.max_stage_idx == 0

    def test_with_values(self):
        p = ScanProgress(
            stage_name="Hashing",
            current_stage_idx=3,
            max_stage_idx=6,
            entries_checked=500,
            entries_to_check=1000,
        )
        assert p.stage_name == "Hashing"
        assert p.entries_checked == 500


class TestToolSettings:
    def test_defaults(self):
        ts = ToolSettings()
        assert ts.dup_check_method == CheckingMethod.HASH
        assert ts.dup_hash_type == HashType.BLAKE3
        assert ts.img_hash_size == 16
        assert ts.big_files_number == 50
        assert ts.big_files_mode == "biggest"

    def test_mutability(self):
        ts = ToolSettings()
        ts.dup_check_method = CheckingMethod.NAME
        assert ts.dup_check_method == CheckingMethod.NAME


class TestAppSettings:
    def test_defaults(self):
        s = AppSettings()
        assert s.recursive_search is True
        assert s.use_cache is True
        assert s.move_to_trash is True
        assert s.thread_number == 0
        assert isinstance(s.included_paths, list)

    def test_included_paths_default(self):
        s = AppSettings()
        assert len(s.included_paths) == 1  # home dir


class TestEnums:
    def test_select_modes(self):
        assert len(SelectMode) >= 10

    def test_delete_methods(self):
        assert DeleteMethod.NONE.value == "NONE"
        assert DeleteMethod.DELETE.value == "DELETE"

    def test_hash_types(self):
        assert HashType.BLAKE3.value == "BLAKE3"
        assert HashType.CRC32.value == "CRC32"
        assert HashType.XXH3.value == "XXH3"
