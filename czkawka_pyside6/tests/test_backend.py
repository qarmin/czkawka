"""Tests for app.backend — command building and JSON parsing."""
import json
import os
import sys
import tempfile

os.environ["QT_QPA_PLATFORM"] = "offscreen"
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from app.models import (
    ActiveTab, AppSettings, ToolSettings, ResultEntry,
    CheckingMethod, HashType, MusicSearchMethod,
)
from app.backend import ScanWorker


def _make_worker(tab: ActiveTab, **kwargs) -> ScanWorker:
    settings = AppSettings()
    settings.included_paths = ["/tmp/test"]
    settings.czkawka_cli_path = "czkawka_cli"
    ts = ToolSettings(**kwargs) if kwargs else ToolSettings()
    return ScanWorker(tab, settings, ts)


class TestCommandBuilding:
    def test_duplicate_hash_command(self):
        w = _make_worker(ActiveTab.DUPLICATE_FILES)
        cmd = w._build_command()
        assert cmd[1] == "dup"
        assert "-d" in cmd
        assert "-s" in cmd
        idx = cmd.index("-s")
        assert cmd[idx + 1] == "HASH"

    def test_duplicate_name_command(self):
        w = _make_worker(ActiveTab.DUPLICATE_FILES, dup_check_method=CheckingMethod.NAME)
        cmd = w._build_command()
        idx = cmd.index("-s")
        assert cmd[idx + 1] == "NAME"

    def test_empty_folders_command(self):
        w = _make_worker(ActiveTab.EMPTY_FOLDERS)
        cmd = w._build_command()
        assert cmd[1] == "empty-folders"

    def test_big_files_command(self):
        w = _make_worker(ActiveTab.BIG_FILES)
        cmd = w._build_command()
        assert cmd[1] == "big"
        assert "-n" in cmd

    def test_big_files_smallest(self):
        w = _make_worker(ActiveTab.BIG_FILES, big_files_mode="smallest")
        cmd = w._build_command()
        assert "-J" in cmd

    def test_similar_images_command(self):
        w = _make_worker(ActiveTab.SIMILAR_IMAGES)
        cmd = w._build_command()
        assert cmd[1] == "image"
        assert "-g" in cmd
        assert "-c" in cmd

    def test_similar_videos_command(self):
        w = _make_worker(ActiveTab.SIMILAR_VIDEOS)
        cmd = w._build_command()
        assert cmd[1] == "video"

    def test_music_tags_command(self):
        w = _make_worker(ActiveTab.SIMILAR_MUSIC)
        cmd = w._build_command()
        assert cmd[1] == "music"
        assert "-s" in cmd
        idx = cmd.index("-s")
        assert cmd[idx + 1] == "TAGS"

    def test_music_content_command(self):
        w = _make_worker(ActiveTab.SIMILAR_MUSIC, music_search_method=MusicSearchMethod.CONTENT)
        cmd = w._build_command()
        idx = cmd.index("-s")
        assert cmd[idx + 1] == "CONTENT"

    def test_broken_files_command(self):
        w = _make_worker(ActiveTab.BROKEN_FILES)
        cmd = w._build_command()
        assert cmd[1] == "broken"

    def test_bad_extensions_command(self):
        w = _make_worker(ActiveTab.BAD_EXTENSIONS)
        cmd = w._build_command()
        assert cmd[1] == "ext"

    def test_bad_names_command(self):
        w = _make_worker(ActiveTab.BAD_NAMES)
        cmd = w._build_command()
        assert cmd[1] == "bad-names"

    def test_empty_files_command(self):
        w = _make_worker(ActiveTab.EMPTY_FILES)
        cmd = w._build_command()
        assert cmd[1] == "empty-files"

    def test_temp_files_command(self):
        w = _make_worker(ActiveTab.TEMPORARY_FILES)
        cmd = w._build_command()
        assert cmd[1] == "temp"

    def test_symlinks_command(self):
        w = _make_worker(ActiveTab.INVALID_SYMLINKS)
        cmd = w._build_command()
        assert cmd[1] == "symlinks"

    def test_no_recursive_flag(self):
        w = _make_worker(ActiveTab.DUPLICATE_FILES)
        w.app_settings.recursive_search = False
        cmd = w._build_command()
        assert "-R" in cmd

    def test_thread_number(self):
        w = _make_worker(ActiveTab.DUPLICATE_FILES)
        w.app_settings.thread_number = 4
        cmd = w._build_command()
        assert "-T" in cmd
        idx = cmd.index("-T")
        assert cmd[idx + 1] == "4"

    def test_excluded_paths(self):
        w = _make_worker(ActiveTab.DUPLICATE_FILES)
        w.app_settings.excluded_paths = ["/tmp/exclude"]
        cmd = w._build_command()
        assert "-e" in cmd

    def test_allowed_extensions(self):
        w = _make_worker(ActiveTab.DUPLICATE_FILES)
        w.app_settings.allowed_extensions = "jpg,png"
        cmd = w._build_command()
        assert "-x" in cmd


class TestResultParsing:
    def test_parse_flat_results(self):
        w = _make_worker(ActiveTab.EMPTY_FILES)
        data = [
            {"path": "/tmp/a.txt", "size": 0, "modified_date": 1000},
            {"path": "/tmp/b.txt", "size": 0, "modified_date": 2000},
        ]
        with tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False) as f:
            json.dump(data, f)
            path = f.name
        try:
            results = w._parse_results(path)
            assert len(results) == 2
            assert results[0].values["File Name"] == "a.txt"
            assert results[1].values["__full_path"] == "/tmp/b.txt"
        finally:
            os.unlink(path)

    def test_parse_grouped_results_dict(self):
        w = _make_worker(ActiveTab.DUPLICATE_FILES)
        data = {
            "1024": [
                [
                    {"path": "/a/file.txt", "size": 1024, "modified_date": 100, "hash": "abc"},
                    {"path": "/b/file.txt", "size": 1024, "modified_date": 200, "hash": "abc"},
                ]
            ]
        }
        with tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False) as f:
            json.dump(data, f)
            path = f.name
        try:
            results = w._parse_results(path)
            headers = [r for r in results if r.header_row]
            files = [r for r in results if not r.header_row]
            assert len(headers) == 1
            assert len(files) == 2
            assert files[0].group_id == 0
            assert files[1].group_id == 0
        finally:
            os.unlink(path)

    def test_parse_empty_json(self):
        w = _make_worker(ActiveTab.EMPTY_FILES)
        with tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False) as f:
            json.dump([], f)
            path = f.name
        try:
            results = w._parse_results(path)
            assert results == []
        finally:
            os.unlink(path)

    def test_parse_missing_file(self):
        w = _make_worker(ActiveTab.EMPTY_FILES)
        results = w._parse_results("/nonexistent/file.json")
        assert results == []

    def test_format_size(self):
        assert ScanWorker._format_size(0) == "0 B"
        assert ScanWorker._format_size(512) == "512 B"
        assert ScanWorker._format_size(1024) == "1.0 KB"
        assert ScanWorker._format_size(1048576) == "1.0 MB"
        assert ScanWorker._format_size(1073741824) == "1.0 GB"

    def test_format_date(self):
        assert ScanWorker._format_date(0) == ""
        result = ScanWorker._format_date(1700000000)
        assert "2023" in result  # Nov 2023
