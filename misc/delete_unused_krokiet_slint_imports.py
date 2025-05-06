import os
import re

script_path = os.path.dirname(os.path.abspath(__file__))
ui_path = f"{script_path}/../krokiet/ui"

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

    for line in lines:
        if line.startswith("import"):
            imports_to_check.append(line)
        else:
            if len(non_import_lines) == 0 and len(line.strip()) == 0:
                continue
            non_import_lines.append(line)

    non_imported_content = "\n".join(non_import_lines)

    for import_line in imports_to_check:
        imported_items = [i.strip() for i in import_line.split("{")[1].split("}")[0].split(",") if len(i.strip()) > 0]
        if not imported_items:
            continue

        from_file = import_line.split("from")[1].strip()

        used_items = []
        for item in imported_items:
            regex = rf"\b{item}\b"
            if len(re.findall(regex, non_imported_content)) >= 1:
                used_items.append(item)

        if used_items:
            updated_line = f"import {{ {', '.join(used_items)} }} from {from_file}"
            updated_line = updated_line.replace(";;", ";")
            updated_lines.append(updated_line)

    if len(updated_lines) != 0:
        updated_lines.append("")

    updated_lines.extend(non_import_lines)
    if len(updated_lines) > 0 and len(updated_lines[-1].strip()) > 0:
        updated_lines.append("")

    with open(file_path, "w", encoding="utf-8") as file:
        file.write("\n".join(updated_lines))