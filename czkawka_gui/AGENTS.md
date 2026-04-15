# czkawka_gui – Architecture Guide

## Status: Maintenance mode only

No new features. Bug-fixes that keep it compatible with `czkawka_core` API
changes are accepted.

---

## Overview

GTK4 frontend using XML UI files authored in Cambalache. All scanning tools
available in `czkawka_core` are exposed via a notebook (tab-based) interface.

---

## Source Layout

```
czkawka_gui/src/
├── main.rs                          # GTK Application setup, GuiData construction
├── initialize_gui.rs                # Initial widget state
├── compute_results.rs               # Converts scan results → GTK ListStore rows
├── saving_loading.rs                # JSON settings load/save
├── language_functions.rs            # LANGUAGES_ALL constant (26 entries)
├── localizer_gui.rs                 # flg! macro, Fluent loader
├── gui_structs/
│   ├── gui_data.rs                  # GuiData – central struct for all UI components
│   ├── gui_main_notebook.rs         # 11-tool tab notebook
│   ├── gui_upper_notebook.rs        # Included/excluded directory tabs
│   ├── gui_settings.rs              # Settings dialog
│   ├── gui_header.rs                # Menu bar (About, Settings)
│   ├── gui_bottom_buttons.rs        # Action buttons (Search, Delete, etc.)
│   ├── gui_progress_dialog.rs       # Scan progress window
│   ├── gui_about.rs                 # About dialog
│   ├── gui_compare_images.rs        # Side-by-side image diff overlay
│   ├── common_tree_view.rs          # TreeView/ListStore rendering + SharedModelEnum
│   ├── common_upper_tree_view.rs    # Dir-list TreeViews
│   ├── gui_popovers_select.rs       # Selection filter popovers
│   └── gui_popovers_sort.rs         # Sort filter popovers
├── connect_things/                  # Callback wiring files
│   ├── connect_button_search.rs     # Scan thread spawn + progress forwarding
│   ├── connect_button_delete.rs     # File deletion with confirmation
│   ├── connect_button_save.rs       # Export results (CSV/JSON)
│   ├── connect_button_select.rs     # Row selection helpers
│   ├── connect_button_sort.rs       # Column-header sort
│   ├── connect_button_stop.rs       # Cancel scan (stop flag)
│   ├── connect_button_move.rs       # Move files to folder
│   ├── connect_button_hardlink.rs   # Hard-link/symlink creation
│   ├── connect_button_compare.rs    # Image comparison launcher
│   ├── connect_duplicate_buttons.rs # Duplicate-specific UI
│   ├── connect_notebook_tabs.rs     # Tab-switch handler
│   ├── connect_selection_of_directories.rs  # Dir-picker dialogs
│   ├── connect_change_language.rs   # Language switch + full UI relabel
│   ├── connect_settings.rs          # Settings window callbacks
│   ├── connect_progress_window.rs   # Progress bar updates from channel
│   ├── connect_show_hide_ui.rs      # Error text visibility toggle
│   ├── connect_similar_image_size_change.rs
│   ├── connect_same_music_mode_changed.rs
│   ├── connect_popovers_select.rs
│   ├── connect_popovers_sort.rs
│   ├── connect_header_buttons.rs
│   ├── connect_about_buttons.rs
│   ├── connect_krokiet_info_dialog.rs  # One-time Krokiet migration notice
│   └── file_chooser_helpers.rs
├── helpers/
│   ├── enums.rs                     # Column index enums for all tools
│   ├── list_store_operations.rs     # Append/read GTK ListStore rows
│   ├── image_operations.rs          # Image resize/preview for GTK
│   ├── model_iter.rs                # GTK TreeView iteration
│   └── mod.rs
├── help_functions.rs                # HEADER_ROW_COLOR, MAIN_ROW_COLOR, set_buttons
├── help_combo_box.rs                # Combo box option arrays (hash algs, etc.)
├── notebook_enums.rs                # NotebookMainEnum (per-tool tab index)
├── notebook_info.rs                 # Tab label + ListStore column definitions
├── opening_selecting_records.rs     # Double-click → open file/folder
├── taskbar_progress.rs              # Windows/dummy taskbar progress
└── gtk_traits.rs                    # Custom GTK trait extensions

ui/
├── main_window.ui      # Cambalache XML (65 KB) – entire main window layout
├── settings.ui         # Settings dialog
├── compare_images.ui   # Image comparison panel
├── popover_select.ui   # Selection filter popover
├── popover_sort.ui     # Sort popover
├── about_dialog.ui     # About window
└── progress.ui         # Scan progress dialog
```

---

## GuiData Struct (`gui_structs/gui_data.rs`)

Central struct passed by clone/reference to every callback:

```rust
pub struct GuiData {
    pub window_main: gtk4::Window,
    pub main_notebook: GuiMainNotebook,         // 11 tool tabs
    pub upper_notebook: GuiUpperNotebook,       // Dir configuration
    pub bottom_buttons: GuiBottomButtons,       // Action buttons
    pub progress_window: GuiProgressDialog,
    pub settings: GuiSettings,
    pub header: GuiHeader,
    pub compare_images: GuiCompareImages,
    pub about: GuiAbout,
    pub popovers_select: GuiSelectPopovers,
    pub popovers_sort: GuiSortPopovers,
    pub entry_info: gtk4::Entry,
    pub text_view_errors: gtk4::TextView,
    pub taskbar_state: Rc<RefCell<TaskbarProgress>>,
    pub shared_buttons: Rc<RefCell<HashMap<NotebookMainEnum, …>>>,
    pub stop_flag: Arc<AtomicBool>,
}
```

Constructed by `GuiData::new_with_application()` which loads all `.ui` files
via `gtk4::Builder`.

---

## Scan Thread Architecture

```
User clicks Search
→ connect_button_search.rs
→ Disables UI, shows progress window
→ Spawn worker thread:
    - Read settings from GTK widgets
    - Create tool struct (e.g. DuplicateFinder)
    - Call tool.search(stop_flag, progress_sender)
    - Send Message::Duplicates(tool) via result_sender
→ Main thread: receive progress updates → update bars
→ On result received:
    - compute_results.rs processes tool data → appends to ListStore
    - Re-enables UI, hides progress
```

**Channel types:**
- `Sender<Message>` – one-shot result channel (tool enum with full data)
- `Sender<ProgressData>` – real-time progress from core

---

## Result Population (`compute_results.rs`)

1. Receive `Message::Duplicates(DuplicateFinder)` from worker thread.
2. Extract sorted file groups from the tool.
3. For each group: insert a **header row** (colored `HEADER_ROW_COLOR`), then
   one row per file (`MAIN_ROW_COLOR`).
4. `append_row_to_list_store()` writes individual cells to `gtk4::ListStore`.
5. `finalize_compute()` stores the tool object in `SubView.shared_model_enum`
   (for later re-use by selection/sort callbacks).

---

## Language Switching (`language_functions.rs`)

26 languages defined as `LANGUAGES_ALL: &[Language]` with `combo_box_text`
(display name) and `short_text` (BCP-47 code).

On language change:
1. Look up `short_text` from selected combo index.
2. Call `localizer.select([lang_id])` for both `czkawka_core` and `czkawka_gui`.
3. Call `gui_data.update_language()` → propagates to all sub-structs which
   update their widget labels via `flg!("key")`.

---

## Settings Storage (`saving_loading.rs`)

JSON file at `~/.config/Czkawka/czkawka_gui_config.json`. Struct `SettingsJson`
serialized with `serde_json`. Includes directories, tool parameters, UI state,
language preference.

---

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `gtk4` 0.11 | GTK4 GUI framework |
| `gdk4` 0.11 | Drawing/rendering |
| `glib` 0.22 | Event loop, `spawn_future_local` |
| `i18n-embed` + `rust-embed` | Fluent translations |
| `open` 5.3 | Launch file manager |
| `image` 0.25 | Preview thumbnails |
| `resvg` 0.47 | SVG icon scaling |
| `rayon` 1.10 | Parallel sort of results |
| `crossbeam-channel` | Result + progress channels |
| `czkawka_core` | Scanning engine |

Optional features (forwarded to core): `heif`, `libraw`, `libavif`,
`xdg_portal_trash`.
