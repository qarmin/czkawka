import csv
import io
import json
from pathlib import Path

from PySide6.QtWidgets import QFileDialog

from ..localizer import tr
from ..models import ResultEntry, TAB_COLUMNS, ActiveTab


class SaveDialog:
    """Save/load results to/from file."""

    # Columns to exclude from export (internal metadata)
    _INTERNAL_KEYS = {"__full_path", "__size_bytes", "__modified_date_ts", "__header", "__group_id", "__checked"}

    @staticmethod
    def save(parent, results: list, save_as_json: bool = False, active_tab: ActiveTab = None) -> bool:
        filter_str = (
            "CSV Files (*.csv);;"
            "JSON Files (*.json);;"
            "Text Files (*.txt);;"
            "All Files (*)"
        )
        default_ext = ".csv"

        path, selected_filter = QFileDialog.getSaveFileName(
            parent, tr("save-dialog-title"), f"results{default_ext}", filter_str
        )
        if not path:
            return False

        try:
            if path.endswith(".json") or "JSON" in selected_filter:
                return SaveDialog._save_json(path, results)
            elif path.endswith(".csv") or "CSV" in selected_filter:
                return SaveDialog._save_csv(path, results, active_tab)
            else:
                return SaveDialog._save_text(path, results)
        except OSError:
            return False

    @staticmethod
    def _save_json(path: str, results: list) -> bool:
        data = []
        for entry in results:
            if entry.header_row:
                data.append({
                    "__header": entry.values.get("__header", ""),
                    "__group_id": entry.group_id,
                })
            else:
                values = dict(entry.values)
                values["__group_id"] = entry.group_id
                values["__checked"] = entry.checked
                data.append(values)
        with open(path, "w") as f:
            json.dump(data, f, indent=2)
        return True

    @staticmethod
    def _save_csv(path: str, results: list, active_tab: ActiveTab = None) -> bool:
        # Determine columns from tab or from first non-header entry
        columns = []
        if active_tab and active_tab in TAB_COLUMNS:
            columns = [c for c in TAB_COLUMNS[active_tab] if c != "Selection"]
        if not columns:
            for entry in results:
                if not entry.header_row:
                    columns = [k for k in entry.values if k not in SaveDialog._INTERNAL_KEYS]
                    break

        columns_with_path = list(columns)
        if "Path" in columns_with_path:
            # Add full path column for convenience
            columns_with_path.append("Full Path")

        with open(path, "w", newline="", encoding="utf-8") as f:
            writer = csv.writer(f)
            writer.writerow(["Group"] + columns_with_path)

            current_group = ""
            for entry in results:
                if entry.header_row:
                    current_group = entry.values.get("__header", f"Group {entry.group_id}")
                    continue
                row = [current_group]
                for col in columns:
                    row.append(entry.values.get(col, ""))
                if "Path" in columns:
                    row.append(entry.values.get("__full_path", ""))
                writer.writerow(row)
        return True

    @staticmethod
    def _save_text(path: str, results: list) -> bool:
        with open(path, "w") as f:
            for entry in results:
                if entry.header_row:
                    f.write(f"\n--- {entry.values.get('__header', 'Group')} ---\n")
                else:
                    path_val = entry.values.get("__full_path", "")
                    size = entry.values.get("Size", "")
                    f.write(f"{path_val}\t{size}\n")
        return True

    @staticmethod
    def load(parent) -> list[ResultEntry] | None:
        """Load results from a previously saved JSON file.

        Returns list of ResultEntry on success, None on cancel/error.
        """
        path, _ = QFileDialog.getOpenFileName(
            parent, tr("load-dialog-title"),
            "",
            "JSON Files (*.json);;All Files (*)"
        )
        if not path:
            return None

        try:
            with open(path) as f:
                data = json.load(f)
        except (json.JSONDecodeError, OSError):
            return None

        if not isinstance(data, list):
            # Could be czkawka_cli dict format {size: [[entries]]}
            return _parse_cli_json(data)

        results = []
        for item in data:
            if not isinstance(item, dict):
                continue

            if "__header" in item:
                results.append(ResultEntry(
                    values={"__header": item["__header"]},
                    header_row=True,
                    group_id=item.get("__group_id", 0),
                ))
            else:
                group_id = item.pop("__group_id", 0)
                checked = item.pop("__checked", False)
                results.append(ResultEntry(
                    values=item,
                    checked=checked,
                    group_id=group_id,
                ))

        return results if results else None


def _parse_cli_json(data: dict) -> list[ResultEntry] | None:
    """Parse raw czkawka_cli JSON output (dict of size buckets or flat list)."""
    results = []
    group_id = 0

    for key, groups in data.items():
        if not isinstance(groups, list):
            continue
        for group in groups:
            if not isinstance(group, list) or len(group) == 0:
                continue

            total_size = sum(e.get("size", 0) for e in group if isinstance(e, dict))
            results.append(ResultEntry(
                values={"__header": f"Group {group_id + 1} ({len(group)} files)"},
                header_row=True,
                group_id=group_id,
            ))

            for entry in group:
                if not isinstance(entry, dict):
                    continue
                path = entry.get("path", "")
                p = Path(path)
                values = {
                    "File Name": p.name,
                    "Path": str(p.parent),
                    "Size": _format_size(entry.get("size", 0)),
                    "Modification Date": str(entry.get("modified_date", "")),
                    "Hash": entry.get("hash", ""),
                    "__full_path": path,
                    "__size_bytes": entry.get("size", 0),
                    "__modified_date_ts": entry.get("modified_date", 0),
                }
                results.append(ResultEntry(values=values, group_id=group_id))

            group_id += 1

    return results if results else None


def _format_size(size_bytes: int) -> str:
    if size_bytes == 0:
        return "0 B"
    units = ["B", "KB", "MB", "GB", "TB"]
    i = 0
    size = float(size_bytes)
    while size >= 1024 and i < len(units) - 1:
        size /= 1024
        i += 1
    return f"{size:.1f} {units[i]}" if i > 0 else f"{int(size)} B"
