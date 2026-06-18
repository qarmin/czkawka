# cedinia – Architecture Guide

## Overview

Android (and secondary desktop) GUI. Architecture mirrors krokiet but adapted
for mobile constraints: touch-optimized layout, JNI file picker, runtime
permissions, system insets, and no `VideoOptimizer` (FFmpeg-based transcoding
is unavailable on Android). `SimilarVideos` is still present, using
audio-fingerprint matching only (`rusty-chromaprint`), not frame hashing.

---

## Entry Points

**Android** (`src/lib.rs`):
```rust
#[unsafe(no_mangle)]
fn android_main(android_app: AndroidApp) {
    setup_android_paths(&android_app);   // JNI: get /data and /cache paths (sets DATA_DIR)
    app::setup_logger_cache();           // installs DualLogger: logcat + cedinia.log file
    asan_smoketest_if_requested();       // no-op unless CEDINIA_ASAN_SMOKETEST is set
    file_picker_android::init(&android_app); // Load DEX + init JNI
    slint::android::init(android_app.clone()).expect(…);
    file_picker_android::setup_nav_bar(); // pin/style the Android nav bar
    app::run_app_with_insets(inset_bottom_px, scale, android_app);
}
```

Logging: on Android `android_logger` would grab the single global `log` slot, so
czkawka_core's file `WriteLogger` never installs (cedinia.log stays 0 bytes).
`app.rs::setup_android_logger` installs a `DualLogger` fanning out to BOTH the
`AndroidLogger` (logcat) and an appended `cedinia.log` in the cache folder, so
log export has real content. Desktop keeps `czkawka_core::setup_logger`.

**Desktop** (`src/app.rs`):
```rust
pub fn run_app() {
    setup_logger_cache();
    run_app_with_insets(0.0, 1.0, ());
}
```

Both converge at `run_app_inner()`: load settings → create `MainWindow` →
wire callbacks → start worker thread → run event loop.

---

## Source Layout

```
cedinia/src/
├── lib.rs                         # Android entry (#[unsafe(no_mangle)] android_main)
├── app.rs                         # Desktop entry + shared run_app_inner()
├── bin/cedinia.rs                 # Desktop binary wrapper
├── common.rs                      # Column index enums (StrData*, IntData*)
├── model.rs                       # FileEntry toggle/count logic
├── compare.rs                     # Image comparison overlay (cancel token, resize, diff)
├── file_actions.rs                # delete_path() (trash on desktop, permanent on Android) + DeleteEvent
├── scan_runner.rs                 # Worker thread + ScanRequest/ScanResult
├── scanners.rs                    # Tool-specific scan builders
├── notifications.rs               # Desktop notifications
├── set_initial_gui_infos.rs       # Initial GUI state
├── thumbnail_loader.rs            # Async image thumbnail loading
├── translations.rs                # translate_items() – populates Translations global
├── volumes.rs                     # Storage volume detection
├── localizer_cedinia.rs           # flc! macro, LANGUAGE_LIST, apply_language_preference()
├── file_picker_android.rs         # JNI + embedded DEX file picker
├── callbacks/
│   ├── callbacks.rs               # Module re-exports
│   ├── scan.rs                    # wire_scan() – scan/stop/tool-change callbacks
│   ├── selection.rs               # wire_selection() – delete + model access
│   ├── selection_ops.rs           # Shared FileEntry helpers: vm_of, get_val_str/int, full_path_of, size_from_entry
│   ├── directories.rs             # wire_directories() – add/remove include/exclude dirs
│   └── misc.rs                    # 10 wire_* fns: cache_info, collect_test, export_logs,
│                                   #   language_change, licenses_popup, notification_settings,
│                                   #   open_path, open_url, permission, save_settings_now
├── settings/
│   ├── mod.rs                     # CediniaSettings struct; load/save JSON
│   └── gui_settings_values.rs     # StringComboBoxItems – combo box option arrays
└── ui/
    ├── main_window.slint          # Root component
    ├── globals/
    │   ├── app_state.slint        # Global AppState, GeneralSettings, tool-specific globals
    │   ├── common.slint           # Enums: ActiveTool, ScanState, ConfirmPopupAction
    │   ├── colors.slint           # CediniaColors theme
    │   └── fonts.slint
    ├── screens/
    │   ├── home_screen.slint      # Tool cards
    │   ├── settings_screen.slint  # Settings screen layout
    │   ├── results_list.slint     # Generic results table
    │   ├── similar_images_gallery.slint
    │   ├── directories_screen.slint # Add/remove directory paths
    │   ├── image_compare.slint
    │   ├── scan_progress.slint
    │   ├── bottom_nav.slint
    │   └── top_bar.slint
    └── components/
        ├── settings/
        │   ├── toggle_row.slint
        │   ├── segment_row.slint
        │   ├── dropdown_row.slint
        │   ├── text_input_row.slint
        │   ├── tool_group_header.slint
        │   └── category_label.slint
        ├── buttons/
        │   ├── touch_button.slint
        │   ├── icon_button.slint
        │   └── circle_icon_button.slint
        ├── file_row.slint
        ├── section_header.slint
        ├── status_chip.slint
        ├── divider.slint
        ├── empty_state.slint
        ├── scan_fab.slint
        ├── scrim_dialog.slint
        ├── popup_title_bar.slint
        ├── context_menu_sheet.slint
        ├── deselection_menu_sheet.slint
        └── selection_menu_sheet.slint
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

## Scan Data Flow

```
1. User selects tool + taps Scan → AppState.on_scan_requested()
2. scan.rs: collect settings from globals → build ScanRequest
3. ScanRequest sent to worker thread via channel
4. Worker: configure tool struct → tool.search(stop_flag, progress_sender)
5. Results stored; ScanResult sent back to main thread
6. invoke_from_event_loop: populate model, update AppState.scan_state
```

---

## Slint UI Architecture

Settings are stored in Slint globals (not set from Rust at startup):

- `GeneralSettings` – language, file sizes, cache, extension filters
- `DuplicateSettings`, `SimilarImagesSettings`, `SameMusicSettings`, …
- `AppState` – scan state, active tool, result models, all callbacks

`apply_settings_to_gui()` (`settings/mod.rs`) reads `CediniaSettings` from
JSON and populates these globals. `collect_settings_from_gui()` reads them back
for saving.

### DropdownRow (`ui/components/settings/dropdown_row.slint`)

Inline accordion, **not** a `PopupWindow`:
- Tapping the row toggles `expanded`, which inserts the option list directly
  below the row inside the same `VerticalLayout`.
- Scrolling is handled by the parent `Flickable`/`ListView`, not a nested
  `ScrollView` - this avoids "close on outside touch" popup-dismissal issues
  when the row sits inside an already-scrollable settings list.
- Each option row tracks pointer movement (`_opt_moved`) so a scroll-drag that
  ends over an option doesn't register as a selection tap.

## Translation System

Same workaround as krokiet:
- All UI text bound to `Translations` global properties.
- `translate_items()` in `src/translations.rs` populates them via `flc!("key")`.
- `LANGUAGE_LIST` in `localizer_cedinia.rs` defines the 27 supported languages.
- `GeneralSettings.language_options` in `ui/app_state.slint` is hardcoded and
  must match `LANGUAGE_LIST` exactly.
- Language index stored in `GeneralSettings.language_idx`; code stored in
  `CediniaSettings.language`.

---

## Settings (`src/settings/mod.rs`)

JSON file (path resolved by `czkawka_core::common::config_cache_path::get_config_cache_path()`,
configured via `set_config_cache_path("cedinia", "cedinia")`):
- Desktop: `~/.config/cedinia/cedinia_settings.json` (directories-next ignores
  the qualifier/org on Linux and lowercases the app name - there is no
  `Czkawka` path segment)
- Android: app-private storage, `$DATA_DIR/files/cedinia/cedinia_settings.json`

Included/excluded directories are persisted separately in a sibling
`cedinia_dirs.json` in the same folder.

`CediniaSettings` (serde Serialize/Deserialize) covers all tool parameters.
`Default` implementation delegates to `serde_json::from_str("{}")` so all
`#[serde(default)]` values are used.

---

## Android-Specific Code

### File Picker (`src/file_picker_android.rs`)

A Kotlin helper class is compiled at build time (via `android-build` crate) and
embedded as DEX bytecode:

```rust
include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"))
```

At runtime:
1. DEX is loaded into an `InMemoryDexClassLoader`.
2. `CediniaFilePicker` instance created via JNI.
3. `launch_pick_directory(is_include, start_path)` calls the Java
   `ACTION_OPEN_DOCUMENT_TREE` intent, hinting the picker to open at `start_path`.
4. Result arrives via `Java_CediniaFilePicker_onDirectoryPicked()` JNI callback
   → dispatched to Rust via `invoke_from_event_loop()`.

### Storage Permissions

Runtime permission check/request via JNI.
`AppState.storage_permission_granted` gates scan initiation.

### System Insets

Android edge-to-edge layout requires padding for navigation bars. Inset sizes
are passed from Kotlin via `android_app.content_rect()` and plumbed through to
Slint as layout properties.

### Android Paths via JNI

`setup_android_paths()` calls `getFilesDir()` and `getCacheDir()` on the
Activity to find private storage paths; these are set as environment variables
and used by `czkawka_core`'s config path logic.

---

## Differences from krokiet

| | cedinia | krokiet |
|---|---|---|
| `SimilarVideos` | Yes (audio fingerprint only, `rusty-chromaprint`) | Yes (frame hashing) |
| `VideoOptimizer` | No (ffmpeg-based transcoding unavailable on Android) | Yes |
| `BadNames` | Yes | Yes |
| File picker | JNI (Android) / rfd (desktop) | rfd |
| Settings | JSON, no presets | JSON, 11 presets |
| Column indices | `common.rs` (own set) | `common.rs` (own set) |
| Build target | `cdylib` (Android) + `bin` (desktop) | `bin` |

---

## Key Dependencies

### Always

| Crate | Purpose |
|-------|---------|
| `slint` 1.15 | UI framework |
| `czkawka_core` | Scanning engine |
| `crossbeam-channel` | Worker thread communication |
| `i18n-embed` + `rust-embed` | Fluent translations |
| `serde_json` | Settings persistence |
| `image` | Thumbnail loading |

### Android only

| Crate | Purpose |
|-------|---------|
| `android-activity` 0.6 | Android lifecycle |
| `jni` 0.22 | Java FFI |
| `android_logger` 0.15 | Logcat output |

### Desktop only

| Crate | Purpose |
|-------|---------|
| `rfd` 0.17 | Native file picker |
| `trash` 5.2 | Move-to-trash |
| `notify-rust` 4 | Desktop notifications |

### Build

```toml
[build-dependencies]
slint-build = "1.15"
android-build = "0.1.2"   # Compiles CediniaFilePicker.java → classes.dex
```

### Android Manifest Permissions

`READ_EXTERNAL_STORAGE`, `WRITE_EXTERNAL_STORAGE`,
`MANAGE_EXTERNAL_STORAGE`, `POST_NOTIFICATIONS`

`Cargo.toml`'s `[package.metadata.android.uses_permission]` also lists
`WAKE_LOCK`, but the gradle-built `android/app/src/main/AndroidManifest.xml`
does not include it - a pre-existing inconsistency in the repo, not a doc bug.

---

## Optional Features (forwarded to czkawka_core)

`heif`, `libraw`, `libavif`, `xdg_portal_trash`
