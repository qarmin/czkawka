use czkawka_core::common_traits::*;
use czkawka_core::*;
use std::{env, process};

fn main() {
    // Parse argument
    let all_arguments: Vec<String> = env::args().skip(1).collect(); // Not need to check program name
    let mut commands_arguments: Vec<String> = Vec::new();

    #[cfg(debug_assertions)]
    println!("{:?}", all_arguments);

    // No arguments, so we print help to allow user to learn more about program
    if all_arguments.is_empty() {
        print_help();
        process::exit(0);
    }

    // Assigning commands with arguments
    let mut arguments: Vec<ArgumentsPair> = Vec::new();

    for argument in all_arguments {
        if argument.starts_with("--") {
            commands_arguments.push(argument);
        } else if argument.starts_with('-') {
            let a: ArgumentsPair = ArgumentsPair { command: argument, argument: Option::None };
            arguments.push(a);
        } else {
            if arguments.is_empty() {
                println!("FATAL ERROR: Trying to use {} without any arguments(like -i -e -delete)", argument);
                process::exit(1);
            }
            if arguments[arguments.len() - 1].argument != Option::None {
                println!(
                    "FATAL ERROR: Trying set second parameter \"{}\" for \"{}\" which already have this parameter \"{}\" ",
                    argument,
                    arguments[arguments.len() - 1].command,
                    arguments[arguments.len() - 1].argument.as_ref().unwrap()
                ); // This may be changed in future to support 2 or more attributes with space
                process::exit(1);
            }
            let last_element = arguments.len() - 1;
            arguments[last_element].argument = Option::from(argument);
        }
    }

    #[cfg(debug_assertions)]
    for a in &arguments {
        println!(
            "Argument number {} - {}",
            a.command,
            match &a.argument {
                Some(t) => t.clone(),
                None => "NO_ARGUMENT".to_string(),
            }
        );
    }

    if commands_arguments.is_empty() {
        println! {"FATAL ERROR: Missing type of app which you want to run, please read help for more info."};
        process::exit(0);
    }
    match commands_arguments[0].as_ref() {
        "--d" => {
            let mut df = duplicate::DuplicateFinder::new();

            if ArgumentsPair::has_command(&arguments, "-i") {
                df.set_included_directory(ArgumentsPair::get_argument(&arguments, "-i", false));
            } else {
                println!("FATAL ERROR: Parameter -i with set of included files is required.");
                process::exit(1);
            }
            if ArgumentsPair::has_command(&arguments, "-e") {
                df.set_excluded_directory(ArgumentsPair::get_argument(&arguments, "-e", false));
            }

            if ArgumentsPair::has_command(&arguments, "-s") {
                let min_size = match ArgumentsPair::get_argument(&arguments, "-s", false).parse::<u64>() {
                    Ok(t) => {
                        if t == 0 {
                            println!("ERROR: Minimum file size must be at least 1 byte.");
                            1
                        } else {
                            t
                        }
                    }
                    Err(_) => {
                        println!("FATAL ERROR: \"{}\" is not valid file size(allowed range <1,u64::max>)", ArgumentsPair::get_argument(&arguments, "-s", false));
                        process::exit(1);
                    }
                };
                df.set_min_file_size(min_size);
            }

            if ArgumentsPair::has_command(&arguments, "-x") {
                df.set_allowed_extensions(ArgumentsPair::get_argument(&arguments, "-x", false));
            }
            if ArgumentsPair::has_command(&arguments, "-k") {
                df.set_excluded_items(ArgumentsPair::get_argument(&arguments, "-k", false));
            }

            if ArgumentsPair::has_command(&arguments, "-o") {
                df.set_recursive_search(false);
            }
            df.set_check_method(duplicate::CheckingMethod::Hash); // Default
            if ArgumentsPair::has_command(&arguments, "-l") {
                let argument_name = ArgumentsPair::get_argument(&arguments, "-l", false).to_lowercase();
                if argument_name == "size" {
                    df.set_check_method(duplicate::CheckingMethod::Size);
                } else if argument_name == "hash" {
                    df.set_check_method(duplicate::CheckingMethod::Hash);
                } else if argument_name == "hashmb" {
                    df.set_check_method(duplicate::CheckingMethod::HashMB);
                } else {
                    println!("-l can only have values hash or size");
                    process::exit(1);
                }
            }

            df.set_delete_method(duplicate::DeleteMethod::None);
            if ArgumentsPair::has_command(&arguments, "-delete") {
                let argument_name = ArgumentsPair::get_argument(&arguments, "-delete", true).to_lowercase();
                if argument_name == "aen" {
                    df.set_delete_method(duplicate::DeleteMethod::AllExceptNewest);
                } else if argument_name == "aeo" {
                    df.set_delete_method(duplicate::DeleteMethod::AllExceptOldest);
                } else if argument_name == "on" {
                    df.set_delete_method(duplicate::DeleteMethod::OneNewest);
                } else if argument_name == "oo" {
                    df.set_delete_method(duplicate::DeleteMethod::OneOldest);
                } else if argument_name == "" {
                    // Default
                    df.set_delete_method(duplicate::DeleteMethod::AllExceptOldest);
                } else {
                    println!(
                        "Invalid argument {} for command -delete, available arguments - aen(All except newest one), aeo(All except oldest one), on(Only one newest), oo(Only one oldest)",
                        argument_name
                    );
                    process::exit(1);
                }
            }

            df.find_duplicates();

            #[allow(clippy::collapsible_if)]
            if ArgumentsPair::has_command(&arguments, "-f") {
                if !df.save_results_to_file(&ArgumentsPair::get_argument(&arguments, "-f", false)) {
                    df.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            df.print_results();

            df.get_text_messages().print_messages();
        }
        "--h" | "--help" => {
            print_help();
        }
        "--e" => {
            let mut ef = empty_folder::EmptyFolder::new();

            if ArgumentsPair::has_command(&arguments, "-i") {
                ef.set_included_directory(ArgumentsPair::get_argument(&arguments, "-i", false));
            } else {
                println!("FATAL ERROR: Parameter -i with set of included files is required.");
                process::exit(1);
            }

            if ArgumentsPair::has_command(&arguments, "-delete") {
                ef.set_delete_folder(true);
            }

            ef.find_empty_folders();

            #[allow(clippy::collapsible_if)]
            if ArgumentsPair::has_command(&arguments, "-f") {
                if !ef.save_results_to_file(&ArgumentsPair::get_argument(&arguments, "-f", false)) {
                    ef.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            ef.print_results();
        }
        "--b" => {
            let mut bf = big_file::BigFile::new();

            if ArgumentsPair::has_command(&arguments, "-i") {
                bf.set_included_directory(ArgumentsPair::get_argument(&arguments, "-i", false));
            } else {
                println!("FATAL ERROR: Parameter -i with set of included files is required.");
                process::exit(1);
            }
            if ArgumentsPair::has_command(&arguments, "-e") {
                bf.set_excluded_directory(ArgumentsPair::get_argument(&arguments, "-e", false));
            }

            if ArgumentsPair::has_command(&arguments, "-l") {
                let number_of_files = match ArgumentsPair::get_argument(&arguments, "-l", false).parse::<usize>() {
                    Ok(t) => {
                        if t == 0 {
                            println!("ERROR: Minimum one biggest file must be showed..");
                            1
                        } else {
                            t
                        }
                    }
                    Err(_) => {
                        println!("FATAL ERROR: \"{}\" is not valid number of files to show(allowed range <1,usize::max>)", ArgumentsPair::get_argument(&arguments, "-s", false));
                        process::exit(1);
                    }
                };
                bf.set_number_of_files_to_check(number_of_files);
            }

            if ArgumentsPair::has_command(&arguments, "-x") {
                bf.set_allowed_extensions(ArgumentsPair::get_argument(&arguments, "-x", false));
            }
            if ArgumentsPair::has_command(&arguments, "-k") {
                bf.set_excluded_items(ArgumentsPair::get_argument(&arguments, "-k", false));
            }

            if ArgumentsPair::has_command(&arguments, "-o") {
                bf.set_recursive_search(false);
            }

            bf.find_big_files();

            #[allow(clippy::collapsible_if)]
            if ArgumentsPair::has_command(&arguments, "-f") {
                if !bf.save_results_to_file(&ArgumentsPair::get_argument(&arguments, "-f", false)) {
                    bf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            bf.print_results();

            bf.get_text_messages().print_messages();
        }
        "--y" => {
            let mut yf = empty_files::EmptyFiles::new();

            if ArgumentsPair::has_command(&arguments, "-i") {
                yf.set_included_directory(ArgumentsPair::get_argument(&arguments, "-i", false));
            } else {
                println!("FATAL ERROR: Parameter -i with set of included files is required.");
                process::exit(1);
            }

            if ArgumentsPair::has_command(&arguments, "-e") {
                yf.set_excluded_directory(ArgumentsPair::get_argument(&arguments, "-e", false));
            }

            if ArgumentsPair::has_command(&arguments, "-k") {
                yf.set_excluded_items(ArgumentsPair::get_argument(&arguments, "-k", false));
            }

            if ArgumentsPair::has_command(&arguments, "-o") {
                yf.set_recursive_search(false);
            }

            if ArgumentsPair::has_command(&arguments, "-delete") {
                yf.set_delete_method(empty_files::DeleteMethod::Delete);
            }

            yf.find_empty_files();

            #[allow(clippy::collapsible_if)]
            if ArgumentsPair::has_command(&arguments, "-f") {
                if !yf.save_results_to_file(&ArgumentsPair::get_argument(&arguments, "-f", false)) {
                    yf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            yf.print_results();

            yf.get_text_messages().print_messages();
        }
        "--t" => {
            let mut tf = temporary::Temporary::new();

            if ArgumentsPair::has_command(&arguments, "-i") {
                tf.set_included_directory(ArgumentsPair::get_argument(&arguments, "-i", false));
            } else {
                println!("FATAL ERROR: Parameter -i with set of included files is required.");
                process::exit(1);
            }

            if ArgumentsPair::has_command(&arguments, "-e") {
                tf.set_excluded_directory(ArgumentsPair::get_argument(&arguments, "-e", false));
            }

            if ArgumentsPair::has_command(&arguments, "-k") {
                tf.set_excluded_items(ArgumentsPair::get_argument(&arguments, "-k", false));
            }

            if ArgumentsPair::has_command(&arguments, "-o") {
                tf.set_recursive_search(false);
            }

            if ArgumentsPair::has_command(&arguments, "-delete") {
                tf.set_delete_method(temporary::DeleteMethod::Delete);
            }

            tf.find_temporary_files();

            #[allow(clippy::collapsible_if)]
            if ArgumentsPair::has_command(&arguments, "-f") {
                if !tf.save_results_to_file(&ArgumentsPair::get_argument(&arguments, "-f", false)) {
                    tf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            tf.print_results();

            tf.get_text_messages().print_messages();
        }
        "--version" | "v" => {
            println!("Czkawka CLI {}", CZKAWKA_VERSION);
            process::exit(0);
        }
        argum => {
            println!("FATAL ERROR: \"{}\" argument is not supported, check help for more info.", argum);
            process::exit(1);
        }
    };
}

fn print_help() {
    println!(
        r###"

  Main commands:
  --h / --help - prints help, also works without any arguments
  --d <-i directory_to_search> [-e exclude_directories = ""] [-k excluded_items = ""] [-s min_size = 1024] [-x allowed_extension = ""] [-l type_of_search = "hash"] [-o] [-f file_to_save = "results.txt"] [-delete = "aeo"] - search for duplicates files
  --e <-i directory_to_search> [-e exclude_directories = ""] [-o] [-f file_to_save] [-delete] - option to find and delete empty folders
  --b <-i directory_to_search> [-e exclude_directories = ""] [-k excluded_items = ""] [-p number_of_files = 50] [-x allowed_extension = ""] [-o] [-f file_to_save = "results.txt"]
  --y <-i directory_to_search> [-e exclude_directories = ""] [-k excluded_items = ""] [-o] [-f file_to_save = "results.txt"] [-delete] - search and delete empty files
  --t <-i directory_to_search> [-e exclude_directories = ""] [-k excluded_items = ""] [-o] [-f file_to_save = "results.txt"] [-delete] - search for temporary files
  --version / --v - prints program name and version

  Options:
    -i directory_to_search - list of directories which should will be searched(absolute path)
    -e exclude_directories - list of directories which will be excluded from search(absolute path)
    -k excluded_items - list of excluded items which contains * wildcard(may be slow, so use exclude_directories where possible)
    -o - this options prevents from recursive check of folders
    -s min_size - minimum size of checked files in bytes, assigning bigger value may speed up searching.
    -p number_of_files - number of showed the biggest files.
    -x allowed_extension - list of checked files with provided extensions. There are also helpful macros which allow to easy use a typcal extensions like IMAGE("jpg,kra,gif,png,bmp,tiff,webp,hdr,svg"), TEXT, VIDEO or MUSIC.
    -l type_of_search - allows to use fastest method which takes into account only size(SIZE), more accurate which takes into account hash of only first 1MB of file(HASHMB) or fully accurate(but the slowest solution) which check hash of all file(HASH).
    -delete - delete found files, in duplicate finder by default remove all except the most oldest one but it can take arguments: aen(All except newest one), aeo(All except oldest one), on(Only one newest), oo(Only one oldest)
    -f file_to_save - saves results to file

  Usage example:
    czkawka --d -i "/home/rafal/,/home/szczekacz" -e "/home/rafal/Pulpit,/home/rafal/Obrazy" -s 25 -x "7z,rar,IMAGE" -l "size" -f "results.txt" -delete "aeo"
    czkawka --d -i "/etc/,/mnt/Miecz" -s 1000 -x "VIDEO" -l "hashmb"
    czkawka --e -i "/home/rafal/rr, /home/gateway" -f "results.txt"
    czkawka --b -i "/home/rafal/,/home/piszczal" -e "/home/rafal/Roman" -p 25 -x "VIDEO" " -f "results.txt"
    czkawka --y -i "/home/rafal/" -e "/etc/" -o -f "results.txt"
    czkawka --t -i "/home/rafal/"  -p 25 -x "VIDEO" " -f "results.txt"

    "###
    );
}

struct ArgumentsPair {
    command: String,
    argument: Option<String>,
}

impl ArgumentsPair {
    pub fn has_command(ar: &[ArgumentsPair], command: &str) -> bool {
        for a in ar {
            if a.command == command {
                return true;
            }
        }
        false
    }
    pub fn get_argument(ar: &[ArgumentsPair], command: &str, can_be_empty: bool) -> String {
        for a in ar {
            if a.command == command {
                if !can_be_empty && a.argument == Option::None {
                    println!("FATAL ERROR: {} commands should have argument passed", command);
                    process::exit(1);
                }
                return match &a.argument {
                    Some(t) => t.clone(),
                    None => "".to_string(),
                };
            }
        }
        panic!("INTERNAL ERROR: Get argument should always return value");
    }
}
