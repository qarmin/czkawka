"""Internationalization support for Kalka using Fluent (.ftl) files.

Follows the same Fluent format as krokiet for consistency.
Translation files are stored in kalka/i18n/{locale}/kalka.ftl.
"""

import locale
import os
from pathlib import Path

from fluent.runtime import FluentLocalization, FluentResourceLoader


_I18N_DIR = Path(__file__).parent.parent / "i18n"
_RESOURCE_FILE = "kalka.ftl"
_FALLBACK_LOCALE = "en"

# Available locales (directories under i18n/ that contain kalka.ftl)
AVAILABLE_LOCALES: list[str] = []

# Active localization instance
_l10n: FluentLocalization | None = None


def _detect_system_locale() -> str:
    """Detect system locale, returning a BCP47-style tag like 'en', 'pl', 'zh-CN'."""
    for env_var in ("LC_MESSAGES", "LC_ALL", "LANG", "LANGUAGE"):
        val = os.environ.get(env_var, "")
        if val:
            # Strip encoding (e.g. "pl_PL.UTF-8" -> "pl_PL")
            val = val.split(".")[0].split("@")[0]
            break
    else:
        val, _ = locale.getdefaultlocale()
        if not val:
            return _FALLBACK_LOCALE
    # Convert underscore to hyphen: pl_PL -> pl-PL
    val = val.replace("_", "-")
    return val


def _discover_locales() -> list[str]:
    """Find all locale directories that contain a kalka.ftl file."""
    locales = []
    if _I18N_DIR.is_dir():
        for d in sorted(_I18N_DIR.iterdir()):
            if d.is_dir() and (d / _RESOURCE_FILE).is_file():
                locales.append(d.name)
    return locales


def _match_locale(requested: str, available: list[str]) -> list[str]:
    """Build a locale negotiation chain: exact -> language-only -> fallback."""
    chain = []
    if requested in available:
        chain.append(requested)
    # Try language-only (e.g. "zh-CN" -> "zh")
    lang = requested.split("-")[0]
    if lang != requested and lang in available:
        chain.append(lang)
    # Always end with fallback
    if _FALLBACK_LOCALE not in chain:
        chain.append(_FALLBACK_LOCALE)
    return chain


def init(locale_override: str | None = None):
    """Initialize the localization system.

    Call this once at startup. If locale_override is None,
    the system locale is auto-detected.
    """
    global _l10n, AVAILABLE_LOCALES

    AVAILABLE_LOCALES = _discover_locales()
    if not AVAILABLE_LOCALES:
        AVAILABLE_LOCALES = [_FALLBACK_LOCALE]

    requested = locale_override or _detect_system_locale()
    chain = _match_locale(requested, AVAILABLE_LOCALES)

    loader = FluentResourceLoader(str(_I18N_DIR / "{locale}"))
    _l10n = FluentLocalization(chain, [_RESOURCE_FILE], loader)


def set_locale(locale_code: str):
    """Switch to a different locale at runtime."""
    init(locale_override=locale_code)


def tr(msg_id: str, **kwargs) -> str:
    """Translate a message ID, with optional keyword arguments for placeholders.

    Usage:
        tr("scan-button")                          # simple
        tr("scan-complete", count=42)               # with variable
        tr("deleted-files", deleted=5, total=10)    # multiple variables
    """
    if _l10n is None:
        init()
    value = _l10n.format_value(msg_id, kwargs if kwargs else None)
    # fluent returns the msg_id itself if not found
    return value


def get_current_locale() -> str:
    """Return the first (best-match) locale in the current chain."""
    if _l10n is None:
        init()
    return _l10n.locales[0] if _l10n.locales else _FALLBACK_LOCALE
