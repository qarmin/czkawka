#!/usr/bin/env python3

import argparse
import pathlib
import re
import sys
from typing import Any, Dict, List, Set

from ftl_utils import parse_ftl_file, find_ftl_file_in_folder, LANGUAGE_NAMES


class Colors:
    GREEN = "\033[92m"
    RED = "\033[91m"
    YELLOW = "\033[93m"
    BLUE = "\033[94m"
    RESET = "\033[0m"
    BOLD = "\033[1m"


def extract_placeholders(text: str) -> Set[str]:
    pattern = re.compile(r"{\s*\$[\w-]+\s*}")
    matches = pattern.findall(text)
    normalized = {re.sub(r"\s+", "", match) for match in matches}
    return normalized


def count_placeholders(text: str) -> Dict[str, int]:
    pattern = re.compile(r"{\s*\$[\w-]+\s*}")
    matches = pattern.findall(text)
    normalized_matches = [re.sub(r"\s+", "", match) for match in matches]

    counts: Dict[str, int] = {}
    for placeholder in normalized_matches:
        counts[placeholder] = counts.get(placeholder, 0) + 1

    return counts


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

    base_counts = count_placeholders(base_value)
    translated_counts = count_placeholders(translated_value)

    for placeholder in base_placeholders:
        if placeholder in translated_placeholders:
            base_count = base_counts.get(placeholder, 0)
            translated_count = translated_counts.get(placeholder, 0)

            if base_count != translated_count:
                errors.append(
                    f"  {Colors.RED}Wrong occurrence count for {placeholder}:{Colors.RESET} expected {base_count}, found {translated_count}"
                )

    # New: validate trailing dot presence/absence consistency
    base_has_dot = base_value.strip().endswith(".")
    translated_has_dot = translated_value.strip().endswith(".")

    if base_has_dot != translated_has_dot:
        if base_has_dot:
            errors.append(
                f"  {Colors.RED}Trailing dot mismatch:{Colors.RESET} source ends with a dot but translation does not"
            )
        else:
            errors.append(
                f"  {Colors.RED}Trailing dot mismatch:{Colors.RESET} source does not end with a dot but translation does"
            )

    return errors


def validate_language_file(
    base_entries: Dict[str, str], lang_file: pathlib.Path, lang_code: str
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


def fix_language_file(lang_file: pathlib.Path, keys_to_remove: Set[str]) -> int:
    content = lang_file.read_text(encoding="utf-8")
    lines = content.split("\n")
    result_lines = []
    i = 0
    removed_count = 0

    while i < len(lines):
        line = lines[i]

        key_match = re.match(r"^([\w][\w-]*)\s*=", line)

        if key_match:
            key = key_match.group(1)

            if key in keys_to_remove:
                removed_count += 1
                i += 1
                while i < len(lines):
                    if lines[i] and lines[i][0] == " ":
                        i += 1
                    elif not lines[i].strip():
                        if i + 1 < len(lines) and lines[i + 1] and lines[i + 1][0] == " ":
                            i += 1
                        else:
                            break
                    else:
                        break
                continue

        result_lines.append(line)
        i += 1

    lang_file.write_text("\n".join(result_lines), encoding="utf-8")
    return removed_count


def fix_trailing_dots_in_language_file(
    lang_file: pathlib.Path, base_entries: Dict[str, str], keys_to_fix: Set[str]
) -> int:
    content = lang_file.read_text(encoding="utf-8")
    lines = content.split("\n")
    result_lines: List[str] = []
    i = 0
    modified_count = 0

    # Parse language file entries once
    lang_entries = parse_ftl_file(lang_file)

    while i < len(lines):
        line = lines[i]
        key_match = re.match(r"^([\w][\w-]*)\s*=\s*(.*)$", line)

        if key_match:
            key = key_match.group(1)

            if key in keys_to_fix and key in lang_entries and key in base_entries:
                # Collect the block lines (initial + continuation lines starting with a space)
                block_start = i
                block_lines = [lines[i]]
                j = i + 1
                while j < len(lines) and (lines[j].startswith(" ") or lines[j].strip() == ""):
                    # include continuation or empty lines that might be part of the value
                    # stop when next non-indented non-empty line appears
                    if lines[j].startswith(" ") or lines[j] == "":
                        block_lines.append(lines[j])
                        j += 1
                    else:
                        break

                # Extract content parts for each block line (preserve whitespace around content)
                content_parts: List[str] = []
                indents: List[str] = []
                for idx, bl in enumerate(block_lines):
                    if idx == 0:
                        m = re.match(r"^([\w][\w-]*)\s*=\s*(.*)$", bl)
                        part = m.group(2) if m else ""
                        content_parts.append(part)
                        indents.append("")
                    else:
                        # capture leading whitespace (indent) and the rest of content
                        m = re.match(r"^(\s*)(.*)$", bl)
                        if m:
                            indent = m.group(1)
                            part = m.group(2)
                        else:
                            indent = ""
                            part = bl
                        content_parts.append(part)
                        indents.append(indent)

                # Determine last content part index (skip trailing empty continuation lines)
                last_idx = len(content_parts) - 1
                while last_idx >= 0 and content_parts[last_idx].strip() == "":
                    last_idx -= 1

                if last_idx >= 0:
                    last_part = content_parts[last_idx]

                    # split into text and trailing spaces to preserve spacing
                    m = re.match(r"^(.*?)(\s*)$", last_part, flags=re.S)
                    text = m.group(1)  # type: ignore
                    trailing_spaces = m.group(2)  # type: ignore

                    base_has_dot = base_entries[key].strip().endswith(".")
                    trans_has_dot = text.endswith(".")

                    new_text = text
                    if base_has_dot and not trans_has_dot:
                        new_text = text + "."
                    elif not base_has_dot and trans_has_dot:
                        # remove all trailing dots from the textual end
                        new_text = re.sub(r"\.+$", "", text)

                    if new_text != text:
                        # replace last content part while keeping other parts intact
                        content_parts[last_idx] = new_text + trailing_spaces

                        # Rebuild block lines preserving original formatting
                        new_block_lines: List[str] = []
                        for idx, part in enumerate(content_parts):
                            if idx == 0:
                                new_block_lines.append(f"{key} = {part}")
                            else:
                                new_block_lines.append(f"{indents[idx]}{part}")

                        # Append new block lines to result and advance index
                        result_lines.extend(new_block_lines)
                        modified_count += 1
                        i = j
                        continue

        # default: copy original line
        result_lines.append(line)
        i += 1

    if modified_count > 0:
        lang_file.write_text("\n".join(result_lines), encoding="utf-8")

    return modified_count


def validate_i18n_folder(
    i18n_path: pathlib.Path, target_languages: List[str] | None = None, fix_mode: bool = False
) -> int:
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
    errors_by_language: Dict[str, Dict[str, Any]] = {}

    for lang_folder in lang_folders:
        lang_code = lang_folder.name
        lang_name = LANGUAGE_NAMES.get(lang_code, "Unknown")

        lang_file = find_ftl_file_in_folder(lang_folder)

        if not lang_file:
            continue

        errors = validate_language_file(base_entries, lang_file, lang_code)

        if errors:
            errors_by_language[lang_code] = {"name": lang_name, "file": lang_file, "errors": errors}
            total_errors += len(errors)

    if not errors_by_language:
        print("All translations are valid!")
        return 0

    if fix_mode:
        print(
            f"\n{Colors.YELLOW}FIX MODE: Fixing trailing-dot mismatches and removing entries with placeholder errors{Colors.RESET}\n"
        )

        total_removed = 0
        total_fixed = 0

        for lang_code in sorted(errors_by_language.keys()):
            data = errors_by_language[lang_code]
            lang_file = data["file"]
            # classify keys by type of error
            keys_to_remove: Set[str] = set()
            keys_to_fix_dots: Set[str] = set()

            for key, msgs in data["errors"].items():
                combined = "\n".join(msgs)
                if (
                    "Missing placeholders" in combined
                    or "Extra placeholders" in combined
                    or "Wrong occurrence count" in combined
                ):
                    keys_to_remove.add(key)
                elif "Trailing dot mismatch" in combined:
                    keys_to_fix_dots.add(key)
                else:
                    # default to removal if unknown error
                    keys_to_remove.add(key)

            removed = 0
            fixed = 0

            if keys_to_remove:
                removed = fix_language_file(lang_file, keys_to_remove)

            if keys_to_fix_dots:
                fixed = fix_trailing_dots_in_language_file(lang_file, base_entries, keys_to_fix_dots)

            total_removed += removed
            total_fixed += fixed

            print(f"{lang_code:8} ({data['name']:25}) - removed {removed:3} entry(ies), fixed {fixed:3} entry(ies)")

        print(
            f"\n{Colors.GREEN}Fixed! Removed {total_removed} entry(ies) and updated {total_fixed} translation(s) with trailing-dot mismatches{Colors.RESET}"
        )
        return 0

    print(f"\nFound errors in {len(errors_by_language)} language(s):\n")

    for lang_code in sorted(errors_by_language.keys()):
        data = errors_by_language[lang_code]
        error_count = len(data["errors"])
        print(f"{lang_code:8} ({data['name']:25}) - {error_count:3} error(s)")

    print(f"\nTotal errors: {total_errors}\n")
    print("=" * 70)
    print("DETAILED ERRORS")
    print("=" * 70 + "\n")

    for lang_code in sorted(errors_by_language.keys()):
        data = errors_by_language[lang_code]
        print(f"\n{Colors.BOLD}{lang_code} ({data['name']}) - {data['file'].name}{Colors.RESET}")
        print("-" * 70)

        for key, error_messages in sorted(data["errors"].items()):
            print(f"\n{Colors.GREEN}Key: {key}{Colors.RESET}")
            for error_msg in error_messages:
                print(error_msg)

    print("\n" + "=" * 70)
    print(f"Validation complete: {total_errors} error(s) found")

    return 1 if total_errors > 0 else 0


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Validate FTL translation files for placeholder consistency",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 misc/ai_translate/validate_translations.py czkawka_gui/i18n
  python3 misc/ai_translate/validate_translations.py krokiet/i18n
  python3 misc/ai_translate/validate_translations.py czkawka_gui/i18n --languages pl de fr
        """,
    )

    parser.add_argument("i18n_folder", type=str, help="Path to the i18n folder containing language subdirectories")

    parser.add_argument("--languages", nargs="+", help="Only validate specific languages (e.g., --languages pl de fr)")

    parser.add_argument("--fix", action="store_true", help="Automatically remove entries with placeholder errors")

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
        exit_code = validate_i18n_folder(i18n_path, args.languages, args.fix)
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
