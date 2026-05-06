# Cedinia - English (fallback)

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
tool_similar_videos = Similar Videos (Audio)
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
home_similar_videos_description = Find videos with similar audio (no FFmpeg needed)

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
stopping_overlay_title = Stopping
stopping_overlay_body = Finishing current scan...
    Please wait.

# Permission popup
permission_title = File Access
permission_body = To scan files, the app needs access to device storage. Without this permission, scanning will not be possible.
grant = Grant
no_permission_scan_warning = No file access - grant permission to scan

# Settings screen tabs
settings_tab_general = General
settings_tab_tools = Tools
settings_tab_diagnostics = Info

# Settings - General tab
settings_use_cache = Use cache
settings_use_cache_desc = Speeds up subsequent scans (hash/images)
settings_ignore_hidden = Ignore hidden files
settings_ignore_hidden_desc = Files and folders starting with '.'
settings_show_notification = Notify when scan finishes
settings_show_notification_desc = Show a system notification on scan completion
settings_notify_only_background = Only when in background
settings_notify_only_background_desc = Skip notification if the app is visible
notifications_disabled_banner = Notifications disabled
notifications_enable_button = Enable
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

# Settings - Tools section labels
settings_duplicates_header = DUPLICATES
settings_check_method_label = COMPARISON METHOD
settings_check_method = Method
settings_hash_type_label = HASH TYPE
settings_hash_type = Hash type
settings_hash_type_desc = Blake3 - is recommended option, CRC32 have small chance of false positives
settings_similar_images_header = SIMILAR IMAGES
settings_similarity_preset = Similarity threshold
settings_similarity_desc = Very High = only near-identical
settings_hash_size = Hash size
settings_hash_size_desc = Larger sizes, have less false positives, but also finds less similar images
settings_hash_alg = Hash algorithm
settings_image_filter = Resize filter
settings_ignore_same_size = Ignore images with the same dimensions
settings_gallery_image_fit_cover = Gallery: crop to square
settings_gallery_image_fit_cover_desc = Fill the tile; disable to keep original aspect ratio
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
settings_temporary_files_header = TEMPORARY FILES
settings_temporary_files_extensions_label = EXTENSIONS
settings_temporary_files_extensions_placeholder = e.g. .tmp,.bak,~
settings_temporary_files_reset = Reset to defaults
settings_broken_files_header = BROKEN FILES
settings_broken_files_note = Resource-intensive scan. For best performance use Krokiet on desktop.
settings_broken_files_types_label = CHECKED TYPES
settings_broken_audio = Audio
settings_broken_pdf = PDF
settings_broken_archive = Archive
settings_broken_image = Image
settings_broken_font = Font
settings_broken_markup = Markup (JSON/XML/TOML)
settings_similar_videos_header = SIMILAR VIDEOS (AUDIO)
settings_similar_videos_audio_preset = Audio similarity preset
settings_similar_videos_audio_preset_desc = Controls how strictly audio must match
settings_bad_names_header = BAD NAMES
settings_bad_names_checks_label = CHECKS
settings_bad_names_uppercase_ext = Uppercase extension
settings_bad_names_emoji = Emoji in name
settings_bad_names_space = Spaces at start/end
settings_bad_names_non_ascii = Non-ASCII characters
settings_bad_names_duplicated = Repeated characters
settings_ignore_same_resolution = Ignore images with the same resolution

# Settings - Appearance section
settings_appearance_label = APPEARANCE
settings_dark_theme = Dark theme
settings_dark_theme_desc = Use dark colour scheme

# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTICS
diagnostics_thumbnails = Thumbnail cache
diagnostics_app_cache = App cache
diagnostics_refresh = Refresh
diagnostics_clear_thumbnails = Clear thumbnails
diagnostics_open_thumbnails_folder = Open folder
diagnostics_clear_cache = Clear cache
diagnostics_open_cache_folder = Open folder
diagnostics_collect_test = File access test
diagnostics_collect_test_desc = Check how many files are accessible
diagnostics_collect_test_run = Run
diagnostics_collect_test_stop = Stop
collect_test_cancelled = Stopped by user
diag_confirm_clear_thumbnails = Clear all thumbnail cache?
diag_confirm_clear_cache = Clear all app cache?
about_repo = Repository
about_translate = Translations
about_donate = Support

# Collect-test result popup
collect_test_title = Test results
collect_test_volumes = Volumes:
collect_test_folders = Folders:
collect_test_files = Files:
collect_test_time = Time:

# Licenses
licenses_label = LICENSE
third_party_licenses = Third-party licenses
licenses_popup_title = Third-party Licenses

# Directories screen
directories_include_header = Include
directories_included = Included
directories_exclude_header = Exclude
directories_excluded_header = Excluded
directories_add = Include
no_paths = No paths - add below
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
and_more_prefix = ...and
and_more_suffix = more

# Gallery / delete popups
gallery_delete_button = Delete
gallery_back = Back
gallery_confirm_delete = Yes, delete
deleting_files = Deleting files...
stop = Stop
files_suffix = files
scanning_fallback = Scanning...
app_subtitle = In honour of the Battle of Cedynia (972 CE)
app_license = Frontend for Czkawka Core - GPL-3.0
about_app_label = ABOUT
cache_label = CACHE

# Notification
scan_completed_notification = Scan completed - { $file_count } items found

# Confirm popups (set from Rust)
confirm_clean_exif = Are you sure you want to clean EXIF tags from { $n } selected files?
confirm_delete_items = Are you sure you want to delete { $n } selected items?
gallery_confirm_delete_msg = You are about to delete { $total_images } images in { $total_groups } groups.
gallery_confirm_delete_warning = All items are selected in { $unsafe_groups } groups!

# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Calculating and comparing audio fingerprints is very resource-intensive and may take a long time. It is recommended to use Krokiet on a desktop system for this task.

# Scan stage labels (shown during scan progress)
stage_collecting_files = Collecting files
stage_scanning_name = Scanning by name
stage_scanning_size_name = Scanning by name and size
stage_scanning_size = Scanning by size
stage_pre_hash = Pre-hashing
stage_full_hash = Hashing
stage_loading_cache = Loading cache
stage_saving_cache = Saving cache
stage_calculating_image_hashes = Calculating image hashes
stage_comparing_images = Comparing images
stage_calculating_video_hashes = Calculating video hashes
stage_checking_files = Checking files
stage_checking_extensions = Checking extensions
stage_checking_names = Checking names
stage_reading_music_tags = Reading music tags
stage_comparing_tags = Comparing tags
stage_calculating_music_fingerprints = Calculating music fingerprints
stage_comparing_fingerprints = Comparing fingerprints
stage_extracting_exif = Reading EXIF tags
stage_creating_video_thumbnails = Creating video thumbnails
stage_processing_videos = Processing videos
stage_deleting = Deleting files
stage_renaming = Renaming files
stage_moving = Moving files
stage_hardlinking = Creating hard links
stage_symlinking = Creating symlinks
stage_optimizing_videos = Optimizing videos
stage_cleaning_exif = Cleaning EXIF
stage_all_hiding_links = Hiding hard links
stage_empty_files_checking_content = Checking file content

# Group headers in scan results
duplicates_group_header = { $count } files  x  { $per_file } / file  =  { $total } total
similar_images_group_header = { $count } similar images
same_music_group_header = { $count } similar tracks
similar_videos_group_header = { $count } similar videos

# Rename confirmation
confirm_rename_items = Are you sure you want to rename { $n } selected files?

# Combo-box option labels (translatable display names)
option_search_mode_biggest = Biggest
option_search_mode_smallest = Smallest
option_similarity_very_high = V.High
option_similarity_high = High
option_similarity_medium = Medium
option_similarity_low = Low
option_similarity_very_low = V.Low
option_similarity_minimal = Min.
option_check_method_hash = Hash
option_check_method_name = Name
option_check_method_size_and_name = Size+Name
option_check_method_size = Size
option_music_method_tags = Tags
option_music_method_audio = Audio
option_min_size_none = None
option_max_size_unlimited = Unlimited
option_audio_preset_identical = Identical
option_audio_preset_clip = Clip in longer
option_audio_preset_similar = Similar

# Volume labels (shown in the directories screen)
volume_internal_storage = Internal Storage
volume_sd_card = Memory Card (SD Card)
volume_storage = Storage Volume

# Directories screen
directories_referenced_tooltip = Referenced (not deleted)
directories_include_section_header = INCLUDED
directories_exclude_section_header = EXCLUDED
directories_custom_paths = Custom Paths
directories_check_button = Analyze
directories_check_popup_title = Directory Statistics
directories_check_label_included = Included paths:
directories_check_label_excluded = Excluded paths:
directories_check_label_referenced = Reference paths:
directories_check_label_would_scan = Files to scan:
directories_check_label_processable = Processable files:
directories_check_scanning = Scanning...
directories_check_warning_no_processable = No processable files found - verify your included/excluded folders
path_edit_title_include = Add to Include
path_edit_title_exclude = Add to Exclude
path_edit_placeholder = Enter path...
path_edit_not_exists = Path does not exist
path_edit_is_dir = Directory
path_edit_is_file = File
path_edit_no_newlines = Paths cannot contain newlines — Enter key is not allowed

ctx_menu_title = Open
ctx_open_file = Open item
ctx_open_folder = Open parent folder
dir_open_folder = Open folder

# Compare view
compare_label = Compare
compare_loading = Loading images…
compare_cancelling = Cancelling…
compare_computing = Computing diff…
compare_mode_normal = Side
compare_mode_split = Split
compare_mode_overlay = Overlay
compare_mode_diff = Diff
compare_res_mismatch = Different resolutions – diff may be inaccurate

