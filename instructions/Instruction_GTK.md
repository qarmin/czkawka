# Czkawka GTK - Instructions

> **Deprecation notice**: Czkawka GTK 12.0 is the last released version. No new binaries will be provided. New users and existing users are encouraged to migrate to [Krokiet](Instruction_Krokiet.md).

## GUI overview

<img src="https://user-images.githubusercontent.com/41945903/148281103-13c00d08-7881-43e8-b6e3-5178473bce85.png" alt="Czkawka GTK main window" width="800" />

The GUI is built from different pieces:
- **1** - Image preview - used in duplicate files and similar images finder. Cannot be resized, but can be disabled.
- **2** - Main Notebook to change the active tool.
- **3** - Main results window - allows choosing, deleting and configuring results.
- **4** - Bottom panel - contains buttons which perform specific actions on data (like selecting them) or hide/show parts of the GUI.
- **5** - Text panel - prints messages, warnings and errors about executed actions. Can be hidden by the user.
- **6** - Directory panel - for selecting directories to include or exclude. Also specifies allowed extensions and file size limits.
- **7** - Buttons to open the About window and Settings, where the scan can be customized.

## Translations

GTK GUI is fully translatable. At least 10 languages are supported (some translations were done automatically and may not be perfect).

## Opening and Manipulating Files

- Double-click a file to open it in the default application.
- To open multiple files: hold CTRL while clicking each file, then double-click one of them with the left mouse button.
- To open the folder containing a file: double-click it with the right mouse button.
- To invert the selection within a group: click a file with the middle mouse button - it will flip the selection of all other files in the same group.

## Adding Directories

By default, the current path is loaded as the included directory and excluded directories are filled with sensible defaults.

You can override this by passing arguments when launching the app:

```
czkawka_gui /home /usr --/home/rafal --/home/zaba
```

This includes `/home` and `/usr` and excludes `/home/rafal` and `/home/zaba`. When command-line arguments are used, the "save on exit" option is disabled - the directory list will not be saved automatically.

Both relative and absolute paths are supported: `../home` and `/home` are both valid.

After adding a path, you can mark it as a **Reference path** by right-clicking it. Reference paths appear in scan results but cannot be acted on (selected, moved, deleted) by any automatic action.

## Common Workflows

### Finding and Removing Duplicates

**Scenario**: Find and remove duplicate files from Downloads, keeping Documents as a protected reference.

1. Open Czkawka GTK and select the **Duplicate Files** tab (panel **2**)
2. In the directory panel (**6**), add your Downloads folder to included directories
3. Add your Documents folder and mark it as Reference Path (right-click → Mark as Reference)
4. Open Settings (**7**) and configure:
   - Check Method: **Hash**
   - Hash Type: **Blake3**
5. Click **Search**
6. After the scan completes, use the bottom panel buttons (**4**) to select duplicates
7. Click **Delete** and confirm

### Finding Similar Images

1. Select **Similar Images** tab
2. Add all photo directories in panel **6**
3. In Settings (**7**), set:
   - Similarity threshold (lower = more strict): 5-10 is a good starting point
   - Hash algorithm: **Gradient** (recommended for photos)
   - Hash size: **16x16** (balanced)
4. Click **Search**
5. Use the image preview panel (**1**) to compare similar images before deciding
6. Select the images to remove, then click **Delete**

### Working with Reference Paths

1. Add both your working directory and reference directory to included directories
2. Right-click the reference directory and select **Mark as Reference Path**
3. Files in reference paths appear in scan results for comparison but are never selected or deleted by automatic actions
4. Run the scan and delete normally - reference files are protected

## Settings

Settings are opened via button **7**. Key options:

- **Check method** - for duplicates: Name, Size, Size+Name, Hash
- **Hash type** - Blake3, XXH3, CRC32
- **Min/Max file size** - filter out files outside this range
- **Use cache** - enable/disable hash and thumbnail caching
- **Use prehash cache** - cache partial hashes; speeds up re-scans on large collections
- **Delete outdated cache entries automatically** - evict stale cache entries each scan

## Tips and Tricks

- **Manually editing directories** - You can directly edit the config file `czkawka_gui_config.txt` to add, remove, or change directories. Reload the config after editing.

- **Column visibility** - Some columns (modification date, file size) may not be visible if others are too wide. Workarounds:
  - Scroll the results list horizontally using the scrollbar
  - Narrow other columns by dragging their dividers

- **Slow cache loading** - If cache loading is slow (large collections scanned previously), rename or delete the relevant `cache_similar_image_*.bin` file. The cache regenerates on the next scan with only the currently scanned files.

- **Partial scanning** - You can stop a scan mid-way; all computed hashes are already saved to cache and speed up the next run.

- **Prehash cache** - Enable in Settings when repeatedly scanning large collections. Partial hashes (first and last 4 KB) are cached, cutting re-scan time significantly.

- **Permanent cache for removable drives** - Disable "Delete outdated cache entries automatically" when scanning external drives that you regularly unplug. Use the "Remove outdated results" button to clean stale entries manually instead.

## Config and Cache File Locations

Configuration files (GTK-specific):

| OS | Path |
|----|------|
| Linux | `~/.config/czkawka/` |
| Linux Flatpak | `~/.var/app/com.github.qarmin.czkawka/config/czkawka/` |
| macOS | `~/Library/Application Support/pl.Qarmin.Czkawka/` |
| Windows | `C:\Users\<user>\AppData\Roaming\Qarmin\Czkawka\config\` |

Cache files (shared across all frontends):

| OS | Path |
|----|------|
| Linux | `~/.cache/czkawka/` |
| Linux Flatpak | `~/.var/app/com.github.qarmin.czkawka/cache/czkawka/` |
| macOS | `~/Library/Caches/pl.Qarmin.Czkawka/` |
| Windows | `C:\Users\<user>\AppData\Local\Qarmin\Czkawka\cache\` |

Override locations:

```shell
CZKAWKA_CONFIG_PATH="/media/rafal/Ventoy/config" \
CZKAWKA_CACHE_PATH="/media/rafal/Ventoy/cache" \
czkawka_gui
```
