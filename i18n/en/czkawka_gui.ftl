# Core
core_similarity_very_high = Very High
core_similarity_high = High
core_similarity_medium = Medium
core_similarity_small = Small
core_similarity_very_small = Very Small
core_similarity_minimal = Minimal

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

duplicate_mode_name_checkbox = Name
duplicate_mode_size_checkbox = Size
duplicate_mode_hash_checkbox = Hash

duplicate_mode_name_checkbox_tooltip = 
        Finds files which have same name.
  
        This mode not checking what file contain inside, so be carefully when using it.
        
duplicate_mode_size_checkbox_tooltip = 
        Finds files which have same size.

        This mode not checking what file contain inside, so be carefully when using it.
        
duplicate_mode_hash_checkbox_tooltip = 
        Finds files which have the same content.
  
        This mode hashes file and later compare this hashes to find duplicates.
  
        Tool heavily uses cache, so second and further scans of same data should be a lot of faster that first.

duplicate_hash_checkbox_blake3 = Blake3 is cryptographic hash function. It is used as default hash algorithm, because it is very fast.
duplicate_hash_checkbox_crc32 = CRC32 is simple hash function. It should be faster than Blake3, but probably may have very rarely some collisions.
duplicate_hash_checkbox_xxh3 = XXH3 is very similar in case of performance and hash quality to Blake3, so such modes can be easily used.

image_hash_checkbox_8 = Default hash size, with very high similarity it produce quite good results and don't save too much data too cache.
image_hash_checkbox_16 = More precise than 8, so can be used to find very similar pictures, but create bigger cache entries.
image_hash_checkbox_32 = Hash of this size provide very big similarity which is more than enough for most usages.
image_hash_checkbox_64 = Paranoid mode, such tool create really big cache files and will catch almost same images.

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

main_label_check_method = Check method:
main_label_hash_type = Hash type:
main_label_hash_size = Hash size:
main_label_size_bytes = Size(bytes)
main_label_min_size = Min:
main_label_max_size = Max:
main_label_shown_files = Number of shown files:
main_label_resize_algorithm = Resize algorithm:
main_label_similarity = Similarity{"   "}

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
        Macros IMAGE, VIDEO, MUSIC, TEXT which adds multiple extensions at once are also available
        Usage example  ".exe, IMAGE, VIDEO, .rar, 7z" - this means that image(e.g. jpg, png), video(e.g. avi, mp4), exe, rar and 7z files will be scanned

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
header_language_button_tooltip = Use Polish or English language in runtime.

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

settings_save_at_exit_button = Save configuration at exit
settings_load_at_start_button = Load configuration at start
settings_confirm_deletion_button = Show confirm dialog when deleting any files
settings_confirm_link_button = Show confirm dialog when hard/symlinks any files
settings_confirm_group_deletion_button = Show confirm dialog when deleting all files in group
settings_show_text_view_button = Show bottom text panel
settings_use_cache_button = Use cache
settings_use_trash_button = Move deleted files to trash

settings_multiple_delete_outdated_cache_checkbutton = Delete outdated cache entries automatically
settings_multiple_delete_outdated_cache_checkbutton_tooltip = 
        Allows to delete outdated cache results which points to non-existent files.

        When enabled, app make sure when loading records, that all points to valid files and ignore broken ones.

        Disabling this option, will help to scan files on external drives, so cache entries about them will not be purged in next scan.

        In case of having hundred of thousands records in cache, it is suggested to enable this option, to speedup cache loading and saving at start and end of scan.

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
progress_scanned = Scanned
progress_files = file
progress_folders = folders
progress_tags = Reading tags of
progress_hashing = Hashing
progress_checking = Checking
progress_size = size
progress_name = name
progress_analyzed_full_hash = Analyzed full hash of 
progress_analyzed_partial_hash = Analyzed partial hash of 

progress_current_stage = Current Stage:{"  "}
progress_all_stages = All Stages:{"  "}

# Other
searching_for_data = Searching data, it may take a while, please wait...

