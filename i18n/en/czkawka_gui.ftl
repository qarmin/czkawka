# Core
core_similarity_very_high = Very High
core_similarity_high = High
core_similarity_medium = Medium
core_similarity_small = Small
core_similarity_very_small = Very Small
core_similarity_minimal = Minimal

core_cannot_open_dir = Cannot open dir {$dir}, reason {$reason}
core_cannot_read_entry_dir = Cannot read entry in dir {$dir}, reason {$reason}
core_cannot_read_metadata_dir = Cannot read metadata in dir {$dir}, reason {$reason}
core_file_not_utf8_name = File {$name} has not valid UTF-8 name(some characters may not be shown)
core_file_modified_before_epoch = File {$name} seems to be modified before Unix Epoch
core_folder_modified_before_epoch = Folder {$name} seems to be modified before Unix Epoch
core_file_no_modification_date = Unable to get modification date from file {$name}, reason {$reason}
core_folder_no_modification_date = Unable to get modification date from folder {$name}, reason {$reason}

# Window titles
window_settings_title = Options
window_main_title = Czkawka (Hiccup)
window_progress_title = Scanning

# General
general_ok_button = Ok
general_close_button = Close

general_bytes = bytes
general_lost = lost

# Main window
music_title_checkbox = Title
music_artist_checkbox = Artist
music_album_title_checkbox = Album Title
music_album_artist_checkbox = Album Artist
music_year_checkbox = Year
music_comparison_checkbox = Approximate Comparison

music_comparison_checkbox_tooltip =
        It searches for similar music files using AI, which uses machine learning to remove parentheses from a phrase, e.g. with this option enabled, the files in question will be considered duplicates:
        
        Świędziżłób     ---     Świędziżłób (Remix Lato 2021)

duplicate_mode_name_combo_box = Name
duplicate_mode_size_combo_box = Size
duplicate_mode_hash_combo_box = Hash

duplicate_hash_type_tooltip = 
        Czkawka offers 3 types of hashes, which could be used:

        Blake3 - cryptographic hash function. It is used as default hash algorithm, because it is very fast.

        CRC32 - simple hash function. It should be faster than Blake3, but probably may have very rarely some collisions.

        XXH3 - very similar in case of performance and hash quality to Blake3, so such modes can be easily used.

duplicate_check_method_tooltip = 
        For now, Czkawka offers three types of method to find duplicates by:

        Name - Finds files which have same name.

        Size - Finds files which have same size.

        Hash - Finds files which have the same content. This mode hashes file and later compare this hashes to find duplicates. This mode is the safest way to find duplicates. Tool heavily uses cache, so second and further scans of same data should be a lot of faster that first. 

image_hash_size_tooltip = 
        Czkawka offers changing size of generated hash for each images. Bigger hash cause allows to finds images with lower amount of differences between  images, but also it is a little slower to use.
        
        Default value for hash is 8 bytes, which allows to find very similar and different images. 16 and 32 hashes should be used only for nearly identical images. 64 bytes hash shouldn't be used, except situation where really small differences are needed to find

image_resize_filter_tooltip = 
        To compute hash of image, library must first resize it. Depend on choosen algorithm, resulted image will looks little different. The fastest algotithm to use, but also one which gives the worst results is Nearest.

image_hash_alg_tooltip = 
        Users can choose one from many algorithms of calculating hash. Each have both strong and weaker points and will give sometimes better and sometimes worse results for different images, so to choose the best one, manual testing is required.

main_notebook_duplicates = Duplicate Files
main_notebook_empty_directories = Empty Directories
main_notebook_big_files = Big Files
main_notebook_empty_files = Empty Files
main_notebook_temporary = Temporary Files
main_notebook_similar_images = Similar Images
main_notebook_similar_videos = Similar Videos
main_notebook_same_music = Music Duplicates
main_notebook_symlinks = Invalid Symlinks
main_notebook_broken_files = Broken Files

main_tree_view_column_file_name = File Name
main_tree_view_column_folder_name = Folder Name
main_tree_view_column_path = Path
main_tree_view_column_modification = Modification Date
main_tree_view_column_size = Size
main_tree_view_column_similarity = Similarity
main_tree_view_column_dimensions = Dimensions
main_tree_view_column_title = Title
main_tree_view_column_artist = Artist
main_tree_view_column_year = Year
main_tree_view_column_album_title = Album Title
main_tree_view_column_album_artist = Album Artist
main_tree_view_column_symlink_file_name = Symlink File Name
main_tree_view_column_symlink_folder = Symlnik Folder
main_tree_view_column_destination_path = Destination Path
main_tree_view_column_type_of_error = Type Of Error

main_label_check_method = Check method
main_label_hash_type = Hash type
main_label_hash_size = Hash size
main_label_size_bytes = Size(bytes)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Number of shown files
main_label_resize_algorithm = Resize algorithm
main_label_similarity = Similarity{"   "}

check_button_general_same_size = Ignore same size
check_button_general_same_size_tooltip = Ignore from results, files which have identical size - usually this are 1:1 duplicates

main_label_size_bytes_tooltip = Size of files which will be used in scan

# Upper window
upper_recursive_button = Recursive
upper_recursive_button_tooltip = If selected, search also for files which are not placed directly under chosen folders

upper_manual_add_included_button = Manual Add
upper_add_included_button = Add
upper_remove_included_button = Remove
upper_manual_add_excluded_button = Manual Add
upper_add_excluded_button = Add
upper_remove_excluded_button =  Remove

upper_manual_add_included_button_tooltip = Allows to add directory name to search by hand
upper_add_included_button_tooltip = Add new directory to search
upper_remove_included_button_tooltip =  Delete directory from search
upper_manual_add_excluded_button_tooltip = Allows to add excluded directory name by hand
upper_add_excluded_button_tooltip = Add directory to be excluded in search
upper_remove_excluded_button_tooltip = Delete directory from excluded

upper_notebook_items_configuration = Items Configuration
upper_notebook_excluded_directories = Excluded Directories
upper_notebook_included_directories = Included Directories

upper_allowed_extensions_tooltip = 
        Allowed extensions must be separated by commas(by default all are available)

        Macros IMAGE, VIDEO, MUSIC, TEXT which adds multiple extensions at once are also available.

        Usage example  ".exe, IMAGE, VIDEO, .rar, 7z" - this means that image(e.g. jpg, png), video(e.g. avi, mp4), exe, rar and 7z files will be scanned.

upper_excluded_items_tooltip = 
        Excluded items must contains * wildcard and should be separated by commas.
        This is slower than Excluded Directories, so use it carefully.

upper_excluded_items = Excluded Items:
upper_allowed_extensions = Allowed Extensions:


# Popovers
popover_select_all = Select all
popover_unselect_all = Unselect all
popover_reverse = Reverse Selection
popover_select_all_except_oldest = Select all except oldest
popover_select_all_except_newest = Select all except newest
popover_select_one_oldest = Select one oldest
popover_select_one_newest = Select one newest
popover_select_custom = Select custom
popover_unselect_custom = Unselect custom
popover_select_all_images_except_biggest = Select all except biggest
popover_select_all_images_except_smallest = Select all except smallest

popover_custom_path_check_button_entry_tooltip = 
        Allows to select records by its path.

        Example usage:
        /home/pimpek/rzecz.txt can be found with /home/pim*

popover_custom_name_check_button_entry_tooltip = 
        Allows to select records by file names.

        Example usage:
        /usr/ping/pong.txt can be found with *ong*

popover_custom_regex_check_button_entry_tooltip = 
        Allows to select records by specified Regex.

        With this mode, searched text is Path with Name

        Example usage:
        /usr/bin/ziemniak.txt can be found with /ziem[a-z]+

        This use default Rust regex implementation, so you can read more about it in https://docs.rs/regex.

popover_custom_not_all_check_button_tooltip = 
        Prevents from selecting all records in group.

        This is enabled by default, because in most of situations user don't want to delete both original and duplicates files, but want to leave at least one file.

        Warning: This setting don't work if already user selected all results in group manually.

popover_custom_regex_path_label = Path
popover_custom_regex_name_label = Name
popover_custom_regex_regex_label = Regex Path + Name
popover_custom_all_in_group_label = Don't select all records in group

popover_custom_mode_unselect = Unselect Custom
popover_custom_mode_select = Select Custom


popover_invalid_regex = Regex is invalid
popover_valid_regex = Regex is valid

# Bottom buttons
bottom_search_button = Search
bottom_select_button = Select
bottom_delete_button = Delete
bottom_save_button = Save
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Move

bottom_search_button_tooltip = Start to search for files/folders
bottom_select_button_tooltip = Selects records. Only selected files/folders can be later processed.
bottom_delete_button_tooltip = Delete selected files/folders
bottom_save_button_tooltip = Save data about search to file
bottom_symlink_button_tooltip = 
        Creates symbolic links.
        Only works when at least 2 results in group are selected.
        First is unchanged and second and later are symlinked to first.
bottom_hardlink_button_tooltip = 
        Creates hardlinks.
        Only works when at least 2 results in group are selected.
        First is unchanged and second and later are hardlinked to first.
bottom_move_button_tooltip = 
        Moves files to chosen folder.
        It copy all files to folder without preserving directory tree.
        When trying to move 2 files with identical name to folder, second will fail and show error.

bottom_show_errors_tooltip = Show/Hide bottom error panel.
bottom_show_upper_notebook_tooltip = Show/Hide upper notebook panel.

# Progress Window
progress_stop_button = Stop

# About Window
about_repository_button_tooltip = Link to repository page with source code.
about_donation_button_tooltip = Link to donation page.
about_instruction_button_tooltip = Link to instruction page.

about_repository_button = Repository
about_donation_button = Donation
about_instruction_button = Instruction

# Header
header_setting_button_tooltip = Opens settings dialog.
header_about_button_tooltip = Opens dialog with info about app.

# Settings
## General
settings_save_at_exit_button_tooltip = Saves configuration to file when closing app.
settings_load_at_start_button_tooltip = 
        Loading at start configuration from file.

        Not selecting this option will load default settings.
settings_confirm_deletion_button_tooltip = Shows confirmation dialog when clicking at delete button.
settings_confirm_link_button_tooltip = Shows confirmation dialog when clicking at hard/symlink button.
settings_confirm_group_deletion_button_tooltip = Shows dialog when trying to remove all records from group.
settings_show_text_view_button_tooltip = Shows error panel at bottom.
settings_use_cache_button_tooltip = Option to which allows to not use cache feature.
settings_use_trash_button_tooltip = When enabled it moves files to trash instead deleting them permanently.
settings_language_label_tooltip = Allows to choose language of interface from available ones.

settings_save_at_exit_button = Save configuration at exit
settings_load_at_start_button = Load configuration at start
settings_confirm_deletion_button = Show confirm dialog when deleting any files
settings_confirm_link_button = Show confirm dialog when hard/symlinks any files
settings_confirm_group_deletion_button = Show confirm dialog when deleting all files in group
settings_show_text_view_button = Show bottom text panel
settings_use_cache_button = Use cache
settings_use_trash_button = Move deleted files to trash
settings_language_label = Language

settings_multiple_delete_outdated_cache_checkbutton = Delete outdated cache entries automatically
settings_multiple_delete_outdated_cache_checkbutton_tooltip = 
        Allows to delete outdated cache results which points to non-existent files.

        When enabled, app make sure when loading records, that all points to valid files and ignore broken ones.

        Disabling this option, will help to scan files on external drives, so cache entries about them will not be purged in next scan.

        In case of having hundred of thousands records in cache, it is suggested to enable this option, to speedup cache loading and saving at start and end of scan.

settings_notebook_general = General
settings_notebook_duplicates = Duplicates
settings_notebook_images = Similar Images
settings_notebook_videos = Similar Video

## Multiple - settings used in multiple tabs
settings_multiple_delete_outdated_cache_checkbutton = Delete outdated cache entries automatically
settings_multiple_delete_outdated_cache_checkbutton_tooltip = 
        Allows to delete outdated cache results which points to non-existent files.

        When enabled, app make sure when loading records, that all points to valid files and ignore broken ones.

        Disabling this option, will help to scan files on external drives, so cache entries about them will not be purged in next scan.

        In case of having hundred of thousands records in cache, it is suggested to enable this option, to speedup cache loading and saving at start and end of scan.

settings_multiple_image_preview_checkbutton_tooltip = Shows preview at right side, when selecting image file.
settings_multiple_image_preview_checkbutton = Show image preview

settings_multiple_clear_cache_button_tooltip = 
        Manually clear cache from outdated entries.
        Should be used only if automatic clearing was disabled.

settings_multiple_clear_cache_button = Remove outdated results from images cache

## Duplicates
settings_duplicates_hide_hard_link_button_tooltip = 
        Hides all files except one, if are points to same data(are hardlinked).

        E.g. in case where on disk there is 7 files which are hardlinked to specific data and one different file with same data but different inode, then in duplicate finder will be visible only one unique file and one file from hardlinked ones.

settings_duplicates_minimal_size_entry_tooltip = 
        Allows to set minimal size of file, which will be cached.

        Choosing smaller value, will generate more records which will speedup search, but slowdown cache loading/saving.

settings_duplicates_prehash_checkbutton_tooltip = 
        Enables caching of prehash(hash computed from small part of file) which allows to earlier throw out non duplicated results.

        It is disabled by default because can cause in some situations slowdowns.

        It is heavily recommended to use it when scanning hundred of thousands or million files, because it can speedup search multiple times.

settings_duplicates_prehash_minimal_entry_tooltip = Minimal size of cached entry.

settings_duplicates_hide_hard_link_button = Hide hard links(only Linux and MacOS)
settings_duplicates_prehash_checkbutton = Use prehash cache

settings_duplicates_minimal_size_cache_label = Minimal size of files in bytes saved to cache
settings_duplicates_minimal_size_cache_prehash_label = Minimal size of files in bytes saved to prehash cache

## Saving/Loading settings
settings_saving_button_tooltip = Save current settings configuration to file.
settings_loading_button_tooltip = Load settings from file and replace current configuration with them.
settings_reset_button_tooltip = Reset current configuration to default one.

settings_saving_button = Save configuration
settings_loading_button = Load configuration
settings_reset_button = Reset configuration

settings_load_orphan_data = Found invalid header in line {$line_number} \"{$line}\" when loading file {$name} (save file may be from different Czkawka version)
settings_load_invalid_bool_value = Found invalid header in line {$line_number} \"{$line}\" which isn't proper value(0/1/true/false) when loading file {$name}


## Opening cache/config folders
settings_folder_cache_open_tooltip = 
        Opens folder where are stored txt files with cache.

        Modifying them may cause to show invalid results but also modifying e.g. path may save time when moving big amount of files to different place.

        You can copy this files between computers to save time on scanning again for files(of course if they have similar directory structure).

        In case of problems with cache, this files can be removed, so app will automatically regenerate them.

settings_folder_settings_open_tooltip = 
        Opens folder where Czkawka config are stored.

        Modifying them by hand, may cause to break your workflow.

settings_folder_cache_open = Open cache folder
settings_folder_settings_open = Open settings folder

# Compute results
compute_stopped_by_user = Searching was stopped by user

compute_found = Found
compute_duplicated_files_in = duplicated files in
compute_groups_which_took = groups which took
compute_groups = groups
compute_duplicates_for = duplicates for

compute_empty_folders = empty folders
compute_empty_files = empty files
compute_biggest_files = biggest files
compute_temporary_files = temporary files
compute_similar_image = images
compute_similar_videos = videos
compute_music_files = music files
compute_symlinks = invalid symlinks
compute_broken_files = broken files

# Progress window
progress_scanning_general_file = Scanning {$file_number} file

progress_scanning_broken_files = Checking {$file_checked}/{$all_files} file
progress_scanning_video = Hashing of {$file_checked}/{$all_files} video
progress_scanning_image = Hashing of {$file_checked}/{$all_files} image
progress_scanning_music_tags_end = Comparing tags of {$file_checked}/{$all_files} music file
progress_scanning_music_tags = Reading tags of {$file_checked}/{$all_files} music file
progress_scanning_empty_folders = Scanning {$folder_number} folder
progress_scanning_size = Scanning size of {$file_number} file
progress_scanning_name = Scanning name of {$file_number} file
progress_analyzed_partial_hash = Analyzed partial hash of {$file_checked}/{$all_files} files
progress_analyzed_full_hash = Analyzed full hash of {$file_checked}/{$all_files} files

progress_current_stage = Current Stage:{"  "}
progress_all_stages = All Stages:{"  "}

# Saving loading 
saving_loading_saving_success = Saved configuration to file 
saving_loading_reset_configuration = Current configuration was cleared.
saving_loading_loading_success = Properly loaded configuration from file

# Invalid symlinks
invalid_symlink_infinite_recursion = Infinite recursion
invalid_symlink_non_existent_destination = Non existent destination file

# Other
searching_for_data = Searching data, it may take a while, please wait...
text_view_messages = MESSAGES
text_view_warnings = WARNINGS
text_view_errors = ERRORS
about_window_motto = This program is free to use and will always be.

# Various dialog
dialogs_ask_next_time = Ask next time
reason_of_error = reason

delete_file_failed = Failed to remove file {$name}, reason {$reason}

delete_title_dialog = Delete confirmation
delete_question_label = Are you sure that you want to delete files?
delete_all_files_in_group_title = Confirmation of deleting all files in group
delete_all_files_in_group_label1 = In some groups there are selected all records.
delete_all_files_in_group_label2 = Are you sure that you want to delete them?
delete_folder_failed = Failed to remove folder {$dir} because folder doesn't exists, you don't have permissions or isn't empty.

hardlink_failed = Failed to hardlink
hard_sym_invalid_selection_title_dialog = Invalid selection with some groups
hard_sym_invalid_selection_label_1 = In some groups there is only 1 record selected and it will be ignored.
hard_sym_invalid_selection_label_2 = To be able to hard/sym link this files, at least 2 results in group needs to be selected.
hard_sym_invalid_selection_label_3 = First in group is recognized as original and is not changed but second and later are modified.
hard_sym_link_title_dialog = Link confirmation
hard_sym_link_label = Are you sure that you want to link this files?

move_folder_failed = Failed to move folder {$name}, reason {$reason}
move_file_failed = Failed to move file {$name}, reason {$reason}
move_files_title_dialog = Choose folder to which you want to move duplicated files
move_files_choose_more_than_1_path = Only 1 path must be selected to be able to copy there duplicated files, selected {$path_number}
move_stats = Properly moved {$num_files}/{$all_files} items

save_results_to_file = Saved results to file {$name}

search_not_choosing_any_music = ERROR: You must select at least one checkbox with music searching types.

include_folders_dialog_title = Folders to include
exclude_folders_dialog_title = Folders to exclude

include_manually_directories_dialog_title = Add directory manually

cache_properly_cleared = Properly cleared cache
cache_clear_duplicates_title = Clearing duplicates cache
cache_clear_similar_images_title = Clearing similar images cache
cache_clear_similar_videos_title = Clearing similar videos cache
cache_clear_message_label_1 = Do you want to clear cache from outdated entries?
cache_clear_message_label_2 = This operation will remove all cache entries which points to invalid files.
cache_clear_message_label_3 = This may speedup a little loading/saving to cache.
cache_clear_message_label_4 = WARNING: Operation will remove all cached data from unplugged external drives, so hash will need to be generated again.

# Show preview
preview_temporary_file = Failed to open temporary image file {$name}, reason {$reason}
preview_0_size = Cannot create preview of image {$name}, with 0 width or height
preview_temporary_image_save = Failed to save temporary image file to {$name}, reason {$reason}
preview_temporary_image_remove = Failed to delete temporary image file {$name}, reason {$reason}
preview_failed_to_create_cache_dir = Failed to create dir {$name} needed by image preview, reason {$reason}
