# Window titles
window_settings_title = Settings
window_main_title = Czkawka (Hiccup)
window_progress_title = Scanning
window_compare_images = Compare Images

# General
general_ok_button = Ok
general_close_button = Close

# Main window
music_title_checkbox = Title
music_artist_checkbox = Artist
music_year_checkbox = Year
music_bitrate_checkbox = Bitrate
music_genre_checkbox = Genre
music_length_checkbox = Length
music_comparison_checkbox = Approximate Comparison
music_checking_by_tags = Tags
music_checking_by_content = Content
same_music_seconds_label = Minimal fragment second duration
same_music_similarity_label = Maximum difference

music_compare_only_in_title_group = Compare only in title
music_compare_only_in_title_group_tooltip =
        When enabled, files are grouped by title and then compared to each other.

        With 10000 files, instead almost 100 million comparisons usually there will be around 20000 comparisons.

same_music_tooltip =
        Searching for similar music files by its content can be configured by setting:

        - The minimum fragment time after which music files can be identified as similar
        - The maximum difference difference between two tested fragments

        The key to good results is to find sensible combinations of these parameters, for provided.

        Setting the minimum time to 5s and the maximum difference to 1.0, will look for almost identical fragments in the files.
        A time of 20s and a maximum difference of 6.0, on the other hand, works well for finding remixes/live versions etc.

        By default, each music file is compared to each other and this can take a lot of time when testing many files, so it is usually better to use reference folders and specifying which files are to be compared with each other(with same amount of files, comparing fingerprints will be faster at least 4x than without reference folders).

music_comparison_checkbox_tooltip =
        It searches for similar music files using AI, which uses machine learning to remove parentheses from a phrase. For example, with this option enabled, the files in question will be considered duplicates:
        
        Świędziżłób     ---     Świędziżłób (Remix Lato 2021)

duplicate_case_sensitive_name = Case Sensitive
duplicate_case_sensitive_name_tooltip =
        When enabled, group only records when they have exactly same name e.g. Żołd <-> Żołd

        Disabling such option will group names without checking if each letter is same size e.g. żoŁD <-> Żołd

duplicate_mode_size_name_combo_box = Size and Name
duplicate_mode_name_combo_box = Name
duplicate_mode_size_combo_box = Size
duplicate_mode_hash_combo_box = Hash

duplicate_hash_type_tooltip = 
        Czkawka offers 3 types of hashes:

        Blake3 - cryptographic hash function. This is the default because it is very fast.

        CRC32 - simple hash function. This should be faster than Blake3, but may very rarely have some collisions.

        XXH3 - very similar in performance and hash quality to Blake3 (but non-cryptographic). So, such modes can be easily interchanged.

duplicate_check_method_tooltip = 
        For now, Czkawka offers three types of method to find duplicates by:

        Name - Finds files which have the same name.

        Size - Finds files which have the same size.

        Hash - Finds files which have the same content. This mode hashes the file and later compares this hash to find duplicates. This mode is the safest way to find duplicates. App heavily uses cache, so second and further scans of the same data should be a lot of faster than the first. 

image_hash_size_tooltip =
        Each checked image produces a special hash which can be compared with each other, and a small difference between them means that these images are similar.

        8 hash size is quite good to find images that are only a little similar to original. With a bigger set of images (>1000), this will produce a big amount of false positives, so I recommend to use  a bigger hash size in this case.

        16 is the default hash size which is quite a good compromise between finding even a little similar images and having only a small amount of hash collisions.

        32 and 64 hashes find only very similar images, but should have almost no false positives (maybe except some images with alpha channel).

image_resize_filter_tooltip = 
        To compute hash of image, the library must first resize it.

        Depend on chosen algorithm, the resulting image used to calculate hash will looks a little different.

        The fastest algorithm to use, but also the one which gives the worst results, is Nearest. It is enabled by default, because with 16x16 hash size lower quality it is not really visible.

        With 8x8 hash size it is recommended to use a different algorithm than Nearest, to have better groups of images.

image_hash_alg_tooltip = 
        Users can choose from one of many algorithms of calculating the hash.

        Each has both strong and weaker points and will sometimes give better and sometimes worse results for different images.

        So, to determine the best one for you, manual testing is required.

big_files_mode_combobox_tooltip = Allows to search for smallest/biggest files
big_files_mode_label = Checked files
big_files_mode_smallest_combo_box = The Smallest
big_files_mode_biggest_combo_box = The Biggest

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
main_notebook_bad_extensions = Bad Extensions

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
main_tree_view_column_bitrate = Bitrate
main_tree_view_column_length = Length
main_tree_view_column_genre = Genre
main_tree_view_column_symlink_file_name = Symlink File Name
main_tree_view_column_symlink_folder = Symlink Folder
main_tree_view_column_destination_path = Destination Path
main_tree_view_column_type_of_error = Type Of Error
main_tree_view_column_current_extension = Current Extension
main_tree_view_column_proper_extensions = Proper Extension

main_label_check_method = Check method
main_label_hash_type = Hash type
main_label_hash_size = Hash size
main_label_size_bytes = Size (bytes)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Number of shown files
main_label_resize_algorithm = Resize algorithm
main_label_similarity = Similarity{"   "}

main_check_box_broken_files_audio = Audio
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Archive
main_check_box_broken_files_image = Image

check_button_general_same_size = Ignore same size
check_button_general_same_size_tooltip = Ignore files with identical size in results - usually these are 1:1 duplicates

main_label_size_bytes_tooltip = Size of files which will be used in scan

# Upper window
upper_tree_view_included_folder_column_title = Folders to Search
upper_tree_view_included_reference_column_title = Reference Folders

upper_recursive_button = Recursive
upper_recursive_button_tooltip = If selected, search also for files which are not placed directly under chosen folders.

upper_manual_add_included_button = Manual Add
upper_add_included_button = Add
upper_remove_included_button = Remove
upper_manual_add_excluded_button = Manual Add
upper_add_excluded_button = Add
upper_remove_excluded_button =  Remove

upper_manual_add_included_button_tooltip =
        Add directory name to search by hand.

        To add multiple paths at once, separate them by ;

        /home/roman;/home/rozkaz will add two directories /home/roman and /home/rozkaz
upper_add_included_button_tooltip = Add new directory to search.
upper_remove_included_button_tooltip =  Delete directory from search.
upper_manual_add_excluded_button_tooltip =
        Add excluded directory name by hand.

        To add multiple paths at once, separate them by ;

        /home/roman;/home/krokiet will add two directories /home/roman and /home/keokiet
upper_add_excluded_button_tooltip = Add directory to be excluded in search.
upper_remove_excluded_button_tooltip = Delete directory from excluded.

upper_notebook_items_configuration = Items Configuration
upper_notebook_excluded_directories = Excluded Directories
upper_notebook_included_directories = Included Directories

upper_allowed_extensions_tooltip = 
        Allowed extensions must be separated by commas (by default all are available).

        The following Macros, which add multiple extensions at once, are also available: IMAGE, VIDEO, MUSIC, TEXT.

        Usage example  ".exe, IMAGE, VIDEO, .rar, 7z" - this means that images (e.g. jpg, png), videos (e.g. avi, mp4), exe, rar, and 7z files will be scanned.

upper_excluded_extensions_tooltip =
        List of disabled files which will be ignored in scan.

        When using both allowed and disabled extensions, this one has higher priority, so file will not be checked.

upper_excluded_items_tooltip = 
        Excluded items must contain * wildcard and should be separated by commas.
        This is slower than Excluded Directories, so use it carefully.

upper_excluded_items = Excluded Items:
upper_allowed_extensions = Allowed Extensions:
upper_excluded_extensions = Disabled Extensions:


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
        Select records by path.

        Example usage:
        /home/pimpek/rzecz.txt can be found with /home/pim*

popover_custom_name_check_button_entry_tooltip = 
        Select records by file names.

        Example usage:
        /usr/ping/pong.txt can be found with *ong*

popover_custom_regex_check_button_entry_tooltip = 
        Select records by specified Regex.

        With this mode, searched text is Path with Name.

        Example usage:
        /usr/bin/ziemniak.txt can be found with /ziem[a-z]+

        This uses the default Rust regex implementation. You can read more about it here: https://docs.rs/regex.

popover_custom_case_sensitive_check_button_tooltip =
        Enables case-sensitive detection.

        When disabled /home/* finds both /HoMe/roman and /home/roman.

popover_custom_not_all_check_button_tooltip = 
        Prevents selecting all records in group.

        This is enabled by default, because in most situations, you don't want to delete both original and duplicates files, but want to leave at least one file.

        WARNING: This setting doesn't work if you have already manually selected all results in a group.

popover_custom_regex_path_label = Path
popover_custom_regex_name_label = Name
popover_custom_regex_regex_label = Regex Path + Name
popover_custom_case_sensitive_check_button = Case sensitive
popover_custom_all_in_group_label = Don't select all records in group

popover_custom_mode_unselect = Unselect Custom
popover_custom_mode_select = Select Custom

popover_sort_file_name = File name
popover_sort_folder_name = Folder name
popover_sort_full_name = Full name
popover_sort_size = Size
popover_sort_selection = Selection

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
bottom_sort_button = Sort
bottom_compare_button = Compare

bottom_search_button_tooltip = Start search
bottom_select_button_tooltip = Select records. Only selected files/folders can be later processed.
bottom_delete_button_tooltip = Delete selected files/folders.
bottom_save_button_tooltip = Save data about search to file
bottom_symlink_button_tooltip = 
        Create symbolic links.
        Only works when at least two results in a group are selected.
        First is unchanged and second and later are symlinked to first.
bottom_hardlink_button_tooltip = 
        Create hardlinks.
        Only works when at least two results in a group are selected.
        First is unchanged and second and later are hardlinked to first.
bottom_hardlink_button_not_available_tooltip =
        Create hardlinks.
        Button is disabled, because hardlinks cannot be created.
        Hardlinks only works with administrator privileges on Windows, so be sure to run app as administrator.
        If app already works with such privileges check for similar issues on Github.
bottom_move_button_tooltip =
        Moves files to chosen directory.
        It copies all files to the directory without preserving the directory tree.
        When trying to move two files with identical name to folder, second will fail and show error.
bottom_sort_button_tooltip =
        Sorts files/folders according to selected method.
bottom_compare_button_tooltip =
        Compare images in the group.

bottom_show_errors_tooltip = Show/Hide bottom text panel.
bottom_show_upper_notebook_tooltip = Show/Hide upper notebook panel.

# Progress Window
progress_stop_button = Stop
progress_stop_additional_message = Stop requested

# About Window
about_repository_button_tooltip = Link to repository page with source code.
about_donation_button_tooltip = Link to donation page.
about_instruction_button_tooltip = Link to instruction page.
about_translation_button_tooltip = Link to Crowdin page with app translations. Officially Polish and English are supported.

about_repository_button = Repository
about_donation_button = Donation
about_instruction_button = Instruction
about_translation_button = Translation

# Header
header_setting_button_tooltip = Opens settings dialog.
header_about_button_tooltip = Opens dialog with info about app.

# Settings
## General
settings_number_of_threads = Number of used threads
settings_number_of_threads_tooltip = Number of used threads, 0 means that all available threads will be used.

settings_use_rust_preview = Use external libraries instead gtk to load previews
settings_use_rust_preview_tooltip =
        Using gtk previews will sometimes be faster and support more formats, but sometimes this could be exactly the opposite.

        If you have problems with loading previews, you may can to try to change this setting.

        On non-linux systems, it is recommended to use this option, because gtk-pixbuf are not always available there so disabling this option will not load previews of some images.

settings_label_restart = You need to restart app to apply settings!

settings_ignore_other_filesystems = Ignore other filesystems (only Linux)
settings_ignore_other_filesystems_tooltip =
        ignores files that are not in the same file system as searched directories.

        Works same like -xdev option in find command on Linux

settings_save_at_exit_button_tooltip = Save configuration to file when closing app.
settings_load_at_start_button_tooltip = 
        Load configuration from file when opening app.

        If not enabled, default settings will be used.
settings_confirm_deletion_button_tooltip = Show confirmation dialog when clicking the delete button.
settings_confirm_link_button_tooltip = Show confirmation dialog when clicking the hard/symlink button.
settings_confirm_group_deletion_button_tooltip = Show warning dialog when trying to delete all records from the group.
settings_show_text_view_button_tooltip = Show text panel at the bottom of the user interface.
settings_use_cache_button_tooltip = Use file cache.
settings_save_also_as_json_button_tooltip = Save cache to (human readable) JSON format. It is possible to modify its content. Cache from this file will be read automatically by app if binary format cache (with bin extension) is missing.
settings_use_trash_button_tooltip = Moves files to trash instead deleting them permanently.
settings_language_label_tooltip = Language for user interface.

settings_save_at_exit_button = Save configuration when closing app
settings_load_at_start_button = Load configuration when opening app
settings_confirm_deletion_button = Show confirm dialog when deleting any files
settings_confirm_link_button = Show confirm dialog when hard/symlinks any files
settings_confirm_group_deletion_button = Show confirm dialog when deleting all files in group
settings_show_text_view_button = Show bottom text panel
settings_use_cache_button = Use cache
settings_save_also_as_json_button = Also save cache as JSON file
settings_use_trash_button = Move deleted files to trash
settings_language_label = Language

settings_multiple_delete_outdated_cache_checkbutton = Delete outdated cache entries automatically
settings_multiple_delete_outdated_cache_checkbutton_tooltip = 
        Delete outdated cache results which point to non-existent files.

        When enabled, app makes sure when loading records, that all records point to valid files (broken ones are ignored).

        Disabling this will help when scanning files on external drives, so cache entries about them will not be purged in the next scan.

        In the case of having hundred of thousands records in cache, it is suggested to enable this, which will speedup cache loading/saving at start/end of the scan.

settings_notebook_general = General
settings_notebook_duplicates = Duplicates
settings_notebook_images = Similar Images
settings_notebook_videos = Similar Video

## Multiple - settings used in multiple tabs
settings_multiple_image_preview_checkbutton_tooltip = Shows preview at right side (when selecting an image file).
settings_multiple_image_preview_checkbutton = Show image preview

settings_multiple_clear_cache_button_tooltip = 
        Manually clear the cache of outdated entries.
        This should only be used if automatic clearing has been disabled.

settings_multiple_clear_cache_button = Remove outdated results from cache.

## Duplicates
settings_duplicates_hide_hard_link_button_tooltip = 
        Hides all files except one, if all point to the same data (are hardlinked).

        Example: In the case where there are (on disk) seven files which are hardlinked to specific data and one different file with same data but a different inode, then in duplicate finder, only one unique file and one file from hardlinked ones will be shown.

settings_duplicates_minimal_size_entry_tooltip = 
        Set the minimal file size which will be cached.

        Choosing a smaller value will generate more records. This will speedup search, but slowdown cache loading/saving.

settings_duplicates_prehash_checkbutton_tooltip = 
        Enables caching of prehash (a hash computed from a small part of the file) which allows earlier dismissal of non-duplicated results.

        It is disabled by default because it can cause slowdowns in some situations.

        It is highly recommended to use it when scanning hundred of thousands or million files, because it can speedup search by multiple times.

settings_duplicates_prehash_minimal_entry_tooltip = Minimal size of cached entry.

settings_duplicates_hide_hard_link_button = Hide hard links (only Linux and macOS)
settings_duplicates_prehash_checkbutton = Use prehash cache

settings_duplicates_minimal_size_cache_label = Minimal size of files (in bytes) saved to cache
settings_duplicates_minimal_size_cache_prehash_label = Minimal size of files (in bytes) saved to prehash cache

## Saving/Loading settings
settings_saving_button_tooltip = Save the current settings configuration to file.
settings_loading_button_tooltip = Load settings from file and replace the current configuration with them.
settings_reset_button_tooltip = Reset the current configuration to the default one.

settings_saving_button = Save configuration
settings_loading_button = Load configuration
settings_reset_button = Reset configuration

## Opening cache/config folders
settings_folder_cache_open_tooltip = 
        Opens the folder where the cache txt files are stored.

        Modifying the cache files may cause invalid results to be shown. However, modifying path may save time when moving a big amount of files to a different location.

        You can copy these files between computers to save time on scanning again for files (of course if they have similar directory structure).

        In the case of problems with the cache, these files can be removed. The app will automatically regenerate them.

settings_folder_settings_open_tooltip = 
        Opens the folder where the Czkawka config is stored.

        WARNING: Manually modifying the config may break your workflow.

settings_folder_cache_open = Open cache folder
settings_folder_settings_open = Open settings folder

# Compute results
compute_stopped_by_user = Searching was stopped by user

compute_found_duplicates_hash_size = Found { $number_files } duplicates in { $number_groups } groups which took { $size }
compute_found_duplicates_name = Found { $number_files } duplicates in { $number_groups } groups
compute_found_empty_folders = Found { $number_files } empty folders
compute_found_empty_files = Found { $number_files } empty files
compute_found_big_files = Found { $number_files } big files
compute_found_temporary_files = Found { $number_files } temporary files
compute_found_images = Found { $number_files } similar images in { $number_groups } groups
compute_found_videos = Found { $number_files } similar videos in { $number_groups } groups
compute_found_music = Found { $number_files } similar music files in { $number_groups } groups
compute_found_invalid_symlinks = Found { $number_files } invalid symlinks
compute_found_broken_files = Found { $number_files } broken files
compute_found_bad_extensions = Found { $number_files } files with invalid extensions

# Progress window
progress_scanning_general_file = Scanning {$file_number} file

progress_scanning_extension_of_files = Checking extension of {$file_checked}/{$all_files} file
progress_scanning_broken_files = Checking {$file_checked}/{$all_files} file
progress_scanning_video = Hashing of {$file_checked}/{$all_files} video
progress_scanning_image = Hashing of {$file_checked}/{$all_files} image
progress_comparing_image_hashes = Comparing {$file_checked}/{$all_files} image hash
progress_scanning_music_tags_end = Comparing tags of {$file_checked}/{$all_files} music file
progress_scanning_music_tags = Reading tags of {$file_checked}/{$all_files} music file
progress_scanning_music_content_end = Comparing fingerprint of {$file_checked}/{$all_files} music file
progress_scanning_music_content = Calculating fingerprint of {$file_checked}/{$all_files} music file
progress_scanning_empty_folders = Scanning {$folder_number} folder
progress_scanning_size = Scanning size of {$file_number} file
progress_scanning_size_name = Scanning name and size of {$file_number} file
progress_scanning_name = Scanning name of {$file_number} file
progress_analyzed_partial_hash = Analyzed partial hash of {$file_checked}/{$all_files} files
progress_analyzed_full_hash = Analyzed full hash of {$file_checked}/{$all_files} files
progress_prehash_cache_loading = Loading prehash cache
progress_prehash_cache_saving = Saving prehash cache
progress_hash_cache_loading = Loading hash cache
progress_hash_cache_saving = Saving hash cache
progress_cache_loading = Loading cache
progress_cache_saving = Saving cache

progress_current_stage = Current Stage:{"  "}
progress_all_stages = All Stages:{"  "}

# Saving loading 
saving_loading_saving_success = Saved configuration to file { $name }.
saving_loading_saving_failure = Failed to save configuration data to file { $name }.
saving_loading_reset_configuration = Current configuration was cleared.
saving_loading_loading_success = Properly loaded app configuration.

saving_loading_invalid_string = For key "{ $key }" found invalid result - "{ $result }" which is not a string.
saving_loading_invalid_int = For key "{ $key }" found invalid result - "{ $result }" which is not a integer.
saving_loading_invalid_bool = For key "{ $key }" found invalid result - "{ $result }" which is not a bool.
saving_loading_decode_problem_bool = Failed to decode bool from key "{ $key }" found "{ $result }" but allowed values are 0, 1, true or false.
saving_loading_saving_same_keys = Trying to save setting with duplicated key "{ $key }".

saving_loading_failed_to_get_home_directory = Failed to get home directory to open/save config file.
saving_loading_folder_config_instead_file = Cannot create or open save configuration file in path "{ $path }" because already there is a folder.
saving_loading_failed_to_create_configuration_folder = Failed configuration to create configuration folder "{ $path }", reason "{ $reason }".
saving_loading_failed_to_create_config_file = Failed to create config file "{ $path }", reason "{ $reason }".
saving_loading_failed_to_read_config_file = Cannot load configuration from "{ $path }" because it does not exist or is not a file.
saving_loading_failed_to_read_data_from_file = Cannot read data from file "{ $path }", reason "{ $reason }".
saving_loading_orphan_data = Found orphan data "{ $data }" in line "{ $line }".
saving_loading_not_valid = Setting "{ $data }" does not exist in current app version.


# Invalid symlinks
invalid_symlink_infinite_recursion = Infinite recursion
invalid_symlink_non_existent_destination = Non-existent destination file

# Other
selected_all_reference_folders = Cannot start search, when all directories are set as reference folders
searching_for_data = Searching data, it may take a while, please wait...
text_view_messages = MESSAGES
text_view_warnings = WARNINGS
text_view_errors = ERRORS
about_window_motto = This program is free to use and will always be.

# Various dialog
dialogs_ask_next_time = Ask next time

delete_file_failed = Failed to delete file {$name}, reason {$reason}

delete_title_dialog = Delete confirmation
delete_question_label = Are you sure that you want to delete files?
delete_all_files_in_group_title = Confirmation of deleting all files in group
delete_all_files_in_group_label1 = In some groups all records are selected.
delete_all_files_in_group_label2 = Are you sure that you want to delete them?
delete_folder_failed = Failed to delete folder {$dir} because folder doesn't exist, you don't have permission or the folder isn't empty.

delete_items_label = { $items } files will be deleted.
delete_items_groups_label = { $items } files from { $groups } groups will be deleted.

hardlink_failed = Failed to hardlink
hard_sym_invalid_selection_title_dialog = Invalid selection with some groups
hard_sym_invalid_selection_label_1 = In some groups there is only one record selected and it will be ignored.
hard_sym_invalid_selection_label_2 = To be able to hard/sym link these files, at least two results in the group need to be selected.
hard_sym_invalid_selection_label_3 = First in group is recognized as original and is not changed but second and later are modified.
hard_sym_link_title_dialog = Link confirmation
hard_sym_link_label = Are you sure that you want to link these files?

move_folder_failed = Failed to move folder {$name}, reason {$reason}
move_file_failed = Failed to move file {$name}, reason {$reason}
move_files_title_dialog = Choose folder to which you want to move duplicated files
move_files_choose_more_than_1_path = Only one path may be selected to be able to copy their duplicated files, selected {$path_number}.
move_stats = Properly moved {$num_files}/{$all_files} items

save_results_to_file = Saved results both to txt and json files into {$name} folder.

search_not_choosing_any_music = ERROR: You must select at least one checkbox with music searching types.
search_not_choosing_any_broken_files = ERROR: You must select at least one checkbox with type of checked broken files.

include_folders_dialog_title = Folders to include
exclude_folders_dialog_title = Folders to exclude

include_manually_directories_dialog_title = Add directory manually

cache_properly_cleared = Properly cleared cache
cache_clear_duplicates_title = Clearing duplicates cache
cache_clear_similar_images_title = Clearing similar images cache
cache_clear_similar_videos_title = Clearing similar videos cache
cache_clear_message_label_1 = Do you want to clear the cache of outdated entries?
cache_clear_message_label_2 = This operation will remove all cache entries which point to invalid files.
cache_clear_message_label_3 = This may slightly speedup loading/saving to cache.
cache_clear_message_label_4 = WARNING: Operation will remove all cached data from unplugged external drives. So each hash will need to be regenerated.

# Show preview
preview_image_resize_failure = Failed to resize image {$name}.
preview_image_opening_failure = Failed to open image {$name}, reason {$reason}

# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Group { $current_group }/{ $all_groups } ({ $images_in_group } images)
compare_move_left_button = L
compare_move_right_button = R
