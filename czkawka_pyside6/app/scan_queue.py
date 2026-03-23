from collections import deque
from PySide6.QtCore import QObject, Signal
from .models import ActiveTab


class ScanQueue(QObject):
    """Queue multiple scan types and run them sequentially."""

    queue_updated = Signal(int)  # queue size
    queue_finished = Signal()
    next_scan = Signal(object)  # ActiveTab to scan next

    def __init__(self, parent=None):
        super().__init__(parent)
        self._queue: deque[ActiveTab] = deque()
        self._running = False

    def add(self, tab: ActiveTab):
        if tab not in self._queue:
            self._queue.append(tab)
            self.queue_updated.emit(len(self._queue))

    def add_all(self, tabs: list[ActiveTab]):
        for tab in tabs:
            self.add(tab)

    def start(self):
        self._running = True
        self._run_next()

    def stop(self):
        self._running = False
        self._queue.clear()
        self.queue_updated.emit(0)

    def on_scan_completed(self):
        """Called when a scan finishes. Triggers next in queue."""
        if self._running:
            self._run_next()

    def _run_next(self):
        if self._queue:
            tab = self._queue.popleft()
            self.queue_updated.emit(len(self._queue))
            self.next_scan.emit(tab)
        else:
            self._running = False
            self.queue_finished.emit()

    @property
    def is_running(self) -> bool:
        return self._running

    @property
    def pending_count(self) -> int:
        return len(self._queue)
