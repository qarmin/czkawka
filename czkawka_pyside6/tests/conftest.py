import pytest
import sys
import os

os.environ["QT_QPA_PLATFORM"] = "offscreen"

# Add parent to path so 'app' package is importable
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
