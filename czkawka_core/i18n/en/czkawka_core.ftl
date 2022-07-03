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
core_file_not_utf8_name = File {$name} does not have a valid UTF-8 name (some characters may not be shown)
core_file_modified_before_epoch = File {$name} seems to be modified before Unix Epoch
core_folder_modified_before_epoch = Folder {$name} seems to be modified before Unix Epoch
core_file_no_modification_date = Unable to get modification date from file {$name}, reason {$reason}
core_folder_no_modification_date = Unable to get modification date from folder {$name}, reason {$reason}

core_missing_no_chosen_included_directory = At least one directory must be provided
core_directory_wildcard_no_supported = Directories: Wildcards in path are not supported, ignoring { $path }
core_directory_relative_path = Directories: Relative path are not supported, ignoring { $path }
core_directory_must_exists = Directories:  Provided folder path must exist, ignoring { $path }
core_directory_must_be_directory = Directories: Provided path must point at the directory, ignoring { $path }
core_included_directory_zero_valid_directories = Included Directory ERROR: Not found even one correct path to included which is required
core_excluded_directory_pointless_slash = Directories: Excluding / is pointless, because it means that no files will be scanned
core_directory_overlap = Directories: All directories to search overlaps with excluded directories
core_directory_unable_to_get_device_id = Directories: Unable to get device id from folder { $path }

core_ffmpeg_not_found = Cannot find proper installation of FFmpeg
core_ffmpeg_not_found_windows = Be sure that ffmpeg.exe and ffprobe.exe are available in PATH or are put directly to same folder where is app executable
core_ffmpeg_missing_in_snap = Similar Videos don't work currently with snap, if you want help look at - { $url }

core_saving_to_cache = Saved to file { $number } cache entries
core_loading_from_cache = Loaded from cache { $number } entries
