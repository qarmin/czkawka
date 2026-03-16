#!/usr/bin/env bash

cmds=(
    "python3 misc/delete_unused_krokiet_slint_imports.py krokiet"
    "python3 misc/delete_unused_krokiet_slint_imports.py cedinia"
    "python3 misc/find_unused_fluent_translations.py czkawka_gui"
    "python3 misc/find_unused_fluent_translations.py krokiet"
    "python3 misc/find_unused_fluent_translations.py cedinia"
    "python3 misc/find_unused_fluent_translations.py czkawka_core"
    "python3 misc/find_unused_slint_translations.py krokiet"
    "python3 misc/find_unused_slint_translations.py cedinia"
    "python3 misc/find_unused_callbacks.py krokiet"
    "python3 misc/find_unused_settings_properties.py krokiet"
    "python3 misc/find_unused_settings_properties.py cedinia"
)

failed=""
for cmd in "${cmds[@]}"; do
    out=$(eval "$cmd" 2>&1)
    if [ $? -ne 0 ]; then
        failed+="=== FAILED: $cmd ===\n$out\n\n"
    fi
done

if [ -n "$failed" ]; then
    echo -e "$failed"
    exit 1
fi

