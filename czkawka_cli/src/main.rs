use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;

use clap::Parser;
use commands::Commands;
use crossbeam_channel::{Receiver, Sender, unbounded};
use czkawka_core::common::config_cache_path::{print_infos_and_warnings, set_config_cache_path};
use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::image::register_image_decoding_hooks;
use czkawka_core::common::logger::{filtering_messages, print_version_mode, setup_logger};
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::set_number_of_threads;
use czkawka_core::common::tool_data::{CommonData, DeleteMethod};
use czkawka_core::common::traits::{AllTraits, FixingItems, PrintResults, Search};
use czkawka_core::tools::bad_extensions::{BadExtensions, BadExtensionsFixParams, BadExtensionsParameters};
use czkawka_core::tools::bad_names::{BadNames, BadNamesParameters, NameFixerParams, NameIssues};
use czkawka_core::tools::big_file::{BigFile, BigFileParameters, SearchMode};
use czkawka_core::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};
use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};
use czkawka_core::tools::empty_files::EmptyFiles;
use czkawka_core::tools::empty_folder::EmptyFolder;
use czkawka_core::tools::exif_remover::{ExifRemover, ExifRemoverParameters, ExifTagsFixerParams};
use czkawka_core::tools::invalid_symlinks::InvalidSymlinks;
use czkawka_core::tools::same_music::{SameMusic, SameMusicParameters};
use czkawka_core::tools::similar_images::{SimilarImages, SimilarImagesParameters};
use czkawka_core::tools::similar_videos::{SimilarVideos, SimilarVideosParameters};
use czkawka_core::tools::temporary::Temporary;
use czkawka_core::tools::video_optimizer::{
    VideoCropFixParams, VideoCropParams, VideoCroppingMechanism, VideoOptimizer, VideoOptimizerFixParams, VideoOptimizerParameters, VideoTranscodeFixParams, VideoTranscodeParams,
};
use log::{debug, error, info};

use crate::commands::{
    Args, BadExtensionsArgs, BadNamesArgs, BiggestFilesArgs, BrokenFilesArgs, CommonCliItems, DMethod, DuplicatesArgs, EmptyFilesArgs, EmptyFoldersArgs, ExifRemoverArgs,
    InvalidSymlinksArgs, SDMethod, SameMusicArgs, SimilarImagesArgs, SimilarVideosArgs, TemporaryArgs, VideoOptimizerArgs,
};
use crate::progress::connect_progress;

mod commands;
mod progress;

#[derive(Debug)]
pub struct CliOutput {
    pub found_any_files: bool,
    pub ignored_error_code_on_found: bool,
    pub output: String,
}

fn main() {
    register_image_decoding_hooks();
    if cfg!(debug_assertions) {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
    let command = Args::parse().command;

    let config_cache_path_set_result = set_config_cache_path("Czkawka", "Czkawka");
    setup_logger(true, "czkawka_cli", filtering_messages);
    print_version_mode("Czkawka cli");
    print_infos_and_warnings(config_cache_path_set_result.infos, config_cache_path_set_result.warnings);

    if cfg!(debug_assertions) {
        debug!("Running command - {command:?}");
    }

    let (progress_sender, progress_receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
    let stop_flag = Arc::new(AtomicBool::new(false));
    let store_flag_cloned = stop_flag.clone();

    let calculate_thread = thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || match command {
            Commands::Duplicates(duplicates_args) => duplicates(duplicates_args, &stop_flag, &progress_sender),
            Commands::EmptyFolders(empty_folders_args) => empty_folders(empty_folders_args, &stop_flag, &progress_sender),
            Commands::BiggestFiles(biggest_files_args) => biggest_files(biggest_files_args, &stop_flag, &progress_sender),
            Commands::EmptyFiles(empty_files_args) => empty_files(empty_files_args, &stop_flag, &progress_sender),
            Commands::Temporary(temporary_args) => temporary(temporary_args, &stop_flag, &progress_sender),
            Commands::SimilarImages(similar_images_args) => similar_images(similar_images_args, &stop_flag, &progress_sender),
            Commands::SameMusic(same_music_args) => same_music(same_music_args, &stop_flag, &progress_sender),
            Commands::InvalidSymlinks(invalid_symlinks_args) => invalid_symlinks(invalid_symlinks_args, &stop_flag, &progress_sender),
            Commands::BrokenFiles(broken_files_args) => broken_files(broken_files_args, &stop_flag, &progress_sender),
            Commands::SimilarVideos(similar_videos_args) => similar_videos(similar_videos_args, &stop_flag, &progress_sender),
            Commands::BadExtensions(bad_extensions_args) => bad_extensions(bad_extensions_args, &stop_flag, &progress_sender),
            Commands::BadNames(bad_names_args) => bad_names(bad_names_args, &stop_flag, &progress_sender),
            Commands::VideoOptimizer(video_optimizer_args) => video_optimizer(video_optimizer_args, &stop_flag, &progress_sender),
            Commands::ExifRemover(exif_remover_args) => exif_remover(exif_remover_args, &stop_flag, &progress_sender),
        })
        .expect("Failed to spawn calculation thread");

    ctrlc::set_handler(move || {
        if store_flag_cloned.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }
        info!("Got Ctrl+C signal, stopping...");
        store_flag_cloned.store(true, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    connect_progress(&progress_receiver);

    let cli_output = calculate_thread.join().expect("Failed to join calculation thread");

    #[expect(clippy::print_stdout)]
    if !cli_output.output.is_empty() {
        println!("{}", cli_output.output);
    }

    if cli_output.found_any_files && !cli_output.ignored_error_code_on_found {
        std::process::exit(11);
    } else {
        std::process::exit(0);
    }
}

fn duplicates(duplicates: DuplicatesArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let DuplicatesArgs {
        common_cli_items,
        reference_directories,
        minimal_file_size,
        maximal_file_size,
        minimal_cached_file_size,
        search_method,
        delete_method,
        hash_type,
        allow_hard_links,
        case_sensitive_name_comparison,
        minimal_prehash_cache_file_size,
        use_prehash_cache,
    } = duplicates;

    let params = DuplicateFinderParameters::new(
        search_method,
        hash_type,
        use_prehash_cache,
        minimal_cached_file_size,
        minimal_prehash_cache_file_size,
        case_sensitive_name_comparison.case_sensitive_name_comparison,
    );
    let mut tool = DuplicateFinder::new(params);

    set_common_settings(&mut tool, &common_cli_items, Some(reference_directories.reference_directories.as_ref()));
    tool.set_minimal_file_size(minimal_file_size);
    tool.set_maximal_file_size(maximal_file_size);
    tool.set_hide_hard_links(!allow_hard_links.allow_hard_links);
    set_advanced_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn empty_folders(empty_folders: EmptyFoldersArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let EmptyFoldersArgs { common_cli_items, delete_method } = empty_folders;

    let mut tool = EmptyFolder::new();

    set_common_settings(&mut tool, &common_cli_items, None);
    set_simple_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn biggest_files(biggest_files: BiggestFilesArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let BiggestFilesArgs {
        common_cli_items,
        number_of_files,
        delete_method,
        smallest_mode,
    } = biggest_files;

    let big_files_mode = if smallest_mode { SearchMode::SmallestFiles } else { SearchMode::BiggestFiles };
    let params = BigFileParameters::new(number_of_files, big_files_mode);
    let mut tool = BigFile::new(params);

    set_common_settings(&mut tool, &common_cli_items, None);
    set_simple_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn empty_files(empty_files: EmptyFilesArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let EmptyFilesArgs { common_cli_items, delete_method } = empty_files;

    let mut tool = EmptyFiles::new();

    set_common_settings(&mut tool, &common_cli_items, None);
    set_simple_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn temporary(temporary: TemporaryArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let TemporaryArgs { common_cli_items, delete_method } = temporary;

    let mut tool = Temporary::new();

    set_common_settings(&mut tool, &common_cli_items, None);
    set_simple_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn similar_images(similar_images: SimilarImagesArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let SimilarImagesArgs {
        common_cli_items,
        reference_directories,
        minimal_file_size,
        maximal_file_size,
        max_difference,
        hash_alg,
        image_filter,
        hash_size,
        delete_method,
        allow_hard_links,
        ignore_same_size,
    } = similar_images;

    let params = SimilarImagesParameters::new(max_difference, hash_size, hash_alg, image_filter, ignore_same_size.ignore_same_size);
    let mut tool = SimilarImages::new(params);

    set_common_settings(&mut tool, &common_cli_items, Some(reference_directories.reference_directories.as_ref()));
    tool.set_minimal_file_size(minimal_file_size);
    tool.set_maximal_file_size(maximal_file_size);
    tool.set_hide_hard_links(!allow_hard_links.allow_hard_links);
    set_advanced_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn same_music(same_music: SameMusicArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let SameMusicArgs {
        common_cli_items,
        reference_directories,
        delete_method,
        minimal_file_size,
        maximal_file_size,
        music_similarity,
        minimum_segment_duration,
        maximum_difference,
        search_method,
        approximate_comparison,
        compare_fingerprints_only_with_similar_titles,
    } = same_music;

    let params = SameMusicParameters::new(
        music_similarity,
        approximate_comparison,
        search_method,
        minimum_segment_duration,
        maximum_difference,
        compare_fingerprints_only_with_similar_titles,
    );
    let mut tool = SameMusic::new(params);

    set_common_settings(&mut tool, &common_cli_items, Some(reference_directories.reference_directories.as_ref()));
    tool.set_minimal_file_size(minimal_file_size);
    tool.set_maximal_file_size(maximal_file_size);
    set_advanced_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn invalid_symlinks(invalid_symlinks: InvalidSymlinksArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let InvalidSymlinksArgs { common_cli_items, delete_method } = invalid_symlinks;

    let mut tool = InvalidSymlinks::new();

    set_common_settings(&mut tool, &common_cli_items, None);
    set_simple_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn broken_files(broken_files: BrokenFilesArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let BrokenFilesArgs {
        common_cli_items,
        delete_method,
        checked_types,
    } = broken_files;

    let mut checked_type = CheckedTypes::NONE;
    for check_type in checked_types {
        checked_type |= check_type;
    }
    let params = BrokenFilesParameters::new(checked_type);
    let mut tool = BrokenFiles::new(params);

    set_common_settings(&mut tool, &common_cli_items, None);
    set_simple_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn similar_videos(similar_videos: SimilarVideosArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let SimilarVideosArgs {
        reference_directories,
        common_cli_items,
        tolerance,
        minimal_file_size,
        maximal_file_size,
        delete_method,
        allow_hard_links,
        ignore_same_size,
        skip_forward_amount,
        crop_detect,
        scan_duration,
    } = similar_videos;

    let params = SimilarVideosParameters::new(
        tolerance,
        ignore_same_size.ignore_same_size,
        skip_forward_amount,
        scan_duration,
        crop_detect,
        false, // creating thumbnails in CLI, makes almost no sense
        10,    // creating thumbnails in CLI, makes almost no sense
        false, // creating thumbnails in CLI, makes almost no sense
        2,     // creating thumbnails in CLI, makes almost no sense
    );
    let mut tool = SimilarVideos::new(params);

    set_common_settings(&mut tool, &common_cli_items, Some(reference_directories.reference_directories.as_ref()));
    tool.set_minimal_file_size(minimal_file_size);
    tool.set_maximal_file_size(maximal_file_size);
    tool.set_hide_hard_links(!allow_hard_links.allow_hard_links);
    set_advanced_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn bad_extensions(bad_extensions: BadExtensionsArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let BadExtensionsArgs { common_cli_items, fix_extensions } = bad_extensions;

    let params = BadExtensionsParameters::new();
    let mut tool = BadExtensions::new(params);

    set_common_settings(&mut tool, &common_cli_items, None);

    tool.search(stop_flag, Some(progress_sender));

    if fix_extensions {
        let fix_params = BadExtensionsFixParams {};
        tool.fix_items(stop_flag, Some(progress_sender), fix_params);
    }

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn bad_names(bad_names: BadNamesArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let BadNamesArgs {
        common_cli_items,
        delete_method,
        uppercase_extension,
        emoji_used,
        space_at_start_or_end,
        non_ascii_graphical,
        restricted_charset,
        remove_duplicated_non_alphanumeric,
        fix_names,
    } = bad_names;

    let restricted_charset_allowed = restricted_charset.and_then(|s| {
        let mut items: Vec<_> = s.chars().collect();
        items.sort_unstable();
        items.dedup();
        if items.is_empty() { None } else { Some(items) }
    });

    let name_issues = NameIssues {
        uppercase_extension,
        emoji_used,
        space_at_start_or_end,
        non_ascii_graphical,
        restricted_charset_allowed,
        remove_duplicated_non_alphanumeric,
    };

    let params = BadNamesParameters::new(name_issues);
    let mut tool = BadNames::new(params);

    set_common_settings(&mut tool, &common_cli_items, None);
    set_simple_delete(&mut tool, delete_method);

    tool.search(stop_flag, Some(progress_sender));

    if fix_names {
        let fix_params = NameFixerParams::default();
        tool.fix_items(stop_flag, Some(progress_sender), fix_params);
    }

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn video_optimizer(video_optimizer: VideoOptimizerArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    use crate::commands::{CropArgs, TranscodeArgs, VideoOptimizerMode as CliVideoOptimizerMode};

    let VideoOptimizerArgs { common_cli_items, mode } = video_optimizer;

    match mode {
        CliVideoOptimizerMode::Transcode(transcode_args) => {
            let TranscodeArgs {
                excluded_codecs,
                generate_thumbnails,
                thumbnail_percentage,
                thumbnail_grid,
                fix_videos,
                target_codec,
                quality,
                fail_if_not_smaller,
                overwrite_original,
                limit_video_size,
                max_width,
                max_height,
                thumbnail_grid_tiles_per_side,
            } = transcode_args;

            let excluded_codecs_vec = excluded_codecs.map_or_else(
                || vec!["hevc".to_string(), "h265".to_string(), "av1".to_string(), "vp9".to_string()],
                |s| s.split(',').map(|c| c.trim().to_string()).collect(),
            );

            let params = VideoOptimizerParameters::VideoTranscode(VideoTranscodeParams::new(
                excluded_codecs_vec,
                generate_thumbnails,
                thumbnail_percentage,
                thumbnail_grid,
                thumbnail_grid_tiles_per_side,
            ));

            let mut tool = VideoOptimizer::new(params);
            set_common_settings(&mut tool, &common_cli_items, None);
            tool.search(stop_flag, Some(progress_sender));

            if fix_videos {
                let fix_params = VideoOptimizerFixParams::VideoTranscode(VideoTranscodeFixParams {
                    codec: target_codec,
                    quality,
                    fail_if_not_smaller,
                    overwrite_original,
                    limit_video_size,
                    max_width,
                    max_height,
                });
                tool.fix_items(stop_flag, Some(progress_sender), fix_params);
            }

            save_and_write_results_to_writer(&tool, &common_cli_items)
        }
        CliVideoOptimizerMode::Crop(crop_args) => {
            let CropArgs {
                crop_mechanism,
                black_pixel_threshold,
                black_bar_percentage,
                max_samples,
                min_crop_size,
                generate_thumbnails,
                thumbnail_percentage,
                thumbnail_grid,
                thumbnail_grid_tiles_per_side,
                fix_videos,
                overwrite_original,
                target_codec,
                quality,
            } = crop_args;

            #[expect(clippy::match_same_arms)]
            let crop_mech = match crop_mechanism.as_str() {
                "blackbars" => VideoCroppingMechanism::BlackBars,
                "staticcontent" => VideoCroppingMechanism::StaticContent,
                _ => VideoCroppingMechanism::BlackBars,
            };

            let params = VideoOptimizerParameters::VideoCrop(VideoCropParams::with_custom_params(
                crop_mech,
                black_pixel_threshold,
                black_bar_percentage,
                max_samples,
                min_crop_size,
                generate_thumbnails,
                thumbnail_percentage,
                thumbnail_grid,
                thumbnail_grid_tiles_per_side,
            ));

            let mut tool = VideoOptimizer::new(params);
            set_common_settings(&mut tool, &common_cli_items, None);
            tool.search(stop_flag, Some(progress_sender));

            if fix_videos {
                let fix_params = VideoOptimizerFixParams::VideoCrop(VideoCropFixParams {
                    overwrite_original,
                    target_codec,
                    quality,
                    crop_mechanism: crop_mech,
                });
                tool.fix_items(stop_flag, Some(progress_sender), fix_params);
            }

            save_and_write_results_to_writer(&tool, &common_cli_items)
        }
    }
}

fn exif_remover(exif_remover: ExifRemoverArgs, stop_flag: &Arc<AtomicBool>, progress_sender: &Sender<ProgressData>) -> CliOutput {
    let ExifRemoverArgs {
        common_cli_items,
        ignored_tags,
        fix_exif,
        override_file,
    } = exif_remover;

    let ignored_tags_vec = ignored_tags.map(|s| s.split(',').map(|tag| tag.trim().to_string()).collect()).unwrap_or_default();

    let params = ExifRemoverParameters::new(ignored_tags_vec);
    let mut tool = ExifRemover::new(params);

    set_common_settings(&mut tool, &common_cli_items, None);

    tool.search(stop_flag, Some(progress_sender));

    if fix_exif {
        let fix_params = ExifTagsFixerParams { override_file };
        tool.fix_items(stop_flag, Some(progress_sender), fix_params);
    }

    save_and_write_results_to_writer(&tool, &common_cli_items)
}

fn save_and_write_results_to_writer<T: CommonData + PrintResults>(component: &T, common_cli_items: &CommonCliItems) -> CliOutput {
    if let Some(file_name) = common_cli_items.file_to_save.file_name()
        && let Err(e) = component.print_results_to_file(file_name)
    {
        error!("Failed to save results to file {e}");
    }
    if let Some(file_name) = common_cli_items.json_compact_file_to_save.file_name()
        && let Err(e) = component.save_results_to_file_as_json(file_name, false)
    {
        error!("Failed to save compact json results to file {e}");
    }
    if let Some(file_name) = common_cli_items.json_pretty_file_to_save.file_name()
        && let Err(e) = component.save_results_to_file_as_json(file_name, true)
    {
        error!("Failed to save pretty json results to file {e}");
    }

    let mut buf_writer = std::io::BufWriter::new(Vec::new());
    if !common_cli_items.do_not_print.do_not_print_results {
        let _ = component.print_results_to_writer(&mut buf_writer).map_err(|e| {
            error!("Failed to print results to output: {e}");
        });
    }

    if !common_cli_items.do_not_print.do_not_print_messages {
        let _ = component.get_text_messages().print_messages_to_writer(&mut buf_writer).map_err(|e| {
            error!("Failed to print results to output: {e}");
        });
    }

    let mut cli_output = CliOutput {
        found_any_files: component.found_any_items(),
        ignored_error_code_on_found: common_cli_items.ignore_error_code_on_found,
        output: String::new(),
    };

    if let Ok(file_vec) = buf_writer.into_inner()
        && let Ok(output) = String::from_utf8(file_vec)
    {
        cli_output.output = output;
    }

    cli_output
}

fn set_simple_delete<T>(component: &mut T, s_delete: SDMethod)
where
    T: AllTraits,
{
    if s_delete.delete_files {
        component.set_delete_method(DeleteMethod::Delete);
    }
    component.set_dry_run(s_delete.dry_run);
    component.set_move_to_trash(s_delete.move_to_trash);
}

fn set_advanced_delete<T>(component: &mut T, a_delete: DMethod)
where
    T: AllTraits,
{
    component.set_delete_method(a_delete.delete_method);
    component.set_dry_run(a_delete.dry_run);
    component.set_move_to_trash(a_delete.move_to_trash);
}

fn set_common_settings<T>(component: &mut T, common_cli_items: &CommonCliItems, reference_directories: Option<&Vec<PathBuf>>)
where
    T: AllTraits,
{
    set_number_of_threads(common_cli_items.thread_number);

    let mut included_directories = common_cli_items.directories.clone();
    if let Some(reference_directories) = reference_directories {
        included_directories.extend_from_slice(reference_directories);
        component.set_reference_paths(reference_directories.clone());
    }

    component.set_included_paths(included_directories);
    component.set_excluded_paths(common_cli_items.excluded_directories.clone());
    component.set_excluded_items(common_cli_items.excluded_items.clone());
    component.set_recursive_search(!common_cli_items.not_recursive);
    #[cfg(target_family = "unix")]
    component.set_exclude_other_filesystems(common_cli_items.exclude_other_filesystems);
    component.set_allowed_extensions(common_cli_items.allowed_extensions.clone());
    component.set_excluded_extensions(common_cli_items.excluded_extensions.clone());
    component.set_use_cache(!common_cli_items.disable_cache);
}
