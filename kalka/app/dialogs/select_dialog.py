from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QHBoxLayout, QLabel, QPushButton,
    QCheckBox, QGroupBox, QComboBox, QFrame
)
from PySide6.QtCore import Signal

from ..models import SelectMode
from ..localizer import tr


class SelectDialog(QDialog):
    """Dialog for selecting/deselecting results with combinable criteria."""
    mode_selected = Signal(object)  # SelectMode
    custom_criteria_selected = Signal(list, str)  # (list of SelectMode, combinator "AND"/"OR")

    # Simple modes (apply directly)
    SIMPLE_MODES = [
        (SelectMode.SELECT_ALL, "select-all"),
        (SelectMode.UNSELECT_ALL, "unselect-all"),
        (SelectMode.INVERT_SELECTION, "invert-selection"),
    ]

    # Combinable criteria (can be combined with AND/OR)
    CRITERIA = [
        (SelectMode.SELECT_BIGGEST_SIZE, "select-biggest-size"),
        (SelectMode.SELECT_SMALLEST_SIZE, "select-smallest-size"),
        (SelectMode.SELECT_NEWEST, "select-newest"),
        (SelectMode.SELECT_OLDEST, "select-oldest"),
        (SelectMode.SELECT_SHORTEST_PATH, "select-shortest-path"),
        (SelectMode.SELECT_LONGEST_PATH, "select-longest-path"),
    ]

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setWindowTitle(tr("select-dialog-title"))
        self.setMinimumWidth(350)

        layout = QVBoxLayout(self)

        # Simple selection buttons
        simple_group = QGroupBox("Quick selection")
        simple_layout = QHBoxLayout(simple_group)
        for mode, ftl_key in self.SIMPLE_MODES:
            btn = QPushButton(tr(ftl_key))
            btn.clicked.connect(lambda checked, m=mode: self._select_simple(m))
            simple_layout.addWidget(btn)
        layout.addWidget(simple_group)

        # Separator
        line = QFrame()
        line.setFrameShape(QFrame.HLine)
        line.setFrameShadow(QFrame.Sunken)
        layout.addWidget(line)

        # Combined criteria
        criteria_group = QGroupBox("Smart selection (combinable)")
        criteria_layout = QVBoxLayout(criteria_group)

        criteria_layout.addWidget(QLabel(
            "Check multiple criteria to combine them.\n"
            "AND = file must match ALL checked criteria.\n"
            "OR = file must match ANY checked criterion."
        ))

        self._checkboxes: list[tuple[QCheckBox, SelectMode]] = []
        for mode, ftl_key in self.CRITERIA:
            cb = QCheckBox(tr(ftl_key))
            criteria_layout.addWidget(cb)
            self._checkboxes.append((cb, mode))

        # Combinator selector
        combinator_layout = QHBoxLayout()
        combinator_layout.addWidget(QLabel("Combine with:"))
        self._combinator = QComboBox()
        self._combinator.addItems(["AND (all must match)", "OR (any must match)"])
        combinator_layout.addWidget(self._combinator)
        criteria_layout.addLayout(combinator_layout)

        # Apply combined button
        apply_btn = QPushButton("Apply combined selection")
        apply_btn.clicked.connect(self._apply_combined)
        criteria_layout.addWidget(apply_btn)

        layout.addWidget(criteria_group)

        # Cancel
        cancel = QPushButton(tr("cancel"))
        cancel.clicked.connect(self.reject)
        layout.addWidget(cancel)

    def _select_simple(self, mode: SelectMode):
        self.mode_selected.emit(mode)
        self.accept()

    def _apply_combined(self):
        selected = [mode for cb, mode in self._checkboxes if cb.isChecked()]
        if not selected:
            return
        if len(selected) == 1:
            self.mode_selected.emit(selected[0])
        else:
            combinator = "AND" if self._combinator.currentIndex() == 0 else "OR"
            self.custom_criteria_selected.emit(selected, combinator)
        self.accept()
