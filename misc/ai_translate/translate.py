#!/usr/bin/env python3
"""
Translation script for FTL (Fluent) files using offline AI translation.

Usage:
    python3 misc/ai_translate/translate.py czkawka_gui/i18n
    python3 misc/ai_translate/translate.py krokiet/i18n
"""

import argparse
import os
import pathlib
import re
import sys
from typing import Dict, List, Tuple


# Language code mapping for translation
LANGUAGE_NAMES = {
    "ar": "Arabic",
    "bg": "Bulgarian",
    "cs": "Czech",
    "de": "German",
    "el": "Greek",
    "en": "English",
    "es-ES": "Spanish",
    "fa": "Persian",
    "fr": "French",
    "it": "Italian",
    "ja": "Japanese",
    "ko": "Korean",
    "nl": "Dutch",
    "no": "Norwegian",
    "pl": "Polish",
    "pt-BR": "Brazilian Portuguese",
    "pt-PT": "Portuguese",
    "ro": "Romanian",
    "ru": "Russian",
    "sv-SE": "Swedish",
    "tr": "Turkish",
    "uk": "Ukrainian",
    "zh-CN": "Simplified Chinese",
    "zh-TW": "Traditional Chinese",
}

# Default model to use (can be changed via config or CLI)
DEFAULT_MODEL = "qwen2.5:7b"


def parse_ftl_file(file_path: pathlib.Path) -> Dict[str, str]:
    """
    Parse an FTL file and extract key-value pairs.
    Handles multiline values.

    Returns a dict where keys are the translation keys and values are the translated text.
    """
    if not file_path.exists():
        return {}

    content = file_path.read_text(encoding="utf-8")
    entries = {}

    # Regular expression to match FTL entries
    # Matches: key = value (potentially multiline)
    # Pattern explanation:
    # ^(\w[\w-]*)\s*=\s* - matches key at start of line
    # (.*?)(?=^\w[\w-]*\s*=|^#|$) - matches value until next key, comment, or end
    pattern = re.compile(
        r'^([\w][\w-]*)\s*=\s*(.*?)(?=^[\w][\w-]*\s*=\s*|^#[^\n]*\n[\w]|^\s*$|\Z)',
        re.MULTILINE | re.DOTALL
    )

    for match in pattern.finditer(content):
        key = match.group(1).strip()
        value = match.group(2).strip()
        entries[key] = value

    return entries


def serialize_ftl_entries(entries: Dict[str, str]) -> str:
    """
    Convert a dictionary of entries back to FTL format.
    """
    lines = []
    for key, value in entries.items():
        # Check if value is multiline
        if '\n' in value:
            lines.append(f"{key} =")
            for line in value.split('\n'):
                if line.strip():
                    lines.append(f"    {line}")
                else:
                    lines.append("")
        else:
            lines.append(f"{key} = {value}")

    return '\n'.join(lines)


def read_ftl_with_structure(file_path: pathlib.Path) -> Tuple[str, Dict[str, str]]:
    """
    Read FTL file preserving comments and structure.
    Returns the full content and parsed entries.
    """
    if not file_path.exists():
        return "", {}

    content = file_path.read_text(encoding="utf-8")
    entries = parse_ftl_file(file_path)

    return content, entries


def translate_text(text: str, target_language: str, model: str = DEFAULT_MODEL) -> str:
    """
    Translate text using Ollama offline AI model.
    """
    try:
        import ollama
    except ImportError:
        print("❌ Error: 'ollama' package not installed.")
        print("   Install it with: pip install ollama")
        print("   Or run: just prepare_translations_deps")
        sys.exit(1)

    language_name = LANGUAGE_NAMES.get(target_language, target_language)

    try:
        response = ollama.chat(
            model=model,
            messages=[{
                "role": "user",
                "content": f"""Translate the following text to {language_name}.
Keep the same tone and style. Preserve any special formatting or placeholders.
Only return the translated text, no explanations or additional text.

Text to translate:
{text}"""
            }]
        )

        translated = response["message"]["content"].strip()

        # Remove quotes if the model added them
        if translated.startswith('"') and translated.endswith('"'):
            translated = translated[1:-1]
        if translated.startswith("'") and translated.endswith("'"):
            translated = translated[1:-1]

        return translated

    except Exception as e:
        print(f"  ⚠ Translation error: {e}")
        return text  # Return original text on error


def find_ftl_file_in_folder(folder: pathlib.Path) -> pathlib.Path | None:
    """
    Find the FTL file in a folder. Should be only one.
    """
    if not folder.exists() or not folder.is_dir():
        return None

    ftl_files = list(folder.glob("*.ftl"))
    if len(ftl_files) == 1:
        return ftl_files[0]
    elif len(ftl_files) > 1:
        print(f"  ⚠ Warning: Multiple FTL files found in {folder}, using first one: {ftl_files[0].name}")
        return ftl_files[0]

    return None


def update_language_file(
    base_entries: Dict[str, str],
    lang_file: pathlib.Path,
    target_lang: str,
    model: str,
    dry_run: bool = False
) -> int:
    """
    Update a language file with missing or untranslated entries.
    Returns the number of translations added/updated.
    """
    # Read existing translations
    lang_content, lang_entries = read_ftl_with_structure(lang_file)

    translations_count = 0

    # Find missing or untranslated keys
    for key, base_value in base_entries.items():
        needs_translation = False

        if key not in lang_entries:
            print(f"    ➕ Missing key: {key}")
            needs_translation = True
        elif lang_entries[key] == base_value:
            print(f"    🔄 Untranslated key (same as English): {key}")
            needs_translation = True

        if needs_translation:
            if not dry_run:
                print(f"       Translating: {base_value[:60]}...")
                translated_value = translate_text(base_value, target_lang, model)
                lang_entries[key] = translated_value
                translations_count += 1
            else:
                print(f"       [DRY RUN] Would translate: {base_value[:60]}...")
                translations_count += 1

    # Write back if changes were made
    if translations_count > 0 and not dry_run:
        # Preserve comments and structure by parsing the original file
        # and inserting new translations in the right places
        new_content = preserve_structure_and_update(lang_content, lang_entries, base_entries)
        lang_file.write_text(new_content, encoding="utf-8")
        print(f"    ✅ Updated {lang_file.name} with {translations_count} translations")

    return translations_count


def preserve_structure_and_update(
    original_content: str,
    updated_entries: Dict[str, str],
    base_entries: Dict[str, str]
) -> str:
    """
    Update the file content while preserving comments and structure.
    """
    lines = original_content.split('\n')
    result_lines = []
    i = 0
    processed_keys = set()

    while i < len(lines):
        line = lines[i]

        # Check if line starts with a key
        key_match = re.match(r'^([\w][\w-]*)\s*=', line)

        if key_match:
            key = key_match.group(1)
            processed_keys.add(key)

            if key in updated_entries:
                # Replace with updated value
                value = updated_entries[key]
                if '\n' in value:
                    result_lines.append(f"{key} =")
                    for v_line in value.split('\n'):
                        if v_line.strip():
                            result_lines.append(f"    {v_line}")
                        else:
                            result_lines.append("")
                else:
                    result_lines.append(f"{key} = {value}")

                # Skip original multiline value if any
                i += 1
                while i < len(lines) and lines[i].startswith('    '):
                    i += 1
                continue

        result_lines.append(line)
        i += 1

    # Add new keys at the end
    new_keys = [k for k in base_entries.keys() if k in updated_entries and k not in processed_keys]
    if new_keys:
        result_lines.append("")
        result_lines.append("# Newly added translations")
        for key in new_keys:
            value = updated_entries[key]
            if '\n' in value:
                result_lines.append(f"{key} =")
                for v_line in value.split('\n'):
                    if v_line.strip():
                        result_lines.append(f"    {v_line}")
                    else:
                        result_lines.append("")
            else:
                result_lines.append(f"{key} = {value}")

    return '\n'.join(result_lines)


def process_i18n_folder(
    i18n_path: pathlib.Path,
    model: str = DEFAULT_MODEL,
    dry_run: bool = False,
    target_languages: List[str] | None = None
):
    """
    Process an i18n folder and translate missing entries.
    """
    print(f"🌍 Processing i18n folder: {i18n_path}")

    # Find the English FTL file
    en_folder = i18n_path / "en"
    if not en_folder.exists():
        print(f"❌ Error: English folder not found at {en_folder}")
        return

    en_file = find_ftl_file_in_folder(en_folder)
    if not en_file:
        print(f"❌ Error: No FTL file found in {en_folder}")
        return

    print(f"📄 Base file: {en_file.name}")

    # Parse English file
    base_entries = parse_ftl_file(en_file)
    print(f"📊 Found {len(base_entries)} entries in base file\n")

    # Get all language folders
    lang_folders = [f for f in i18n_path.iterdir() if f.is_dir() and f.name != "en"]
    lang_folders.sort()

    # Filter languages if specified
    if target_languages:
        lang_folders = [f for f in lang_folders if f.name in target_languages]

    total_translations = 0

    for lang_folder in lang_folders:
        lang_code = lang_folder.name
        print(f"🔤 Processing language: {lang_code} ({LANGUAGE_NAMES.get(lang_code, 'Unknown')})")

        # Find or create the FTL file
        lang_file = find_ftl_file_in_folder(lang_folder)

        if not lang_file:
            # Create new file with same name as base
            lang_file = lang_folder / en_file.name
            print(f"  📝 Creating new file: {lang_file.name}")
            lang_file.touch()

        # Update the file
        count = update_language_file(base_entries, lang_file, lang_code, model, dry_run)
        total_translations += count

        print()

    print(f"✨ Complete! Total translations: {total_translations}")


def main():
    parser = argparse.ArgumentParser(
        description="Translate FTL files using offline AI",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 misc/ai_translate/translate.py czkawka_gui/i18n
  python3 misc/ai_translate/translate.py krokiet/i18n --model qwen2.5:7b
  python3 misc/ai_translate/translate.py czkawka_gui/i18n --dry-run
  python3 misc/ai_translate/translate.py czkawka_gui/i18n --languages pl de fr
        """
    )

    parser.add_argument(
        "i18n_folder",
        type=str,
        help="Path to the i18n folder containing language subdirectories"
    )

    parser.add_argument(
        "--model",
        type=str,
        default=DEFAULT_MODEL,
        help=f"Ollama model to use for translation (default: {DEFAULT_MODEL})"
    )

    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would be translated without making changes"
    )

    parser.add_argument(
        "--languages",
        nargs="+",
        help="Only process specific languages (e.g., --languages pl de fr)"
    )

    args = parser.parse_args()

    # Convert to absolute path
    i18n_path = pathlib.Path(args.i18n_folder)
    if not i18n_path.is_absolute():
        i18n_path = pathlib.Path.cwd() / i18n_path

    if not i18n_path.exists():
        print(f"❌ Error: Path does not exist: {i18n_path}")
        sys.exit(1)

    if not i18n_path.is_dir():
        print(f"❌ Error: Path is not a directory: {i18n_path}")
        sys.exit(1)

    try:
        process_i18n_folder(i18n_path, args.model, args.dry_run, args.languages)
    except KeyboardInterrupt:
        print("\n\n⚠ Interrupted by user")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()

