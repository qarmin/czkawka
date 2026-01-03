#!/usr/bin/env python3

import pathlib
import re
from typing import Dict


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


def parse_ftl_file(file_path: pathlib.Path) -> Dict[str, str]:
    if not file_path.exists():
        return {}

    content = file_path.read_text(encoding="utf-8")
    entries = {}
    lines = content.split("\n")
    i = 0

    while i < len(lines):
        line = lines[i]

        key_match = re.match(r"^([\w][\w-]*)\s*=\s*(.*)", line)

        if key_match:
            key = key_match.group(1).strip()
            first_line_value = key_match.group(2).strip()

            value_lines = []
            if first_line_value:
                value_lines.append(first_line_value)

            i += 1
            while i < len(lines):
                next_line = lines[i]

                if re.match(r"^[\w][\w-]*\s*=", next_line):
                    break
                if next_line.startswith("#"):
                    break

                if next_line and next_line[0] == " ":
                    value_lines.append(next_line.strip())
                    i += 1
                elif not next_line.strip():
                    j = i + 1
                    has_more_content = False
                    while j < len(lines):
                        if lines[j] and lines[j][0] == " ":
                            has_more_content = True
                            break
                        elif lines[j].strip() and not lines[j].startswith("#"):
                            break
                        j += 1

                    if has_more_content:
                        value_lines.append("")
                        i += 1
                    else:
                        break
                else:
                    break

            value = "\n".join(value_lines) if value_lines else ""
            entries[key] = value
        else:
            i += 1

    return entries


def find_ftl_file_in_folder(folder: pathlib.Path) -> pathlib.Path | None:
    if not folder.exists() or not folder.is_dir():
        return None

    ftl_files = list(folder.glob("*.ftl"))
    if len(ftl_files) == 1:
        return ftl_files[0]
    elif len(ftl_files) > 1:
        print(f"  Warning: Multiple FTL files found in {folder}, using first one: {ftl_files[0].name}")
        return ftl_files[0]

    return None
