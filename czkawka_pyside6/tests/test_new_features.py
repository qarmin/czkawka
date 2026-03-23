"""Tests for newly added features."""
import json
import os
import sys
import tempfile

import pytest

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


from app.models import ActiveTab, ResultEntry
from app.scan_history import ScanHistory, ScanRecord
from app.scan_queue import ScanQueue


class TestScanHistory:
    def test_creation(self):
        h = ScanHistory()
        assert isinstance(h.get_records(), list)

    def test_add_record(self):
        h = ScanHistory()
        initial = len(h.get_records())
        h.add("DUPLICATE_FILES", ["/tmp"], 100, 50, 5.5)
        assert len(h.get_records()) == initial + 1
        record = h.get_records()[-1]
        assert record.tool == "DUPLICATE_FILES"
        assert record.entries_found == 100
        assert record.groups_found == 50
        assert record.duration_seconds == 5.5

    def test_max_records(self):
        h = ScanHistory()
        h._records = []
        for i in range(150):
            h.add("TEST", ["/tmp"], i, 0, 1.0)
        assert len(h.get_records()) <= ScanHistory.MAX_RECORDS

    def test_clear(self):
        h = ScanHistory()
        h.add("TEST", ["/tmp"], 1, 0, 1.0)
        h.clear()
        assert len(h.get_records()) == 0

    def test_persistence(self, tmp_path):
        h = ScanHistory()
        h._path = tmp_path / "test_history.json"
        h.add("TEST", ["/tmp"], 42, 10, 2.5)

        h2 = ScanHistory()
        h2._path = tmp_path / "test_history.json"
        h2._load()
        assert len(h2.get_records()) >= 1
        assert h2.get_records()[-1].entries_found == 42


class TestScanQueue:
    def test_creation(self, qapp):
        q = ScanQueue()
        assert q.pending_count == 0
        assert q.is_running is False

    def test_add(self, qapp):
        q = ScanQueue()
        q.add(ActiveTab.DUPLICATE_FILES)
        assert q.pending_count == 1

    def test_add_duplicate(self, qapp):
        q = ScanQueue()
        q.add(ActiveTab.DUPLICATE_FILES)
        q.add(ActiveTab.DUPLICATE_FILES)
        assert q.pending_count == 1  # No duplicates

    def test_add_all(self, qapp):
        q = ScanQueue()
        q.add_all([ActiveTab.DUPLICATE_FILES, ActiveTab.EMPTY_FILES, ActiveTab.BIG_FILES])
        assert q.pending_count == 3

    def test_stop(self, qapp):
        q = ScanQueue()
        q.add_all([ActiveTab.DUPLICATE_FILES, ActiveTab.EMPTY_FILES])
        q.stop()
        assert q.pending_count == 0
        assert q.is_running is False

    def test_start_emits_next(self, qapp):
        q = ScanQueue()
        q.add(ActiveTab.DUPLICATE_FILES)
        received = []
        q.next_scan.connect(lambda tab: received.append(tab))
        q.start()
        assert len(received) == 1
        assert received[0] == ActiveTab.DUPLICATE_FILES

    def test_sequential_execution(self, qapp):
        q = ScanQueue()
        q.add_all([ActiveTab.DUPLICATE_FILES, ActiveTab.EMPTY_FILES])
        received = []
        q.next_scan.connect(lambda tab: received.append(tab))
        q.start()
        assert received == [ActiveTab.DUPLICATE_FILES]
        q.on_scan_completed()
        assert received == [ActiveTab.DUPLICATE_FILES, ActiveTab.EMPTY_FILES]
        q.on_scan_completed()
        assert q.is_running is False


class TestSaveLoad:
    def test_save_load_roundtrip(self, tmp_path):
        from app.dialogs.save_dialog import SaveDialog

        results = [
            ResultEntry(values={"__header": "Group 1", "__group_id": 0}, header_row=True, group_id=0),
            ResultEntry(values={"File Name": "a.txt", "Path": "/home", "__full_path": "/home/a.txt",
                                "__size_bytes": 1024, "__modified_date_ts": 1000}, group_id=0),
        ]

        # Save as JSON manually (can't use file dialog in test)
        save_path = str(tmp_path / "test_results.json")
        data = []
        for entry in results:
            if entry.header_row:
                data.append({"__header": entry.values.get("__header", ""), "__group_id": entry.group_id})
            else:
                values = dict(entry.values)
                values["__group_id"] = entry.group_id
                values["__checked"] = entry.checked
                data.append(values)
        with open(save_path, "w") as f:
            json.dump(data, f)

        # Load
        with open(save_path) as f:
            loaded_data = json.load(f)

        loaded = []
        for item in loaded_data:
            if "__header" in item:
                loaded.append(ResultEntry(
                    values={"__header": item["__header"]},
                    header_row=True,
                    group_id=item.get("__group_id", 0),
                ))
            else:
                gid = item.pop("__group_id", 0)
                chk = item.pop("__checked", False)
                loaded.append(ResultEntry(values=item, checked=chk, group_id=gid))

        assert len(loaded) == 2
        assert loaded[0].header_row is True
        assert loaded[1].values["File Name"] == "a.txt"

    def test_load_cli_format(self, tmp_path):
        from app.dialogs.save_dialog import _parse_cli_json

        data = {
            "1024": [
                [
                    {"path": "/a/file.txt", "size": 1024, "modified_date": 100, "hash": "abc"},
                    {"path": "/b/file.txt", "size": 1024, "modified_date": 200, "hash": "abc"},
                ]
            ]
        }

        results = _parse_cli_json(data)
        assert results is not None
        headers = [r for r in results if r.header_row]
        files = [r for r in results if not r.header_row]
        assert len(headers) == 1
        assert len(files) == 2


class TestDiffDialog:
    def test_creation(self, qapp):
        from app.dialogs.diff_dialog import DiffDialog
        e1 = ResultEntry(values={"File Name": "a.txt", "Path": "/home",
                                  "Size": "1.0 KB", "Modification Date": "2024-01-01",
                                  "__full_path": "/home/a.txt", "__size_bytes": 1024,
                                  "__modified_date_ts": 1000, "Hash": "abc123"})
        e2 = ResultEntry(values={"File Name": "a.txt", "Path": "/tmp",
                                  "Size": "1.0 KB", "Modification Date": "2024-01-02",
                                  "__full_path": "/tmp/a.txt", "__size_bytes": 1024,
                                  "__modified_date_ts": 2000, "Hash": "abc123"})
        dialog = DiffDialog(e1, e2)
        assert dialog.windowTitle() == "File Comparison"

    def test_diff_summary_identical(self, qapp):
        from app.dialogs.diff_dialog import DiffDialog
        e1 = ResultEntry(values={"__full_path": "/a/f.txt", "__size_bytes": 100,
                                  "__modified_date_ts": 1000, "Size": "100 B",
                                  "Modification Date": "2024-01-01"})
        e2 = ResultEntry(values={"__full_path": "/b/f.txt", "__size_bytes": 100,
                                  "__modified_date_ts": 1000, "Size": "100 B",
                                  "Modification Date": "2024-01-01"})
        dialog = DiffDialog(e1, e2)
        summary = dialog._compute_diff_summary(e1, e2)
        assert "different directories" in summary

    def test_diff_summary_size_differs(self, qapp):
        from app.dialogs.diff_dialog import DiffDialog
        e1 = ResultEntry(values={"__full_path": "/a/f.txt", "__size_bytes": 100,
                                  "__modified_date_ts": 1000, "Size": "100 B"})
        e2 = ResultEntry(values={"__full_path": "/a/g.txt", "__size_bytes": 200,
                                  "__modified_date_ts": 1000, "Size": "200 B"})
        dialog = DiffDialog(e1, e2)
        summary = dialog._compute_diff_summary(e1, e2)
        assert "Size differs" in summary
