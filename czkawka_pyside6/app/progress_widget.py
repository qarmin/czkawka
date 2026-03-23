import json
import time
from pathlib import Path

from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QProgressBar, QLabel, QHBoxLayout
)
from PySide6.QtCore import Qt, QTimer

from .models import ActiveTab, ScanProgress


class ProgressWidget(QWidget):
    """Two-bar progress widget matching Slint/Krokiet feature parity.

    Shows:
      - Current stage progress bar (entries or bytes within one stage)
      - Overall progress bar (across all stages)
      - Stage name with counts
      - Elapsed time
      - Phase step indicators
    """

    # File where we persist the last file-collection count per directory set,
    # so we can estimate the collection stage percentage on the next scan.
    _ESTIMATE_FILE = Path.home() / ".config" / "czkawka_pyside6" / "scan_estimates.json"

    _BAR_STYLE = """
        QProgressBar {{
            border: 1px solid #555; border-radius: 3px;
            text-align: center; background-color: #2a2a2a;
            font-size: 10px; color: #ccc;
        }}
        QProgressBar::chunk {{
            background: qlineargradient(x1:0, y1:0, x2:1, y2:0,
                stop:0 {c1}, stop:1 {c2});
            border-radius: 2px;
        }}
    """

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setVisible(False)
        self._start_time = 0.0
        self._active_tab = ActiveTab.DUPLICATE_FILES
        self._last_collection_count = 0  # Files found during collection phase
        self._estimates: dict[str, int] = {}
        self._load_estimates()
        self._setup_ui()

        self._timer = QTimer(self)
        self._timer.setInterval(500)
        self._timer.timeout.connect(self._update_elapsed)

    # ── UI setup ──────────────────────────────────────────────

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(8, 6, 8, 6)
        layout.setSpacing(3)

        # Row 1: stage label + elapsed
        row1 = QHBoxLayout()
        self._stage_label = QLabel("Initializing...")
        self._stage_label.setStyleSheet("font-weight: bold; font-size: 12px;")
        row1.addWidget(self._stage_label)
        row1.addStretch()
        self._elapsed_label = QLabel("")
        self._elapsed_label.setStyleSheet("color: #888; font-size: 11px;")
        row1.addWidget(self._elapsed_label)
        layout.addLayout(row1)

        # Row 2: current stage bar  "Current stage"  NN%
        row2 = QHBoxLayout()
        row2.setSpacing(6)
        lbl2 = QLabel("Current")
        lbl2.setStyleSheet("color: #aaa; font-size: 10px;")
        lbl2.setFixedWidth(48)
        row2.addWidget(lbl2)
        self._stage_bar = QProgressBar()
        self._stage_bar.setFixedHeight(14)
        self._stage_bar.setTextVisible(False)
        self._stage_bar.setStyleSheet(
            self._BAR_STYLE.format(c1="#1b4332", c2="#2d6a4f")
        )
        row2.addWidget(self._stage_bar)
        self._stage_pct = QLabel("")
        self._stage_pct.setFixedWidth(40)
        self._stage_pct.setAlignment(Qt.AlignRight | Qt.AlignVCenter)
        self._stage_pct.setStyleSheet("color: #aaa; font-size: 10px;")
        row2.addWidget(self._stage_pct)
        layout.addLayout(row2)

        # Row 3: overall bar  "Overall"  NN%
        row3 = QHBoxLayout()
        row3.setSpacing(6)
        lbl3 = QLabel("Overall")
        lbl3.setStyleSheet("color: #aaa; font-size: 10px;")
        lbl3.setFixedWidth(48)
        row3.addWidget(lbl3)
        self._overall_bar = QProgressBar()
        self._overall_bar.setFixedHeight(14)
        self._overall_bar.setTextVisible(False)
        self._overall_bar.setStyleSheet(
            self._BAR_STYLE.format(c1="#2d6a4f", c2="#40916c")
        )
        row3.addWidget(self._overall_bar)
        self._overall_pct = QLabel("")
        self._overall_pct.setFixedWidth(40)
        self._overall_pct.setAlignment(Qt.AlignRight | Qt.AlignVCenter)
        self._overall_pct.setStyleSheet("color: #aaa; font-size: 10px;")
        row3.addWidget(self._overall_pct)
        layout.addLayout(row3)

        # Row 4: detail counts
        row4 = QHBoxLayout()
        self._detail_label = QLabel("")
        self._detail_label.setStyleSheet("color: #888; font-size: 10px;")
        row4.addWidget(self._detail_label)
        row4.addStretch()
        self._size_label = QLabel("")
        self._size_label.setStyleSheet("color: #888; font-size: 10px;")
        self._size_label.setAlignment(Qt.AlignRight | Qt.AlignVCenter)
        row4.addWidget(self._size_label)
        layout.addLayout(row4)

        # Row 5: step indicators
        self._steps_label = QLabel("")
        self._steps_label.setStyleSheet("color: #666; font-size: 9px;")
        self._steps_label.setAlignment(Qt.AlignCenter)
        self._steps_label.setWordWrap(True)
        layout.addWidget(self._steps_label)

    # ── Public API ────────────────────────────────────────────

    def start(self, tab: ActiveTab = None):
        if tab is not None:
            self._active_tab = tab
        self._start_time = time.monotonic()
        self._last_collection_count = 0
        self.setVisible(True)

        for bar in (self._stage_bar, self._overall_bar):
            bar.setMaximum(0)  # indeterminate
            bar.setValue(0)
        self._stage_pct.setText("")
        self._overall_pct.setText("")
        self._stage_label.setText("Starting scan...")
        self._detail_label.setText("")
        self._size_label.setText("")
        self._elapsed_label.setText("0s")
        self._steps_label.setText("")
        self._timer.start()

    def stop(self):
        self._timer.stop()
        elapsed = time.monotonic() - self._start_time if self._start_time else 0
        self._elapsed_label.setText(f"Completed in {self._format_time(elapsed)}")

        for bar, lbl in ((self._stage_bar, self._stage_pct),
                         (self._overall_bar, self._overall_pct)):
            bar.setMaximum(100)
            bar.setValue(100)
            lbl.setText("100%")

        self._stage_label.setText("Scan complete")
        self._steps_label.setText("")

        # Save collection count for next-scan estimation
        if self._last_collection_count > 0:
            self._save_estimate(self._last_collection_count)

        QTimer.singleShot(3000, self._auto_hide)

    def update_progress(self, progress: ScanProgress):
        """Main update method called by the scan runner."""
        stage_name = progress.stage_name or progress.step_name or ""
        idx = progress.current_stage_idx
        max_idx = progress.max_stage_idx
        checked = progress.entries_checked
        to_check = progress.entries_to_check
        b_checked = progress.bytes_checked
        b_to_check = progress.bytes_to_check

        # ── Stage label ──
        self._stage_label.setText(stage_name)

        # ── Update step indicators using stage index ──
        if max_idx > 0:
            self._update_steps_from_index(idx, max_idx)

        # ── Current-stage bar ──
        is_collecting = (idx == 0 and to_check == 0)

        if is_collecting:
            # Collection phase: use estimate from previous scan
            self._last_collection_count = max(self._last_collection_count, checked)
            estimate = self._get_estimate()
            if estimate > 0 and checked > 0:
                pct = min(99, int(checked * 100 / estimate))
                self._stage_bar.setMaximum(100)
                self._stage_bar.setValue(pct)
                self._stage_pct.setText(f"~{pct}%")
                self._detail_label.setText(f"{checked:,} / ~{estimate:,} files")
            else:
                self._stage_bar.setMaximum(0)  # indeterminate
                self._stage_pct.setText("")
                self._detail_label.setText(f"{checked:,} files" if checked else "")

        elif to_check > 0:
            # Normal stage with known total
            if b_to_check > 0:
                # Byte-based progress (hashing)
                pct = min(99, int(b_checked * 100 / b_to_check))
                self._stage_bar.setMaximum(100)
                self._stage_bar.setValue(pct)
                self._stage_pct.setText(f"{pct}%")
                self._detail_label.setText(f"{checked:,} / {to_check:,}")
                self._size_label.setText(
                    f"{self._format_size(b_checked)} / {self._format_size(b_to_check)}"
                )
            else:
                # Entry-count based progress
                pct = min(99, int(checked * 100 / to_check))
                self._stage_bar.setMaximum(100)
                self._stage_bar.setValue(pct)
                self._stage_pct.setText(f"{pct}%")
                self._detail_label.setText(f"{checked:,} / {to_check:,}")
                self._size_label.setText("")

        else:
            # Cache loading/saving or unknown total
            self._stage_bar.setMaximum(0)  # indeterminate spinner
            self._stage_pct.setText("")
            self._detail_label.setText("")
            self._size_label.setText("")

        # ── Overall bar ──
        if max_idx > 0:
            if to_check > 0:
                stage_frac = (b_checked / b_to_check) if b_to_check > 0 else (checked / to_check)
            elif is_collecting and self._get_estimate() > 0 and checked > 0:
                stage_frac = min(0.99, checked / self._get_estimate())
            else:
                stage_frac = 0
            overall = (idx + min(stage_frac, 0.99)) / (max_idx + 1)
            overall_pct = min(99, int(overall * 100))
            self._overall_bar.setMaximum(100)
            self._overall_bar.setValue(overall_pct)
            self._overall_pct.setText(f"{overall_pct}%")
        else:
            self._overall_bar.setMaximum(0)
            self._overall_pct.setText("")

    # ── Collection estimate persistence ───────────────────────

    def _get_estimate_key(self) -> str:
        """Key for the estimate cache based on active tab."""
        return self._active_tab.name

    def _get_estimate(self) -> int:
        return self._estimates.get(self._get_estimate_key(), 0)

    def _save_estimate(self, count: int):
        self._estimates[self._get_estimate_key()] = count
        try:
            self._ESTIMATE_FILE.parent.mkdir(parents=True, exist_ok=True)
            self._ESTIMATE_FILE.write_text(json.dumps(self._estimates))
        except OSError:
            pass

    def _load_estimates(self):
        try:
            if self._ESTIMATE_FILE.exists():
                self._estimates = json.loads(self._ESTIMATE_FILE.read_text())
        except (json.JSONDecodeError, OSError):
            self._estimates = {}

    # ── Step indicator ────────────────────────────────────────

    def _update_steps_from_index(self, current_idx: int, max_idx: int):
        """Build step display directly from stage index."""
        total_stages = max_idx + 1
        parts = []
        for i in range(total_stages):
            if i < current_idx:
                parts.append(f"[{i+1} done]")
            elif i == current_idx:
                parts.append(f"[{i+1} >>]")
            else:
                parts.append(f"[{i+1}]")
        self._steps_label.setText("  ".join(parts))

    # ── Internals ─────────────────────────────────────────────

    def _auto_hide(self):
        self.setVisible(False)

    def _update_elapsed(self):
        elapsed = time.monotonic() - self._start_time
        self._elapsed_label.setText(self._format_time(elapsed))

    @staticmethod
    def _format_time(seconds: float) -> str:
        if seconds < 60:
            return f"{int(seconds)}s"
        minutes = int(seconds // 60)
        secs = int(seconds % 60)
        if minutes < 60:
            return f"{minutes}m {secs}s"
        hours = minutes // 60
        mins = minutes % 60
        return f"{hours}h {mins}m"

    @staticmethod
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
