import json
from pathlib import Path
from PySide6.QtCore import QObject, Signal
from .models import (
    ActiveTab, AppSettings, ToolSettings, ResultEntry, ScanProgress
)


class AppState(QObject):
    """Central application state manager."""

    # Signals
    tab_changed = Signal(object)  # ActiveTab
    scan_started = Signal()
    scan_finished = Signal()
    scan_progress_updated = Signal(object)  # ScanProgress
    results_updated = Signal()
    settings_changed = Signal()
    included_paths_changed = Signal()
    excluded_paths_changed = Signal()
    preview_image_changed = Signal(str)

    def __init__(self):
        super().__init__()
        self.active_tab = ActiveTab.DUPLICATE_FILES
        self.scanning = False
        self.processing = False
        self.stop_requested = False
        self.settings = AppSettings()
        self.tool_settings = ToolSettings()
        self.results: dict[ActiveTab, list[ResultEntry]] = {}
        self.progress = ScanProgress()
        self.info_text = ""
        self.preview_image_path = ""
        self._config_path = Path.home() / ".config" / "czkawka_pyside6"
        self._config_path.mkdir(parents=True, exist_ok=True)
        self.load_settings()

    def set_active_tab(self, tab: ActiveTab):
        if tab != self.active_tab:
            self.active_tab = tab
            self.tab_changed.emit(tab)

    def set_scanning(self, scanning: bool):
        self.scanning = scanning
        if scanning:
            self.stop_requested = False
            self.scan_started.emit()
        else:
            self.scan_finished.emit()

    def request_stop(self):
        self.stop_requested = True

    def update_progress(self, progress: ScanProgress):
        self.progress = progress
        self.scan_progress_updated.emit(progress)

    def set_results(self, tab: ActiveTab, results: list[ResultEntry]):
        self.results[tab] = results
        self.results_updated.emit()

    def get_results(self, tab: ActiveTab = None) -> list[ResultEntry]:
        if tab is None:
            tab = self.active_tab
        return self.results.get(tab, [])

    def get_checked_results(self, tab: ActiveTab = None) -> list[ResultEntry]:
        return [r for r in self.get_results(tab) if r.checked and not r.header_row]

    def get_selected_count(self, tab: ActiveTab = None) -> int:
        return len(self.get_checked_results(tab))

    def save_settings(self):
        config_file = self._config_path / "settings.json"
        data = {
            "included_paths": self.settings.included_paths,
            "excluded_paths": self.settings.excluded_paths,
            "excluded_items": self.settings.excluded_items,
            "allowed_extensions": self.settings.allowed_extensions,
            "excluded_extensions": self.settings.excluded_extensions,
            "minimum_file_size": self.settings.minimum_file_size,
            "maximum_file_size": self.settings.maximum_file_size,
            "recursive_search": self.settings.recursive_search,
            "use_cache": self.settings.use_cache,
            "save_as_json": self.settings.save_as_json,
            "move_to_trash": self.settings.move_to_trash,
            "hide_hard_links": self.settings.hide_hard_links,
            "thread_number": self.settings.thread_number,
            "dark_theme": self.settings.dark_theme,
            "show_image_preview": self.settings.show_image_preview,
            "czkawka_cli_path": self.settings.czkawka_cli_path,
        }
        try:
            config_file.write_text(json.dumps(data, indent=2))
        except OSError:
            pass

    def load_settings(self):
        config_file = self._config_path / "settings.json"
        if config_file.exists():
            try:
                data = json.loads(config_file.read_text())
                s = self.settings
                s.included_paths = data.get("included_paths", s.included_paths)
                s.excluded_paths = data.get("excluded_paths", s.excluded_paths)
                s.excluded_items = data.get("excluded_items", s.excluded_items)
                s.allowed_extensions = data.get("allowed_extensions", s.allowed_extensions)
                s.excluded_extensions = data.get("excluded_extensions", s.excluded_extensions)
                s.minimum_file_size = data.get("minimum_file_size", s.minimum_file_size)
                s.maximum_file_size = data.get("maximum_file_size", s.maximum_file_size)
                s.recursive_search = data.get("recursive_search", s.recursive_search)
                s.use_cache = data.get("use_cache", s.use_cache)
                s.save_as_json = data.get("save_as_json", s.save_as_json)
                s.move_to_trash = data.get("move_to_trash", s.move_to_trash)
                s.hide_hard_links = data.get("hide_hard_links", s.hide_hard_links)
                s.thread_number = data.get("thread_number", s.thread_number)
                s.dark_theme = data.get("dark_theme", s.dark_theme)
                s.show_image_preview = data.get("show_image_preview", s.show_image_preview)
                s.czkawka_cli_path = data.get("czkawka_cli_path", s.czkawka_cli_path)
            except (json.JSONDecodeError, OSError):
                pass
