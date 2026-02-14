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

core_cannot_start_scan_no_included_paths = Cannot start scan, because there are no included paths
core_skip_exist_check_all_included_paths_nonexistent = Cannot start scan, because all included paths do not exist
core_missing_no_chosen_included_path = No valid included path was chosen(excluded paths could have excluded all included paths)
core_reference_included_paths_same = Cannot start scan where all valid included paths are also referenced paths, try to validate or disable referenced paths
core_path_must_exists = Provided path must exist, ignoring { $path }
core_must_be_directory_or_file = Provided path must point to a vaild directory or file, ignoring { $path }
core_excluded_paths_pointless_slash = Excluding / is pointless, because it means no files will be scanned
core_paths_unable_to_get_device_id = Unable to get device id from folder { $path }

core_needs_allowed_extensions_limited_by_tool = Cannot start scan, when all extensions available in this tool ({ $extensions }) were excluded from scan
core_needs_allowed_extensions = Cannot start scan, when all extensions were excluded from scan
core_needs_to_set_at_least_one_broken_option = Cannot start scan, when there is no broken option set to scan for
core_needs_to_set_at_least_one_bad_name_option = Cannot start scan, when there is no bad name option set to scan for

core_ffmpeg_not_found = Cannot find a proper installation of FFmpeg or FFprobe. These are external programs that must be installed manually.
core_ffmpeg_not_found_windows = Be sure that ffmpeg.exe and ffprobe.exe are available in PATH or are placed directly in the same folder as the app executable

core_invalid_symlink_infinite_recursion = Infinite recursion
core_invalid_symlink_non_existent_destination = Non-existent destination file

core_messages_limit_reached_characters = Number of messages exceeded the set limit ({$current}/{$limit} characters), so the output was truncated. To read the full output, disable the limiting option in settings.
core_messages_limit_reached_lines = Number of messages exceeded the set limit ({$current}/{$limit} lines), so the output was truncated. To read the full output, disable the limiting option in settings.

core_error_moving_to_trash = Error while moving "{ $file }" to the trash: { $error }
core_error_removing = Error while removing "{ $file }": { $error }

core_no_similarity_method_selected = Cannot find similar music files without a selected similarity method

core_failed_to_spawn_command = Failed to spawn command: { $reason }
core_failed_to_check_process_status = Failed to check process status: { $reason }
core_failed_to_wait_for_process = Failed to wait for process: { $reason }
core_failed_to_read_video_properties = Failed to read video properties: { $reason }
core_failed_to_execute_ffmpeg = Failed to execute ffmpeg: { $reason }
core_ffmpeg_failed_with_status = ffmpeg failed with status { $status }: { $stderr } (command: { $command })
core_failed_to_load_image_frame = Failed to load image frame: { $reason }
core_failed_to_extract_frame = Failed to extract frame at { $time } seconds from "{ $file }": { $reason }
core_failed_to_save_thumbnail = Failed to save thumbnail for "{ $file }": { $reason }
core_failed_get_frame_at_timestamp = Failed to get frame at timestamp { $timestamp } from "{ $file }": { $reason }
core_failed_get_frame_from_file = Failed to get frame from "{ $file }" at timestamp { $timestamp }: { $reason }
core_invalid_crop_rectangle = Invalid crop rectangle: left={ $left }, top={ $top }, right={ $right }, bottom={ $bottom }
core_failed_to_crop_video_file = Failed to crop video file "{ $file }": { $reason }
core_cropped_video_not_created = Cropped video file was not created: { $temp }
core_unable_check_hash_of_file = Unable to check hash of file "{ $file }", reason { $reason }
core_error_checking_hash_of_file = Error happened when checking hash of file "{ $file }", reason { $reason }
core_image_zero_dimensions = Image has zero width or height "{ $path }"
core_image_open_failed = Cannot open image file "{ $path }": { $reason }
core_not_directory_remove = Trying to remove folder "{ $path }" which is not a directory
core_cannot_read_directory = Cannot read directory "{ $path }"
core_cannot_read_entry_from_directory = Cannot read entry from directory "{ $path }"
core_folder_contains_file_inside = Folder contains file "{ $entry }" inside "{ $folder }"
core_unknown_directory_entry = Unable to determine file type of directory entry "{ $entry }" inside "{ $path }"
core_video_width_exceeds_limit = Video width { $width } exceeds the limit of { $limit }
core_video_height_exceeds_limit = Video height { $height } exceeds the limit of { $limit }
core_failed_to_process_video = Failed to process video file { $file }: { $reason }
core_optimized_file_larger = Optimized file { $optimized } (size: { $new_size }) is not smaller than original { $original } (size: { $original_size })
core_unknown_codec = Unknown codec: { $codec }
core_invalid_video_optimizer_mode = Invalid video optimizer mode: '{ $mode }'. Allowed values: transcode, crop
core_folder_does_not_exist = Folder does not exist: { $folder }
core_path_not_directory = Path is not a directory: { $folder }
core_test_error_for_folder = Test error for folder: { $folder }
core_unknown_exif_tag_group = Unknown EXIF tag group: { $tag }
core_error_comparing_fingerprints = Error while comparing fingerprints: { $reason }
core_failed_to_generate_thumbnail_frames_different_dimensions = Failed to generate thumbnail for "{ $file }": extracted frames have different dimensions
core_failed_to_generate_thumbnail = Failed to generate thumbnail for "{ $file }": { $reason }
core_failed_to_extract_frame_at_seek_time = Failed to extract frame at { $time } seconds from "{ $file }": { $reason }
core_video_file_does_not_exist = Video file does not exist (could be removed between scan/later steps): "{ $path }"
core_image_too_large = Image is too large ({ $width }x{ $height }) - more than supported { $max } pixels
core_failed_to_get_video_metadata = Failed to get video metadata for file "{ $file }": { $reason }
core_failed_to_get_video_codec = Failed to get video codec for file "{ $file }"
core_failed_to_get_video_duration = Failed to get video duration for file "{ $file }"
core_failed_to_get_video_dimensions = Failed to get video dimensions for file "{ $file }"
core_frame_dimensions_mismatch = Frame dimensions for timestamp { $timestamp } do not match the first frame dimensions ({ $first_w }x{ $first_h })
core_failed_to_load_data_from_cache = Failed to load data from cache file { $file }, reason { $reason }
core_failed_to_load_data_from_json_cache = Failed to load data from json cache file { $file }, reason { $reason }
core_failed_to_replace_with_optimized = Failed to replace file "{ $file }" with optimized version: { $reason }
core_failed_to_write_data_to_cache = Cannot write data to cache file "{ $file }", reason { $reason }
core_properly_saved_cache_entries = Properly saved to file { $count } cache entries.
core_video_processing_stopped_by_user = Video processing was stopped by user
core_thumbnail_generation_stopped_by_user = Thumbnail generation was stopped by user
core_failed_to_optimize_video = Failed to optimize video "{ $file }": { $reason }
core_failed_to_crop_video = Failed to crop video "{ $file }": { $reason }
core_failed_to_get_metadata_of_optimized_file = Failed to get metadata of optimized file "{ $file }": { $reason }
core_cannot_create_config_folder = Cannot create config folder "{ $folder }", reason { $reason }
core_cannot_create_cache_folder = Cannot create cache folder "{ $folder }", reason { $reason }
core_cannot_create_or_open_cache_file = Cannot create or open cache file "{ $file }", reason { $reason }
core_cannot_set_config_cache_path = Cannot set config/cache path - config and cache will not be used.
core_invalid_extension_contains_space = { $extension } is not a valid extension because it contains empty space inside
core_invalid_extension_contains_dot = { $extension } is not a valid extension because it contains dot inside
core_ffmpeg_unknown_encoder = Cannot encode { $file } using the { $encoder } encoder. The current FFmpeg build does not support this encoder. Use a different FFmpeg version with the required codec support or select another encoder.
core_ffmpeg_error = FFmpeg error while processing { $file }, status code { $code }, reason { $reason }