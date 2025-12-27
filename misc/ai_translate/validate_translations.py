#!/usr/bin/env python3

import argparse
import pathlib
import re
import sys
from typing import Dict, List, Set

from ftl_utils import parse_ftl_file, find_ftl_file_in_folder, LANGUAGE_NAMES


class Colors:
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    RESET = '\033[0m'
    BOLD = '\033[1m'


def extract_placeholders(text: str) -> Set[str]:
    pattern = re.compile(r'\{\s*\$[\w-]+\s*\}')
    matches = pattern.findall(text)
    normalized = {re.sub(r'\s+', '', match) for match in matches}
    return normalized


def validate_translation(base_value: str, translated_value: str, key: str) -> List[str]:
    errors = []

    base_placeholders = extract_placeholders(base_value)
    translated_placeholders = extract_placeholders(translated_value)

    missing = base_placeholders - translated_placeholders
    extra = translated_placeholders - base_placeholders

    if missing:
        errors.append(f"  {Colors.RED}Missing placeholders:{Colors.RESET} {', '.join(sorted(missing))}")
    if extra:
        errors.append(f"  {Colors.RED}Extra placeholders:{Colors.RESET} {', '.join(sorted(extra))}")

    return errors


def validate_language_file(
    base_entries: Dict[str, str],
    lang_file: pathlib.Path,
    lang_code: str
) -> Dict[str, List[str]]:
    lang_entries = parse_ftl_file(lang_file)

    errors_by_key = {}

    for key, base_value in base_entries.items():
        if key not in lang_entries:
            continue

        translated_value = lang_entries[key]

        validation_errors = validate_translation(base_value, translated_value, key)

        if validation_errors:
            errors_by_key[key] = validation_errors

    return errors_by_key


def validate_i18n_folder(i18n_path: pathlib.Path, target_languages: List[str] | None = None) -> int:
    print(f"Validating i18n folder: {i18n_path}")

    en_folder = i18n_path / "en"
    if not en_folder.exists():
        print(f"Error: English folder not found at {en_folder}")
        return 1

    en_file = find_ftl_file_in_folder(en_folder)
    if not en_file:
        print(f"Error: No FTL file found in {en_folder}")
        return 1

    print(f"Base file: {en_file.name}")

    base_entries = parse_ftl_file(en_file)
    print(f"Found {len(base_entries)} entries in base file\n")

    lang_folders = [f for f in i18n_path.iterdir() if f.is_dir() and f.name != "en"]
    lang_folders.sort()

    if target_languages:
        lang_folders = [f for f in lang_folders if f.name in target_languages]

    print("=" * 70)
    print("VALIDATION RESULTS")
    print("=" * 70)

    total_errors = 0
    errors_by_language = {}

    for lang_folder in lang_folders:
        lang_code = lang_folder.name
        lang_name = LANGUAGE_NAMES.get(lang_code, 'Unknown')

        lang_file = find_ftl_file_in_folder(lang_folder)

        if not lang_file:
            continue

        errors = validate_language_file(base_entries, lang_file, lang_code)

        if errors:
            errors_by_language[lang_code] = {
                'name': lang_name,
                'file': lang_file,
                'errors': errors
            }
            total_errors += len(errors)

    if not errors_by_language:
        print("All translations are valid!")
        return 0

    print(f"\nFound errors in {len(errors_by_language)} language(s):\n")

    for lang_code in sorted(errors_by_language.keys()):
        data = errors_by_language[lang_code]
        error_count = len(data['errors'])
        print(f"{lang_code:8} ({data['name']:25}) - {error_count:3} error(s)")

    print(f"\nTotal errors: {total_errors}\n")
    print("=" * 70)
    print("DETAILED ERRORS")
    print("=" * 70 + "\n")

    for lang_code in sorted(errors_by_language.keys()):
        data = errors_by_language[lang_code]
        print(f"\n{Colors.BOLD}{lang_code} ({data['name']}) - {data['file'].name}{Colors.RESET}")
        print("-" * 70)

        for key, error_messages in sorted(data['errors'].items()):
            print(f"\n{Colors.GREEN}Key: {key}{Colors.RESET}")
            for error_msg in error_messages:
                print(error_msg)

    print("\n" + "=" * 70)
    print(f"Validation complete: {total_errors} error(s) found")

    return 1 if total_errors > 0 else 0


def main():
    parser = argparse.ArgumentParser(
        description="Validate FTL translation files for placeholder consistency",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 misc/ai_translate/validate_translations.py czkawka_gui/i18n
  python3 misc/ai_translate/validate_translations.py krokiet/i18n
  python3 misc/ai_translate/validate_translations.py czkawka_gui/i18n --languages pl de fr
        """
    )

    parser.add_argument(
        "i18n_folder",
        type=str,
        help="Path to the i18n folder containing language subdirectories"
    )

    parser.add_argument(
        "--languages",
        nargs="+",
        help="Only validate specific languages (e.g., --languages pl de fr)"
    )

    args = parser.parse_args()

    i18n_path = pathlib.Path(args.i18n_folder)
    if not i18n_path.is_absolute():
        i18n_path = pathlib.Path.cwd() / i18n_path

    if not i18n_path.exists():
        print(f"Error: Path does not exist: {i18n_path}")
        sys.exit(1)

    if not i18n_path.is_dir():
        print(f"Error: Path is not a directory: {i18n_path}")
        sys.exit(1)

    try:
        exit_code = validate_i18n_folder(i18n_path, args.languages)
        sys.exit(exit_code)
    except KeyboardInterrupt:
        print("\n\nInterrupted by user")
        sys.exit(1)
    except Exception as e:
        print(f"\nError: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()

