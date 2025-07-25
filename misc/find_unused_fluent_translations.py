import os
import sys


def find_files(root: str, ext: str, folder: str | None) -> list[str]:
    files = []
    for dirpath, _, filenames in os.walk(root):
        for f in filenames:
            if f.endswith(ext) and (folder is None or folder in dirpath):
                files.append(os.path.join(dirpath, f))
    return files


def read_files(files: list[str]) -> str:
    content = ""
    for f in files:
        with open(f, "r", encoding="utf-8") as file:
            content += file.read() + "\n"
    return content


def extract_ftl_keys(ftl_path: str) -> list[str]:
    keys = []
    with open(ftl_path, "r", encoding="utf-8") as f:
        lines = f.readlines()
        for line in lines:
            line = line.strip()
            if "=" not in line:
                continue
            key = line.split("=")[0].strip()
            keys.append(key)

    # Find duplicated keys
    seen = set()
    duplicates = set()
    for key in keys:
        if key in seen:
            duplicates.add(key)
        else:
            seen.add(key)

    if duplicates:
        print(f"Warning: Found duplicated keys in {format_green(ftl_path)}: {format_green(', '.join(duplicates))}")
        exit(1)

    return keys


def format_green(text: str) -> str:
    return f"\033[92m{text}\033[0m"


if len(sys.argv) < 2:
    print("Usage: python find_unused_fluent_translations.py <folder>")
    sys.exit(1)

folder = sys.argv[1]
rust_files = find_files(folder, ".rs", None)
ftl_files = find_files(folder, f"{folder}.ftl", "/en")
rust_content = read_files(rust_files)

print(f"Found {len(rust_files)} Rust files and {len(ftl_files)} FTL files in {folder}")

found = False

for ftl_file in ftl_files:
    unused = []
    keys = extract_ftl_keys(ftl_file)
    print(f"Found {len(keys)} keys in {ftl_file}")
    for key in keys:
        if f'"{key}"' not in rust_content:
            unused.append(key)
    if unused:
        print(f"Unused keys in {ftl_file}:")
        for key in unused:
            print(f"  {format_green(key)}")
        found = True

if found:
    sys.exit(1)
