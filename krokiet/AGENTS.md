# krokiet – Architecture Guide

## Overview

Primary desktop GUI. Built with [Slint](https://slint.dev/) (GPL-3.0). The UI
is declared in `ui/*.slint`; Rust wires all callbacks and drives the Slint data
models. All scanning logic lives in `czkawka_core`.

---

## Source Layout

```
krokiet/
├── build.rs                          # slint_build::compile("ui/main_window.slint")
├── src/
│   ├── main.rs                       # Entry point: load settings, create MainWindow,
│   │                                  #   wire all callbacks, run event loop
│   ├── shared_models.rs              # SharedModels – Arc<Mutex<…>> of all tool results
│   ├── common.rs                     # Column index enums (IntDataDuplicateFiles, …)
│   │                                  #   and sort index helpers
│   ├── localizer_krokiet.rs          # flk! macro, LANGUAGE_LOADER_KROKIET
│   ├── set_initial_gui_info.rs       # Asserts Slint combo lists == Rust enums
│   ├── set_initial_scroll_list_data_indexes.rs
│   ├── simpler_model.rs              # Minimal serializable row struct for threading
│   ├── audio_player.rs               # Play sound on scan completion
│   ├── notification_manager.rs       # Desktop notifications
│   ├── create_calculate_task_size.rs # Background total-size calculator
│   ├── clear_outdated_video_thumbnails.rs
│   ├── connect_scan.rs               # Routes scan to per-tool functions
│   ├── connect_scan/                 # One file per tool (duplicate.rs, …)
│   ├── connect_progress_receiver.rs  # Receives ProgressData → updates UI bar
│   ├── connect_row_selection.rs      # Row checkbox logic, group handling
│   ├── connect_sort.rs               # Column-header sort (string + int indices)
│   ├── connect_translation.rs        # LANGUAGE_LIST, change_language(), translate_items()
│   ├── connect_compare.rs            # Image comparison overlay
│   ├── connect_show_preview.rs       # File preview / thumbnail
│   ├── connect_open.rs               # Open file/folder in system app
│   ├── connect_save.rs               # Export results via PrintResults
│   ├── connect_show_confirmation.rs  # Confirmation popup before destructive action
│   ├── connect_directories_changes.rs
│   ├── connect_clean_cache.rs
│   ├── connect_tab_changed.rs
│   ├── connect_stop.rs               # Sets stop_flag = true
│   ├── connect_rfd.rs                # File picker (rfd)
│   ├── connect_select/
│   │   ├── mod.rs
│   │   └── custom_select.rs          # Advanced filter popup
│   ├── file_actions/
│   │   ├── connect_delete.rs
│   │   ├── connect_move.rs
│   │   ├── connect_rename.rs
│   │   ├── connect_hardlink.rs
│   │   ├── connect_symlink.rs
│   │   ├── connect_optimize_video.rs
│   │   └── connect_clean_exif.rs
│   ├── model_operations/
│   │   ├── mod.rs                    # Slint model ↔ thread-safe model conversion
│   │   └── model_processor.rs        # Parallel item processing framework
│   └── settings/
│       ├── mod.rs                    # Load/save JSON settings (11 presets)
│       ├── model.rs                  # BasicSettings, SettingsCustom structs
│       └── combo_box.rs              # StringComboBoxItems + regenerate_items()
└── ui/
    ├── main_window.slint             # Root component; imports everything; exports globals
    ├── common.slint                  # Enums + structs: ActiveTab, SingleMainListModel, …
    ├── gui_state.slint               # Global GuiState (transient runtime state)
    ├── settings.slint                # Global Settings (persisted)
    ├── translations.slint            # Global Translations (all UI strings)
    ├── callabler.slint               # Global Callabler (all Rust→UI callbacks)
    ├── left_side_panel.slint         # Tool selector + tool settings sub-panel
    ├── main_lists.slint              # Results table (bound to model)
    ├── action_buttons.slint          # Scan / Stop / Select / Delete / … buttons
    ├── bottom_panel.slint            # Progress bar + error text + status
    ├── progress.slint                # Real-time scan progress widget
    ├── preview.slint                 # File preview / thumbnail side panel
    ├── image_compare.slint           # Side-by-side image diff overlay
    ├── settings_list.slint           # Settings tab form
    ├── tool_settings.slint           # Per-tool options (hash alg, threshold, …)
    ├── about.slint                   # About tab
    ├── selectable_tree_view.slint    # Custom tree widget for path lists
    ├── fonts.slint                   # FontSizes global
    ├── color_palette.slint           # Dark-theme color definitions
    └── popup_*.slint                 # 15 modal dialogs
```

---

## Slint Performance: ListView vs. for-in / ScrollView

When displaying more than a few dozen items, **always use `ListView`** (from
`std-widgets.slint`)
rather than a bare `ScrollView` / `for` loop inside a `VerticalLayout`.

```slint
// SLOW – instantiates every item upfront, O(n) per frame
ScrollView {
    VerticalLayout {
        for item in model : Row { ... }
    }
}

// SLOW - same as above
VerticalLayout {
    for item in model : Row { ... }
}

// FAST – virtual scroll via Flickable's Repeater optimization
ListView {
    for item in model : Row { ... }
}
```

`ListView` uses Slint's Repeater-inside-Flickable
optimization: only visible rows (plus a small buffer) are instantiated at any
given time.  With a plain `ScrollView` + `VerticalLayout`, all N items are
created and visited on every frame.

Reference: Slint issue [#11021](https://github.com/slint-ui/slint/issues/11021).

---

## SharedModels (`src/shared_models.rs`)

```rust
pub struct SharedModels {
    pub shared_duplication_state:       Option<DuplicateFinder>,
    pub shared_empty_folders_state:     Option<EmptyFolder>,
    // … one field per tool (14 total)
}
```

Created once in `main.rs` as `Arc<Mutex<SharedModels>>`, cloned and passed to
every callback that needs scan results (preview, compare, save, delete, …).

Worker threads lock exclusively to **write** results after a scan finishes.
UI callbacks lock briefly to **read** results for previews / exports.

---

## Scan Data Flow

```
1. User clicks Scan → on_scan_starting(active_tab)
2. connect_scan.rs: collect_settings() → ScanData → route to scan_<tool>()
3. scan_<tool>(): spawn worker thread
       → create and configure tool struct
       → tool.search(stop_flag, progress_sender)
       → store tool in shared_models.lock()
       → populate Slint VecModel<SingleMainListModel>
       → app.upgrade_in_event_loop(|app| app.set_<tool>_model(model))
4. UI re-renders with new model
```

---

## Slint Data Model

```rust
// ui/common.slint
export struct SingleMainListModel {
    checked: bool,           // row selected by user
    header_row: bool,        // group header
    filled_header_row: bool, // header with formatted summary
    focused_row: bool,      // UI highlight
    val_str: [string],       // text columns
    val_int: [int],          // numeric columns (raw values for sorting)
}
```

Column indices are constants in `src/common.rs`:

```rust
pub enum StrDataDuplicateFiles { Size, Name, Path, ModificationDate }
pub enum IntDataDuplicateFiles { ModificationDatePart1, ModificationDatePart2,
                                  SizePart1, SizePart2 }
```

Slint cannot store `i64`, so dates and file sizes are split into two `i32` fields.

---

## Translation System

Slint has no native Fluent support. Workaround:
1. All UI text is bound to properties in the `Translations` global
   (`ui/translations.slint`), not hardcoded in `.slint`.
2. `translate_items()` in `connect_translation.rs` (~350 lines) sets every
   property via `flk!("key")` after language changes.
3. Language list defined as `LANGUAGE_LIST: &[Language]` in
   `connect_translation.rs`.
4. Hardcoded combo-box lists in `.slint` files (e.g. `Settings.languages_list`)
   are verified at startup against Rust enums via `assert_eq!` in
   `set_initial_gui_info.rs` (workaround for [slint#7632](https://github.com/slint-ui/slint/issues/7632)).

---

## Settings (`src/settings/`)

Two-tier JSON system:

| File | Content |
|------|---------|
| `~/.config/Czkawka/krokiet/base.json` | `BasicSettings` – default preset index, preset names, theme, window size |
| `~/.config/Czkawka/krokiet/preset_N.json` | `SettingsCustom` – paths, extensions, tool parameters |

11 preset slots (indices 0–10). Slot 10 is reserved for CLI-mode overrides.
`StringComboBoxItems::regenerate_items()` builds all combo box option arrays
from Rust enums; used both for settings serialization and UI initialization.

---

## Callback Registration Pattern

```rust
let weak = app.as_weak();
app.global::<Callabler>().on_some_action(move || {
    let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
    // use app …
});
```

`expect()` is correct here: if the window is gone, no callback should fire.
Cross-thread updates use `weak.upgrade_in_event_loop(|app| { … })`.

---

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `slint` 1.15 | UI framework (GPL-3.0) |
| `czkawka_core` | Scanning engine |
| `i18n-embed` + `rust-embed` | Fluent translations |
| `crossbeam-channel` | Progress + result channels |
| `rayon` | Parallel result sorting |
| `serde_json` | Settings persistence |
| `image` | Thumbnail loading |
| `rfd` 0.17 | File picker (xdg-portal backend) |
| `notify-rust` | Desktop notifications |
| `rodio` | Scan-complete sound (optional `audio` feature) |
| `fontique` | Font discovery (dlopen, avoids C dep) |

### Optional features

- `audio` – scan-complete sound via `rodio`
- Renderer features: `winit_femtovg` (default), `winit_software` (default),
  `skia_opengl`, `skia_vulkan`, etc. – select GPU backend at compile time
- `heif`, `libraw`, `libavif`, `xdg_portal_trash` – forwarded to `czkawka_core`

### Build

`build.rs` compiles `ui/main_window.slint` with style `fluent-dark` (overridable
via `SLINT_STYLE` env var).
