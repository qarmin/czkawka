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
core_cannot_read_metadata_file = Cannot read metadata of file {$file}, reason {$reason}
core_file_modified_before_epoch = File {$name} seems to have been modified before the Unix Epoch
core_folder_modified_before_epoch = Folder {$name} seems to have been modified before the Unix Epoch
core_file_no_modification_date = Unable to get modification date from file {$name}, reason {$reason}
core_folder_no_modification_date = Unable to get modification date from folder {$name}, reason {$reason}

core_missing_no_chosen_included_path = No valid included path was chosen(excluded paths could have excluded all included paths)
core_reference_included_paths_same = Cannot start scan where all valid included paths are also referenced paths, try to validate or disable referenced paths
core_path_must_exists = Provided path must exist, ignoring { $path }
core_must_be_directory_or_file = Provided path must point to a vaild directory or file, ignoring { $path }
core_excluded_paths_pointless_slash = Excluding / is pointless, because it means no files will be scanned
core_paths_unable_to_get_device_id = Unable to get device id from folder { $path }

core_needs_allowed_extensions_limited_by_tool = Cannot start scan, when all extensions available in this tool ({ $extensions }) were excluded from scan
core_needs_allowed_extensions = Cannot start scan, when all extensions were excluded from scan
core_needs_to_set_at_least_one_broken_option = Cannot start scan, when there is no broken option set to scan for

core_ffmpeg_not_found = Cannot find a proper installation of FFmpeg or FFprobe. These are external programs that must be installed manually.
core_ffmpeg_not_found_windows = Be sure that ffmpeg.exe and ffprobe.exe are available in PATH or are placed directly in the same folder as the app executable

core_invalid_symlink_infinite_recursion = Infinite recursion
core_invalid_symlink_non_existent_destination = Non-existent destination file

core_messages_limit_reached_characters = Number of messages exceeded the set limit ({$current}/{$limit} characters), so the output was truncated. To read the full output, disable the limiting option in settings.
core_messages_limit_reached_lines = Number of messages exceeded the set limit ({$current}/{$limit} lines), so the output was truncated. To read the full output, disable the limiting option in settings.

core_error_moving_to_trash = Error while moving "{ $file }" to the trash: { $error }
core_error_removing = Error while removing "{ $file }": { $error }
