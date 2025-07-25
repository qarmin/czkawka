import os
import sys


# Find translations in krokiet/src/translations.slint
# Check if in other slint files they are used
# Check if in all items from translations are used in a way f"set_{}(" in src/connect_translation.rs


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
    return keys


def extract_slint_properties(slint_path: str) -> list[str]:
    properties = []
    with open(slint_path, "r", encoding="utf-8") as f:
        lines = f.readlines()
        for slint_line in lines:
            if "property " not in slint_line:
                continue
            properties.append(slint_line.split(">")[1].split(":")[0].strip())
    properties.sort()
    return properties


def format_green(text: str) -> str:
    return f"\033[92m{text}\033[0m"


if len(sys.argv) < 2:
    print("Usage: python find_unused_slint_translations.py <folder>")
    sys.exit(1)

folder = sys.argv[1]
rust_translation_content = open(f"{folder}/src/connect_translation.rs", "r", encoding="utf-8").read()

missing_in_slint = []

slint_files = find_files(folder, ".slint", folder)
assert any([file for file in slint_files if "translations.slint" in file]), (
    "No translations.slint found in krokiet folder"
)
slint_files = [file for file in slint_files if "translations.slint" not in file]
slint_files_content = read_files(slint_files)
arguments = extract_slint_properties(f"{folder}/ui/translations.slint")
print(f"Found {len(arguments)} arguments in translations.slint")

# Check if all arguments are used in Slint files
for argument in arguments:
    if f"Translations.{argument}" not in slint_files_content:
        missing_in_slint.append(argument)
missing_in_slint.sort()

missing_in_rust = []
for argument in arguments:
    if f"set_{argument}(" not in rust_translation_content:
        missing_in_rust.append(argument)
missing_in_rust.sort()

if len(missing_in_rust) > 0:
    print(
        "---- Arguments not used in Rust translation file: " + ", ".join(format_green(arg) for arg in missing_in_rust)
    )
if len(missing_in_slint) > 0:
    print("---- Arguments not used in Slint files: " + ", ".join(format_green(arg) for arg in missing_in_slint))

if len(missing_in_slint) > 0 or len(missing_in_rust) > 0:
    sys.exit(1)
