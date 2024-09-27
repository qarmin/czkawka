#![allow(clippy::needless_late_init)]
#![warn(clippy::unwrap_used)]

use std::thread;

use clap::Parser;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use log::error;

use commands::Commands;
use czkawka_core::bad_extensions::{BadExtensions, BadExtensionsParameters};
use czkawka_core::big_file::{BigFile, BigFileParameters, SearchMode};
use czkawka_core::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};
use czkawka_core::common::{print_version_mode, set_number_of_threads, setup_logger, DEFAULT_THREAD_SIZE};
use czkawka_core::common_tool::{CommonData, DeleteMethod};
#[allow(unused_imports)] // It is used in release for print_results_to_output().
use czkawka_core::common_traits::*;
use czkawka_core::duplicate::{DuplicateFinder, DuplicateFinderParameters};
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::progress_data::ProgressData;
use czkawka_core::same_music::{SameMusic, SameMusicParameters};
use czkawka_core::similar_images::{return_similarity_from_similarity_preset, test_image_conversion_speed, SimilarImages, SimilarImagesParameters};
use czkawka_core::similar_videos::{SimilarVideos, SimilarVideosParameters};
use czkawka_core::temporary::Temporary;

use crate::commands::{
    Args, BadExtensionsArgs, BiggestFilesArgs, BrokenFilesArgs, CommonCliItems, DuplicatesArgs, EmptyFilesArgs, EmptyFoldersArgs, InvalidSymlinksArgs, SameMusicArgs,
    SimilarImagesArgs, SimilarVideosArgs, TemporaryArgs,
};
use crate::progress::connect_progress;

mod commands;
mod progress;

fn main() {
    let command = Args::parse().command;

    setup_logger(true);
    print_version_mode();

    if cfg!(debug_assertions) {
        println!("{command:?}");
    }

    let (progress_sender, progress_receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
    let (stop_sender, stop_receiver): (Sender<()>, Receiver<()>) = bounded(1);

    let calculate_thread = thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || match command {
            Commands::Duplicates(duplicates_args) => duplicates(duplicates_args, &stop_receiver, &progress_sender),
            Commands::EmptyFolders(empty_folders_args) => empty_folders(empty_folders_args, &stop_receiver, &progress_sender),
            Commands::BiggestFiles(biggest_files_args) => biggest_files(biggest_files_args, &stop_receiver, &progress_sender),
            Commands::EmptyFiles(empty_files_args) => empty_files(empty_files_args, &stop_receiver, &progress_sender),
            Commands::Temporary(temporary_args) => temporary(temporary_args, &stop_receiver, &progress_sender),
            Commands::SimilarImages(similar_images_args) => similar_images(similar_images_args, &stop_receiver, &progress_sender),
            Commands::SameMusic(same_music_args) => same_music(same_music_args, &stop_receiver, &progress_sender),
            Commands::InvalidSymlinks(invalid_symlinks_args) => invalid_symlinks(invalid_symlinks_args, &stop_receiver, &progress_sender),
            Commands::BrokenFiles(broken_files_args) => broken_files(broken_files_args, &stop_receiver, &progress_sender),
            Commands::SimilarVideos(similar_videos_args) => similar_videos(similar_videos_args, &stop_receiver, &progress_sender),
            Commands::BadExtensions(bad_extensions_args) => bad_extensions(bad_extensions_args, &stop_receiver, &progress_sender),
            Commands::Tester {} => test_image_conversion_speed(),
        })
        .expect("Failed to spawn calculation thread");
    ctrlc::set_handler(move || {
        println!("Get Ctrl+C signal, stopping...");
        if let Err(e) = stop_sender.send(()) {
            eprintln!("Failed to send stop signal {e}(it is possible that the program is already stopped)");
        };
    })
    .expect("Error setting Ctrl-C handler");

    connect_progress(&progress_receiver);

    calculate_thread.join().expect("Failed to join calculation thread");
}

fn duplicates(duplicates: DuplicatesArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let DuplicatesArgs {
        common_cli_items,
        minimal_file_size,
        maximal_file_size,
        minimal_cached_file_size,
        search_method,
        delete_method,
        hash_type,
        allow_hard_links,
        dry_run,
        case_sensitive_name_comparison,
        minimal_prehash_cache_file_size,
        use_prehash_cache,
    } = duplicates;

    let params = DuplicateFinderParameters::new(
        search_method,
        hash_type,
        !allow_hard_links.allow_hard_links,
        use_prehash_cache,
        minimal_cached_file_size,
        minimal_prehash_cache_file_size,
        case_sensitive_name_comparison.case_sensitive_name_comparison,
    );
    let mut item = DuplicateFinder::new(params);

    set_common_settings(&mut item, &common_cli_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);

    item.find_duplicates(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn empty_folders(empty_folders: EmptyFoldersArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let EmptyFoldersArgs { common_cli_items, delete_folders } = empty_folders;

    let mut item = EmptyFolder::new();

    set_common_settings(&mut item, &common_cli_items);
    if delete_folders {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_empty_folders(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn biggest_files(biggest_files: BiggestFilesArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let BiggestFilesArgs {
        common_cli_items,
        number_of_files,
        delete_files,
        smallest_mode,
    } = biggest_files;

    let big_files_mode = if smallest_mode { SearchMode::SmallestFiles } else { SearchMode::BiggestFiles };
    let params = BigFileParameters::new(number_of_files, big_files_mode);
    let mut item = BigFile::new(params);

    set_common_settings(&mut item, &common_cli_items);
    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_big_files(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn empty_files(empty_files: EmptyFilesArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let EmptyFilesArgs { common_cli_items, delete_files } = empty_files;

    let mut item = EmptyFiles::new();

    set_common_settings(&mut item, &common_cli_items);
    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_empty_files(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn temporary(temporary: TemporaryArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let TemporaryArgs { common_cli_items, delete_files } = temporary;

    let mut item = Temporary::new();

    set_common_settings(&mut item, &common_cli_items);
    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_temporary_files(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn similar_images(similar_images: SimilarImagesArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let SimilarImagesArgs {
        common_cli_items,
        minimal_file_size,
        maximal_file_size,
        similarity_preset,
        hash_alg,
        image_filter,
        hash_size,
        delete_method,
        dry_run,
        allow_hard_links,
        ignore_same_size,
    } = similar_images;

    let similarity = return_similarity_from_similarity_preset(&similarity_preset, hash_size);
    let params = SimilarImagesParameters::new(
        similarity,
        hash_size,
        hash_alg,
        image_filter,
        ignore_same_size.ignore_same_size,
        !allow_hard_links.allow_hard_links,
    );
    let mut item = SimilarImages::new(params);

    set_common_settings(&mut item, &common_cli_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);

    item.find_similar_images(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn same_music(same_music: SameMusicArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let SameMusicArgs {
        common_cli_items,
        delete_method,
        minimal_file_size,
        maximal_file_size,
        music_similarity,
        dry_run,
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
    let mut item = SameMusic::new(params);

    set_common_settings(&mut item, &common_cli_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);

    item.find_same_music(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn invalid_symlinks(invalid_symlinks: InvalidSymlinksArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let InvalidSymlinksArgs { common_cli_items, delete_files } = invalid_symlinks;

    let mut item = InvalidSymlinks::new();

    set_common_settings(&mut item, &common_cli_items);
    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_invalid_links(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn broken_files(broken_files: BrokenFilesArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let BrokenFilesArgs {
        common_cli_items,
        delete_files,
        checked_types,
    } = broken_files;

    let mut checked_type = CheckedTypes::NONE;
    for check_type in checked_types {
        checked_type |= check_type;
    }
    let params = BrokenFilesParameters::new(checked_type);
    let mut item = BrokenFiles::new(params);

    set_common_settings(&mut item, &common_cli_items);
    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_broken_files(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn similar_videos(similar_videos: SimilarVideosArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let SimilarVideosArgs {
        common_cli_items,
        tolerance,
        minimal_file_size,
        maximal_file_size,
        delete_method,
        dry_run,
        allow_hard_links,
        ignore_same_size,
    } = similar_videos;

    let params = SimilarVideosParameters::new(tolerance, ignore_same_size.ignore_same_size, !allow_hard_links.allow_hard_links);
    let mut item = SimilarVideos::new(params);

    set_common_settings(&mut item, &common_cli_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);

    item.find_similar_videos(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn bad_extensions(bad_extensions: BadExtensionsArgs, stop_receiver: &Receiver<()>, progress_sender: &Sender<ProgressData>) {
    let BadExtensionsArgs { common_cli_items } = bad_extensions;

    let params = BadExtensionsParameters::new();
    let mut item = BadExtensions::new(params);

    set_common_settings(&mut item, &common_cli_items);

    item.find_bad_extensions_files(Some(stop_receiver), Some(progress_sender));

    save_and_print_results(&mut item, &common_cli_items);
}

fn save_and_print_results<T: CommonData + PrintResults>(component: &mut T, common_cli_items: &CommonCliItems) {
    if let Some(file_name) = common_cli_items.file_to_save.file_name() {
        if let Err(e) = component.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }
    if let Some(file_name) = common_cli_items.json_compact_file_to_save.file_name() {
        if let Err(e) = component.save_results_to_file_as_json(file_name, false) {
            error!("Failed to save compact json results to file {e}");
        }
    }
    if let Some(file_name) = common_cli_items.json_pretty_file_to_save.file_name() {
        if let Err(e) = component.save_results_to_file_as_json(file_name, true) {
            error!("Failed to save pretty json results to file {e}");
        }
    }

    component.print_results_to_output();

    component.get_text_messages().print_messages();
}

fn set_common_settings<T>(component: &mut T, common_cli_items: &CommonCliItems)
where
    T: CommonData + PrintResults,
{
    set_number_of_threads(common_cli_items.thread_number);

    component.set_included_directory(common_cli_items.directories.clone());
    component.set_excluded_directory(common_cli_items.excluded_directories.clone());
    component.set_excluded_items(common_cli_items.excluded_items.clone());
    component.set_recursive_search(!common_cli_items.not_recursive);
    #[cfg(target_family = "unix")]
    component.set_exclude_other_filesystems(common_cli_items.exclude_other_filesystems);
    component.set_allowed_extensions(common_cli_items.allowed_extensions.clone().join(","));
}
