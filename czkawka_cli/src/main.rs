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
    Args, BadExtensionsArgs, BiggestFilesArgs, BrokenFilesArgs, DuplicatesArgs, EmptyFilesArgs, EmptyFoldersArgs, InvalidSymlinksArgs, SameMusicArgs, SimilarImagesArgs,
    SimilarVideosArgs, TemporaryArgs,
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
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        minimal_file_size,
        maximal_file_size,
        minimal_cached_file_size,
        allowed_extensions,
        search_method,
        delete_method,
        hash_type,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        allow_hard_links,
        dry_run,
        case_sensitive_name_comparison,
    } = duplicates;

    set_number_of_threads(thread_number.thread_number);

    let mut item = DuplicateFinder::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_minimal_cache_file_size(minimal_cached_file_size);
    item.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    item.set_check_method(search_method);
    item.set_delete_method(delete_method.delete_method);
    item.set_hash_type(hash_type);
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    item.set_ignore_hard_links(!allow_hard_links.allow_hard_links);
    item.set_dry_run(dry_run.dry_run);
    item.set_case_sensitive_name_comparison(case_sensitive_name_comparison.case_sensitive_name_comparison);

    item.find_duplicates(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn empty_folders(empty_folders: EmptyFoldersArgs) {
    let EmptyFoldersArgs {
        thread_number,
        directories,
        delete_folders,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        excluded_directories,
        excluded_items,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
    } = empty_folders;

    set_number_of_threads(thread_number.thread_number);

    let mut item = EmptyFolder::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    if delete_folders {
        item.set_delete_method(DeleteMethod::Delete);
    }
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    item.find_empty_folders(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn biggest_files(biggest_files: BiggestFilesArgs) {
    let BiggestFilesArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        allowed_extensions,
        number_of_files,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        delete_files,
        smallest_mode,
    } = biggest_files;

    set_number_of_threads(thread_number.thread_number);

    let mut item = BigFile::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    item.set_number_of_files_to_check(number_of_files);
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }
    if smallest_mode {
        item.set_search_mode(SearchMode::SmallestFiles);
    }

    item.find_big_files(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn empty_files(empty_files: EmptyFilesArgs) {
    let EmptyFilesArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        allowed_extensions,
        delete_files,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
    } = empty_files;

    set_number_of_threads(thread_number.thread_number);

    let mut item = EmptyFiles::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_empty_files(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn temporary(temporary: TemporaryArgs) {
    let TemporaryArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        delete_files,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        not_recursive,
    } = temporary;

    set_number_of_threads(thread_number.thread_number);

    let mut item = Temporary::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_temporary_files(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn similar_images(similar_images: SimilarImagesArgs) {
    let SimilarImagesArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        minimal_file_size,
        maximal_file_size,
        similarity_preset,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        hash_alg,
        image_filter,
        hash_size,
        delete_method,
        dry_run,
    } = similar_images;

    set_number_of_threads(thread_number.thread_number);

    let mut item = SimilarImages::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    item.set_image_filter(image_filter);
    item.set_hash_alg(hash_alg);
    item.set_hash_size(hash_size);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);

    item.set_similarity(return_similarity_from_similarity_preset(&similarity_preset, hash_size));

    item.find_similar_images(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn same_music(same_music: SameMusicArgs) {
    let SameMusicArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        delete_method,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        minimal_file_size,
        maximal_file_size,
        music_similarity,
        dry_run,
        minimum_segment_duration,
        maximum_difference,
        search_method,
    } = same_music;

    set_number_of_threads(thread_number.thread_number);

    let mut item = SameMusic::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    item.set_music_similarity(music_similarity);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);
    item.set_minimum_segment_duration(minimum_segment_duration);
    item.set_maximum_difference(maximum_difference);
    item.set_check_type(search_method);

    item.find_same_music(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn invalid_symlinks(invalid_symlinks: InvalidSymlinksArgs) {
    let InvalidSymlinksArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        allowed_extensions,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        delete_files,
    } = invalid_symlinks;

    set_number_of_threads(thread_number.thread_number);

    let mut item = InvalidSymlinks::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_invalid_links(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn broken_files(broken_files: BrokenFilesArgs) {
    let BrokenFilesArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        allowed_extensions,
        delete_files,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
    } = broken_files;

    set_number_of_threads(thread_number.thread_number);

    let mut item = BrokenFiles::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    if delete_files {
        item.set_delete_method(DeleteMethod::Delete);
    }

    item.find_broken_files(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn similar_videos(similar_videos: SimilarVideosArgs) {
    let SimilarVideosArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        tolerance,
        minimal_file_size,
        maximal_file_size,
        allowed_extensions,
        delete_method,
        dry_run,
    } = similar_videos;

    set_number_of_threads(thread_number.thread_number);

    let mut item = SimilarVideos::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    item.set_minimal_file_size(minimal_file_size);
    item.set_maximal_file_size(maximal_file_size);
    item.set_tolerance(tolerance);
    item.set_delete_method(delete_method.delete_method);
    item.set_dry_run(dry_run.dry_run);

    item.find_similar_videos(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn bad_extensions(bad_extensions: BadExtensionsArgs) {
    let BadExtensionsArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        file_to_save,
        json_compact_file_to_save,
        json_pretty_file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        allowed_extensions,
    } = bad_extensions;

    set_number_of_threads(thread_number.thread_number);

    let mut item = BadExtensions::new();

    item.set_included_directory(directories.directories);
    item.set_excluded_directory(excluded_directories.excluded_directories);
    item.set_excluded_items(excluded_items.excluded_items);
    item.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    item.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    item.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    item.find_bad_extensions_files(None, None);

    save_results_to_files(file_to_save.file_name(), json_compact_file_to_save.file_name(), json_pretty_file_to_save.file_name(), &item);

    if !cfg!(debug_assertions) {
        item.print_results_to_output();
    }
    item.get_text_messages().print_messages();
}

fn save_results_to_files<T: PrintResults>(txt_file_name: Option<&str>, compact_json_file_name: Option<&str>, pretty_json_file_name: Option<&str>, item: &T) {
    if let Some(file_name) = txt_file_name {
        if let Err(e) = item.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }
    if let Some(file_name) = compact_json_file_name {
        if let Err(e) = item.save_results_to_file_as_json(file_name, false) {
            error!("Failed to save compact json results to file {e}");
        }
    }
    if let Some(file_name) = pretty_json_file_name {
        if let Err(e) = item.save_results_to_file_as_json(file_name, true) {
            error!("Failed to save pretty json results to file {e}");
        }
    }
}
