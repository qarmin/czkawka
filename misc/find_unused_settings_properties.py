import re
import os
from pathlib import Path
from typing import Set, Dict, List, Tuple


def extract_settings_properties(settings_file: Path) -> Dict[str, str]:
    properties = {}

    with open(settings_file, "r", encoding="utf-8") as f:
        content = f.read()

    # Match property definitions like:
    # in-out property <bool> dark_theme: true;
    # in-out property <string> language_value: "English";
    # out property <length> path_px: 350px;
    property_pattern = r"(?:in-out|out|in)\s+property\s+<([^>]+)>\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*:"

    for match in re.finditer(property_pattern, content):
        prop_type = match.group(1).strip()
        prop_name = match.group(2).strip()
        properties[prop_name] = prop_type

    return properties


def find_rust_files(project_root: Path) -> List[Path]:
    rust_files = []
    krokiet_src = project_root / "krokiet" / "src"

    if krokiet_src.exists():
        for rust_file in krokiet_src.rglob("*.rs"):
            rust_files.append(rust_file)

    return rust_files


def check_property_usage_in_rust(rust_files: List[Path], property_name: str) -> Tuple[bool, bool]:
    getter_pattern = rf"\.get_{property_name}\("
    setter_pattern = rf"\.set_{property_name}\("

    has_getter = False
    has_setter = False

    for rust_file in rust_files:
        try:
            with open(rust_file, "r", encoding="utf-8") as f:
                content = f.read()

                if re.search(getter_pattern, content):
                    has_getter = True
                if re.search(setter_pattern, content):
                    has_setter = True

                # Early exit if both found
                if has_getter and has_setter:
                    return True, True
        except Exception as e:
            print(f"Warning: Could not read {rust_file}: {e}")
            continue

    return has_getter, has_setter


def main() -> None:
    script_dir = Path(__file__).parent
    project_root = script_dir.parent

    settings_file = project_root / "krokiet" / "ui" / "settings.slint"

    if not settings_file.exists():
        print(f"Error: Settings file not found at {settings_file}")
        return

    print(f"Reading properties from: {settings_file}")
    properties = extract_settings_properties(settings_file)
    print(f"Found {len(properties)} properties in settings.slint\n")

    print("Finding Rust files...")
    rust_files = find_rust_files(project_root)
    print(f"Found {len(rust_files)} Rust files to check\n")

    # Check each property
    missing_getters = []
    missing_setters = []
    missing_both = []

    print("Checking property usage in Rust code...")
    print("-" * 80)

    for prop_name, prop_type in sorted(properties.items()):
        has_getter, has_setter = check_property_usage_in_rust(rust_files, prop_name)

        if not has_getter and not has_setter:
            missing_both.append((prop_name, prop_type))
        elif not has_getter:
            missing_getters.append((prop_name, prop_type))
        elif not has_setter:
            missing_setters.append((prop_name, prop_type))

    # Print results
    print("\n" + "=" * 80)
    print("RESULTS")
    print("=" * 80)

    if missing_both:
        print(f"\nProperties with NO getter AND NO setter ({len(missing_both)}):")
        print("-" * 80)
        for prop_name, prop_type in missing_both:
            print(f"  • {prop_name:<50} <{prop_type}>")

    if missing_getters:
        print(f"\nProperties with NO getter (but has setter) ({len(missing_getters)}):")
        print("-" * 80)
        for prop_name, prop_type in missing_getters:
            print(f"  • {prop_name:<50} <{prop_type}>")

    if missing_setters:
        print(f"\nProperties with NO setter (but has getter) ({len(missing_setters)}):")
        print("-" * 80)
        for prop_name, prop_type in missing_setters:
            print(f"  • {prop_name:<50} <{prop_type}>")

    if not missing_both and not missing_getters and not missing_setters:
        print("\nAll properties have both getters and setters!")

    # Summary
    print("\n" + "=" * 80)
    print("SUMMARY")
    print("=" * 80)
    print(f"Total properties:               {len(properties)}")
    print(
        f"Properties fully used:          {len(properties) - len(missing_both) - len(missing_getters) - len(missing_setters)}"
    )
    print(f"Properties missing both:        {len(missing_both)}")
    print(f"Properties missing getter only: {len(missing_getters)}")
    print(f"Properties missing setter only: {len(missing_setters)}")
    print("=" * 80)


if __name__ == "__main__":
    main()
