# Core
core_similarity_original = Original
core_similarity_very_high = Very High
core_similarity_high = High
core_similarity_medium = Medium
core_similarity_small = Small
core_similarity_very_small = Very Small
core_similarity_minimal = Minimal

core_cannot_open_dir = Cannot open dir {$dir}, reason {$reason}
core_cannot_read_entry_dir = Cannot read entry in dir {$dir}, reason {$reason}
core_cannot_read_metadata_dir = Cannot read metadata in dir {$dir}, reason {$reason}
core_file_modified_before_epoch = File {$name} seems to be modified before Unix Epoch
core_folder_modified_before_epoch = Folder {$name} seems to be modified before Unix Epoch
core_file_no_modification_date = Unable to get modification date from file {$name}, reason {$reason}
core_folder_no_modification_date = Unable to get modification date from folder {$name}, reason {$reason}

core_missing_no_chosen_included_directory = At least one directory must be provided
core_directory_must_exists = Directories:  Provided folder path must exist, ignoring { $path }
core_directory_must_be_directory = Directories: Provided path must point at the directory, ignoring { $path }
core_included_directory_zero_valid_directories = Included Directory ERROR: Not found even one correct path to included which is required
core_excluded_directory_pointless_slash = Directories: Excluding / is pointless, because it means that no files will be scanned
core_directory_overlap = Directories: All directories to search overlaps with excluded directories
core_directory_unable_to_get_device_id = Directories: Unable to get device id from folder { $path }

core_ffmpeg_not_found = Cannot find proper installation of FFmpeg, this is external program, that you need to install manually.
core_ffmpeg_not_found_windows = Be sure that ffmpeg.exe and ffprobe.exe are available in PATH or are put directly to same folder where is app executable

core_invalid_symlink_infinite_recursion = Infinite recursion
core_invalid_symlink_non_existent_destination = Non-existent destination file

core_messages_limit_reached_characters = Number of messages exceeded the set limit ({$current}/{$limit} characters), so the output was truncated. To read the full output, disable the limiting option in settings.
core_messages_limit_reached_lines = Number of messages exceeded the set limit ({$current}/{$limit} lines), so the output was truncated. To read the full output, disable the limiting option in settings.

rust_error_moving_to_trash = Error while moving "{ $file }" to trash: { $error }
rust_error_removing = Error while removing "{ $file }" : { $error }
