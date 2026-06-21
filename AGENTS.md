# Czkawka – Codebase Guide

## Language

All code, comments, commit messages, and documentation must be written in **English**.

---

## Rust Style

Applies to every Rust crate in the workspace (`czkawka_core`, `czkawka_cli`, `czkawka_gui`,
`krokiet`, `cedinia`). Program-specific deviations or elaborations live in that program's own
`AGENTS.md`.

**Formatting & lints**
- Format with `cargo +nightly fmt`, then `cargo fmt` for stable compatibility
  (`.rustfmt.toml`: `max_width = 120`, granular import grouping `group_imports = "StdExternalCrate"`).
- Code must compile without clippy warnings. Suppress a lint locally only with a comment
  explaining why.

**Errors & panics**
- Prefer `?`, `ok_or_else`, `map_err` over panic-based flows.
- `unwrap()` only in tests.
- `expect()` in production only when BOTH hold: there is no correct recovery path, and
  continuing would likely hide a serious bug - always with a precise, actionable message. This
  is visible throughout Krokiet/Cedinia callback code: a Slint weak reference is upgraded with
  `expect()` because if the window is already gone there is nothing meaningful to do except crash.
- Never silently swallow errors with `let _ = ...` unless the failure is genuinely irrelevant.
- Model domain errors with explicit types (`thiserror`/enums), not raw strings.
- Wrap I/O errors with context (e.g. file path) instead of letting std's bare "Permission
  denied" propagate - without the path it's hard to find the source.
- Don't ignore `Result`; handle it explicitly.

**Code shape**
- Keep functions focused; once one grows past ~30-50 lines, extract its logical steps into
  separate, well-named functions. Never carve a long function into sections with comment
  headers or bare `{ ... }` scope blocks - a `// validate input` comment over a block is the
  signal to pull it out into `validate_input(...)`, not to fence it off.
- Name things by intent, not by container/type (`user_count`, not `vec_users_len`).
- Make invalid states unrepresentable: enums over strings, `Option<T>` over sentinel values
  (`-1`, empty string).
- Prefer iterators and collection methods over manual index-based loops.
- Prefer `#[derive(...)]` over manual impls when behavior is standard.
- Avoid unnecessary copies and allocations; prefer references and `Cow` where it helps.
- Prefer external crates over reimplementing them.

**Comments**
Keep comments short and minimal - a single terse line, not a paragraph. Code should be
self-documenting through clear naming. Add a comment only when the _why_ is not easily inferred
from reading the code - algorithmic choices, non-obvious constraints, workarounds for external
library bugs, etc. Do not restate what the code already says. Never use the `—`/`–` dash
characters in code, comments, or commit messages - plain `-` only (ASCII box-drawing diagrams are
the only exception).

Exception: `czkawka_core` is a library - every frontend in this workspace depends on it, and so do
external consumers who only read its public API, not its internals. Its `pub fn`/`pub struct`
surface warrants more explanation than the rest of the workspace; see `czkawka_core/AGENTS.md`.

**Tests, fuzzers, benchmarks**
- Cover as much code as possible with tests; keep them readable with explicit `assert_eq!` and
  input data close to the assertions.
- Add fuzzers/examples/benchmarks where it helps show usage, find bugs, or measure the
  performance impact of a change.

**`unsafe`**
Avoid whenever possible. Every `unsafe` block documents its invariants and why the safety
requirements hold (see cedinia's `#[unsafe(no_mangle)] android_main`).

**Files & logging**
- Keep source files at or below 500 lines; split large files into modules.
- Always log via `log` (`handsome_logger`); reach for `tracing` only if actually needed.

**Auto-vectorization**
LLVM vectorizes a loop when it can prove: no loop-carried dependency, no aliasing, no bounds
checks, and a favorable cost model.
- Enable it: `.iter()`/`.iter_mut()` + `.zip()` instead of index loops (drops bounds checks);
  `.chunks_exact(N)`/`.chunks_exact_mut(N)` + `.remainder()` for aligned SIMD chunk processing;
  `(&[T], &mut [T])` over `(&mut [T], &mut [T])` so LLVM can assume no aliasing; keep the loop
  body uniform (no per-element `if`/`match`, no early returns); use a reduction (`.sum()`/
  `.fold()`) instead of a running accumulator that depends on its own previous value.
- Kills it: `slice[i]` index loops, value-based `continue` inside an element loop, mutable
  state shared across iterations.
- Verify: inspect asm on godbolt.org for `vmulps`/`vpaddb`/`vpsubusb` vs scalar `mulss`/`movss`;
  measure the delta with `RUSTFLAGS="-C opt-level=3 -C no-vectorize-loops -C no-vectorize-slp"`;
  build with `RUSTFLAGS="-C target-cpu=native"` to unlock AVX2/AVX-512 on the host CPU.
- When LLVM won't vectorize, reach for explicit SIMD: `std::arch` (stable, arch-specific),
  `std::simd`/`portable_simd` (nightly, portable), or the stable `wide`/`pulp` crates.

**Performance micro-hints**
- Pre-allocate with `Vec::with_capacity(n)` / `HashMap::with_capacity(n)` when the count is
  roughly known - avoids the realloc ladder.
- Reuse buffers across calls (`buf.clear()`) instead of reallocating.
- For almost-always-short collections, consider `SmallVec`/`ArrayVec` (stack-allocated until
  they spill).
- Use the smallest enum discriminant that fits (`#[repr(u8)]`/`#[repr(u16)]`) for tighter
  packing and better cache density.
- Wrap Criterion bench inputs/outputs in `std::hint::black_box(...)` to stop dead-code
  elimination from invalidating the measurement.

---

## Project Goals

Two properties are non-negotiable across every sub-project:

1. **Performance first** – scanning and file operations must be fast. Parallelism via `rayon`,
   efficient algorithms (e.g. perceptual hashing, blake3), and careful memory use are the norm.
   Avoid unnecessary allocations or copies in hot paths.

2. **Minimal non-Rust dependencies** – every additional C/C++ library (or any other non-Rust
   language) makes cross-compilation harder, narrows the set of supported targets, and increases
   build complexity for contributors. Prefer pure-Rust crates. If a native library is truly
   necessary (e.g. `libheif`, `libraw`), gate it behind an optional Cargo feature so the default
   build stays fully Rust.

---

## `just fix` – baseline quality gate

Running `just fix` must pass before any merge request. It runs, in order:

1. Repo-wide auto-fix of `—`/`–`/`―` dash characters back to plain `-` (`.rs`, `.slint`, `.md`,
   `.ftl`; `AGENTS.md` and `justfile` themselves are excluded) – the enforcement behind the
   "never use em/en dash" style rule above.
2. `uv run ruff format --line-length 120` – Python code formatting.
3. `uv run mypy misc --strict` – static type checking for all scripts in `misc/`.
4. `bash misc/run_checks.sh` – project-specific checks:
   - `delete_unused_krokiet_slint_imports.py` for krokiet and cedinia
   - `find_unused_fluent_translations.py` for all four projects
   - `find_unused_slint_translations.py` for krokiet and cedinia
   - `find_unused_callbacks.py` for krokiet and cedinia
   - `find_unused_settings_properties.py` for krokiet and cedinia
5. `cargo +nightly fmt` – Rust formatting.
6. `cargo clippy --fix --all-features --all-targets` – Rust linting (single pass).
7. `cargo +nightly fmt` + `cargo fmt` again – re-format whatever clippy's fixes touched.

For a clippy pass that also covers the `--no-default-features` build, run `just clip` separately
(two passes: `--all-features` and `--no-default-features --features winit_software`).

If `just fix` produces any output on stderr or exits non-zero the code is not ready for review.

---

## Workspace Structure

```
czkawka/
├── czkawka_core/   # Scanning logic – shared library used by all frontends
├── czkawka_cli/    # Command-line interface
├── czkawka_gui/    # Legacy GTK 4 GUI (maintenance mode only)
├── krokiet/        # Primary desktop GUI – Slint-based
├── cedinia/        # Android / mobile GUI – Slint-based
└── misc/           # Scripts: AI translation, validation, benchmarks, CI helpers
```

Cargo workspace resolver v3, minimum Rust 1.94.1, edition 2024 throughout.

---

## czkawka_core

The shared scanning engine. Every frontend depends on it; it has no UI dependency.

**Key modules:**
- `common/` – `CommonToolData` (settings, stop-flag, progress sender), `DirTraversal`, cache,
  extension filtering, path helpers, progress types.
- `tools/` – One sub-module per scanning tool:
  `duplicate`, `empty_folder`, `empty_files`, `big_file`, `similar_images`, `similar_videos`,
  `same_music`, `broken_files`, `bad_extensions`, `bad_names`, `invalid_symlinks`, `temporary`,
  `exif_remover`, `video_optimizer`.
- `localizer_core.rs` – Fluent translation loader for Rust-side messages.

Each tool implements the `CommonData` trait (shared settings access) and `PrintResults` (CSV/JSON
export). The tool struct is constructed, configured, then its `find_*()` method is called in a
worker thread. Progress is reported over a `crossbeam` channel; a stop `AtomicBool` is polled to
support cancellation.

---

## krokiet

The primary desktop GUI. Built with [Slint](https://slint.dev/) (GPL-3.0). The UI is declared
in `ui/*.slint`; Rust connects callbacks and drives the Slint model.

**Build:** `slint_build` compiles `.slint` sources at build time; `slint::include_modules!()`
exposes the generated types.

**Entry point:** `src/main.rs`
- Loads settings, creates `MainWindow`, wires up all callbacks, starts the event loop.

**Callback pattern:**
```rust
let weak = app.as_weak();
app.global::<Callabler>().on_some_action(move || {
    let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
    // ...
});
```
Each feature area lives in a dedicated `connect_*.rs` file (e.g. `connect_scan.rs`,
`connect_compare.rs`, `connect_delete_button.rs`).

**`SharedModels`** (`src/shared_models.rs`):
An `Arc<Mutex<SharedModels>>` holds the last scan result and parameters of scan of each tool. It is passed to every
`connect_*` function that needs to access or mutate scan state from a background thread.

**Model layer:**
- The Slint UI is driven by `ModelRc<VecModel<SingleMainListModel>>`.
- `SingleMainListModel` carries `val_str: [string]` and `val_int: [int]` vectors – a flat,
  index-based row representation.
- Column indices for each tool are defined as constants in `src/common.rs`
  (`StrDataSimilarImages`, `StrDataDuplicates`, …).

**Translation:** `flk!("key")` / `flk!("key", var = value)` macros defined in
`src/localizer_krokiet.rs`. Language files in `i18n/<lang-code>/krokiet.ftl`.

---

## cedinia

The Android (and secondary desktop) GUI. Architecture mirrors Krokiet but adapts to mobile
constraints. Compiled as `cdylib` for Android (loaded via `android-activity`).

**Entry points:**
- Android: `#[unsafe(no_mangle)] fn android_main(android_app: AndroidApp)` in `src/lib.rs`
- Desktop: `fn run_app()` in `src/app.rs`

**Android-specific:**
- File picker uses JNI to call into a Kotlin/Java helper embedded via `include_bytes!` (DEX).
- Storage permissions requested at runtime; `AppState.storage_permission_granted` gates scanning.
- System insets (`inset_top`, `inset_bottom`) plumbed through to Slint for edge-to-edge layout.
- `android_logger` routes Rust log output to logcat.

**Differences from Krokiet:**
- Has `SimilarVideos` (audio-fingerprint matching only, via `rusty-chromaprint`), but not
  `VideoOptimizer` - ffmpeg-based transcoding/crop-detection is not available on Android.
- Touch-optimised UI (`cedinia/ui/`); momentum-scroll views, bottom sheets, FAB.
- `flc!` macro (cedinia-specific) in `src/localizer_cedinia.rs`.
- See `cedinia/AGENTS.md` ("Differences from krokiet") for the full comparison table.

**Translation:** `flc!("key")` macro; language files in `cedinia/i18n/<lang-code>/cedinia.ftl`.

---

## czkawka_cli

Thin wrapper around `czkawka_core`. Uses `clap` (derive API) for argument parsing and `indicatif`
for progress bars. No GUI code. Results printed via the tool's `PrintResults` trait.

---

## czkawka_gui

Legacy GTK 4 GUI. **Maintenance mode only** – no new features are added. Bug-fixes that
keep it compatible with core API changes are accepted.

---

## misc/

**Translation tooling** (`ai_translate/`):
- `translate.py` – AI-powered batch translation into all supported languages.
- `validate_translations.py` – Checks placeholder consistency across translations.
  Pass `--fix` to automatically remove invalid entries.

**Dead-code detection** (run by `run_checks.sh`, see `just fix` above):
- `find_unused_fluent_translations.py` / `find_unused_slint_translations.py` – Unused
  translation keys.
- `find_unused_callbacks.py` – Slint callbacks never invoked from Rust.
- `find_unused_settings_properties.py` – Settings struct fields never read by the UI.
- `delete_unused_krokiet_slint_imports.py` – Removes dead `import` lines from `.slint` files.

**Packaging / release:**
- `gen_cedinia_licenses.py` – Generates `THIRD_PARTY_LICENSES.txt` from Cargo metadata.
- `gen_android_icons.py` – Generates cedinia's Android adaptive-icon assets from an SVG logo.
- `simplify_and_minify_svg.py` – Minifies SVG icons via Inkscape.
- `pack_all_backends.sh` / `.ps1` – Bundles an all-backends krokiet binary with per-backend
  launcher scripts into a release zip.
- `flathub.sh` – Generates Flatpak cargo-sources metadata for the Flathub manifest.
- `add_icon_exe/` – Cargo helper crate that embeds the `.ico` into Windows binaries at build time.
- `docker/` – `Dockerfile` for containerized builds.
- `nix/` – Nix flake (`flake.nix`, `packages.nix`) for Nix-based builds.
- `install_scripts/` – `install_linux.sh`, `install_macos.sh`, `install_windows.bat` end-user
  installers.

**Dev utilities:**
- `remove_comments.py` – Strips comments from source files (one-off cleanup tool).
- `compare_files.sh` – Diffs MD5 hashes of CI build artifacts across runs to check determinism.
- `run_checks.sh` – Runs all the dead-code detection scripts above; invoked by `just fix`.

**Benchmarks** (standalone Cargo crates):
- `test_image_perf/`, `test_read_perf/` – Microbenchmarks for image hashing / file reading.
- `test_compilation_speed_size/` – Tracks build time and binary size across changes.

---

## i18n

All user-visible strings use [Fluent](https://projectfluent.org/) (`.ftl` files).

| Project      | Macro  | File pattern                                |
|--------------|--------|---------------------------------------------|
| krokiet      | `flk!` | `krokiet/i18n/<lang>/krokiet.ftl`           |
| cedinia      | `flc!` | `cedinia/i18n/<lang>/cedinia.ftl`           |
| czkawka_core | `flc!` | `czkawka_core/i18n/<lang>/czkawka_core.ftl` |
| czkawka_gui  | `flg!` | `czkawka_gui/i18n/<lang>/czkawka_gui.ftl`   |

English is the source/fallback language. All other locales are AI-translated and then validated.

**Important:** Only edit the English `.ftl` files (`i18n/en/`) directly in this repository.
All other language files are managed through [Crowdin](https://crowdin.com/) and will be
**overwritten** when translations are pulled from Crowdin. Any manual edits to non-English
`.ftl` files in the repo will be lost on the next `just unpack_translations` run.

---

## Slint UI conventions

- **Hidden Text elements for width measurement** – where a layout element must adapt its width to
  translated label text, add off-screen `Text` instances (`x: -10000px; y: -10000px; height: 0`)
  and compute `preferred-width` at runtime (see `LeftSidePanel`, `CompareInfoBar`).
- **Enums over strings** – UI state that takes a fixed set of values should use a Slint `enum`,
  not a `string` (e.g. `ConfirmPopupAction`, `ActiveTool`, `ScanState`).
- **Global state** – Application-wide state lives in Slint `global` blocks (`GuiState`,
  `AppState`, `Settings`, `Translations`, …). Rust reads/writes via `app.global::<GlobalName>()`.

---

## Build profiles (Cargo.toml)

| Profile        | Purpose                                                              |
|----------------|----------------------------------------------------------------------|
| `release`      | Standard release                                                     |
| `fast_release` | Incremental, stripped – fast iteration                               |
| `rdebug`       | Release + full debug symbols (profiling)                             |
| `fastest`      | Max opt, LTO, panic=abort – mostly benchmarks/poc how fast it can be |
| `fastci`       | Small binary, fast CI builds                                         |

---

## justfile quick reference

```
just run krokiet          # debug run
just runr krokiet         # fast_release run
just fix                  # format + clippy + Python checks
just translate            # AI-translate all projects
just validate_translations [--fix]
just pack_translations    # create i18n_translations.zip for Crowdin
just unpack_translations <path>
just android              # build + install + launch on device
just androidr             # release variant
```
