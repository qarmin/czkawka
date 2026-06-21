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
15. [Snap & Flatpak Packages](#snap--flatpak-packages)
16. [Performance & Large Scans](#performance--large-scans)
17. [CLI Usage](#cli-usage)
18. [Security - Antivirus False Positives](#security---antivirus-false-positives)
19. [Common Error Messages](#common-error-messages)
20. [Portable / Custom Data Paths](#portable--custom-data-paths)
21. [Hardlink Behavior & Safety](#hardlink-behavior--safety)
22. [Symlink Handling](#symlink-handling)
23. [Scanning Phones / Android Devices](#scanning-phones--android-devices)
24. [Saving & Loading Scan Results](#saving--loading-scan-results)
25. [How Duplicate Detection Works Internally](#how-duplicate-detection-works-internally)

---

## General & Project Overview

### Q: What is the difference between Czkawka and Krokiet?

Both share the same scanning engine (`czkawka_core`). The difference is the frontend:

- **Krokiet** - the current recommended GUI, built with the Slint framework. Statically linked, no external GUI dependencies, works reliably on Windows, macOS, and Linux. Actively developed.
- **Czkawka GTK** - the old GTK4-based GUI. Deprecated since v12.0; no new features - all development happens in Krokiet. GTK worked well on Linux, but outside Linux (Windows and macOS) it had a lot of problems - transparent/unclickable windows, blurry HiDPI text, broken previews, renderer crashes - many of which could not be fully fixed because of the state of the GTK4 Windows/macOS ports. Krokiet was created largely to escape these cross-platform GTK issues.
- **czkawka_cli** - the command-line interface for scripting and automation.

Just use Krokiet - it is the recommended frontend for all platforms.

### Q: Is Czkawka safe? Does it access the Internet?

Czkawka and Krokiet do not make any network connections. The application has no telemetry, no update checks, and no analytics. If you observe network traffic in a sandboxed analysis, it is typically caused by the analysis environment itself.

If you want to confirm this yourself, look through `Cargo.lock` in the repository - there are no networking/HTTP/telemetry crates among the dependencies (no `reqwest`, `hyper`, `curl`, etc.), so the app has no code path that could reach the Internet.

The actively maintained download sources are:
- [GitHub releases page](https://github.com/qarmin/czkawka/releases) - the primary, always-current source
- [crates.io](https://crates.io/)

The [Flathub](https://flathub.org/) package (GTK GUI) still exists but is **no longer maintained by the author and is frozen at v10.0** - it lags far behind. There is no Krokiet Flatpak.

Sites such as `czkawka.net`, `czkawka.com`, `czawka.net` and similar are **not** official and may be unsafe.

### Q: Can I run two instances of Czkawka at the same time?

You can run instances of different tools simultaneously (each tool has its own cache file). However, you must **not** run two instances of the same tool at the same time - doing so can corrupt the cache for that tool.

### Q: What is the project license?

The project is **not** uniformly MIT - it depends on the component, and for the Slint apps it depends on whether you mean the source or the finished binary:

- **czkawka_core**, **czkawka_cli**, and the **Czkawka GTK GUI** - MIT.
- **Krokiet** and **Cedinia** - their **own application source code is MIT** (see `LICENSE_MIT_CODE` in each crate), **but the apps as a whole are GPL-3.0-only**. This is because they link the Slint UI framework under its free license, which is GPL-3.0; so the resulting combined work / binary must be distributed under GPL-3.0 (see `LICENSE_GPL_APP`). In other words: you may reuse their code under MIT, but a built Krokiet/Cedinia is GPL.
- All **images and audio files** - CC BY 4.0.

The per-component `LICENSE_*` files in each crate directory hold the exact texts.

### Q: Is there a web-based UI or Docker-based web interface for Czkawka?

There are third-party Docker images by jlesage that wrap the GUI in a VNC/web UI accessible from a browser: `jlesage/krokiet` (Krokiet) and `jlesage/czkawka` (the GTK GUI).

### Q: Are there nightly / pre-release builds available?

Yes. Nightly builds compiled from the latest master branch commits are published at https://github.com/qarmin/czkawka/releases/tag/Nightly. They may contain unknown bugs but include the newest features and fixes before an official release.

---

## Installation & Requirements

### Q: What are the system requirements?

- **Linux**: Ubuntu 22.04 or newer (glibc 2.35+). The pre-built binaries are statically linked (Krokiet) or dynamically linked against GTK4 (GTK GUI).
- **Windows**: Windows 10 or newer. Krokiet binaries are self-contained.
- **macOS**: Krokiet binaries are available for both Intel (x86_64) and Apple Silicon (ARM64). The GTK GUI binaries require GTK4 to be installed (e.g. via Homebrew).

### Q: How do I run it, and what do I need to install? (Linux / Windows / macOS)

Krokiet itself is a single self-contained binary - just download, run, and it works. You only need to install extra system libraries if you want optional functionality: `ffmpeg` for the Similar Videos tool, and `libheif`/`libavif`/`libraw` if you use a build with those image-format features. (The GTK GUI additionally needs GTK4 itself at runtime.)

There are ready-made scripts in the repo that install these dependencies for you (`misc/install_scripts/`):
- `install_linux.sh` (run with `sudo`) - auto-detects apt / dnf / pacman / zypper and installs `ffmpeg` + `gtk4` (base) and `libheif`/`libraw`/`libavif`/`dav1d` (optional).
- `install_macos.sh` - installs (and offers to set up Homebrew, then) `ffmpeg libheif libraw libavif` via `brew`.
- `install_windows.bat` - installs `ffmpeg` via `winget`; notes that `libheif`/`libraw`/`libavif` are only available through MSYS2 builds.

Per platform:

- **Linux**: download `linux_krokiet_x86_64` (or `_arm64`), `chmod +x` it, and run. For HEIF/AVIF/RAW use a `heif_raw_avif` build and install the matching libs (or just run `sudo misc/install_scripts/install_linux.sh`). The GTK GUI needs GTK4 installed.
- **Windows**: download a `windows_krokiet_on_*` build and run the `.exe` - no runtime to install for the core app. Install `ffmpeg` (e.g. `winget install Gyan.FFmpeg` or `misc/install_scripts/install_windows.bat`) only if you need Similar Videos.
- **macOS**: download `mac_krokiet_arm64` (Apple Silicon) or `mac_krokiet_x86_64` (Intel), then:

  ```bash
  chmod +x mac_krokiet_arm64
  ./mac_krokiet_arm64
  ```

  If macOS says "cannot be opened because it is from an unidentified developer", right-click the file and choose "Open", then confirm. If it opens as text in TextEdit, the executable bit is not set - run `chmod +x` first. For optional codecs use a `heif_avif` build and `brew install ffmpeg libheif libavif libraw` (or run `misc/install_scripts/install_macos.sh`). The GTK GUI (`mac_czkawka_gui_*`) additionally needs `brew install gtk4`.

### Q: Which Linux packages are available?

- **Pre-built binaries**: Download from the GitHub releases page (recommended, always up to date). This is now effectively the only source the author actively maintains.
- **Flatpak** (GTK GUI, `com.github.qarmin.czkawka`): Was author-maintained but is **no longer updated and is stuck at v10.0**. It still works but lags far behind; hopefully a new maintainer takes it over. No Krokiet Flatpak exists.
- **AppImage**: **No longer provided.** AppImages were dropped (random AppImage-specific bugs, little value over the plain Linux binaries). Use the pre-built binary instead.
- **Snap**: The author **used to publish a Snap but no longer maintains it**. It is outdated and has known permission issues (external drives, NFS); prefer the pre-built binary.
- **AUR / Debian / other distro repos**: Community-maintained, not by the project author. They may lag several versions behind. For the newest version use the GitHub binaries.

### Q: Similar Videos does not work - ffmpeg not found

The **Similar Videos** tool requires `ffmpeg` to be installed separately and available in `PATH`. Krokiet and czkawka_cli do not bundle ffmpeg.

| OS      | Installation                                                                                                                                                                                                           |
|---------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Linux   | `sudo apt install ffmpeg` (Debian/Ubuntu), `sudo pacman -S ffmpeg` (Arch)                                                                                                                                              |
| macOS   | `brew install ffmpeg`                                                                                                                                                                                                  |
| Windows | Install from [ffmpeg.org](https://ffmpeg.org/download.html) and add the folder containing `ffmpeg.exe` to the system `PATH`. On Windows, you can also place `ffmpeg.exe` in the same folder as the Krokiet executable. |

After installation, restart Krokiet or the CLI. On Windows, a new terminal session (or system restart) may be required for the PATH change to take effect.

---

## Which Build / Binary to Use

The release assets stack a few naming components, e.g. `<os>_<app>[_<features>][_<backend>]_<arch>`. Once you know the legend below, the same scheme applies to every platform, so there is nothing extra to learn per OS.

### Legend

- `krokiet` - primary graphical version of the application, fully supported and actively developed, includes new features and ongoing improvements
- `gtk_gui` (`czkawka_gui`) - legacy GTK-based graphical version, maintenance mode only, receives critical fixes but no new features
- `cli` (`czkawka_cli`) - command-line version of the app
- `cedinia` - experimental Android app
- `arm` / `arm64`, `x86_64` - CPU architecture. Most Windows/Linux machines use `x86_64`, while on Mac the `arm` (Apple Silicon) version is becoming the most common choice
- `heif`, `raw`, `avif` - additional image-format features that require extra libraries installed on the OS (libheif / libraw / libavif)
- `skia_opengl`, `skia_vulkan`, `femtovg_wgpu`, `all_backends` - alternative Krokiet rendering backends; different builds may fix some problems like blurry fonts or graphics crashes. `all_backends` bundles all of them (plus launcher scripts) in one package
- `apk` / `aab` - Cedinia Android package formats - just use `apk`; `aab` is used only for testing purposes
- (Windows only) `on_linux` / `on_windows` - which host the `.exe` was cross-compiled on

### Q: Which one should I pick?

Pick `krokiet` for your architecture with the **default** backend - it is built to be the most compatible everywhere. Only reach for a `skia_opengl` / `skia_vulkan` / `femtovg_wgpu` / `all_backends` build if the default one shows blurry text, fails to render, or crashes on your GPU. Choose a `heif` / `raw` / `avif` build only if you need those image formats (and install the matching system libraries). On Windows, if you are unsure between `on_linux` and `on_windows`, start with `windows_krokiet_on_linux`.

### Q: On Linux Krokiet shows nothing or crashes with a rendering error

Try the software renderer by setting the environment variable before launching:

```bash
SLINT_BACKEND=software krokiet
```

On systems without a GPU (VMs, headless servers), this is often the only option.

The default `linux_krokiet_*` binary only ships the femtovg and software renderers. If you want to try Skia/OpenGL/Vulkan/wgpu backends too, download the **`linux_krokiet_all_backends_*`** build - it is packaged as a **ZIP that contains the binary plus small bash launcher scripts**, one per renderer, that just set the right `SLINT_BACKEND` and start the app. So instead of exporting the variable yourself you can simply run, for example, `./krokiet_winit_skia_opengl.sh`, `./krokiet_winit_skia_vulkan.sh`, `./krokiet_winit_software.sh`, `./krokiet_femtovg_wgpu.sh`, or `./krokiet_winit_femtovg.sh` until one renders correctly.

### Q: Which Windows GTK GUI variant is provided?

Only one GTK build is shipped: `windows_czkawka_gui_gtk_412.zip` (GTK 4.12). The ZIP also contains `czkawka_cli.exe` and small `.bat` launchers that set `GSK_RENDERER` (cairo / opengl / vulkan) for the GTK GUI. If the GTK GUI renders a black window or fails to start with the default renderer, run those launchers instead of the `.exe` directly until one works (cairo is the safest software fallback). The GTK GUI is deprecated - prefer Krokiet, which needs no GTK at all.

---

## GTK Deprecation & Migration to Krokiet

### Q: I get a popup in Czkawka GTK saying to switch to Krokiet. Is GTK really deprecated?

Yes. **Version 12.0 is the last released version of the Czkawka GTK frontend** - no new GTK binaries will be provided, and no new features are planned. (Any Docker or distro packages built on top of it are third-party, not maintained by the project author.)

All new features and active development happen in **Krokiet**. Users should migrate to Krokiet.

### Q: Czkawka GTK works for me. Why should I switch?

The GTK frontend has persistent platform issues on Windows and macOS that cannot be fully fixed due to the nature of the GTK4 Windows/macOS port. These include:
- Transparent / unclickable window (Windows, #1904)
- Tiny or blurry text on HiDPI displays
- WebP preview not working (Windows, #1095)
- Various crashes when sorting, pasting text, or using custom select (#967, #1170)

Krokiet avoids all of these by using the Slint framework, which has a pure-Rust rendering pipeline and no dependency on GTK.

### Q: Does Czkawka work on Windows 7 or older Windows versions?

Windows 10 is the minimum supported version. The Rust standard library has dropped support for older Windows versions, so running on Windows 7 or Windows XP is not supported and is very unlikely to work with any current release. There are no plans to support systems that have themselves been out of support for years. Windows 10 will likely keep working until Rust itself drops it, which is probably ~10 years away.

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

This is usually a GPU driver or rendering backend issue. Try, in order:

1. **Try a different build / rendering backend.** The default backend is meant to be the most compatible, but no single backend works with every system and GPU driver. Download the `all_backends` build and start it through its per-backend launcher scripts (or set `SLINT_BACKEND` yourself) until one renders correctly.
2. Force the software renderer: `SLINT_BACKEND=software krokiet` (Linux/macOS), or set `SLINT_BACKEND=software` in Windows system environment variables and restart. On machines without a GPU (VMs, headless servers) this is often the only option.
3. Update your GPU drivers.

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
- `config_general.json` - window size, language, dark/light theme, current preset, etc.
- `config_preset_N.json` (N = 0-10) - per-preset scan directories and tool settings. There are 11 slots: 10 user presets plus a reserved one ("CLI Folders").
- `config_custom_select_state.json` - saved state for the custom-select dialog.

### Q: How large is the cache? Can it fill my disk?

The cache stores only metadata (hashes, timestamps, sizes) - not file contents. A typical large scan (150 TB of files) may produce a cache a few hundred MB in size at most, not terabytes.

### Q: What does "Remove outdated results from cache" do?

It removes cache entries for files that no longer exist on disk. This keeps the cache lean and avoids stale entries being used in future scans. Click it periodically if you have deleted many files since your last scan, and you want to increase the speed of loading/saving the cache.

### Q: Should I delete the cache to fix problems?

If you suspect cache corruption (e.g., scan results look wrong, or the app crashes when loading cache), you can delete the cache files in `~/.cache/czkawka/`. The next scan will rebuild the cache from scratch, which will be slower but may fix your issues.

### Q: What is the difference between the prehash cache file and the hash cache file?

The duplicate finder uses a two-stage hashing pipeline:
- **Prehash**: a fast partial hash computed over a small chunk from both the start and the end of each file (in the current version - older versions read only the start). Used to quickly eliminate files that cannot possibly be duplicates (they differ in prehash). Files that share the same size and prehash are promoted to the full hash stage.
- **Full hash**: a cryptographically strong hash of the entire file content. Only computed for files that survived the prehash stage.

Both stages have separate cache files. This is why you see two cache files for the duplicate tool. The prehash cache is much cheaper to build; the full hash cache is the authoritative deduplication signal.

### Q: Can I edit the cache files manually to change file paths (e.g., after moving files to a different location)?

The cache is stored as a binary `.bin` file by default. When the `.bin` file is absent, the application falls back to loading a `.json` file if one exists. To enable JSON cache saving, turn on "Save also as JSON" in the GUI settings. You can then edit the JSON and remove the `.bin` file to force the app to load the JSON version.

Changing directory paths in the JSON cache directly is possible in principle, but the entries also contain file sizes and modification timestamps, so any path change must remain consistent with the actual filesystem state. Stale entries are simply ignored on the next scan.

---

## Duplicate Files - Not Finding Expected Duplicates

### Q: Czkawka does not find my duplicate files even though I can see they are identical

The most common reason is the **minimum file size** setting. In the Krokiet GUI the default minimum is **16 KB (16 384 bytes)**, so smaller files are ignored. (The CLI uses per-tool defaults, and for `dup` that default is lower - 8 192 bytes.) To scan smaller files:

- **Krokiet**: Settings > minimum file size > set to a low value (the GUI value is in KB).
- **GTK GUI**: "Items configuration" tab > "Size (bytes) Min" > set to `1`.
- **CLI**: add `--minimal-file-size 1` to the command.

### Q: I have files below 16 KB that are duplicates - why are they excluded by default?

A large share of users scan whole drives looking for duplicates, and at that scale tiny files (a few bytes to a few KB - config snippets, empty placeholders, license headers, icons) are very often identical by pure coincidence without being meaningful duplicates. Surfacing thousands of such matches mostly just buries the larger duplicates the user actually came to find and makes the results harder to review. The 16 KB floor keeps the focus on duplicates worth acting on. If you specifically want the small ones, lower the minimum size as shown above.

### Q: Duplicates mode found files in the Recycle Bin / Trash

**In the GUI** this should not happen by default: the **default Excluded Items** already cover the trash on every platform - `*:\$RECYCLE.BIN\*` (and other Windows system dirs) on Windows, and `*/Trash/*` + `*/.Trash-*/*` on Linux/macOS. So out of the box, scanning even a whole drive root (`C:\` or `/`) skips the Recycle Bin / Trash.

If trash files still show up in the GUI, your **Excluded Items** field was probably cleared or edited and lost those defaults. Restore them (or reset excluded items to the default) - for example add back:
- Windows: `*:\$RECYCLE.BIN\*`
- Linux/macOS: `*/Trash/*`, `*/.Trash-*/*`

**In the CLI** these defaults are *not* applied automatically - you must add the exclusions yourself with `-E` / `--excluded-items`. To make this easy there are two macros that expand to the same patterns the GUI uses:
- `$TRASH` - just the OS trash / recycle bin (`*/Trash/*,*/.Trash-*/*` on Linux/macOS, `*:\$RECYCLE.BIN\*` on Windows).
- `DEFAULT` - the full set of default GUI exclusions (trash plus `.git`, `node_modules`, caches, Windows system dirs, etc.).

```bash
# Skip the trash while scanning a whole drive:
czkawka_cli dup -d / -E '$TRASH'

# Or apply the complete GUI default exclusion set:
czkawka_cli dup -d / -E DEFAULT
```

(Quote `$TRASH` in shells like bash/zsh so it is not treated as a variable.)

### Q: The scan found far fewer files than I expected during the "full hash" stage - is something wrong?

No. The number shown for "Analyzed full hash of X/Y files" reflects only the files that reached the full-hash stage, not the total number of files scanned. Files that differ in size or prehash never need a full hash, so Y can be much smaller than the total file count. This is expected and correct behavior.

### Q: The scan seems to freeze near the end (and Stop/Cancel does nothing)

The progress bar is already size-based, so a very large last file legitimately takes a while - but the more common reason it looks *frozen* is that per-file work (decoding, hashing, verifying a file) is run as one indivisible task. The app can only stop *between* tasks, not in the middle of one, so while a single huge or slow-to-decode file is being processed there is nothing to interrupt - and Rust cannot forcibly abort it either, because the work runs in-process as a normal function call, not as a separate external process (usually) that could simply be killed. So a single pathological file can make the whole scan appear stuck and unresponsive to Stop until it finishes.

For some tools this has been improved by splitting large tasks into smaller, individually-interruptible chunks so Stop reacts quickly - this already applies to e.g. duplicate finding and similar-video search. Other tools may still block until the current file is done.

### Q: How are symlinks and hard links treated in the duplicate scan?

- **Symbolic links are not followed and not scanned.** During file collection a symlink is skipped entirely - the scanner never compares a symlink against its target, so a symlink will not be reported as a "duplicate" of the file it points to. To scan the contents a symlink points at, add the real target path as an included directory.
- **Hard links are ignored by default** (the "Ignore hard links" / `hide_hard_links` option is on in both Krokiet and the CLI), so multiple names sharing one inode are counted once rather than flagged as duplicates of each other. Turn that option off (CLI `-L` / `--allow-hard-links`) if you want hard links treated as separate files.

---

## Similar Images

### Q: Similar Images does not find known duplicates of images

Check these settings:
1. **Minimum file size**: default is 16 KB. Images smaller than this are skipped. Lower it if needed.
2. **Max difference (similarity threshold)**: a value of `0` means only effectively identical hashes are matched. The Krokiet GUI default is `10`. The allowed range scales with hash size (up to about 40 at hash size 64), so raise it to catch looser near-duplicates. (The CLI's default is lower - `5` - so if you scan via CLI you may need to raise `-s`.)
3. **Hash algorithm and hash size**: different algorithms catch different types of similarity. The default in Krokiet is Mean hash with hash size 16. If you miss duplicates, try Gradient or DoubleGradient with hash size 32 or 64.
4. The image format must be supported. See the next question.

### Q: What image formats does Similar Images support?

Supported in all builds: JPEG, PNG, BMP, TIFF, WebP, TGA, plus the less common FF (Farbfeld), QOI, EXR, and JXL. (These are the extensions the Similar Images tool actually enumerates.)

Formats requiring optional features:
- **HEIC/HEIF**: needs a build with the `heif` feature (requires the `libheif` system library on Linux/macOS).
- **AVIF**: needs a build with the `libavif` feature - compile with `--features libavif` or use a binary variant whose name includes `avif`.
- **RAW** (CR2, NEF, ARW, etc.): basic RAW support is **always built in** via the pure-Rust `rawler` library, but it does not cover every format and not always fully. The optional `libraw` feature adds the `libraw` backend - the industry-standard library - which supports a larger set of RAW formats more completely. So RAW works without any special build; use a `libraw` build only if your camera's format is missing or mis-decoded.

### Q: Can Similar Images find duplicates across different image formats (e.g., JPEG vs WebP vs PNG)?

Yes. The Similar Images tool compares pixel content regardless of the file format. A JPEG and a WebP that contain the same visual content will be grouped together, as long as both formats are supported by the build you are using.

### Q: Can Similar Images detect greyscale images as similar to their color originals?

Yes - and there is currently no built-in option to exclude greyscale matches. All perceptual hash algorithms (Mean, Gradient, Double Gradient, Blockhash) ignore color and work on luminance, so a greyscale conversion of an image will appear highly similar to the original. If you want to avoid greyscale false positives, you must filter results manually or with a post-processing script.

### Q: Can Similar Images detect horizontally mirrored or rotated images?

Yes - via the **Geometric invariance** setting (CLI `--geometric-invariance`). It has three modes:
- `off` (default) - compare images as-is.
- `mirror-flip` - also match mirrored/flipped variants.
- `mirror-flip-rotate90` - also match 90/180/270-degree rotations.

These modes make matching more expensive, which is why they are off by default. EXIF orientation is always applied before hashing regardless of this setting.

### Q: I have many false positives in Similar Images (unrelated images grouped together)

Lower the **Max difference** (similarity threshold). The default setting `10` can match visually distinct images if they share large uniform areas (e.g., black borders, white backgrounds). Try `4` or `5` for stricter matching. Also try **increasing the hash size** (e.g. 32 or 64): a larger hash captures more detail, so unrelated images are less likely to collide - at the cost of being slightly stricter about re-encoded/resized copies.

### Q: What do the hash algorithm and hash size settings mean? Which should I use?

The settings control the perceptual hashing step in Similar Images:
- **Hash algorithm**: how the image is converted to a short fingerprint. The available algorithms are `Mean`, `Gradient`, `VertGradient`, `DoubleGradient`, `Median`, and `Blockhash`. `Mean` (the Krokiet GUI default) is the fastest and most permissive. `Gradient` / `VertGradient` / `DoubleGradient` are more sensitive to structural differences. `Median` is more robust against brightness/color shifts. `Blockhash` is a different approach that can find different near-duplicates.
- **Hash size**: 8, 16, 32, or 64 bits per dimension. Larger values produce longer, more precise fingerprints that reduce false positives but may miss re-encoded versions.

Practical guidance: for finding re-encoded/resized copies of the same image, start with Mean hash + size 16 and a "max difference" of 8-10. For stricter matching (fewer false positives), use Gradient/Double Gradient with size 32 and lower the max difference to 4-5.

### Q: HEIC images are not found

Ensure you are using a build that includes the `heif` feature. The plain `linux_krokiet_all_backends_x86_64` binary does **not** include HEIF/AVIF support. Use a `heif_raw_avif` variant (e.g. `linux_krokiet_heif_raw_avif_x86_64`) or compile with the appropriate feature flags:

```bash
cargo run --bin krokiet --features "winit_femtovg,winit_software,heif,libavif,libraw"
```

---

## Similar Videos & ffmpeg

### Q: How does Similar Videos detect duplicate videos?

Similar Videos works by sampling frames from the video and computing visual perceptual hashes, then comparing those hash sequences across short temporal "windows". The matching engine is provided by the project's own `similario_core` crate. `ffmpeg`/`ffprobe` must be installed for frame extraction.

### Q: What are the main tuning settings in Similar Videos?

The Krokiet GUI exposes visual presets (Custom, Near-identical, Similar, Movies) that set sensible values; the underlying parameters (also exposed individually in the CLI) are:

- **Max difference / tolerance**: how different two videos may be and still match (`0-20`; GUI default `15`, CLI `-t` default `10`). Lower = stricter.
- **Skip forward amount** (`--skip-forward-amount`, default `15` s): how many seconds to seek into the video before sampling begins, to skip intros/black openers (`0-300`; `0` = no skip).
- **Scan duration** (`-A`, default `10` s): how long a span of the video is sampled to build the signature.
- **Window count** (default per preset, `1-20`): how many temporal windows are sampled. More windows = more accurate but slower.
- **Duration tolerance %**: videos are pre-grouped by total duration; two videos are only compared if their durations differ by no more than this percentage.
- **Min matching windows fraction** (`0.0-1.0`): the fraction of windows that must agree to call two videos "same content".

There is also an optional **audio fingerprint** comparison (off by default, resource-intensive) with its own similarity/duration parameters. Note: there is no setting that simply skips both the start *and* end of every video, nor a single "minimum video duration" toggle - very short videos are skipped because they cannot fill the sampled windows.

### Q: Similar Videos scan starts but finds nothing, or produces many false positives

Similar Videos compares videos by sampling frames and computing visual hashes. False positives (unrelated videos grouped) are common when:
- The tolerance threshold is too high (lower the "Max difference" setting).
- Videos share common intro/outro sequences.
- Videos have unusual encoding (e.g., very low resolution, corrupt files).

If instead the scan finds **nothing** (real duplicates are missed), the cause is usually that the sampled span does not capture enough of the video. Several settings feed into this, so check them together:
- **Scan duration** too short - too small a span is sampled to build a reliable signature; increase it.
- **Window count** too low - fewer temporal windows means less of the video is compared; raise it.
- **Skip forward amount** too large - it can seek past the only content two clips share.
- **Min matching windows fraction** too high, or **Duration tolerance %** too low - both can reject genuine matches before they are compared.
- The tolerance / **Max difference** is too low (stricter) - raise it.

The algorithm is fundamentally heuristic and works best for finding re-encoded copies of the same source material, not for finding videos that happen to have a few similar-looking scenes.

### Q: Similar Videos cache - is it reused across scans?

Yes, the video hash cache is stored in `~/.cache/czkawka/` and is reused on subsequent scans. Each video is processed once; subsequent scans load the pre-computed hash from cache.

Note that the cache is keyed by the video **processing** parameters (the settings that affect how each video is sampled and hashed - scan duration, window count, skip forward amount, etc.). Changing any of these makes the app use a **different** cache file, because the computed hashes would otherwise differ. So after changing a processing parameter you have to rescan, and that scan re-processes the videos from scratch. Parameters that only affect how the already-computed hashes are **grouped** into the similar-videos list (e.g. max difference / min matching windows) do not invalidate the cache.

---

## Reference Paths / Reference Folders

### Q: What is a Reference Folder / Reference Path?

A reference folder is a directory added to the scan but **protected from deletion or modification**. Files in reference folders appear in the results only for comparison - they show which "keep" candidate they matched against. Files outside reference folders can be selected and deleted normally.

Typical use case: add your main, well-organized collection folder (e.g. your curated photo or music library) as a reference, add a downloads folder as a regular included path, then scan. Only files in the downloads folder will be selectable for deletion; the files in your main collection are always preserved.

### Q: How do I delete duplicates from one folder while keeping the copy in another folder?

Use Reference Folders. Add the folder you want to protect as a reference folder (check the "Reference" checkbox next to it in the directory list). After the scan, only files outside the reference folder appear as selectable. Use "Select All" and then delete - this will only delete the non-reference copies.

### Q: With a reference folder set, will groups that have duplicates only within the non-reference folder still appear?

No. When a reference folder is set, only groups that contain at least one file from the reference folder are shown. Groups that are entirely within non-reference directories are hidden. This is intentional: the reference folder marks "originals", so a group is only meaningful when at least one original is present.

### Q: How do I use reference folders in the CLI?

Use the `-r` / `--reference-directories` flag. To specify multiple reference directories, repeat the flag:

```bash
czkawka_cli dup -d /path/to/search -r /path/to/ref1 -r /path/to/ref2 -D AEO
```

Using comma- or semicolon-separated values in a single `-r` flag does **not** work. Each reference directory must be a separate `-r` flag.

---

## Deleting, Moving, Hardlinking Files

### Q: Delete does nothing / "Deleted 0 items, failed to remove 1 items"

Common causes:
1. **Trash on Samba/NFS shares**: Krokiet offers two separate actions - a permanent **Delete** button and a **Move to trash** button (and the CLI permanently deletes by default, with `-y`/`--move-to-trash` opting into trash). On network shares (SMB, NFS) the move-to-trash path can fail because the remote filesystem has no usable trash location. **Fix**: use the permanent **Delete** button (or omit `-y` in the CLI) instead of moving to trash.
2. **Read-only files/folders**: the file itself may be writable but its parent directory is read-only. Check permissions on the containing folder.

### Q: Move is slow - it copies the whole file instead of just renaming

On the same filesystem, Krokiet performs a rename (which is instant). If the source and destination are on different filesystems (e.g., moving from one drive to another), a full copy-then-delete is performed, which is expected to be slow. This is not a bug.

### Q: Flatpak version moves deleted files to a different Trash folder than the system uses

This is a known behavior of the Flatpak sandbox. Files moved to trash inside a Flatpak container go to `~/.Trash-<uid>/` rather than `~/.local/share/Trash/`. This cannot be configured and is a limitation of the Flatpak sandbox, not of Czkawka itself.

### Q: How does Hardlink work? What does it actually do to my files?

Hardlink replaces selected duplicate files with hard links pointing to one retained copy. After hardlinking, all files share the same inode - they appear as separate filenames but consume disk space only once. **Note**: hardlinking across different filesystems or partitions is not possible. Such a pair is not silently skipped - the operation fails for it and an error is reported (e.g. "Failed to hardlink ...: Invalid cross-device link"); the original file is left untouched, so nothing is lost.

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

## Bad Extensions Tool

### Q: Bad Extensions says my file has the wrong extension. How do I rename it?

In **Krokiet**: select the files in the results list and use the **Rename** button (or right-click > Rename). The suggested new extension is shown in the "Extra" column (`current_ext -> correct_ext`).

In the **CLI**: pass `-F` / `--fix-extensions` to `bad-ext` and the matched files are renamed automatically to use the detected correct extension:

```bash
czkawka_cli bad-ext -d /path/to/scan -F
```

In the **GTK GUI**: renaming from the GUI is not supported. Export results and rename manually.

### Q: Bad Extensions incorrectly flags a file's extension, or makes other obvious mistakes

The extension detection uses file magic bytes (the `infer` crate). Many file formats share the same leading bytes, so there is no unambiguous way to pick a single "correct" extension from the content alone. Examples:
- ZIP-based formats (`.zip`, `.docx`, `.xlsx`, `.pptx`, `.jar`, `.apk`, `.epub`) all start with the same ZIP signature.
- Old Microsoft Office formats (`.doc`, `.xls`, `.ppt`) are compound binary files that share the same OLE container header.

In those cases the tool can only report the family it detects, not which specific extension is "right". Some less common formats may also simply be missing from the library's database. You can exclude specific extensions from the scan if needed.

---

## Scanning External Drives, NAS, Network Paths

### Q: How do I exclude hidden/system directories like Synology's @eadir or macOS .DS_Store directories?

Use the **Excluded Items** field (not Excluded Directories). Add a wildcard pattern that matches the directory name anywhere in the path. Examples:

- Synology `@eadir`: `*/@ea*`
- macOS `.Spotlight-V100`, `.fseventsd`, etc.: `*/.Spotlight*`, `*/.fseventsd*`
- Any hidden directory (starts with `.`): use Excluded Directories to add the specific paths.

The excluded items patterns use `*` as a wildcard and match against the full path of each file/directory.

### Q: Cannot scan a Windows network share (SMB path) with Krokiet on Windows

On Windows, UNC paths like `\\server\share\` may not be shown in the file picker dialog. Use the "Manual add" text input in the directories panel to type or paste the UNC path directly.

If that still gives you trouble, it is worth mapping the share to a drive letter (e.g. `net use Z: \\server\share` or Explorer > "Map network drive"), then scan the resulting `Z:\` path instead - a regular drive letter behaves more predictably than a raw UNC path.

### Q: Czkawka / Krokiet cannot access `C:\System Volume Information` - Access is denied

This is expected. System directories and volumes reserved by Windows are inaccessible to normal user processes. Czkawka logs access errors but continues scanning the rest of the directory tree. These errors are harmless.

### Q: Can I use drag and drop to add directories in Krokiet?

Not currently. Drag and drop to add directories is not implemented, because Slint (the UI framework Krokiet uses) does not yet expose drag-and-drop support. Once Slint adds it, it can be implemented in Krokiet. For now, use the "Add" button or type/paste the path directly in the text field.

---

## Snap & Flatpak Packages

### Q: The Snap package is outdated / has permission problems

I used to publish a Snap but I no longer maintain it - I dropped Snap builds in v9.0, so the last Snap is stuck at **v8.0** and it has known permission issues for external drives and NFS mounts. Use the **pre-built binary** from the GitHub releases page instead (recommended).

### Q: The Flatpak version is out of date

Yes - I no longer maintain the Flathub package (GTK GUI, `com.github.qarmin.czkawka`); it is **frozen at v10.0**, so it is many versions behind. It may be adopted by a new maintainer in the future. For the latest version, use the pre-built binary from the GitHub releases page.

---

## Performance & Large Scans

### Q: Scanning is very slow on a traditional HDD

Scanning speed on HDDs is limited by seek time. The application collects file metadata and sorts files by inode number before reading, which improves sequential read patterns. However, with very large directories, performance will still be significantly slower than on an SSD.

Tips:
- Enable the **cache** (enabled by default). The second scan of the same files is much faster.
- Enable **prehash** (enabled by default in Krokiet). This adds a fast partial-hash stage that eliminates most non-duplicates before the full hash.
- Limit the scan to specific subdirectories rather than scanning an entire multi-TB drive in one run.
- It is possible to limit how many threads read from disk at once, but in practice this currently helps little: file data is already read in fairly large chunks (1-2 MB at a time), so the threads are not generating the kind of tiny random reads that throttling the thread count would smooth out.

### Q: The UI becomes very slow / laggy after a scan with thousands of groups

In Krokiet itself this should not happen - the results list is virtualized and Krokiet can render millions of entries without any problem. The one place that can genuinely slow down is the **bottom text/output panel**: when it accumulates a very large amount of text, rendering that text gets slow. That is a Slint text-rendering bug, not a problem with the result list. As a workaround, collapse or hide the bottom panel.

### Q: CPU usage rises sharply during a large Similar Images or Similar Videos scan - is this expected?

Yes. Different stages stress different resources: some steps are heavily CPU-bound (the perceptual-hashing / decoding work, parallelized across all cores, which makes CPU usage spike), while others are mostly disk-bound (reading file contents). This is normal. If you want to keep CPU usage down, you can limit the number of threads (CLI `-T <N>` / `--thread-number`, or the thread setting in the GUI).

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

The CLI does not show a real-time progress bar (no spinner/percentage in current releases). It prints the high-level stage transitions as the scan proceeds, but there is no live per-file progress counter.

### Q: How do I suppress CLI output / run silently?

Use `--do-not-print-results` (`-N`) to suppress the results listing, and `--do-not-print-messages` (`-M`) to suppress informational messages and warnings. Both can be combined.

### Q: How do I limit the number of threads the CLI uses?

Use `-T <N>` / `--thread-number <N>`. Setting `0` (the default) uses all available CPU threads.

### Q: How do I exclude files from scanning by extension?

Use `-P <ext>` / `--excluded-extensions <ext>` to exclude specific extensions. Use `-x <ext>` / `--allowed-extensions <ext>` to scan only specific extensions. The `--allowed-extensions` flag also accepts macros: `IMAGE`, `VIDEO`, `MUSIC`, `TEXT`.

### Q: How do I exclude paths/items in the CLI, and does it have the same trash defaults as the GUI?

Use `-E <pattern>` / `--excluded-items <pattern>` with wildcards (e.g. `-E '*/temp*'`). Unlike the GUI, the CLI does **not** apply any default exclusions automatically. Two macros expand to the same patterns the GUI uses by default:
- `$TRASH` - the OS trash / recycle bin only (`*/Trash/*,*/.Trash-*/*` on Linux/macOS, `*:\$RECYCLE.BIN\*` on Windows).
- `DEFAULT` - the full default GUI exclusion set (trash plus `.git`, `node_modules`, caches, Windows system dirs, etc.).

```bash
czkawka_cli dup -d / -E '$TRASH'     # skip the trash/recycle bin
czkawka_cli dup -d / -E DEFAULT      # apply all GUI default exclusions
```

Quote `$TRASH` so your shell does not expand it as a variable.

### Q: Is there a dry-run mode in the CLI?

Yes. Add `--dry-run` to preview what would be deleted without actually deleting anything. Note that dry-run output currently goes to the console only and is not written to the result file.

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

If you genuinely need to scan such a location, first check your own permissions on it (ownership and mode), and only as a last resort run the application as administrator / root - though scanning a whole system as root is rarely what you actually want, and it lets the tool act on files a normal user could not.

### Q: "Error during trash operation" / files not moved to trash

Usually means the trash is on a different filesystem from the file (e.g., the file is on a network share or a secondary drive that does not have a `.Trash-<uid>` directory). Solutions:
- Use permanent delete instead of move-to-trash.
- For network shares (NFS, SMB), permanent delete is the only reliable option.

### Q: "malformed stream: mp3 invalid main_data offset" in Broken Files scan

This message comes from the `symphonia` decoding library, which checks the bitstream strictly: it reports that the MP3 header contains a `main_data_begin` value pointing outside the bitstream. `symphonia` is quite pedantic and flags such issues even when they are harmless - many MP3 files carry this "error" because of encoder quirks, and most media players (and likely your own player) decode and play them just fine. The file is still playable; you can ignore this result or keep the file.

### Q: The app shows ".fuse_hidden..." files

These files are created automatically by the operating system / FUSE-based filesystems - this is not something Czkawka or Krokiet does. When a file that is still open in another application is "deleted", FUSE renames it to a hidden `.fuse_hidden...` name and only truly removes it once the last handle is closed. They are not duplicates in the usual sense. You can exclude them by adding `*/.fuse_hidden*` to the excluded items list.

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

Czkawka uses hashes only (size + prehash + full hash pipeline) - no byte-by-byte comparison. This is reliable: candidates are first grouped by exact file size, and only files of the same size are ever compared by hash, so the hash space each comparison draws from is tiny. A collision would require two different files of the *identical* size to also produce the same full hash, which in practice never happens with the default Blake3 (a cryptographic hash). A byte-by-byte pass would only confirm what the hashes already establish, while being much slower, so it is intentionally not done.

### Q: What hash algorithm does the duplicate finder use?

The default hash algorithm is **Blake3** (very fast, cryptographically strong). The other two options are **CRC32** (a fast checksum, not cryptographic) and **XXH3** (xxHash, very fast, not cryptographic). There is no SHA-256/SHA-512 option. For deduplication any of the three is fine in practice; Blake3 is the recommended default because it combines high speed with cryptographic-grade collision resistance.

### Q: How does the duplicate finder handle files that differ only in name but are otherwise identical?

Two files with different names but identical content (same size and same hash) are reported as duplicates. The filename is not part of the duplicate comparison when using the "Hash" check method. If you want to find duplicates by name only, use the "Name" check method instead.
