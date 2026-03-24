# Kalka – English translations (Fluent format)
# This file follows the same Fluent convention as krokiet/i18n/

# ── Window & app ──────────────────────────────────────────
main-window-title = Kalka - Data Cleaner
about-title = About Kalka
about-app-name = Kalka
about-subtitle = PySide6 / Qt 6 Edition
about-version = Version 11.0.1
about-description =
    Kalka is a simple, fast and free app to remove
    unnecessary files from your computer.

    This PySide6/Qt interface uses the czkawka_cli backend
    for all scanning and file operations.

    Features:
      - Find duplicate files (by hash, name, or size)
      - Find empty files and folders
      - Find similar images, videos, and music
      - Find broken files and invalid symlinks
      - Find files with bad extensions or names
      - Remove EXIF metadata from images
      - Optimize and crop videos

    Licensed under MIT License
    https://github.com/qarmin/czkawka
about-logo-tooltip = About Kalka
version-label = Kalka v11.0.1

# ── Status bar ────────────────────────────────────────────
status-ready = Ready
status-tab = Tab: { $tab_name }
status-scanning = Scanning: { $tab_name }...
status-scan-complete = Scan complete: found { $count } entries
status-scan-stopped = Scan stopped by user
status-error = Error: { $message }
status-deleted = Deleted { $count } file(s)
status-deleted-dry-run = [DRY RUN] Deleted { $count } file(s)
status-moved = Moved { $count } file(s)
status-moved-dry-run = [DRY RUN] Moved { $count } file(s)
status-copied = Copied { $count } file(s)
status-copied-dry-run = [DRY RUN] Copied { $count } file(s)
status-hardlinks-created = Created { $count } hardlink(s)
status-symlinks-created = Created { $count } symlink(s)
status-exif-cleaned = Cleaned EXIF from { $count } file(s)
status-extensions-fixed = Extensions fixed
status-names-fixed = Names fixed
status-results-saved = Results saved successfully
status-results-loaded = Loaded { $count } entries from file
status-video-optimize = Video optimization: use CLI directly for this feature

# ── Buttons ───────────────────────────────────────────────
scan-button = Scan
stop-button = Stop
select-button = Select
delete-button = Delete
move-button = Move
save-button = Save
load-button = Load
load-button-tooltip = Load previously saved results
sort-button = Sort
hardlink-button = Hardlink
symlink-button = Symlink
rename-button = Rename
clean-exif-button = Clean EXIF
optimize-button = Optimize

# ── Tool names ────────────────────────────────────────────
tool-duplicate-files = Duplicate Files
tool-empty-folders = Empty Folders
tool-big-files = Big Files
tool-empty-files = Empty Files
tool-temporary-files = Temporary Files
tool-similar-images = Similar Images
tool-similar-videos = Similar Videos
tool-similar-music = Similar Music
tool-invalid-symlinks = Invalid Symlinks
tool-broken-files = Broken Files
tool-bad-extensions = Bad Extensions
tool-bad-names = Bad Names
tool-exif-remover = EXIF Remover
tool-video-optimizer = Video Optimizer

# ── Column headers ────────────────────────────────────────
column-selection = Selection
column-size = Size
column-file-name = File Name
column-path = Path
column-modification-date = Modification Date
column-hash = Hash
column-similarity = Similarity
column-resolution = Resolution
column-title = Title
column-artist = Artist
column-year = Year
column-bitrate = Bitrate
column-genre = Genre
column-length = Length
column-folder-name = Folder Name
column-symlink-name = Symlink Name
column-symlink-path = Symlink Path
column-destination-path = Destination Path
column-type-of-error = Type of Error
column-error-type = Error Type
column-current-extension = Current Extension
column-proper-extension = Proper Extension
column-codec = Codec

# ── Left panel ────────────────────────────────────────────
settings-tooltip = Application Settings
tool-settings-tooltip = Tool-specific Settings
fallback-app-name = Kalka

# ── Settings panel ────────────────────────────────────────
settings-title = Settings
settings-close = Close
settings-tab-general = General
settings-tab-directories = Directories
settings-tab-filters = Filters
settings-tab-preview = Preview
settings-window-title = Kalka Settings

# General settings
settings-cli-path = czkawka_cli Path:
settings-browse = Browse
settings-thread-count = Thread Count:
settings-thread-auto = Auto (all cores)
settings-recursive = Recursive search
settings-use-cache = Use cache for faster rescans
settings-move-to-trash = Move to trash instead of permanent delete
settings-hide-hard-links = Hide hard links
settings-save-as-json = Save results as JSON (instead of text)
settings-select-cli-binary = Select czkawka_cli binary

# Directories settings
settings-included-dirs = Included Directories
settings-excluded-dirs = Excluded Directories
settings-add = Add
settings-remove = Remove
settings-select-dir-include = Select Directory to Include
settings-select-dir-exclude = Select Directory to Exclude

# Filters settings
settings-excluded-items = Excluded Items:
settings-excluded-items-hint = Wildcard patterns, comma-separated (e.g. *.tmp,cache_*)
settings-allowed-extensions = Allowed Extensions:
settings-allowed-extensions-hint = e.g. jpg,png,gif
settings-excluded-extensions = Excluded Extensions:
settings-excluded-extensions-hint = e.g. log,tmp
settings-min-file-size = Minimum File Size:
settings-min-file-size-hint = In bytes (e.g. 1024)
settings-max-file-size = Maximum File Size:
settings-max-file-size-hint = In bytes (leave empty for no limit)

# Preview settings
settings-show-image-preview = Show image preview

# ── Tool settings panel ──────────────────────────────────
tool-settings-title = Tool Settings

# Duplicate files
subsettings-check-method = Check Method:
subsettings-hash-type = Hash Type:
subsettings-case-sensitive = Case sensitive name comparison

# Similar images
subsettings-hash-size = Hash Size:
subsettings-resize-algorithm = Resize Algorithm:
subsettings-image-hash-type = Hash Type:
subsettings-ignore-same-size = Ignore same size
subsettings-max-difference = Max Difference:

# Similar videos
subsettings-crop-detect = Crop Detect:
subsettings-skip-forward = Skip Forward (s):
subsettings-hash-duration = Hash Duration (s):

# Similar music
subsettings-audio-check-type = Audio Check Type:
subsettings-tag-matching = Tag Matching
subsettings-approximate-comparison = Approximate comparison
subsettings-fingerprint-matching = Fingerprint Matching
subsettings-compare-similar-titles = Compare with similar titles

# Big files
subsettings-method = Method:
subsettings-the-biggest = The Biggest
subsettings-the-smallest = The Smallest
subsettings-number-of-files = Number of Files:

# Broken files
subsettings-file-types = File types to check:

# Bad names
subsettings-check-for = Check for:
subsettings-uppercase-ext = Uppercase extension
subsettings-uppercase-ext-hint = Files with .JPG, .PNG etc.
subsettings-emoji = Emoji in name
subsettings-emoji-hint = Files containing emoji characters
subsettings-space = Space at start/end
subsettings-space-hint = Leading or trailing whitespace
subsettings-non-ascii = Non-ASCII characters
subsettings-non-ascii-hint = Characters outside ASCII range
subsettings-remove-duplicated = Remove duplicated non-alphanumeric
subsettings-remove-duplicated-hint = e.g. file--name..txt
subsettings-restricted-charset = Restricted charset:
subsettings-restricted-charset-hint = Allowed special chars, comma-separated

# EXIF
subsettings-ignored-exif-tags = Ignored EXIF Tags:
subsettings-ignored-exif-tags-hint = Tags to ignore, comma-separated

# Video optimizer
subsettings-mode = Mode:
subsettings-crop-settings = Crop Settings
subsettings-crop-type = Crop Type:
subsettings-black-pixel-threshold = Black Pixel Threshold:
subsettings-black-bar-min-pct = Black Bar Min %:
subsettings-max-samples = Max Samples:
subsettings-min-crop-size = Min Crop Size:
subsettings-transcode-settings = Transcode Settings
subsettings-excluded-codecs = Excluded Codecs:
subsettings-target-codec = Target Codec:
subsettings-quality = Quality:
subsettings-fail-if-bigger = Fail if not smaller

# ── Progress widget ──────────────────────────────────────
progress-initializing = Initializing...
progress-starting = Starting scan...
progress-current = Current
progress-overall = Overall
progress-scan-complete = Scan complete
progress-completed-in = Completed in { $time }
progress-done = done

# ── Results view ─────────────────────────────────────────
results-no-results = No results
results-found-grouped = Found { $total } files ({ $size }) in { $groups } groups
results-found-flat = Found { $total } entries ({ $size })
results-selected = Selected: { $selected }/{ $total } ({ $selected_size }/{ $total_size })

# ── Preview panel ────────────────────────────────────────
preview-title = Preview
preview-no-preview = No preview
preview-file-not-found = File not found
preview-not-available = Preview not available
    for this file type
preview-cannot-load = Cannot load image

# ── Context menu ─────────────────────────────────────────
context-open-file = Open File
context-open-folder = Open Containing Folder
context-select = Select
context-deselect = Deselect

# ── Bottom panel ─────────────────────────────────────────
bottom-included-dirs = Included Directories:
bottom-excluded-dirs = Excluded Directories:

# ── Delete dialog ────────────────────────────────────────
delete-dialog-title = Delete Files
delete-dialog-message = Are you sure you want to delete { $count } selected file(s)?
delete-dialog-trash = Move to trash instead of permanent delete
delete-dialog-dry-run = Dry run (preview only, no files will be deleted)
delete-dialog-confirm = Delete

# ── Move dialog ──────────────────────────────────────────
move-dialog-title = Move/Copy Files
move-dialog-message = Move or copy { $count } selected file(s) to:
move-dialog-placeholder = Select destination folder...
move-dialog-preserve = Preserve folder structure
move-dialog-copy-mode = Copy instead of move
move-dialog-dry-run = Dry run (preview only, no files will be moved)
move-dialog-confirm = Move
move-dialog-select-dest = Select Destination

# ── Rename dialog ────────────────────────────────────────
rename-dialog-confirm = Rename
rename-dialog-ext-message = Fix extensions for { $count } selected file(s)?

    Files will be renamed to use their proper extensions.
rename-dialog-names-message = Fix names for { $count } selected file(s)?

    Files with problematic names will be renamed.

# ── Select dialog ────────────────────────────────────────
select-dialog-title = Select Results
select-dialog-prompt = Choose selection mode:
select-all = Select All
unselect-all = Unselect All
invert-selection = Invert Selection
select-biggest-size = Select Biggest (by Size)
select-smallest-size = Select Smallest (by Size)
select-newest = Select Newest
select-oldest = Select Oldest
select-shortest-path = Select Shortest Path
select-longest-path = Select Longest Path
cancel = Cancel

# ── Sort dialog ──────────────────────────────────────────
sort-dialog-title = Sort Results
sort-by = Sort by:
sort-ascending = Ascending

# ── Hardlink / symlink dialogs ───────────────────────────
hardlink-dialog-title = Create Hardlinks
hardlink-dialog-message = Replace { $count } selected file(s) with hardlinks to:
    { $reference }?
symlink-dialog-title = Create Symlinks
symlink-dialog-message = Replace { $count } selected file(s) with symlinks to:
    { $reference }?
no-reference-title = No Reference
no-reference-message = Cannot determine reference file. Leave at least one file unchecked in the group.

# ── EXIF dialog ──────────────────────────────────────────
exif-dialog-title = Clean EXIF
exif-dialog-message = Remove EXIF metadata from { $count } selected file(s)?

# ── Video optimize dialog ────────────────────────────────
video-optimize-title = Video Optimization
video-optimize-message = Video optimization for { $count } file(s) will be performed using czkawka_cli. Check the status bar for progress.

# ── Warnings / errors ────────────────────────────────────
no-directories-title = No Directories
no-directories-message = Please add at least one directory to scan in the bottom panel.
no-selection-title = No Selection
no-selection-delete = No files selected for deletion.
no-selection-move = No files selected.
no-results-title = No Results
no-results-save = No results to save.
no-destination-title = No Destination
no-destination-message = Please select a destination folder.
scan-error-title = Scan Error

# ── Save/load dialogs ────────────────────────────────────
save-dialog-title = Save Results
load-dialog-title = Load Results
