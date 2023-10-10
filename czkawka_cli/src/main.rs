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
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        allow_hard_links,
        dryrun,
        case_sensitive_name_comparison,
    } = duplicates;

    set_number_of_threads(thread_number.thread_number);

    let mut df = DuplicateFinder::new();

    df.set_included_directory(directories.directories);
    df.set_excluded_directory(excluded_directories.excluded_directories);
    df.set_excluded_items(excluded_items.excluded_items);
    df.set_minimal_file_size(minimal_file_size);
    df.set_maximal_file_size(maximal_file_size);
    df.set_minimal_cache_file_size(minimal_cached_file_size);
    df.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    df.set_check_method(search_method);
    df.set_delete_method(delete_method);
    df.set_hash_type(hash_type);
    df.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    df.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    df.set_ignore_hard_links(!allow_hard_links.allow_hard_links);
    df.set_dryrun(dryrun.dryrun);
    df.set_case_sensitive_name_comparison(case_sensitive_name_comparison.case_sensitive_name_comparison);

    df.find_duplicates(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = df.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        df.print_results_to_output();
    }
    df.get_text_messages().print_messages();
}

fn empty_folders(empty_folders: EmptyFoldersArgs) {
    let EmptyFoldersArgs {
        thread_number,
        directories,
        delete_folders,
        file_to_save,
        excluded_directories,
        excluded_items,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
    } = empty_folders;

    set_number_of_threads(thread_number.thread_number);

    let mut ef = EmptyFolder::new();

    ef.set_included_directory(directories.directories);
    ef.set_excluded_directory(excluded_directories.excluded_directories);
    ef.set_excluded_items(excluded_items.excluded_items);
    ef.set_delete_folder(delete_folders);
    #[cfg(target_family = "unix")]
    ef.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    ef.find_empty_folders(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = ef.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        ef.print_results_to_output();
    }
    ef.get_text_messages().print_messages();
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
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        delete_files,
        smallest_mode,
    } = biggest_files;

    set_number_of_threads(thread_number.thread_number);

    let mut bf = BigFile::new();

    bf.set_included_directory(directories.directories);
    bf.set_excluded_directory(excluded_directories.excluded_directories);
    bf.set_excluded_items(excluded_items.excluded_items);
    bf.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    bf.set_number_of_files_to_check(number_of_files);
    bf.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    bf.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    if delete_files {
        bf.set_delete_method(DeleteMethod::Delete);
    }
    if smallest_mode {
        bf.set_search_mode(SearchMode::SmallestFiles);
    }

    bf.find_big_files(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = bf.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        bf.print_results_to_output();
    }
    bf.get_text_messages().print_messages();
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
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
    } = empty_files;

    set_number_of_threads(thread_number.thread_number);

    let mut ef = EmptyFiles::new();

    ef.set_included_directory(directories.directories);
    ef.set_excluded_directory(excluded_directories.excluded_directories);
    ef.set_excluded_items(excluded_items.excluded_items);
    ef.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    ef.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    ef.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    if delete_files {
        ef.set_delete_method(DeleteMethod::Delete);
    }

    ef.find_empty_files(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = ef.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        ef.print_results_to_output();
    }
    ef.get_text_messages().print_messages();
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
        not_recursive,
    } = temporary;

    set_number_of_threads(thread_number.thread_number);

    let mut tf = Temporary::new();

    tf.set_included_directory(directories.directories);
    tf.set_excluded_directory(excluded_directories.excluded_directories);
    tf.set_excluded_items(excluded_items.excluded_items);
    tf.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    tf.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    if delete_files {
        tf.set_delete_method(DeleteMethod::Delete);
    }

    tf.find_temporary_files(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = tf.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        tf.print_results_to_output();
    }
    tf.get_text_messages().print_messages();
}

fn similar_images(similar_images: SimilarImagesArgs) {
    let SimilarImagesArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        file_to_save,
        minimal_file_size,
        maximal_file_size,
        similarity_preset,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        hash_alg,
        image_filter,
        hash_size,
    } = similar_images;

    set_number_of_threads(thread_number.thread_number);

    let mut sf = SimilarImages::new();

    sf.set_included_directory(directories.directories);
    sf.set_excluded_directory(excluded_directories.excluded_directories);
    sf.set_excluded_items(excluded_items.excluded_items);
    sf.set_minimal_file_size(minimal_file_size);
    sf.set_maximal_file_size(maximal_file_size);
    sf.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    sf.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    sf.set_image_filter(image_filter);
    sf.set_hash_alg(hash_alg);
    sf.set_hash_size(hash_size);

    sf.set_similarity(return_similarity_from_similarity_preset(&similarity_preset, hash_size));

    sf.find_similar_images(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = sf.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        sf.print_results_to_output();
    }
    sf.get_text_messages().print_messages();
}

fn same_music(same_music: SameMusicArgs) {
    let SameMusicArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        // delete_files,
        file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        minimal_file_size,
        maximal_file_size,
        music_similarity,
    } = same_music;

    set_number_of_threads(thread_number.thread_number);

    let mut mf = SameMusic::new();

    mf.set_included_directory(directories.directories);
    mf.set_excluded_directory(excluded_directories.excluded_directories);
    mf.set_excluded_items(excluded_items.excluded_items);
    mf.set_minimal_file_size(minimal_file_size);
    mf.set_maximal_file_size(maximal_file_size);
    mf.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    mf.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    mf.set_music_similarity(music_similarity);

    // if delete_files {
    //     // TODO mf.set_delete_method(same_music::DeleteMethod::Delete);
    // }

    mf.find_same_music(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = mf.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        mf.print_results_to_output();
    }
    mf.get_text_messages().print_messages();
}

fn invalid_symlinks(invalid_symlinks: InvalidSymlinksArgs) {
    let InvalidSymlinksArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        allowed_extensions,
        file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        delete_files,
    } = invalid_symlinks;

    set_number_of_threads(thread_number.thread_number);

    let mut ifs = InvalidSymlinks::new();

    ifs.set_included_directory(directories.directories);
    ifs.set_excluded_directory(excluded_directories.excluded_directories);
    ifs.set_excluded_items(excluded_items.excluded_items);
    ifs.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    ifs.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    ifs.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    if delete_files {
        ifs.set_delete_method(DeleteMethod::Delete);
    }

    ifs.find_invalid_links(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = ifs.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        ifs.print_results_to_output();
    }
    ifs.get_text_messages().print_messages();
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
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
    } = broken_files;

    set_number_of_threads(thread_number.thread_number);

    let mut br = BrokenFiles::new();

    br.set_included_directory(directories.directories);
    br.set_excluded_directory(excluded_directories.excluded_directories);
    br.set_excluded_items(excluded_items.excluded_items);
    br.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    br.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    br.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    if delete_files {
        br.set_delete_method(DeleteMethod::Delete);
    }

    br.find_broken_files(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = br.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        br.print_results_to_output();
    }
    br.get_text_messages().print_messages();
}

fn similar_videos(similar_videos: SimilarVideosArgs) {
    let SimilarVideosArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        tolerance,
        minimal_file_size,
        maximal_file_size,
        allowed_extensions,
    } = similar_videos;

    set_number_of_threads(thread_number.thread_number);

    let mut vr = SimilarVideos::new();

    vr.set_included_directory(directories.directories);
    vr.set_excluded_directory(excluded_directories.excluded_directories);
    vr.set_excluded_items(excluded_items.excluded_items);
    vr.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    vr.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    vr.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
    vr.set_minimal_file_size(minimal_file_size);
    vr.set_maximal_file_size(maximal_file_size);
    vr.set_tolerance(tolerance);

    vr.find_similar_videos(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = vr.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        vr.print_results_to_output();
    }
    vr.get_text_messages().print_messages();
}

fn bad_extensions(bad_extensions: BadExtensionsArgs) {
    let BadExtensionsArgs {
        thread_number,
        directories,
        excluded_directories,
        excluded_items,
        file_to_save,
        not_recursive,
        #[cfg(target_family = "unix")]
        exclude_other_filesystems,
        allowed_extensions,
    } = bad_extensions;

    set_number_of_threads(thread_number.thread_number);

    let mut be = BadExtensions::new();

    be.set_included_directory(directories.directories);
    be.set_excluded_directory(excluded_directories.excluded_directories);
    be.set_excluded_items(excluded_items.excluded_items);
    be.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
    be.set_recursive_search(!not_recursive.not_recursive);
    #[cfg(target_family = "unix")]
    be.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

    be.find_bad_extensions_files(None, None);

    if let Some(file_name) = file_to_save.file_name() {
        if let Err(e) = be.print_results_to_file(file_name) {
            error!("Failed to save results to file {e}");
        }
    }

    if !cfg!(debug_assertions) {
        be.print_results_to_output();
    }
    be.get_text_messages().print_messages();
}
