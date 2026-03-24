import json
from datetime import datetime
from pathlib import Path
from dataclasses import dataclass, asdict
from PySide6.QtCore import QStandardPaths


@dataclass
class ScanRecord:
    timestamp: str
    tool: str
    directories: list[str]
    entries_found: int
    groups_found: int
    duration_seconds: float


class ScanHistory:
    """Persists a log of past scans."""

    MAX_RECORDS = 100

    def __init__(self):
        config_dir = QStandardPaths.writableLocation(QStandardPaths.AppConfigLocation)
        self._path = Path(config_dir) / "scan_history.json" if config_dir else Path.home() / ".config" / "czkawka" / "scan_history.json"
        self._records: list[ScanRecord] = []
        self._load()

    def add(self, tool: str, directories: list[str], entries: int,
            groups: int, duration: float):
        record = ScanRecord(
            timestamp=datetime.now().isoformat(timespec="seconds"),
            tool=tool,
            directories=directories,
            entries_found=entries,
            groups_found=groups,
            duration_seconds=round(duration, 1),
        )
        self._records.append(record)
        if len(self._records) > self.MAX_RECORDS:
            self._records = self._records[-self.MAX_RECORDS:]
        self._save()

    def get_records(self) -> list[ScanRecord]:
        return list(self._records)

    def clear(self):
        self._records = []
        self._save()

    def _save(self):
        try:
            self._path.parent.mkdir(parents=True, exist_ok=True)
            data = [asdict(r) for r in self._records]
            self._path.write_text(json.dumps(data, indent=2))
        except OSError:
            pass

    def _load(self):
        try:
            if self._path.exists():
                data = json.loads(self._path.read_text())
                self._records = [ScanRecord(**d) for d in data]
        except (json.JSONDecodeError, OSError, TypeError):
            self._records = []
