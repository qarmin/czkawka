import argparse
import pathlib
import re
import sys
from typing import Any, Dict, List, Tuple

from ftl_utils import parse_ftl_file, find_ftl_file_in_folder, LANGUAGE_NAMES


# DEFAULT_MODEL = "qwen2.5:7b"
# DEFAULT_MODEL = "qwen2.5:32b"
# DEFAULT_MODEL = "zongwei/gemma3-translator:4b"
DEFAULT_MODEL = "translategemma:12b"

IGNORED_KEYS = [
    "bottom_symlink_button",
    "bottom_hardlink_button",
    "main_tree_view_column_fps",
    "main_check_box_broken_files_pdf",
    "general_ok_button",
    "duplicate_mode_hash_combo_box",
    "compare_move_left_button",
    "compare_move_right_button",
    "ok_button",
    "ref",
]


def serialize_ftl_entries(entries: Dict[str, str]) -> str:
    lines = []
    for key, value in entries.items():
        if "\n" in value:
            lines.append(f"{key} =")
            for line in value.split("\n"):
                if line.strip():
                    lines.append(f"    {line}")
                else:
                    lines.append("")
        else:
            lines.append(f"{key} = {value}")

    return "\n".join(lines)


def read_ftl_with_structure(file_path: pathlib.Path) -> Tuple[str, Dict[str, str]]:
    if not file_path.exists():
        return "", {}

    content = file_path.read_text(encoding="utf-8")
    entries = parse_ftl_file(file_path)

    return content, entries


def translate_text(text: str, target_language: str, model: str = DEFAULT_MODEL) -> str:
    try:
        import ollama  # type: ignore
    except ImportError:
        print("Error: 'ollama' package not installed.")
        print("   Install it with: pip install ollama")
        print("   Or run: just prepare_translations_deps")
        sys.exit(1)

    language_name = LANGUAGE_NAMES.get(target_language, target_language)

    try:
        response = ollama.chat(
            model=model,
            messages=[
                {
                    "role": "user",
                    "content": f"""Translate the following text to {language_name}.
Keep the same tone and style. Preserve any special formatting or placeholders.
Only return the translated text, no explanations or additional text.

Text to translate:
{text}""",
                }
            ],
        )

        translated: str = str(response["message"]["content"]).strip()

        if translated.startswith('"') and translated.endswith('"'):
            translated = translated[1:-1]
        if translated.startswith("'") and translated.endswith("'"):
            translated = translated[1:-1]

        return translated

    except Exception as e:
        print(f"  Translation error: {e}")
        return text


def analyze_language_file(
    base_entries: Dict[str, str], lang_file: pathlib.Path, target_lang: str
) -> Tuple[Dict[str, str], int, int]:
    lang_content, lang_entries = read_ftl_with_structure(lang_file)

    missing_keys = {}
    ignored_count = 0

    for key, base_value in base_entries.items():
        if key in IGNORED_KEYS:
            if key not in lang_entries or lang_entries[key] == base_value:
                ignored_count += 1
            continue

        needs_translation = False

        if key not in lang_entries:
            needs_translation = True
        elif lang_entries[key] == base_value:
            needs_translation = True

        if needs_translation:
            missing_keys[key] = base_value

    return missing_keys, len(missing_keys), ignored_count


def update_language_file_content(lang_file: pathlib.Path, translations: Dict[str, str]) -> None:
    if not translations:
        return

    content = lang_file.read_text(encoding="utf-8")
    lines = content.split("\n")
    result_lines = []
    i = 0
    processed_keys = set()

    while i < len(lines):
        line = lines[i]

        key_match = re.match(r"^([\w][\w-]*)\s*=", line)

        if key_match:
            key = key_match.group(1)
            processed_keys.add(key)

            if key in translations:
                value = translations[key]
                if "\n" in value:
                    result_lines.append(f"{key} = ")
                    for v_line in value.split("\n"):
                        if v_line.strip():
                            result_lines.append(f"        {v_line}")
                        else:
                            result_lines.append("")
                else:
                    result_lines.append(f"{key} = {value}")

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

    new_keys = [k for k in translations.keys() if k not in processed_keys]
    if new_keys:
        if result_lines and result_lines[-1].strip():
            result_lines.append("")
        for key in new_keys:
            value = translations[key]
            if "\n" in value:
                result_lines.append(f"{key} = ")
                for v_line in value.split("\n"):
                    if v_line.strip():
                        result_lines.append(f"        {v_line}")
                    else:
                        result_lines.append("")
            else:
                result_lines.append(f"{key} = {value}")

    lang_file.write_text("\n".join(result_lines), encoding="utf-8")


def process_i18n_folder(
    i18n_path: pathlib.Path,
    model: str = DEFAULT_MODEL,
    dry_run: bool = False,
    target_languages: List[str] | None = None,
) -> None:
    print(f"Processing i18n folder: {i18n_path}")

    en_folder = i18n_path / "en"
    if not en_folder.exists():
        print(f"Error: English folder not found at {en_folder}")
        return

    en_file = find_ftl_file_in_folder(en_folder)
    if not en_file:
        print(f"Error: No FTL file found in {en_folder}")
        return

    print(f"Base file: {en_file.name}")

    base_entries = parse_ftl_file(en_file)
    print(f"Found {len(base_entries)} entries in base file\n")

    lang_folders = [f for f in i18n_path.iterdir() if f.is_dir() and f.name != "en"]
    lang_folders.sort()

    if target_languages:
        lang_folders = [f for f in lang_folders if f.name in target_languages]

    print("=" * 70)
    print("ANALYSIS PHASE - Reading all files")
    print("=" * 70)

    analysis_results: Dict[str, Dict[str, Any]] = {}

    for lang_folder in lang_folders:
        lang_code = lang_folder.name
        lang_name = LANGUAGE_NAMES.get(lang_code, "Unknown")

        lang_file = find_ftl_file_in_folder(lang_folder)

        if not lang_file:
            lang_file = lang_folder / en_file.name
            if not lang_file.exists():
                lang_file.touch()

        missing_keys, count, ignored_count = analyze_language_file(base_entries, lang_file, lang_code)
        analysis_results[lang_code] = {
            "name": lang_name,
            "file": lang_file,
            "missing_keys": missing_keys,
            "count": count,
            "ignored_count": ignored_count,
        }

        status = f"{count:3} phrases to translate"
        if ignored_count > 0:
            status += f", {ignored_count:3} ignored"
        print(f"{lang_code:8} ({lang_name:25}) - {status}")

    total_to_translate = sum(int(r["count"]) for r in analysis_results.values())
    total_ignored = sum(int(r["ignored_count"]) for r in analysis_results.values())
    print(f"\nTotal phrases to translate: {total_to_translate}")
    if total_ignored > 0:
        print(f"Total phrases ignored: {total_ignored}")

    if total_to_translate == 0:
        print("\nNo translations needed.")
        return

    if dry_run:
        print("\n[DRY RUN] Would translate the above phrases")
        return

    print("\n" + "=" * 70)
    print("TRANSLATION PHASE")
    print("=" * 70 + "\n")

    total_translated = 0

    for lang_code, data in analysis_results.items():
        count = int(data["count"])
        if count == 0:
            continue

        lang_name = str(data["name"])
        missing_keys = dict(data["missing_keys"])
        lang_file = pathlib.Path(data["file"])

        print(f"Translating {lang_code} ({lang_name}) - {count} phrases")

        translations = {}
        for idx, (key, base_value) in enumerate(missing_keys.items(), 1):
            print(f"  [{idx}/{count}] {key}: {base_value[:50]}...")
            translated_value = translate_text(base_value, lang_code, model)
            translations[key] = translated_value

        update_language_file_content(lang_file, translations)
        total_translated += count
        print(f"  Updated {lang_file.name}\n")

    print(
        f"Complete! Translated {total_translated} phrases across {len([r for r in analysis_results.values() if int(r['count']) > 0])} languages"
    )


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Translate FTL files using offline AI",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 misc/ai_translate/translate.py czkawka_gui/i18n
  python3 misc/ai_translate/translate.py krokiet/i18n --model qwen2.5:7b
  python3 misc/ai_translate/translate.py czkawka_gui/i18n --dry-run
  python3 misc/ai_translate/translate.py czkawka_gui/i18n --languages pl de fr
        """,
    )

    parser.add_argument("i18n_folder", type=str, help="Path to the i18n folder containing language subdirectories")

    parser.add_argument(
        "--model",
        type=str,
        default=DEFAULT_MODEL,
        help=f"Ollama model to use for translation (default: {DEFAULT_MODEL})",
    )

    parser.add_argument("--dry-run", action="store_true", help="Show what would be translated without making changes")

    parser.add_argument("--languages", nargs="+", help="Only process specific languages (e.g., --languages pl de fr)")

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
        process_i18n_folder(i18n_path, args.model, args.dry_run, args.languages)
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
