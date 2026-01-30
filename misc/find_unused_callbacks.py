import os
import sys
import re

excluded = ["theme_changed"]  # Executed from Slint


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


def extract_callbacks(slint_path: str) -> list[str]:
    callbacks = []
    with open(slint_path, "r", encoding="utf-8") as f:
        content = f.read()

    pattern = r"callback\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\("
    for match in re.finditer(pattern, content):
        callback_name = match.group(1)
        callbacks.append(callback_name)

    return callbacks


def format_green(text: str) -> str:
    return f"\033[92m{text}\033[0m"


if len(sys.argv) < 2:
    print("Usage: python find_unused_callbacks.py <folder>")
    sys.exit(1)

folder = sys.argv[1]
callabler_path = f"{folder}/ui/callabler.slint"

if not os.path.exists(callabler_path):
    print(f"Error: {callabler_path} not found")
    sys.exit(1)

callbacks = extract_callbacks(callabler_path)
print(f"Found {len(callbacks)} callbacks in callabler.slint")

rust_files = find_files(f"{folder}/src", ".rs", None)
rust_content = read_files(rust_files)

errors_found = False

for callback in callbacks:
    if callback in excluded:
        continue

    pattern = rf"\.on_{callback}\("
    matches = list(re.finditer(pattern, rust_content))

    if len(matches) == 0:
        print(f"Error: Callback {format_green(callback)} has NO Rust implementation")
        errors_found = True
    elif len(matches) > 1:
        print(f"Error: Callback {format_green(callback)} has {len(matches)} Rust implementations (expected 1)")
        errors_found = True

if errors_found:
    sys.exit(1)
else:
    print("All callbacks have exactly 1 Rust implementation")
