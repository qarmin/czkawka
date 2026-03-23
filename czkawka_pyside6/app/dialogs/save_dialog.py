from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QLabel, QDialogButtonBox,
    QCheckBox, QFileDialog
)


class SaveDialog:
    """Save results to file (uses native file dialog)."""

    @staticmethod
    def save(parent, results: list, save_as_json: bool = False) -> bool:
        if save_as_json:
            filter_str = "JSON Files (*.json);;All Files (*)"
            default_ext = ".json"
        else:
            filter_str = "Text Files (*.txt);;All Files (*)"
            default_ext = ".txt"

        path, _ = QFileDialog.getSaveFileName(
            parent, "Save Results", f"results{default_ext}", filter_str
        )
        if not path:
            return False

        try:
            import json
            if save_as_json:
                data = []
                for entry in results:
                    if not entry.header_row:
                        # Filter out internal keys
                        values = {k: v for k, v in entry.values.items()
                                  if not k.startswith("__")}
                        data.append(values)
                with open(path, "w") as f:
                    json.dump(data, f, indent=2)
            else:
                with open(path, "w") as f:
                    for entry in results:
                        if entry.header_row:
                            f.write(f"\n--- {entry.values.get('__header', 'Group')} ---\n")
                        else:
                            path_val = entry.values.get("__full_path", "")
                            size = entry.values.get("Size", "")
                            f.write(f"{path_val}\t{size}\n")
            return True
        except OSError:
            return False
