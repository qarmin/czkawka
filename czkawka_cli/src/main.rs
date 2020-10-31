mod commands;

use commands::Commands;

#[allow(unused_imports)] // It is used in release for print_results().
use czkawka_core::common_traits::*;

use czkawka_core::{
    big_file::BigFile,
    duplicate::DuplicateFinder,
    empty_files::{self, EmptyFiles},
    empty_folder::EmptyFolder,
    similar_files::SimilarImages,
    temporary::{self, Temporary},
    zeroed::{self, ZeroedFiles},
};
use std::{path::PathBuf, process};
use structopt::StructOpt;

fn path_list_to_str(path_list: Vec<PathBuf>) -> String {
    let path_list: Vec<String> = path_list.into_iter().filter_map(|a| a.into_os_string().into_string().ok()).collect();
    path_list.join(",")
}

fn main() {
    let command = Commands::from_args();

    #[cfg(debug_assertions)]
    println!("{:?}", command);

    match command {
        Commands::Duplicates {
            directories,
            excluded_directories,
            excluded_items,
            minimal_file_size,
            allowed_extensions,
            search_method,
            delete_method,
            file_to_save,
            not_recursive,
        } => {
            let mut df = DuplicateFinder::new();

            df.set_included_directory(path_list_to_str(directories.directories));
            df.set_excluded_directory(path_list_to_str(excluded_directories.excluded_directories));
            df.set_excluded_items(path_list_to_str(excluded_items.excluded_items));
            df.set_minimal_file_size(minimal_file_size);
            df.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            df.set_check_method(search_method);
            df.set_delete_method(delete_method);
            df.set_recursive_search(!not_recursive.not_recursive);

            df.find_duplicates(None);

            if let Some(file_name) = file_to_save.file_name() {
                if !df.save_results_to_file(file_name) {
                    df.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            df.print_results();
            df.get_text_messages().print_messages();
        }
        Commands::EmptyFolders { directories, delete_folders, file_to_save } => {
            let mut ef = EmptyFolder::new();

            ef.set_included_directory(path_list_to_str(directories.directories));
            ef.set_delete_folder(delete_folders);

            ef.find_empty_folders(None);

            if let Some(file_name) = file_to_save.file_name() {
                if !ef.save_results_to_file(file_name) {
                    ef.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            ef.print_results();
            ef.get_text_messages().print_messages();
        }
        Commands::BiggestFiles {
            directories,
            excluded_directories,
            excluded_items,
            allowed_extensions,
            number_of_files,
            file_to_save,
            not_recursive,
        } => {
            let mut bf = BigFile::new();

            bf.set_included_directory(path_list_to_str(directories.directories));
            bf.set_excluded_directory(path_list_to_str(excluded_directories.excluded_directories));
            bf.set_excluded_items(path_list_to_str(excluded_items.excluded_items));
            bf.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            bf.set_number_of_files_to_check(number_of_files);
            bf.set_recursive_search(!not_recursive.not_recursive);

            bf.find_big_files(None);

            if let Some(file_name) = file_to_save.file_name() {
                if !bf.save_results_to_file(file_name) {
                    bf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            bf.print_results();
            bf.get_text_messages().print_messages();
        }
        Commands::EmptyFiles {
            directories,
            excluded_directories,
            excluded_items,
            allowed_extensions,
            delete_files,
            file_to_save,
            not_recursive,
        } => {
            let mut ef = EmptyFiles::new();

            ef.set_included_directory(path_list_to_str(directories.directories));
            ef.set_excluded_directory(path_list_to_str(excluded_directories.excluded_directories));
            ef.set_excluded_items(path_list_to_str(excluded_items.excluded_items));
            ef.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            ef.set_recursive_search(!not_recursive.not_recursive);

            if delete_files {
                ef.set_delete_method(empty_files::DeleteMethod::Delete);
            }

            ef.find_empty_files(None);

            if let Some(file_name) = file_to_save.file_name() {
                if !ef.save_results_to_file(file_name) {
                    ef.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            ef.print_results();
            ef.get_text_messages().print_messages();
        }
        Commands::Temporary {
            directories,
            excluded_directories,
            excluded_items,
            delete_files,
            file_to_save,
            not_recursive,
        } => {
            let mut tf = Temporary::new();

            tf.set_included_directory(path_list_to_str(directories.directories));
            tf.set_excluded_directory(path_list_to_str(excluded_directories.excluded_directories));
            tf.set_excluded_items(path_list_to_str(excluded_items.excluded_items));
            tf.set_recursive_search(!not_recursive.not_recursive);

            if delete_files {
                tf.set_delete_method(temporary::DeleteMethod::Delete);
            }

            tf.find_temporary_files(None);

            if let Some(file_name) = file_to_save.file_name() {
                if !tf.save_results_to_file(file_name) {
                    tf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            tf.print_results();
            tf.get_text_messages().print_messages();
        }
        Commands::SimilarImages {
            directories,
            excluded_directories,
            excluded_items,
            file_to_save,
            minimal_file_size,
            not_recursive,
        } => {
            let mut sf = SimilarImages::new();

            sf.set_included_directory(path_list_to_str(directories.directories));
            sf.set_excluded_directory(path_list_to_str(excluded_directories.excluded_directories));
            sf.set_excluded_items(path_list_to_str(excluded_items.excluded_items));
            sf.set_minimal_file_size(minimal_file_size);
            sf.set_recursive_search(!not_recursive.not_recursive);

            sf.find_similar_images(None);

            if let Some(file_name) = file_to_save.file_name() {
                if !sf.save_results_to_file(file_name) {
                    sf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            sf.print_results();
            sf.get_text_messages().print_messages();
        }

        Commands::ZeroedFiles {
            directories,
            excluded_directories,
            excluded_items,
            allowed_extensions,
            delete_files,
            file_to_save,
            not_recursive,
            minimal_file_size,
        } => {
            let mut zf = ZeroedFiles::new();

            zf.set_included_directory(path_list_to_str(directories.directories));
            zf.set_excluded_directory(path_list_to_str(excluded_directories.excluded_directories));
            zf.set_excluded_items(path_list_to_str(excluded_items.excluded_items));
            zf.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            zf.set_minimal_file_size(minimal_file_size);
            zf.set_recursive_search(!not_recursive.not_recursive);

            if delete_files {
                zf.set_delete_method(zeroed::DeleteMethod::Delete);
            }

            zf.find_zeroed_files(None);

            if let Some(file_name) = file_to_save.file_name() {
                if !zf.save_results_to_file(file_name) {
                    zf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            zf.print_results();
            zf.get_text_messages().print_messages();
        }
    }
}
