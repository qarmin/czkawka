# Cedinia – English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Duplicates
tool_empty_folders = Empty Folders
tool_similar_images = Similar Images
tool_empty_files = Empty Files
tool_temporary_files = Temporary Files
tool_big_files = Biggest Files
tool_broken_files = Broken Files
tool_bad_extensions = Bad Extensions
tool_same_music = Music Duplicates
tool_bad_names = Bad Names
tool_exif_remover = EXIF Data
tool_directories = Directories
tool_settings = Settings

# Home screen tool card descriptions
home_dup_description = Find files with the same content
home_empty_folders_description = Directories without content
home_similar_images_description = Find visually similar photos
home_empty_files_description = Files with zero size
home_temp_files_description = Temporary and cached files
home_big_files_description = Biggest/Smallest files on disk
home_broken_files_description = PDF, audio, images, archives
home_bad_extensions_description = Files with invalid extension
home_same_music_description = Similar audio files by tags
home_bad_names_description = Files with problematic characters in name
home_exif_description = Images with EXIF metadata

# Results list
scanning = Scanning in progress...
stopping = Stopping...
no_results = No results
press_start = Press START to scan
select_label = Sel.
deselect_label = Desel.
list_label = List
gallery_label = Gal.

# Selection popup
selection_popup_title = Select
select_all = Select all
select_except_one = Select all except one
select_except_largest = Select all except largest
select_except_smallest = Select all except smallest
select_largest = Select largest
select_smallest = Select smallest
select_except_highest_res = Select all except highest resolution
select_except_lowest_res = Select all except lowest resolution
select_highest_res = Select highest resolution
select_lowest_res = Select lowest resolution
invert_selection = Invert selection
close = Close

# Deselection popup
deselection_popup_title = Deselect
deselect_all = Deselect all
deselect_except_one = Deselect all except one

# Confirm popup
cancel = Cancel
delete = Delete
rename = Rename

# Delete errors popup
delete_errors_title = Failed to delete some files:
ok = OK

# Stopping overlay
stopping_overlay_title = ■ Stopping
stopping_overlay_body = Finishing current scan…\nPlease wait.

# Permission popup
permission_title = 🔒 File Access
permission_body = To scan files, the app needs access to device storage. Without this permission, scanning will not be possible.
grant = Grant
no_permission_scan_warning = No file access – grant permission to scan

# Settings screen tabs
settings_tab_general = General
settings_tab_tools = Tools
settings_tab_diagnostics = Info

# Settings — General tab
settings_use_cache = Use cache
settings_use_cache_desc = Speeds up subsequent scans (hash/images)
settings_ignore_hidden = Ignore hidden files
settings_ignore_hidden_desc = Files and folders starting with '.'
settings_scan_label = SCAN
settings_filters_label = FILTERS (some tools)
settings_min_file_size = Min. file size
settings_max_file_size = Max. file size
settings_language = Language
settings_language_restart = Requires app restart
settings_common_label = COMMON SETTINGS
settings_excluded_items = EXCLUDED ITEMS (glob patterns, comma separated)
settings_excluded_items_placeholder = e.g. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = ALLOWED EXTENSIONS (empty = all)
settings_allowed_extensions_placeholder = e.g. jpg, png, mp4
settings_excluded_extensions = EXCLUDED EXTENSIONS
settings_excluded_extensions_placeholder = e.g. bak, tmp, log

# Settings — Tools section labels
settings_duplicates_header = DUPLICATES
settings_check_method_label = COMPARISON METHOD
settings_check_method = Method
settings_hash_type_label = HASH TYPE
settings_hash_type = Hash type
settings_hash_type_desc = Blake3 – fastest; CRC32/xxH3 – alternatives
settings_similar_images_header = SIMILAR IMAGES
settings_similarity_preset = Similarity threshold
settings_similarity_desc = Very High = only near-identical
settings_hash_size = Hash size
settings_hash_size_desc = Larger = more accurate, slower
settings_hash_alg = Hash algorithm
settings_image_filter = Resize filter
settings_ignore_same_size = Ignore images with the same dimensions
settings_big_files_header = BIGGEST FILES
settings_search_mode = Search mode
settings_file_count = File count
settings_same_music_header = MUSIC DUPLICATES
settings_music_check_method = Comparison mode
settings_music_compare_tags_label = COMPARED TAGS
settings_music_title = Title
settings_music_artist = Artist
settings_music_year = Year
settings_music_length = Length
settings_music_genre = Genre
settings_music_bitrate = Bitrate
settings_music_approx = Approximate tag comparison
settings_broken_files_header = BROKEN FILES
settings_broken_files_types_label = CHECKED TYPES
settings_broken_audio = Audio
settings_broken_pdf = PDF
settings_broken_archive = Archive
settings_broken_image = Image
settings_bad_names_header = BAD NAMES
settings_bad_names_checks_label = CHECKS
settings_bad_names_uppercase_ext = Uppercase extension
settings_bad_names_emoji = Emoji in name
settings_bad_names_space = Spaces at start/end
settings_bad_names_non_ascii = Non-ASCII characters
settings_bad_names_duplicated = Repeated characters

# Settings — Diagnostics tab
diagnostics_header = DIAGNOSTICS
diagnostics_thumbnails = Thumbnail cache
diagnostics_app_cache = App cache
diagnostics_refresh = Refresh
diagnostics_clear_thumbnails = Clear thumbnails
diagnostics_clear_cache = Clear cache
diagnostics_collect_test = Scan test
diagnostics_collect_test_desc = Scans each volume recursively
diagnostics_collect_test_run = Run
diagnostics_collect_test_stop = Stop
about_repo = Repository
about_translate = Translations
about_donate = Support

# Collect-test result popup
collect_test_title = 📊 Test results
collect_test_volumes = 💾 Volumes:
collect_test_folders = 📁 Folders:
collect_test_files = 📄 Files:
collect_test_time = ⏱ Time:
collect_test_ms = " ms"

# Directories screen
directories_include_header = Directories to scan
directories_exclude_header = Excluded directories
directories_add = + Add
no_paths = No paths – add below
directories_volume_header = Volumes
directories_volume_refresh = Refresh
directories_volume_add = Add

# Bottom navigation
nav_home = Start
nav_dirs = Directories
nav_settings = Settings

# Status messages set from Rust
status_ready = Ready
status_stopped = Stopped
status_no_results = No results
status_deleted_selected = Deleted selected
status_deleted_with_errors = Deleted with errors
scan_not_started = Scan not started
found_items_prefix = Found
found_items_suffix = items
deleted_items_prefix = Deleted
deleted_items_suffix = items
deleted_errors_suffix = errors
renamed_prefix = Renamed
renamed_files_suffix = files
renamed_errors_suffix = errors
cleaned_exif_prefix = Cleaned EXIF from
cleaned_exif_suffix = files
cleaned_exif_errors_suffix = errors
and_more_prefix = …and
and_more_suffix = more

# Gallery / delete popups
gallery_delete_button = Delete
gallery_back = Back
gallery_confirm_delete = Yes, delete
deleting_files = Deleting files…
stop = Stop
files_suffix = files
scanning_fallback = Scanning…
app_subtitle = In honour of the Battle of Cedynia (972 CE)
app_license = Frontend for Czkawka Core  •  GPL-3.0
about_app_label = ABOUT
cache_label = CACHE

