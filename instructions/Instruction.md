# Instruction

Czkawka consists of several independent frontends sharing a common core library. Each frontend has its own dedicated guide:

- [Krokiet GUI (Slint frontend)](Instruction_Krokiet.md) - recommended GUI for all platforms
- [CLI (command-line frontend)](Instruction_CLI.md) - full flag reference and automation examples
- [GTK GUI (legacy)](Instruction_GTK.md) - **12.0 is the last version**, migrate to Krokiet
- [Core library integration](Instruction_Core.md) - tool descriptions, API, embedding czkawka_core
- [FAQ](FAQ.md) - frequently asked questions from 1200+ GitHub issues (blank window, ffmpeg, min file size, etc.)
- [Translations](Translations.md) - how to contribute or add a new language

---

## Terminology (shared across all frontends)

- **Reference paths**
  - After adding directories or files, you can mark them as "Reference paths" by checking the checkbox next to them (in the UI) or using the CLI flag where available.
  - Reference paths are used for comparison but are protected: they cannot be modified, moved, or deleted by any automatic action in the UI or CLI.
  - Supported by: Duplicate Files, Similar Images, Similar Videos, Same Music.
  - Use cases: compare a working folder against a master backup, protect original files when removing duplicates, use a dataset as a read-only baseline.

- **Included / Excluded paths**
  - Included paths are scanned by the tools. Excluded paths are explicitly ignored during scans.

- **Excluded items**
  - Glob patterns matched against full file and directory paths (e.g. `*/tmp*`, `*/.git`). More flexible than excluded paths but slightly slower.

- **Configuration vs Cache**
  - Configuration files are frontend-specific. Configuration is not shared between GTK, Krokiet, and CLI.
  - Cache files are shared across all frontends. They contain computed data (hashes, thumbnails, analysis results) placed in a shared cache directory so all frontends can reuse computed results.

---

## Config/Cache files

**Configuration files (per frontend, not shared):**

- GTK stores `czkawka_gui_config.txt` under the user config directory.
- Krokiet stores `config_general.json` and `config_preset_N.json` under the user config directory (`~/.config/krokiet/` on Linux).
- CLI does not store configuration files.

**Cache files (shared across all frontends):**

Cache contains computed results (hashes, thumbnails, parsed metadata) and is shared to avoid recomputation.

Notable files:
- `cache_similar_image_SIZE_HASH_FILTER.bin/.json` - image hashes
- `cache_duplicates_HASH.txt` - duplicate file hashes
- `cache_similar_videos.bin/.json` - video signatures
- `cache_broken_files.txt` - broken file check results

JSON-format cache files can be manually edited (useful when moving a collection to a different disk or machine). To generate them, enable "Save also as JSON" in settings. By default `.bin` files are loaded; if missing, the `.json` fallback is used.

**Default paths (GTK and CLI use "Czkawka" for both config and cache; Krokiet uses "Krokiet" for config but "Czkawka" for cache):**

| OS | GTK/CLI Config | Krokiet Config | Cache (all frontends) |
|----|----------------|----------------|-----------------------|
| Linux | `~/.config/czkawka/` | `~/.config/krokiet/` | `~/.cache/czkawka/` |
| macOS | `~/Library/Application Support/pl.Qarmin.Czkawka/` | `~/Library/Application Support/pl.Qarmin.Krokiet/` | `~/Library/Caches/pl.Qarmin.Czkawka/` |
| Windows | `%APPDATA%\Qarmin\Czkawka\config\` | `%APPDATA%\Qarmin\Krokiet\config\` | `%LOCALAPPDATA%\Qarmin\Czkawka\cache\` |

Override with environment variables:

```shell
CZKAWKA_CONFIG_PATH="/media/rafal/Ventoy/config" CZKAWKA_CACHE_PATH="/media/rafal/Ventoy/cache" krokiet
```

Portable setup script (place next to binary on a USB drive):

```shell
#!/bin/bash
CZKAWKA_CONFIG_PATH="$(dirname "$(realpath "$0")")/config"
CZKAWKA_CACHE_PATH="$(dirname "$(realpath "$0")")/cache"
./krokiet
```

---

## Tips, Tricks and Known Bugs

- **Speedup with LTO**  
  Add to `Cargo.toml` for a small performance boost and a large binary size reduction:
  ```toml
  [profile.release]
  lto = "thin" # or "fat"
  ```

- **Speedup with native CPU instructions**  
  Compiling with native CPU optimizations gives a 10-20% speedup for image hashing on x86_64-v4:
  ```shell
  RUSTFLAGS="-C target-cpu=native" cargo build --release
  ```
  Or set it globally in `~/.cargo/config.toml`:
  ```toml
  [target.x86_64-unknown-linux-gnu]
  rustflags = ["-C", "target-cpu=native"]
  ```

- **Slow cache loading**  
  If loading cache is slow (a very large collection was scanned previously), rename or delete the relevant cache file (e.g. `cache_similar_image_*.bin`). The cache regenerates on the next scan with only the currently scanned entries.

- **Partial scanning**  
  You can stop a scan mid-way. All computed hashes are already written to cache and will speed up the next full scan.

- **Prehash cache**  
  Caches partial hashes (first and last 4 KB) of large files so re-scans only need to fully hash new or changed files. **Enabled by default in Krokiet.** In the CLI it must be explicitly enabled with `--use-prehash-cache`. Disable if the cache file size is a concern.

- **Cache for removable drives**  
  Disable "Delete outdated cache entries automatically" when scanning external drives that you regularly unplug. Use "Remove outdated results" manually instead to avoid entries being evicted on disconnect.
