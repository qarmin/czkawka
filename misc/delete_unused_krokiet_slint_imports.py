import os
import re

script_path = os.path.dirname(os.path.abspath(__file__))
ui_path = f"{script_path}/../krokiet/ui"

# Collect all .slint files in the ui_path directory
collected_files = [
    os.path.join(root, file)
    for root, _, files in os.walk(ui_path)
    for file in files if file.endswith(".slint")
]

for file_path in collected_files:
    with open(file_path, "r", encoding="utf-8") as file:
        content = file.read()
        lines = content.splitlines()

    non_import_lines = []
    imports_to_check = []
    updated_lines = []

    # Separate imports and other lines
    for line in lines:
        if line.startswith("import"):
            imports_to_check.append(line)
        else:
            non_import_lines.append(line)

    # Check usage of each import
    for import_line in imports_to_check:
        imported_items = re.search(r"\{(.*?)\}", import_line)
        if not imported_items:
            continue

        imported_items = [item.strip() for item in imported_items.group(1).split(",")]
        from_file = import_line.split("from")[1].strip()

        # Check if each imported item is used in the file
        used_items = []
        for item in imported_items:
            regex = rf"\b{item}\b"
            if len(re.findall(regex, "\n".join(non_import_lines))) >= 2:
                used_items.append(item)

        # Reassemble the import line with only used items
        if used_items:
            updated_lines.append(f"import {{ {', '.join(used_items)} }} from {from_file};")

    # Add non-import lines back
    updated_lines.extend(non_import_lines)

    # Write the updated content back to the file
    with open(file_path, "w", encoding="utf-8") as file:
        file.write("\n".join(updated_lines))