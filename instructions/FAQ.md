# Czkawka / Krokiet - Frequently Asked Questions

This document collects recurring questions and problems from the GitHub issue tracker.
It is derived from hundreds of real user reports and is updated alongside the project.

## Related Documentation

- [Instruction_Krokiet.md](Instruction_Krokiet.md) - full guide for the Krokiet GUI (recommended frontend)
- [Instruction_CLI.md](Instruction_CLI.md) - full guide for the command-line interface
- [Instruction_Core.md](Instruction_Core.md) - internals: algorithms, cache, configuration format
- [Instruction_GTK.md](Instruction_GTK.md) - guide for the legacy GTK GUI (deprecated since v12.0)

---

## Table of Contents

1. [General & Project Overview](#general--project-overview)
2. [Installation & Requirements](#installation--requirements)
3. [Which Build / Binary to Use](#which-build--binary-to-use)
4. [GTK Deprecation & Migration to Krokiet](#gtk-deprecation--migration-to-krokiet)
5. [Krokiet Rendering Issues (blank/transparent window, GPU)](#krokiet-rendering-issues)
6. [Cache & Config Files](#cache--config-files)
7. [Duplicate Files - Not Finding Expected Duplicates](#duplicate-files---not-finding-expected-duplicates)
8. [Similar Images](#similar-images)
9. [Similar Videos & ffmpeg](#similar-videos--ffmpeg)
10. [Reference Paths / Reference Folders](#reference-paths--reference-folders)
11. [Deleting, Moving, Hardlinking Files](#deleting-moving-hardlinking-files)
12. [Selection & "Select Custom"](#selection--select-custom)
13. [Bad Extensions Tool](#bad-extensions-tool)
14. [Scanning External Drives, NAS, Network Paths](#scanning-external-drives-nas-network-paths)
15. [Snap, Flatpak, AppImage Packages](#snap-flatpak-appimage-packages)
16. [Performance & Large Scans](#performance--large-scans)
17. [CLI Usage](#cli-usage)
18. [Security - Antivirus False Positives](#security---antivirus-false-positives)
19. [Common Error Messages](#common-error-messages)
20. [Unsupported / Not Planned Features](#unsupported--not-planned-features)
21. [Portable / Custom Data Paths](#portable--custom-data-paths)
22. [Hardlink Behavior & Safety](#hardlink-behavior--safety)
23. [Symlink Handling](#symlink-handling)
24. [Scanning Phones / Android Devices](#scanning-phones--android-devices)
25. [Saving & Loading Scan Results](#saving--loading-scan-results)
26. [How Duplicate Detection Works Internally](#how-duplicate-detection-works-internally)
27. [Windows-Specific Issues](#windows-specific-issues)

---

## General & Project Overview

### Q: What is the difference between Czkawka and Krokiet?

Both share the same scanning engine (`czkawka_core`). The difference is the frontend:

- **Krokiet** - the current recommended GUI, built with the Slint framework. Statically linked, no external GUI dependencies, works reliably on Windows, macOS, and Linux. Actively developed.
- **Czkawka GTK** - the old GTK4-based GUI. Deprecated since v12.0 and receives only critical bug fixes. New features are implemented in Krokiet only.
- **czkawka_cli** - the command-line interface for scripting and automation.

When in doubt, use Krokiet.

### Q: Is Czkawka safe? Does it access the Internet?

Czkawka and Krokiet do not make any network connections. The application has no telemetry, no update checks, and no analytics. If you observe network traffic in a sandboxed analysis, it is typically caused by the analysis environment itself or by the GTK runtime.

The only officially supported download sources are:
- [GitHub releases page](https://github.com/qarmin/czkawka/releases)
- [crates.io](https://crates.io/)
- [Flathub](https://flathub.org/) (GTK GUI only, Krokiet Flatpak not yet published as of mid-2026)

Sites such as `czkawka.net`, `czkawka.com`, `czawka.net` and similar are **not** official and may be unsafe.

### Q: Can I run two instances of Czkawka at the same time?

You can run instances of different tools simultaneously (each tool has its own cache file). However, you must **not** run two instances of the same tool at the same time - doing so can corrupt the cache for that tool.

### Q: What is the project license?

Czkawka and Krokiet are released under the MIT license. You can find the full license text in the `LICENSE` file in the repository root.

### Q: Is there a web-based UI or Docker-based web interface for Czkawka?

The official GTK Docker image (`jlesage/czkawka`) provides a VNC-based web UI. There is also a community project called **Schluckauf** (https://github.com/fadykuzman/schluckauf) that provides a self-hosted browser-based UI for reviewing duplicate photos found by the Czkawka CLI. It parses Czkawka's JSON output and lets you mark duplicates as keep/trash via keyboard shortcuts.

Krokiet itself does not have a web interface. If you need web access, use the CLI and process its JSON output with an external tool.

### Q: Are there nightly / pre-release builds available?

Yes. Nightly builds compiled from the latest master branch commits are published at https://github.com/qarmin/czkawka/releases/tag/Nightly. They may contain unknown bugs but include the newest features and fixes before an official release.

---

## Installation & Requirements

### Q: What are the system requirements?

- **Linux**: Ubuntu 22.04 or newer (glibc 2.35+). The pre-built binaries are statically linked (Krokiet) or dynamically linked against GTK4 (GTK GUI).
- **Windows**: Windows 10 or newer. Krokiet binaries are self-contained.
- **macOS**: Krokiet binaries are available for both Intel (x86_64) and Apple Silicon (ARM64). The GTK GUI binaries require GTK4 to be installed (e.g. via Homebrew).

### Q: How do I install on macOS?

For **Krokiet** (recommended): download `mac_krokiet_arm64` (Apple Silicon) or `mac_krokiet_x86_64` (Intel) from the releases page, make it executable, and run it:

```bash
chmod +x mac_krokiet_arm64
./mac_krokiet_arm64
```

If macOS shows a "cannot be opened because it is from an unidentified developer" dialog, right-click the file and choose "Open", then confirm.

If macOS opens the file as a text document in TextEdit, the file does not have the executable bit set - run the `chmod +x` command first.

For the **GTK GUI** (`mac_czkawka_gui_*`): GTK4 must be installed via Homebrew (`brew install gtk4`). The `_heif_avif` variant additionally requires `libheif` and `libavif` (`brew install libheif libavif`).

### Q: Which Linux packages are available?

- **Pre-built binaries**: Download from the GitHub releases page (recommended, always up to date).
- **Flatpak** (GTK GUI): Available on Flathub (`com.github.qarmin.czkawka`). The Krokiet Flatpak is not yet published on Flathub.
- **AppImage**: Available on the releases page.
- **Snap**: The Snap package is **no longer maintained**. Use Flatpak or the AppImage instead.
- **AUR (Arch Linux)**: `czkawka` package available.
- **Debian / Ubuntu**: Official Debian package exists but may lag several versions behind the current release.

### Q: Similar Videos does not work - ffmpeg not found

The **Similar Videos** tool requires `ffmpeg` to be installed separately and available in `PATH`. Krokiet and czkawka_cli do not bundle ffmpeg.

| OS | Installation |
|----|-------------|
| Linux | `sudo apt install ffmpeg` (Debian/Ubuntu), `sudo pacman -S ffmpeg` (Arch) |
| macOS | `brew install ffmpeg` |
| Windows | Install from [ffmpeg.org](https://ffmpeg.org/download.html) and add the folder containing `ffmpeg.exe` to the system `PATH`. On Windows, you can also place `ffmpeg.exe` in the same folder as the Krokiet executable. |

After installation, restart Krokiet or the CLI. On Windows, a new terminal session (or system restart) may be required for the PATH change to take effect.

---

## Which Build / Binary to Use

### Q: There are several Windows Krokiet binaries. Which one should I pick?

The release page provides these Krokiet variants for Windows:

| Binary name | Renderer | Notes |
|-------------|----------|-------|
| `windows_krokiet.exe` | femtovg (OpenGL via Rust) | Default, no external DLL dependencies. Best starting point. |
| `windows_krokiet_on_windows_skia_opengl.exe` | Skia + OpenGL | Better text rendering on most systems. Requires C/C++ runtime DLLs bundled in the ZIP. |
| `windows_krokiet_on_windows_skia_vulkan.exe` | Skia + Vulkan | May hang the entire system on some GPU drivers - use with caution. |

Start with the default `windows_krokiet.exe`. If text looks blurry or fonts are wrong, try the `skia_opengl` variant. Avoid the Vulkan variant unless you know your GPU drivers support it correctly.

### Q: On Linux Krokiet shows nothing or crashes with a rendering error

Try the software renderer by setting the environment variable before launching:

```bash
SLINT_BACKEND=software krokiet
```

On systems without a GPU (VMs, headless servers), this is often the only option.

### Q: Which macOS binary should I use?

Use `mac_krokiet_all_backends_arm64` (Apple Silicon) or `mac_krokiet_all_backends_x86_64` (Intel). The `all_backends` variant supports multiple rendering backends and is the most compatible.

If you need HEIF/AVIF support, use the `_heif_avif_all_backends_*` variant, but note that `libheif` must be installed on the system.

### Q: What is the difference between "winversion" and "linversion" Krokiet binaries for Windows?

Both variants should work identically. The difference is the compilation host:
- `winversion` - compiled natively on Windows. Requires the [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist) to be installed.
- `linversion` - cross-compiled on Linux. Self-contained, no extra runtime required.

If you are unsure, use `linversion`. Both should produce identical results at runtime.

### Q: What is the difference between "gtk_46" and "gtk_412" Windows GTK GUI variants?

The number refers to the GTK version bundled with the binary. `gtk_46` uses GTK 4.6, `gtk_412` uses GTK 4.12. Both are the legacy GTK GUI (deprecated). If `gtk_412` does not launch on your system, try `gtk_46` - it is older but more compatible. Alternatively, use Krokiet which does not require GTK at all.

### Q: What does the "heif_avif" / "libraw" label on binaries mean?

These variants are compiled with optional support for additional image formats:
- `heif` / `heif_avif` - adds HEIC/HEIF and AVIF image support (requires `libheif` / `libavif` as a system library on macOS/Linux; bundled on Windows in some releases).
- `libraw` - adds RAW camera format support (CR2, NEF, ARW, etc.).

The base binary without these suffixes does not read HEIC/HEIF, AVIF, or RAW files.

---

## GTK Deprecation & Migration to Krokiet

### Q: I get a popup in Czkawka GTK saying to switch to Krokiet. Is GTK really deprecated?

Yes. **Version 12.0 is the last release of the Czkawka GTK frontend**. It enters maintenance mode: only critical bug fixes for the unofficial Docker and Debian packages may still be applied. No new features will be added to the GTK version.

All new features and active development happen in **Krokiet**. Users should migrate to Krokiet.

### Q: Czkawka GTK works for me. Why should I switch?

The GTK frontend has persistent platform issues on Windows and macOS that cannot be fully fixed due to the nature of the GTK4 Windows/macOS port. These include:
- Transparent / unclickable window (Windows, #1904)
- Tiny or blurry text on HiDPI displays
- WebP preview not working (Windows, #1095)
- Various crashes when sorting, pasting text, or using custom select (#967, #1170)

Krokiet avoids all of these by using the Slint framework, which has a pure-Rust rendering pipeline and no dependency on GTK.

### Q: Does Czkawka work on Windows 7 or older Windows versions?

Windows 10 is the minimum officially supported version. GTK4 (used by the legacy GUI) and the Rust standard library have both dropped support for older Windows versions. Running on Windows 7 or Windows XP is not supported and is very unlikely to work with any current release.

### Q: The GTK GUI shows text too small on my 4K display

For the GTK GUI: create or edit `%LocalAppData%\gtk-4.0\gtk.css` (Windows) or `~/.config/gtk-4.0/gtk.css` (Linux) with:

```css
* {
    font-size: 22px;
}
```

Adjust the value as needed. Alternatively, use Krokiet, which supports a manual scale factor in Settings.

---

## Krokiet Rendering Issues

### Q: Krokiet window is completely black / blank / invisible

This is usually a GPU driver or rendering backend issue. Try:

1. Use the software renderer: `SLINT_BACKEND=software krokiet` (Linux/macOS) or set the environment variable `SLINT_BACKEND=software` in Windows system environment variables, then restart.
2. On Windows, try the `skia_opengl` variant instead of the default binary.
3. Update your GPU drivers.

### Q: Krokiet panics with "The wayland library could not be loaded"

This is a library path issue on some distributions (notably NixOS). The Krokiet binary needs `libwayland-client.so` to be in `LD_LIBRARY_PATH`. On NixOS, see the project's NixOS packaging notes and ensure the `wayland` package is available in the build/run environment.

### Q: Krokiet crashes or shows garbled text on Linux with a custom WM (not a full DE)

This is often caused by a missing or incompatible font shaping library. Try the `software` backend or the `skia_opengl` backend. If you use a tiling WM without XWayland, ensure XWayland or a proper Wayland compositor is running.

### Q: Krokiet on Windows says it is blocked by antivirus / Windows Defender

This is a false positive. The binaries are compiled from source via GitHub Actions CI, and the SHA256 checksums can be verified against the CI build artifacts. The detection is typically caused by Link-Time Optimization (LTO), which produces binary patterns similar to obfuscated code.

You can verify the binary on [VirusTotal](https://www.virustotal.com/) - the overwhelming majority of engines report it as clean, and any detections are from obscure engines.

If Defender blocks the download, you can try the `skia_opengl` variant or compile from source. You can also report the false positive to Microsoft via the Defender Feedback portal.

### Q: The GTK GUI (czkawka_gui) crashes on macOS with "Unrecognized image file format" / pixbuf error

This is a known crash in the GTK GUI when GTK's pixbuf loaders are missing or corrupted. Fixes to try in order:

1. Install GTK4 and its dependencies via Homebrew: `brew install gtk4 adwaita-icon-theme`
2. Regenerate the pixbuf loader cache: `gdk-pixbuf-query-loaders --update-cache`
3. Clear stale config: `rm -rf "$HOME/Library/Application Support/pl.Qarmin.Czkawka"` and `rm -rf "$HOME/Library/Caches/pl.Qarmin.Czkawka"`, then relaunch.

If none of these work, use **Krokiet** instead - it does not depend on GTK and does not have this issue.

---

## Cache & Config Files

### Q: Where are the config and cache files stored?

| Platform | Config | Cache |
|----------|--------|-------|
| Linux | `~/.config/krokiet/` | `~/.cache/czkawka/` |
| macOS | `~/Library/Application Support/pl.Qarmin.Krokiet/` | `~/Library/Caches/pl.Qarmin.Czkawka/` |
| Windows | `%APPDATA%\Qarmin\Krokiet\config\` | `%LOCALAPPDATA%\Qarmin\Czkawka\cache\` |
| Flatpak | `~/.var/app/com.github.qarmin.czkawka/config/czkawka/` | `~/.var/app/com.github.qarmin.czkawka/cache/czkawka/` |

The cache is **shared between all frontends** (CLI, Krokiet, GTK). Both Krokiet and the GTK GUI read and write the same cache files in `~/.cache/czkawka/`.

The Krokiet config files are:
- `config_general.json` - window size, language, dark/light theme, etc.
- `config_preset_N.json` (N = 0-9) - per-preset scan directories and tool settings.

### Q: How large is the cache? Can it fill my disk?

The cache stores only metadata (hashes, timestamps, sizes) - not file contents. A typical large scan (150 TB of files) may produce a cache a few hundred MB in size at most, not terabytes. If the settings pane shows "1.83 TiB cached", that is the total size of all files that were scanned, not the cache file size.

### Q: What does "Remove outdated results from cache" do?

It removes cache entries for files that no longer exist on disk. This keeps the cache lean and avoids stale entries being used in future scans. Click it periodically if you have deleted many files since your last scan.

### Q: Should I delete the cache to fix problems?

If you suspect cache corruption (e.g., scan results look wrong, or the app crashes when loading cache), you can delete the cache files in `~/.cache/czkawka/`. The next scan will rebuild the cache from scratch, which will be slower but correct.

### Q: Do not run two instances scanning the same tool at the same time

Each tool (duplicates, similar images, etc.) uses a separate cache file. Running two instances of the same tool simultaneously can corrupt that tool's cache. Running different tools in parallel is safe.

### Q: What is the difference between the prehash cache file and the hash cache file?

The duplicate finder uses a two-stage hashing pipeline:
- **Prehash**: a fast partial hash computed over the first few KB of each file. Used to quickly eliminate files that cannot possibly be duplicates (they differ in prehash). Files that share the same size and prehash are promoted to the full hash stage.
- **Full hash**: a cryptographically strong hash of the entire file content. Only computed for files that survived the prehash stage.

Both stages have separate cache files. This is why you see two cache files for the duplicate tool. The prehash cache is much cheaper to build; the full hash cache is the authoritative deduplication signal.

### Q: Can I edit the cache files manually to change file paths (e.g., after moving files to a different location)?

The cache is stored as a binary `.bin` file by default. When the `.bin` file is absent, the application falls back to loading a `.json` file if one exists. To enable JSON cache saving, turn on "Save also as JSON" in the GUI settings. You can then edit the JSON and remove the `.bin` file to force the app to load the JSON version.

Changing directory paths in the JSON cache directly is possible in principle, but the entries also contain file sizes and modification timestamps, so any path change must remain consistent with the actual filesystem state. Stale entries are simply ignored on the next scan.

---

## Duplicate Files - Not Finding Expected Duplicates

### Q: Czkawka does not find my duplicate files even though I can see they are identical

The most common reason is the **minimum file size** setting. By default, files smaller than **16 384 bytes (16 KB)** are ignored. To scan smaller files:

- **Krokiet**: Settings > Duplicate Files > Min file size > set to `1` (bytes) or `0`.
- **GTK GUI**: "Items configuration" tab > "Size (bytes) Min" > set to `1`.
- **CLI**: add `--minimal-file-size 1` to the command.

### Q: I have files below 16 KB that are duplicates - why are they excluded by default?

The 16 KB default exists to avoid hashing very small files (license headers, empty placeholders, etc.) that are frequently identical by coincidence but are not meaningful duplicates. For most workflows involving media files (photos, videos, audio), 16 KB is a sensible floor.

### Q: Duplicates mode found files in the Recycle Bin / Trash

Czkawka scans whatever directories you add. If you add your entire drive root (`C:\` or `/`), it will scan the Recycle Bin (`$RECYCLE.BIN`) or Trash (`.Trash-*`). To exclude them:

- Add an exclusion item in the **Excluded Items** field. Example patterns:
  - Windows: `*:\$RECYCLE.BIN\*`
  - Linux: `*/Trash/*`, `*/.Trash-*/*`

On Linux, the default excluded items already contain `*/Trash/*` and `*/.Trash-*/*` patterns.

### Q: The scan found far fewer files than I expected during the "full hash" stage - is something wrong?

No. The number shown for "Analyzed full hash of X/Y files" reflects only the files that reached the full-hash stage, not the total number of files scanned. Files that differ in size or prehash never need a full hash, so Y can be much smaller than the total file count. This is expected and correct behavior.

### Q: The scan seems stuck at 99-100% for a long time

This is expected behavior when the last remaining files are very large. The progress bar shows file count; the final large files can take most of the total time. More recent versions show a size-based progress indicator that is more accurate.

### Q: Czkawka finds my symlinks as duplicates of the files they point to

By default hard links are filtered, but regular symlinks are followed and their targets are compared. If you use symlinks to intentionally alias files, add the symlink directory to the excluded paths or use the "Ignore same inode" option.

---

## Similar Images

### Q: Similar Images does not find known duplicates of images

Check these settings:
1. **Minimum file size**: default is 16 KB. Images smaller than this are skipped. Lower it if needed.
2. **Max difference (similarity threshold)**: a value of `0` means only bitwise identical files are matched. Increase it (e.g. to `8` or `10`) to catch near-duplicates.
3. **Hash algorithm and hash size**: different algorithms catch different types of similarity. The default in Krokiet is Mean hash with hash size 16. If you miss duplicates, try Gradient or Double Gradient with hash size 32 or 64.
4. The image format must be supported. See the next question.

### Q: What image formats does Similar Images support?

Supported in all builds: JPEG, PNG, GIF, BMP, TIFF, WebP, ICO, PNM, TGA, and most common formats handled by the `image` crate.

Formats requiring optional features:
- **HEIC/HEIF**: needs a build with the `heif` feature (requires `libheif` system library on Linux/macOS, bundled on Windows in some releases).
- **AVIF**: needs a build with the `libavif` feature (Linux pre-built binaries as of 11.0.1 do not include AVIF support - compile with `--features libavif` or use the `heif_avif` binary variant where available).
- **RAW** (CR2, NEF, ARW, etc.): needs a build with the `libraw` feature.

### Q: Results of Similar Images are different on each run (non-deterministic)

This is fixed in recent versions (starting around 10.x). Earlier versions had a race condition in the comparison phase that caused different groupings on each run. Update to the latest release.

### Q: Can Similar Images find duplicates across different image formats (e.g., JPEG vs WebP vs PNG)?

Yes. The Similar Images tool compares pixel content regardless of the file format. A JPEG and a WebP that contain the same visual content will be grouped together, as long as both formats are supported by the build you are using.

### Q: Can Similar Images detect greyscale images as similar to their color originals?

Yes - and there is currently no built-in option to exclude greyscale matches. All perceptual hash algorithms (Mean, Gradient, Double Gradient, Blockhash) ignore color and work on luminance, so a greyscale conversion of an image will appear highly similar to the original. If you want to avoid greyscale false positives, you must filter results manually or with a post-processing script.

### Q: Similar Images does not detect horizontally mirrored or rotated images

Detecting mirrored/flipped images is not yet implemented. Detecting images with EXIF rotation tags is supported (the EXIF orientation is applied before hashing). Detecting images that are physically rotated in their pixel data (not just via EXIF) is a planned feature but not available yet.

### Q: I have many false positives in Similar Images (unrelated images grouped together)

Lower the **Max difference** (similarity threshold). The default setting `10` can match visually distinct images if they share large uniform areas (e.g., black borders, white backgrounds). Try `4` or `5` for stricter matching.

### Q: What do the hash algorithm and hash size settings mean? Which should I use?

The settings control the perceptual hashing step in Similar Images:
- **Hash algorithm**: how the image is converted to a short fingerprint. `Mean` is the fastest and most permissive. `Gradient` and `Double Gradient` are more sensitive to structural differences. `Blockhash` is a different approach that can find different near-duplicates.
- **Hash size**: 8, 16, 32, or 64 bits per dimension. Larger values produce longer, more precise fingerprints that reduce false positives but may miss re-encoded versions.

Practical guidance: for finding re-encoded/resized copies of the same image, start with Mean hash + size 16 and a "max difference" of 8-10. For stricter matching (fewer false positives), use Gradient/Double Gradient with size 32 and lower the max difference to 4-5.

### Q: HEIC images are not found / "The image format could not be determined" error for HEIC files

Ensure you are using a build that includes the `heif` feature. The `linux_krokiet_all_backends_x86_64` binary as of v11.0.1 does **not** include HEIF/AVIF support. Use the `_heif_avif_` variant or compile with the appropriate feature flags:

```bash
cargo run --bin krokiet --features "winit_femtovg,winit_software,heif,libavif"
```

---

## Similar Videos & ffmpeg

### Q: How does Similar Videos detect duplicate videos?

Similar Videos works by sampling frames from the video at fixed intervals and computing visual perceptual hashes of those frames. The similarity is then the hamming distance between the frame-hash sequences. The algorithm is provided by the `vid_dup_finder_lib` library; a detailed description is available at https://github.com/Farmadupe/vid_dup_finder_lib#how-it-works.

### Q: What do the "Skip duration" and "Video has duration" settings do in Similar Videos?

- **Skip duration**: the number of seconds skipped at the start and end of each video before frame sampling begins. Useful for skipping common intros or outros.
- **Video has duration**: the minimum total duration a video must have to be included in the scan. Very short clips (below approximately a few seconds) are skipped by default.

Lowering the skip duration captures more of each video but may increase false positives if many videos share a common opener. Lowering the minimum duration allows very short clips to be compared.

### Q: Similar Videos scan starts but finds nothing, or produces many false positives

Similar Videos compares videos by sampling frames and computing visual hashes. False positives (unrelated videos grouped) are common when:
- The tolerance threshold is too high (lower the "Max difference" setting).
- Videos share common intro/outro sequences.
- Videos have unusual encoding (e.g., very low resolution, corrupt files).

The algorithm is fundamentally heuristic and works best for finding re-encoded copies of the same source material, not for finding videos that happen to have a few similar-looking scenes.

### Q: Similar Videos popup window / command prompt appears briefly on Windows

In older versions, ffmpeg was launched as a visible process on Windows. This is fixed in current releases; ffmpeg now runs without a visible console window.

### Q: Similar Videos cache - is it reused across scans?

Yes, the video hash cache is stored in `~/.cache/czkawka/` and is reused on subsequent scans. Each video is processed once; subsequent scans load the pre-computed hash from cache.

---

## Reference Paths / Reference Folders

### Q: What is a Reference Folder / Reference Path?

A reference folder is a directory added to the scan but **protected from deletion or modification**. Files in reference folders appear in the results only for comparison - they show which "keep" candidate they matched against. Files outside reference folders can be selected and deleted normally.

Typical use case: add your main archive as reference, add a downloads folder as a regular included path, then scan. Only files in the downloads folder will be selectable for deletion; the archive is always preserved.

### Q: How do I delete duplicates from one folder while keeping the copy in another folder?

Use Reference Folders. Add the folder you want to protect as a reference folder (check the "Reference" checkbox next to it in the directory list). After the scan, only files outside the reference folder appear as selectable. Use "Select All" and then delete - this will only delete the non-reference copies.

### Q: With a reference folder set, will groups that have duplicates only within the non-reference folder still appear?

No. When a reference folder is set, only groups that contain at least one file from the reference folder are shown. Groups that are entirely within non-reference directories are hidden. This is intentional: the reference folder marks "originals", so a group is only meaningful when at least one original is present.

### Q: I set a reference folder but files inside it are still selectable / deleted

This is a known issue in some older versions of the GTK GUI (particularly when using the MSYS2 Windows build). Upgrade to the latest Krokiet, where reference folder protection is implemented correctly.

### Q: How do I use reference folders in the CLI?

Use the `-r` / `--reference-directories` flag. To specify multiple reference directories, repeat the flag:

```bash
czkawka_cli dup -d /path/to/search -r /path/to/ref1 -r /path/to/ref2 -D AEO
```

Using comma- or semicolon-separated values in a single `-r` flag does **not** work. Each reference directory must be a separate `-r` flag.

### Q: CLI with reference folder and `-D` (delete) does nothing - 0 files deleted

When all duplicate files have identical timestamps and sizes, some delete modes (AEN, AEO) cannot determine which is "newest" or "oldest" and skip deletion. Use a delete mode that does not rely on timestamps, or check that the reference folder logic is correctly identifying which copy to preserve. This is a known limitation tracked in issue #1815.

---

## Deleting, Moving, Hardlinking Files

### Q: Delete does nothing / "Deleted 0 items, failed to remove 1 items"

Common causes:
1. **Trash on Samba/NFS shares**: Krokiet v11 defaults to moving deleted files to the trash. On network shares (SMB, NFS), moving to trash fails because the remote filesystem does not support it. **Fix**: in Settings, switch the delete method from "Move to trash" to "Permanent delete", or use the dedicated "Delete permanently" button (added in v12.x).
2. **Read-only files/folders**: the file itself may be writable but its parent directory is read-only. Check permissions on the containing folder.
3. **Flatpak sandbox**: the Flatpak version has restricted filesystem access. Grant full filesystem access in Flatpak permissions, or use the AppImage / native binary.

### Q: Move is slow - it copies the whole file instead of just renaming

On the same filesystem, Krokiet performs a rename (which is instant). If the source and destination are on different filesystems (e.g., moving from one drive to another), a full copy-then-delete is performed, which is expected to be slow. This is not a bug.

### Q: Flatpak version moves deleted files to a different Trash folder than the system uses

This is a known behavior of the Flatpak sandbox. Files moved to trash inside a Flatpak container go to `~/.Trash-<uid>/` rather than `~/.local/share/Trash/`. This cannot be configured and is a limitation of the Flatpak sandbox, not of Czkawka itself.

### Q: How does Hardlink work? What does it actually do to my files?

Hardlink replaces selected duplicate files with hard links pointing to one retained copy. After hardlinking, all files share the same inode - they appear as separate filenames but consume disk space only once. **Note**: hardlinking across different filesystems or partitions is not possible; the operation silently skips such pairs.

### Q: When hardlinking multiple groups at once, do files from different groups get linked together?

No. Each group is processed independently. Within group A, the unselected file becomes the "original" and selected files become hard links to it. Within group B the same happens independently. Files from different groups are never linked together.

---

## Selection & "Select Custom"

### Q: "Select Custom" does not work / selects nothing

The "Select Custom" dialog selects files based on a text pattern matched against the file name, path, or a regex. Common mistakes:
- The pattern must match a substring of the full path. Use `*` as a wildcard.
- On Windows, use forward slashes or escaped backslashes in patterns; backslash in patterns can behave unexpectedly.
- The "Don't select all files in a group" checkbox: if checked, at least one file in each group is always left unselected (a safety measure to avoid deleting all copies). Uncheck it if you want to select all matches regardless.

### Q: How do I select all files except those in a specific folder?

1. Use "Select Custom" with the path of the folder you want to keep (selects files in that folder).
2. Then use "Invert Selection" to swap the selection.

Or set the folder you want to keep as a **Reference Folder** - reference folder files are never selected by any automatic selection mode.

### Q: If I click "Select All" and then delete, will all copies of a file be deleted including the last one?

No. The automatic selection modes ("Select All Except Oldest", "Select All Except Newest", etc.) always leave at least one file in each group unselected. The plain "Select All" button selects every checked row, but each group's header row (the first entry) is always left unselected by the automatic modes. If you manually check every row yourself, you can select all files in a group - but even then, the delete operation will warn you before proceeding.

When using the CLI with `-D AEN` / `-D AEO` etc., the delete mode is described by its name: "All Except Newest" always keeps exactly one file per group.

### Q: Can I select files by extension using "Select Custom"?

Yes. In the "Regex Path + Name" field, enter a regex that matches the extension. For example, to select all `.mp3` files:

```
.*mp3
```

Or for `.flac` files:
```
.*\.flac$
```

### Q: "Select Custom" on Windows - the "Path" filter selects nothing

On Windows, use a plain path prefix without a trailing backslash or wildcard. For example, enter `C:\Users\username\Desktop` (not `C:\Users\username\Desktop\*`). The filter matches any file whose path contains the entered string as a substring.

### Q: "Select Biggest/Smallest Resolution" options are inverted

This was a bug in v11.0.1, fixed in a subsequent release. Update to the latest version.

---

## Bad Extensions Tool

### Q: Bad Extensions says my file has the wrong extension. How do I rename it?

In **Krokiet** (v11.0+): select the files in the results list and use the **Rename** button (or right-click > Rename). The suggested new extension is shown in the "Extra" column (`current_ext -> correct_ext`).

In the **CLI**: the CLI does not rename files automatically. Export results to a file (`-f results.txt`), then process the output with a script. Example with `sed`:

```bash
czkawka_cli bad-ext -d /path/to/scan -f results.txt
# Parse results.txt and generate mv commands
```

In the **GTK GUI**: renaming from the GUI is not supported. Export results and rename manually.

### Q: Bad Extensions incorrectly flags `.pub` files as `.msi`, or other obvious mistakes

The extension detection uses file magic bytes (the `infer` crate). Some file types share similar signatures or the library's database is incomplete. Known issues:
- Old Microsoft Office formats (`.doc`, `.xls`, `.ppt`) are compound binary files that are hard to distinguish from each other.
- Some file types (`.pub`, `.mdb`, etc.) may not be in the library's database.

These are upstream library limitations. You can exclude specific extensions from the scan if needed.

---

## Scanning External Drives, NAS, Network Paths

### Q: How do I exclude hidden/system directories like Synology's @eadir or macOS .DS_Store directories?

Use the **Excluded Items** field (not Excluded Directories). Add a wildcard pattern that matches the directory name anywhere in the path. Examples:

- Synology `@eadir`: `*/@ea*`
- macOS `.Spotlight-V100`, `.fseventsd`, etc.: `*/.Spotlight*`, `*/.fseventsd*`
- Any hidden directory (starts with `.`): use Excluded Directories to add the specific paths.

The excluded items patterns use `*` as a wildcard and match against the full path of each file/directory.

### Q: Czkawka does not scan my external USB drive

If you use the Snap version: Snap has strict filesystem sandboxing. Run:
```bash
sudo snap connect czkawka:removable-media
```
to grant access to removable media. However, the Snap package is no longer maintained - switch to Flatpak or the AppImage.

If you use the Flatpak version: grant full filesystem access in Flatpak permissions, or use "Other Files" to add the mount point manually in the directory picker.

For the native binary: if the drive is mounted under `/run/media/` (common on Fedora/openSUSE), note that `/run` is in the default excluded items list. Remove `/run` from excluded items or add the specific mount path to the included directories.

### Q: Cannot scan a Windows network share (SMB path) with Krokiet on Windows

On Windows, UNC paths like `\\server\share\` may not be shown in the file picker dialog. Use the "Manual add" text input in the directories panel to type or paste the UNC path directly.

### Q: Czkawka / Krokiet cannot access `C:\System Volume Information` - Access is denied

This is expected. System directories and volumes reserved by Windows are inaccessible to normal user processes. Czkawka logs access errors but continues scanning the rest of the directory tree. These errors are harmless.

---

## Snap, Flatpak, AppImage Packages

### Q: The Snap package is outdated / has permission problems

The **Snap package is no longer maintained**. It receives no updates and has known permission issues with external drives and NFS mounts. Use one of these alternatives:
- **Flatpak** from Flathub (`com.github.qarmin.czkawka`) - GTK GUI only.
- **AppImage** from the GitHub releases page.
- **Pre-built binary** from the GitHub releases page.

### Q: The Flatpak version is out of date

The Flathub version tracks the official releases but may lag a few weeks behind. The Flatpak is maintained by a separate maintainer. For the latest version, use the AppImage or pre-built binary from the GitHub releases page.

### Q: How do I use the CLI via Flatpak?

The Flatpak package contains only the GUI. Flathub does not accept CLI-only applications. To use the CLI:
- Download the CLI binary from the GitHub releases page.
- Or install via your system's native package manager.

---

## Performance & Large Scans

### Q: Scanning is very slow on a traditional HDD

Scanning speed on HDDs is limited by seek time. The application collects file metadata and sorts files by inode number before reading, which improves sequential read patterns. However, with very large directories, performance will still be significantly slower than on an SSD.

Tips:
- Enable the **cache** (enabled by default). The second scan of the same files is much faster.
- Enable **prehash** (enabled by default in Krokiet). This adds a fast partial-hash stage that eliminates most non-duplicates before the full hash.
- Limit the scan to specific subdirectories rather than scanning an entire multi-TB drive in one run.

### Q: Scanning becomes very slow or the system becomes unresponsive when scanning millions of files

When scanning a very large number of files (millions), the file collection phase can take many minutes, especially on HDDs. The app may appear stuck. You can run the CLI with `RUST_LOG=debug` to see what stage it is in:

```bash
RUST_LOG=debug czkawka_cli dup -d /path
```

The progress line shows the current count in the format `Collecting files: N`. If the count is still growing, the app is working normally.

For scanning 100TB+ datasets, expect the initial scan to take several hours. Subsequent scans are faster because the cache holds pre-computed hashes.

### Q: The progress bar shows 100% but the scan is still running for a long time

The percentage is based on file count. The last few files can be very large (e.g., video files) and take a disproportionate amount of time to hash. This is not a bug. Newer versions show a size-based progress bar that is more accurate.

### Q: The UI becomes very slow / laggy after a scan with thousands of groups

This is a known performance issue with the Slint log/output panel. When there are 5000+ result groups, the log accumulates a large amount of text and rendering it becomes slow. As a workaround, collapse or hide the log panel at the bottom of the window. This is a Slint framework issue being tracked upstream.

### Q: CPU usage rises sharply during a large Similar Images or Similar Videos scan - is this expected?

Yes. The first phase (collecting files, computing sizes) uses only a few threads. The hashing phase is CPU-intensive and parallelized across all available cores, which causes CPU usage to spike. This is normal and desirable behavior.

### Q: Can Czkawka use GPU acceleration for hashing?

No. GPU hashing is not implemented and is unlikely to be added - disk I/O is typically the bottleneck, not CPU hash computation. Even with a GPU, the bottleneck would remain the disk read speed.

---

## CLI Usage

### Q: How do I scan multiple directories with the CLI?

Repeat the `-d` / `--directories` flag for each directory. **Do not** separate paths with spaces or commas within a single flag:

```bash
# Correct:
czkawka_cli dup -d /path/to/dir1 -d /path/to/dir2

# Wrong (will fail):
czkawka_cli dup -d /path/to/dir1 /path/to/dir2
```

### Q: How do I use multiple reference directories with the CLI?

Repeat the `-r` / `--reference-directories` flag:

```bash
czkawka_cli dup -d /data -r /archive1 -r /archive2
```

### Q: How do I delete duplicates via CLI?

Use the `-D` flag with a delete strategy. Example - keep the oldest copy, delete all others:

```bash
czkawka_cli dup -d /path -D AEO
```

Available strategies: `AEN` (all except newest), `AEO` (all except oldest), `ON` (only newest), `OO` (only oldest), `AEB` (all except biggest), `AES` (all except smallest), `OB` (only biggest), `OS` (only smallest), `HARD` (replace with hard links).

Add `--dry-run` to preview what would be deleted without actually deleting anything.

### Q: CLI `--allowed-extensions` seems to exclude extensions instead of including them

This was a bug in older versions (the filter logic was inverted). It is fixed in recent releases. Update to the latest version.

### Q: How do I save results to a file?

Use the `-f` / `--file-results` flag for a plain text output, or `-C` for compact JSON, or `-p` for pretty-printed JSON:

```bash
czkawka_cli dup -d /path -f results.txt          # plain text
czkawka_cli dup -d /path -C results.json         # compact JSON
czkawka_cli dup -d /path -p results_pretty.json  # pretty JSON
```

### Q: Does the CLI have a progress display?

The CLI does not show a real-time progress bar by default (no spinner/percentage in current releases). Run with `RUST_LOG=debug` to see verbose progress information including the current scanning stage and file counts.

### Q: How do I suppress CLI output / run silently?

Use `--do-not-print-results` (`-N`) to suppress the results listing, and `--do-not-print-messages` (`-M`) to suppress informational messages and warnings. Both can be combined.

### Q: How do I limit the number of threads the CLI uses?

Use `-T <N>` / `--thread-number <N>`. Setting `0` (the default) uses all available CPU threads.

### Q: How do I exclude files from scanning by extension?

Use `-P <ext>` / `--excluded-extensions <ext>` to exclude specific extensions. Use `-x <ext>` / `--allowed-extensions <ext>` to scan only specific extensions. The `--allowed-extensions` flag also accepts macros: `IMAGE`, `VIDEO`, `MUSIC`, `TEXT`.

### Q: Is there a dry-run mode in the CLI?

Yes. Add `--dry-run` to preview what would be deleted without actually deleting anything. Note that dry-run output currently goes to the console only and is not written to the result file.

### Q: The CLI Flatpak version - how do I run it?

The Flatpak only contains the GUI. Download the `linux_czkawka_cli_x86_64` binary from the GitHub releases page and run it directly.

---

## Security - Antivirus False Positives

### Q: Windows Defender / antivirus detects Krokiet as malware

This is a well-documented false positive affecting Rust binaries compiled with Link-Time Optimization (LTO). The binary is built from open source code via a public CI (GitHub Actions), and the SHA256 checksums match the CI artifacts.

To verify:
1. Check the [VirusTotal scan](https://www.virustotal.com/) of the file - nearly all engines should report clean.
2. Verify the SHA256 hash of the downloaded binary against the GitHub Actions CI artifact.

To resolve the Defender block: allow the file in Windows Defender by clicking "More info > Run anyway", or add an exclusion for the binary in Defender settings.

If your antivirus continues to flag it, report the file as a false positive to your antivirus vendor.

---

## Common Error Messages

### Q: "Cannot open dir /home/user/.ssh, reason Permission denied (os error 13)"

This is normal. Czkawka tries to scan all directories under the included path. Some system directories (`.ssh`, `/proc`, `/sys`, `/dev`) are not readable by normal users. These errors are logged and the scan continues. To suppress them, add those directories to the excluded paths.

### Q: "Failed to hash file, reason Too short" (Similar Videos)

The video file is shorter than the minimum duration needed for the similarity algorithm to work (approximately a few seconds). Very short clips are skipped. This is expected behavior.

### Q: "Error during trash operation" / files not moved to trash

Usually means the trash is on a different filesystem from the file (e.g., the file is on a network share or a secondary drive that does not have a `.Trash-<uid>` directory). Solutions:
- Use permanent delete instead of move-to-trash.
- For network shares (NFS, SMB), permanent delete is the only reliable option.

### Q: "malformed stream: mp3 invalid main_data offset" in Broken Files scan

This message is produced by the audio decoding library when the MP3 header contains a `main_data_begin` value that points outside the bitstream. Many MP3 files contain this "error" due to encoder quirks, and most media players silently tolerate it. The file is still playable. You can ignore this result or choose to keep the file.

### Q: The app shows ".fuse_hidden..." files as duplicates

These are temporary files created by FUSE-based filesystems (e.g., when a file is opened by another application and then "deleted" - the file is renamed to a hidden name until the last file handle is closed). They are not duplicates in the usual sense. You can exclude them by adding `*/.fuse_hidden*` to the excluded items list.

### Q: Krokiet on macOS shows garbled text in Chinese/Japanese

Krokiet uses bundled fonts and may not include CJK (Chinese/Japanese/Korean) glyphs. This is a known limitation. A workaround is to copy a CJK-capable font file into the directory from which Krokiet is launched, or to set the `SLINT_FONT_PATH` environment variable. This is tracked as a known issue on the project.

---

## Unsupported / Not Planned Features

The following are frequently requested but are not planned for implementation:

- **Finding duplicate folders**: finding folders whose contents are identical is not implemented. The tool finds duplicate files.
- **Detecting rotated/mirrored images**: detecting images that have been physically flipped or rotated in their pixel data (beyond EXIF orientation) is not yet implemented.
- **GPU-accelerated hashing**: disk I/O is the bottleneck, not hash computation.
- **Pause and resume a scan**: scans cannot be paused mid-way. You can stop and restart; the cache preserves already-computed hashes. On Linux, you can suspend the process with `kill -STOP <PID>` and resume it with `kill -CONT <PID>`.
- **Scanning inside archives (ZIP, RAR, etc.)**: files inside archives are not scanned. Extract them first.
- **Finding duplicate directories**: the tool finds duplicate files, not duplicate directory structures.
- **Apple Photos library support**: the Photos library database format is proprietary and not supported.
- **Browser / internet access**: the tool has no internet access, no update checks, no telemetry.
- **Loading saved scan results back into the GUI**: scan results can be exported to TXT and JSON files, but re-importing them into the application is not currently supported. The TXT/JSON formats were designed for external scripting, not for re-loading into the app.
- **OneDrive / cloud storage without local sync**: Czkawka reads files from the local filesystem. If you add an OneDrive or similar cloud folder, the OS must download each file before Czkawka can hash it. There is no way to scan cloud storage without downloading the files.
- **Comparing files only within the same folder (not across folders)**: this "same-folder-only" comparison mode is not implemented. The workaround is to add each folder as a separate scan and run it individually.
- **"Mark as not a duplicate" / ignore list**: there is no persistent ignore list to suppress specific pairs from appearing in future scans. You can work around this by moving files you want to keep together into a subfolder and excluding it, or by using a reference folder.
- **Scanning files without an extension as images**: Similar Images requires files to have a known image extension. Extensionless files are not scanned even if their content is a valid image.

---

## Portable / Custom Data Paths

### Q: Can I make Krokiet store its config and cache in a custom location (portable mode)?

Yes. Set the following environment variables before launching the application:

- `CZKAWKA_CACHE_PATH` - path where cache files are stored
- `CZKAWKA_CONFIG_PATH` - path where config files are stored

These variables are read by `czkawka_core`, so they apply to all frontends (CLI, GTK GUI, and Krokiet).

Example (Windows):
```bat
set CZKAWKA_CACHE_PATH=D:\czkawka\cache
set CZKAWKA_CONFIG_PATH=D:\czkawka\config
krokiet.exe
```

Example (Linux/macOS):
```bash
CZKAWKA_CACHE_PATH=/opt/czkawka/cache CZKAWKA_CONFIG_PATH=/opt/czkawka/config krokiet
```

---

## Hardlink Behavior & Safety

### Q: I accidentally hardlinked system or application DLL files. How do I undo this?

Hardlinking system files can cause instability because changes to one hard-linked file affect all names pointing to the same inode. To undo hard links, you need to copy each file to break the link:

On Windows (PowerShell):
```powershell
# For each affected file:
Copy-Item "C:\path\to\file.dll" "C:\path\to\file.dll.bak"
Remove-Item "C:\path\to\file.dll"
Rename-Item "C:\path\to\file.dll.bak" "file.dll"
```

On Linux/macOS:
```bash
# cp --remove-destination creates a new independent copy
cp --remove-destination original.so original.so
```

To prevent this in the future: always add system directories (`C:\Windows`, `/usr`, `/lib`, etc.) to the Excluded Directories list before scanning.

### Q: Does "Hide hardlinks" in scan settings prevent already-hardlinked files from appearing in results?

Yes. When "Ignore same inode" (or "Hide hardlinks") is enabled, files that share the same inode number are treated as a single logical file and only one representative is shown in results. This prevents already-hardlinked files from being flagged as duplicates.

---

## Symlink Handling

### Q: Does Czkawka follow symlinks when scanning?

By default, Czkawka does **not** follow symbolic links. A symlink is treated as its own filesystem entry; the scan does not traverse into the symlink target. This prevents accidental deletion of original files when a symlink points to them.

If you want to scan the contents of a symlinked directory, add the target path directly as an included directory instead of the symlink.

### Q: What happens if I symlink files with Czkawka? Does it delete the original?

When you use the "Symlink" action, Czkawka replaces the selected (duplicate) files with symlinks pointing to the first unselected file in the group. The original (unselected) file is kept intact. The selected files are replaced in-place; their directory entry becomes a symlink.

On Windows, creating symlinks requires elevated privileges or Developer Mode to be enabled.

---

## Scanning Phones / Android Devices

### Q: Can Czkawka scan files on a connected phone (MTP)?

No. Czkawka reads files through the standard filesystem API. MTP (Media Transfer Protocol, used by Android phones) is not a real filesystem and is not accessible via normal file paths on Windows or Linux. To scan phone contents, either:
- Mount the phone's storage via a tool that creates a local filesystem mount (e.g., `jmtpfs` on Linux).
- Copy the files to a local drive, scan, then act on results.

### Q: Can Czkawka run on Android?

Not officially. `czkawka_core` is platform-independent and can be compiled for Android, but there is no official Android app. Community members have successfully run Krokiet inside Termux (using termux-x11 or SSH X forwarding) on arm64 devices, but this is unsupported and requires manual compilation. See discussion #1073 for community notes.

---

## Saving & Loading Scan Results

### Q: Can I save scan results to review later without re-scanning?

You can export results to files using the "Save" button in the GUI or the `-f` / `-C` / `-p` flags in the CLI. These files can be reviewed externally. However, you cannot load them back into the GUI to resume where you left off - the application must re-run the scan to display results interactively.

### Q: Can I export results as JSON for use in my own scripts?

Yes. Use `-C <filename>` for compact JSON or `-p <filename>` for pretty-printed JSON in the CLI. In the GUI, enable "Save also as JSON" in settings and use the Save button. The JSON format contains file paths, sizes, modification dates, and hashes (for duplicate files).

---

## How Duplicate Detection Works Internally

### Q: Does Czkawka use only hashes, or does it do a full byte-by-byte comparison to confirm duplicates?

By default, Czkawka uses hashes only (size + prehash + full hash pipeline). A full byte-by-byte comparison is not performed. Hash collisions are theoretically possible but extremely unlikely with the default hash algorithm (Blake3). If you require absolute certainty, you would need to use an external tool to verify the matches.

### Q: What hash algorithm does the duplicate finder use?

The default hash algorithm is **Blake3** (very fast, cryptographically strong). You can also choose SHA-256, SHA-512, and xxHash variants in the settings. All provide collision resistance sufficient for practical deduplication; Blake3 is the best choice for performance.

### Q: How does the duplicate finder handle files that differ only in name but are otherwise identical?

Two files with different names but identical content (same size and same hash) are reported as duplicates. The filename is not part of the duplicate comparison when using the "Hash" check method. If you want to find duplicates by name only, use the "Name" check method instead.

---

## Windows-Specific Issues

### Q: Krokiet fails to delete files with "Access is denied (os error 5)"

The most common cause is that the individual files have the read-only attribute set, even if the parent folder does not. Check and remove the read-only attribute from the files:
1. Select the files in File Explorer.
2. Right-click > Properties > uncheck "Read-only".

Another workaround is to move the files to a new folder - the read-only attribute is sometimes reset during the move.

### Q: The GTK GUI shows a black screen on Windows. How do I fix it?

Set the `GSK_RENDERER` environment variable to `cairo` to force the Cairo software renderer. Create a `.bat` file in the same directory as the executable:

```bat
set GSK_RENDERER=cairo
start czkawka_gui.exe
```

Run the `.bat` file instead of the executable directly. Alternatively, switch to Krokiet which does not have this issue.

### Q: The Windows GTK GUI binary (.exe) does nothing when I double-click it

On some Windows systems, the GTK GUI fails silently. Try:
1. Run from a command prompt to see any error output.
2. Try the `gtk_46` variant if the `gtk_412` variant fails (or vice versa).
3. Switch to Krokiet (`windows_krokiet.exe`) which has no GTK dependency.

### Q: Can I use drag and drop to add directories in Krokiet on Windows?

Drag and drop to add directories to the search list was supported in some older versions but has had regressions. If drag and drop does not work, use the "Add" button or type/paste the path directly in the text field.
