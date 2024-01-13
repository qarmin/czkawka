#![allow(clippy::needless_late_init)]

use clap::Parser;
use log::error;

use commands::Commands;
use czkawka_core::bad_extensions::BadExtensions;
use czkawka_core::big_file::{BigFile, SearchMode};
use czkawka_core::broken_files::BrokenFiles;
use czkawka_core::common::{print_version_mode, set_number_of_threads, setup_logger};
use czkawka_core::common_tool::{CommonData, DeleteMethod};
#[allow(unused_imports)] // It is used in release for print_results_to_output().
use czkawka_core::common_traits::*;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::SameMusic;
use czkawka_core::similar_images::{return_similarity_from_similarity_preset, test_image_conversion_speed, SimilarImages};
use czkawka_core::similar_videos::SimilarVideos;
use czkawka_core::temporary::Temporary;

use crate::commands::{
    Args, BadExtensionsArgs, BiggestFilesArgs, BrokenFilesArgs, CommonCliItems, DuplicatesArgs, EmptyFilesArgs, EmptyFoldersArgs, InvalidSymlinksArgs, SameMusicArgs,
    SimilarImagesArgs, SimilarVideosArgs, TemporaryArgs,
};

mod commands;

fn main() {
    let command = Args::parse().command;

    setup_logger(true);
    print_version_mode();

    if cfg!(debug_assertions) {
        println!("{command:?}");
    }

    match command {
        Commands::Duplicates(duplicates_args) => duplicates(duplicates_args),
        Commands::EmptyFolders(empty_folders_args) => empty_folders(empty_folders_args),
        Commands::BiggestFiles(biggest_files_args) => biggest_files(biggest_files_args),
        Commands::EmptyFiles(empty_files_args) => empty_files(empty_files_args),
        Commands::Temporary(temporary_args) => temporary(temporary_args),
        Commands::SimilarImages(similar_images_args) => similar_images(similar_images_args),
        Commands::SameMusic(same_music_args) => same_music(same_music_args),
        Commands::InvalidSymlinks(invalid_symlinks_args) => invalid_symlinks(invalid_symlinks_args),
        Commands::BrokenFiles(broken_files_args) => broken_files(broken_files_args),
        Commands::SimilarVideos(similar_videos_args) => similar_videos(similar_videos_args),
        Commands::BadExtensions(bad_extensions_args) => bad_extensions(bad_extensions_args),
        Commands::Tester {} => {
            test_image_conversion_speed();
        }
    }
}

fn duplicates(duplicates: DuplicatesArgs) {
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
    } = duplicates;

    let mut item = DuplicateFinder::new();

    set_common_settings(&mut item, &common_cli_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_minimal_cache_file_size(minimal_cached_file_size);
    item.set_check_method(search_method);
    item.set_delete_method(delete_method.delete_method);
    item.set_hash_type(hash_type);
    item.set_ignore_hard_links(!allow_hard_links.allow_hard_links);
    item.set_dry_run(dry_run.dry_run);
    item.set_case_sensitive_name_comparison(case_sensitive_name_comparison.case_sensitive_name_comparison);

    item.find_duplicates(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn empty_folders(empty_folders: EmptyFoldersArgs) {
    let EmptyFoldersArgs { common_cli_items, delete_folders } = empty_folders;

    let mut item = EmptyFolder::new();

    set_common_settings(&mut item, &common_cli_items);
    if delete_folders {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_empty_folders(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn biggest_files(biggest_files: BiggestFilesArgs) {
    let BiggestFilesArgs {
        common_cli_items,
        number_of_files,
        delete_files,
        smallest_mode,
    } = biggest_files;

    let mut item = BigFile::new();

    set_common_settings(&mut item, &common_cli_items);
    item.set_number_of_files_to_check(number_of_files);
    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }
    if smallest_mode {
        item.set_search_mode(SearchMode::SmallestFiles);
    }

    item.find_big_files(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn empty_files(empty_files: EmptyFilesArgs) {
    let EmptyFilesArgs { common_cli_items, delete_files } = empty_files;

    let mut item = EmptyFiles::new();

    set_common_settings(&mut item, &common_cli_items);

    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_empty_files(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn temporary(temporary: TemporaryArgs) {
    let TemporaryArgs { common_cli_items, delete_files } = temporary;

    let mut item = Temporary::new();

    set_common_settings(&mut item, &common_cli_items);

    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_temporary_files(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn similar_images(similar_images: SimilarImagesArgs) {
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
    } = similar_images;

    let mut item = SimilarImages::new();

    set_common_settings(&mut item, &common_cli_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_image_filter(image_filter);
    item.set_hash_alg(hash_alg);
    item.set_hash_size(hash_size);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);

    item.set_similarity(return_similarity_from_similarity_preset(&similarity_preset, hash_size));

    item.find_similar_images(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn same_music(same_music: SameMusicArgs) {
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
    } = same_music;

    let mut item = SameMusic::new();

    set_common_settings(&mut item, &common_cli_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_music_similarity(music_similarity);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);
    item.set_minimum_segment_duration(minimum_segment_duration);
    item.set_maximum_difference(maximum_difference);
    item.set_check_type(search_method);

    item.find_same_music(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn invalid_symlinks(invalid_symlinks: InvalidSymlinksArgs) {
    let InvalidSymlinksArgs { common_cli_items, delete_files } = invalid_symlinks;

    let mut item = InvalidSymlinks::new();

    set_common_settings(&mut item, &common_cli_items);
    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_invalid_links(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn broken_files(broken_files: BrokenFilesArgs) {
    let BrokenFilesArgs { common_cli_items, delete_files } = broken_files;

    let mut item = BrokenFiles::new();

    set_common_settings(&mut item, &common_cli_items);

    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_broken_files(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn similar_videos(similar_videos: SimilarVideosArgs) {
    let SimilarVideosArgs {
        common_cli_items,
        tolerance,
        minimal_file_size,
        maximal_file_size,
        delete_method,
        dry_run,
    } = similar_videos;

    let mut item = SimilarVideos::new();

    set_common_settings(&mut item, &common_cli_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_tolerance(tolerance);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);

    item.find_similar_videos(None, None);

    save_and_print_results(&mut item, &common_cli_items);
}

fn bad_extensions(bad_extensions: BadExtensionsArgs) {
    let BadExtensionsArgs { common_cli_items } = bad_extensions;

    let mut item = BadExtensions::new();

    set_common_settings(&mut item, &common_cli_items);

    item.find_bad_extensions_files(None, None);

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
    if !cfg!(debug_assertions) {
        component.print_results_to_output();
    }
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
