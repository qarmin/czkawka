import os

script_path = os.path.dirname(os.path.abspath(__file__))

translations_slint_path = f"{script_path}/../krokiet/ui/translations.slint"

start_item = "    in-out property <string> "

rust_items = []
translation_items = []
# in-out property <string> scan_button_text: "Scan";
# translation.set_scan_button_text(flk!("scan_button").into());

with open(translations_slint_path, "r", encoding="utf-8") as file:
    for line in file:
        if line.startswith(start_item):
            line = line[len(start_item):]
            value = line.split("\"")[1]
            line = line.split(":")[0].strip()
            assert line.endswith("_text"), line
            item = line[:-5]
            rust_items.append(f"    translation.set_{item}_text(flk!(\"{item}\").into());")
            translation_items.append(f"{item} = {value}")
        elif "property" in line:
            assert False

for item in rust_items:
    print(item)

print("##############################################################")

for item in translation_items:
    print(item)