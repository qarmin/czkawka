#!/usr/bin/env python3

"""Detect characters from foreign writing systems in FTL translation files.

AI translation tools sometimes leak characters from completely unrelated
scripts into a translation (e.g. CJK ideographs, Cyrillic or Katakana inside a
Persian/Arabic file), or even paste their own chain-of-thought in another
language instead of the translation. Placeholder validation does not catch
this, so this script flags any *letter* whose script is not expected for the
target language.

What is always accepted, regardless of language:
  - basic ASCII letters A-Z / a-z  (brand and technical terms: MIT, JSON, gtk, https, regex, ...)
  - digits, punctuation, symbols, separators, combining marks, control/format chars
    (so placeholders like { $name }, Arabic-Indic / Persian digits and ZWNJ are fine)

Only letters (Unicode category L*) are script-checked. A letter is reported when
its script is not in the set expected for the language.

Usage:
  python3 misc/ai_translate/validate_charset.py krokiet/i18n
  python3 misc/ai_translate/validate_charset.py czkawka_core/i18n --languages fa ar
"""

import argparse
import pathlib
import sys
import unicodedata
from typing import Dict, List, Set, Tuple

from ftl_utils import LANGUAGE_NAMES, find_ftl_file_in_folder, parse_ftl_file


class Colors:
    GREEN = "\033[92m"
    RED = "\033[91m"
    YELLOW = "\033[93m"
    BLUE = "\033[94m"
    RESET = "\033[0m"
    BOLD = "\033[1m"


# Script that each language is written in. ASCII Latin is always allowed on top
# of this (for brand/technical terms), so e.g. "fa" expects Arabic only.
EXPECTED_SCRIPTS: Dict[str, Set[str]] = {
    "ar": {"Arabic"},
    "fa": {"Arabic"},
    "bg": {"Cyrillic"},
    "ru": {"Cyrillic"},
    "uk": {"Cyrillic"},
    "el": {"Greek"},
    "hi": {"Devanagari"},
    "ja": {"Han", "Hiragana", "Katakana"},
    "ko": {"Hangul", "Han"},
    "zh-CN": {"Han"},
    "zh-TW": {"Han"},
    # Latin-script languages
    "cs": {"Latin"},
    "de": {"Latin"},
    "en": {"Latin"},
    "es-ES": {"Latin"},
    "fr": {"Latin"},
    "id": {"Latin"},
    "it": {"Latin"},
    "nl": {"Latin"},
    "no": {"Latin"},
    "pl": {"Latin"},
    "pt-BR": {"Latin"},
    "pt-PT": {"Latin"},
    "ro": {"Latin"},
    "sv-SE": {"Latin"},
    "tr": {"Latin"},
    "vi": {"Latin"},
}

# (start, end, script) ranges, only needed to classify *letters*.
_SCRIPT_RANGES: List[Tuple[int, int, str]] = [
    (0x0041, 0x024F, "Latin"),
    (0x0250, 0x02AF, "Latin"),  # IPA extensions
    (0x1E00, 0x1EFF, "Latin"),  # Latin Extended Additional (Vietnamese)
    (0x0370, 0x03FF, "Greek"),
    (0x1F00, 0x1FFF, "Greek"),
    (0x0400, 0x052F, "Cyrillic"),
    (0x0590, 0x05FF, "Hebrew"),
    (0x0600, 0x06FF, "Arabic"),
    (0x0750, 0x077F, "Arabic"),
    (0x08A0, 0x08FF, "Arabic"),
    (0xFB50, 0xFDFF, "Arabic"),
    (0xFE70, 0xFEFF, "Arabic"),
    (0x0900, 0x097F, "Devanagari"),
    (0x0980, 0x09FF, "Bengali"),
    (0x0E00, 0x0E7F, "Thai"),
    (0x1100, 0x11FF, "Hangul"),
    (0x3130, 0x318F, "Hangul"),
    (0xAC00, 0xD7A3, "Hangul"),
    (0x3040, 0x309F, "Hiragana"),
    (0x30A0, 0x30FF, "Katakana"),
    (0x31F0, 0x31FF, "Katakana"),
    (0x3400, 0x4DBF, "Han"),
    (0x4E00, 0x9FFF, "Han"),
    (0xF900, 0xFAFF, "Han"),
    (0x20000, 0x2A6DF, "Han"),
]


def script_of_letter(ch: str) -> str:
    cp = ord(ch)
    for start, end, script in _SCRIPT_RANGES:
        if start <= cp <= end:
            return script
    if 0xFF00 <= cp <= 0xFFEF:
        return "CJK-fullwidth"
    return "Other"


def find_foreign_chars(value: str, allowed: Set[str], source_chars: Set[str]) -> Dict[str, str]:
    """Return {char: script} for every disallowed letter in value.

    Letters that also appear in the English source string for the same key are
    ignored - they are intentional examples (e.g. the BadNames hint lists
    "ą, ć, ñ", and some tooltips use "Żołd" as an example)."""
    bad: Dict[str, str] = {}
    for ch in value:
        # ASCII letters are always fine (brands / tech terms).
        if ("A" <= ch <= "Z") or ("a" <= ch <= "z"):
            continue
        cat = unicodedata.category(ch)
        # Only letters are script-checked; marks/digits/punct/symbols/separators/control are fine.
        if not cat.startswith("L"):
            continue
        if ch in source_chars:
            continue
        script = script_of_letter(ch)
        if script not in allowed:
            bad[ch] = script
    return bad


def describe_chars(chars: Dict[str, str]) -> str:
    parts = []
    for ch, script in sorted(chars.items(), key=lambda kv: ord(kv[0])):
        parts.append(f"{ch!r} (U+{ord(ch):04X} {script})")
    return ", ".join(parts)


def validate_language_file(
    lang_file: pathlib.Path, allowed: Set[str], base_entries: Dict[str, str]
) -> Dict[str, Tuple[Dict[str, str], str]]:
    entries = parse_ftl_file(lang_file)
    problems: Dict[str, Tuple[Dict[str, str], str]] = {}
    for key, value in entries.items():
        source_chars = set(base_entries.get(key, ""))
        bad = find_foreign_chars(value, allowed, source_chars)
        if bad:
            problems[key] = (bad, value)
    return problems


def validate_i18n_folder(i18n_path: pathlib.Path, target_languages: List[str] | None) -> int:
    print(f"Charset validation for: {i18n_path}\n")

    en_folder = i18n_path / "en"
    en_file = find_ftl_file_in_folder(en_folder) if en_folder.exists() else None
    base_entries = parse_ftl_file(en_file) if en_file else {}

    lang_folders = sorted(f for f in i18n_path.iterdir() if f.is_dir() and f.name != "en")
    if target_languages:
        lang_folders = [f for f in lang_folders if f.name in target_languages]

    results: Dict[str, Dict[str, Tuple[Dict[str, str], str]]] = {}
    skipped: List[str] = []

    for lang_folder in lang_folders:
        lang_code = lang_folder.name
        allowed = EXPECTED_SCRIPTS.get(lang_code)
        if allowed is None:
            skipped.append(lang_code)
            continue
        lang_file = find_ftl_file_in_folder(lang_folder)
        if not lang_file:
            continue
        problems = validate_language_file(lang_file, allowed, base_entries)
        if problems:
            results[lang_code] = problems

    if skipped:
        print(f"{Colors.YELLOW}Skipped (no expected-script mapping): {', '.join(skipped)}{Colors.RESET}\n")

    if not results:
        print(f"{Colors.GREEN}No foreign-script characters found.{Colors.RESET}")
        return 0

    print("=" * 70)
    print("SUMMARY (entries containing foreign-script letters)")
    print("=" * 70)
    for lang_code in sorted(results, key=lambda c: -len(results[c])):
        name = LANGUAGE_NAMES.get(lang_code, "Unknown")
        print(f"{lang_code:8} ({name:22}) - {len(results[lang_code]):4} suspicious entry(ies)")

    total = sum(len(v) for v in results.values())
    print(f"\nTotal suspicious entries: {total}\n")

    print("=" * 70)
    print("DETAILS")
    print("=" * 70)
    for lang_code in sorted(results):
        name = LANGUAGE_NAMES.get(lang_code, "Unknown")
        print(f"\n{Colors.BOLD}{lang_code} ({name}){Colors.RESET}")
        print("-" * 70)
        for key, (bad, value) in sorted(results[lang_code].items()):
            snippet = value.replace("\n", "\\n")
            if len(snippet) > 120:
                snippet = snippet[:117] + "..."
            print(f"  {Colors.GREEN}{key}{Colors.RESET}")
            print(f"    {Colors.RED}{describe_chars(bad)}{Colors.RESET}")
            print(f"    {snippet}")

    return 1


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Detect foreign-script characters in FTL translation files",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 misc/ai_translate/validate_charset.py krokiet/i18n
  python3 misc/ai_translate/validate_charset.py czkawka_core/i18n --languages fa ar
        """,
    )
    parser.add_argument("i18n_folder", type=str, help="Path to the i18n folder containing language subdirectories")
    parser.add_argument("--languages", nargs="+", help="Only validate specific languages (e.g., --languages fa ar)")
    args = parser.parse_args()

    i18n_path = pathlib.Path(args.i18n_folder)
    if not i18n_path.is_absolute():
        i18n_path = pathlib.Path.cwd() / i18n_path

    if not i18n_path.is_dir():
        print(f"Error: Path is not a directory: {i18n_path}")
        sys.exit(1)

    sys.exit(validate_i18n_folder(i18n_path, args.languages))


if __name__ == "__main__":
    main()
