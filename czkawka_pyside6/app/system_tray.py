from PySide6.QtWidgets import QSystemTrayIcon, QMenu
from PySide6.QtGui import QAction, QIcon


class SystemTray:
    """System tray icon with minimize-to-tray and scan notification."""

    def __init__(self, main_window):
        self._window = main_window
        self._tray = QSystemTrayIcon(main_window)

        icon = main_window.windowIcon()
        if not icon.isNull():
            self._tray.setIcon(icon)

        self._tray.setToolTip("Czkawka PySide6")

        menu = QMenu()
        show_action = QAction("Show/Hide", main_window)
        show_action.triggered.connect(self._toggle_window)
        menu.addAction(show_action)

        scan_action = QAction("Start Scan", main_window)
        scan_action.triggered.connect(main_window._start_scan)
        menu.addAction(scan_action)

        menu.addSeparator()

        quit_action = QAction("Quit", main_window)
        quit_action.triggered.connect(main_window.close)
        menu.addAction(quit_action)

        self._tray.setContextMenu(menu)
        self._tray.activated.connect(self._on_activated)
        self._tray.show()

    def _toggle_window(self):
        if self._window.isVisible():
            self._window.hide()
        else:
            self._window.show()
            self._window.raise_()
            self._window.activateWindow()

    def _on_activated(self, reason):
        if reason == QSystemTrayIcon.ActivationReason.Trigger:
            self._toggle_window()

    def notify(self, title: str, message: str):
        self._tray.showMessage(title, message, QSystemTrayIcon.MessageIcon.Information, 5000)
