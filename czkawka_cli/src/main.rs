use czkawka_core::duplicate::Info;
use czkawka_core::{duplicate, empty_folder};
use std::{env, process};

fn main() {
    // Parse argument
    let mut all_arguments: Vec<String> = env::args().collect();
    let mut commands_arguments: Vec<String> = Vec::new();

    // println!("{:?}", all_arguments);

    all_arguments.remove(0); // Removing program name from arguments

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
                println!("FATAL ERROR: Trying set second parameter {}, but only one is supported", argument); // This may be changed in future to support 2 or more attributes with space
                process::exit(1);
            }
            let last_element = arguments.len() - 1;
            arguments[last_element].argument = Option::from(argument);
        }
    }

    // for a in &arguments {
    //     println!(
    //         "Argument number {} - {}",
    //         a.command,
    //         match &a.argument {
    //             Some(t) => t.clone(),
    //             None => "MISSING_ARGUMENT".to_string(),
    //         }
    //     );
    // }

    if commands_arguments.is_empty() {
        println! {"FATAL ERROR: Missing type of app which you want to run, please read help for more info."};
        process::exit(0);
    }
    match commands_arguments[0].as_ref() {
        "--d" => {
            let mut df = duplicate::DuplicateFinder::new();
            let mut check_method: duplicate::CheckingMethod = duplicate::CheckingMethod::HASH;

            if ArgumentsPair::has_command(&arguments, "-i") {
                if !df.set_include_directory(ArgumentsPair::get_argument(&arguments, "-i", false)) {
                    process::exit(1);
                }
            } else {
                println!("FATAL ERROR: Parameter -i with set of included files is required.");
                process::exit(1);
            }
            if ArgumentsPair::has_command(&arguments, "-e") {
                df.set_exclude_directory(ArgumentsPair::get_argument(&arguments, "-e", false));
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
            if ArgumentsPair::has_command(&arguments, "-l") {
                let argument_name = ArgumentsPair::get_argument(&arguments, "-l", false).to_lowercase();
                if argument_name == "size" {
                    check_method = duplicate::CheckingMethod::SIZE;
                } else if argument_name == "hash" {
                    check_method = duplicate::CheckingMethod::HASH;
                } else {
                    println!("-l can only have values hash or size");
                    process::exit(1);
                }
            }

            let mut delete_method: duplicate::DeleteMethod = duplicate::DeleteMethod::None;
            if ArgumentsPair::has_command(&arguments, "-delete") {
                delete_method = duplicate::DeleteMethod::AllExceptOldest;
                let argument_name = ArgumentsPair::get_argument(&arguments, "-delete", true).to_lowercase();
                if argument_name == "aen" {
                    delete_method = duplicate::DeleteMethod::AllExceptNewest;
                } else if argument_name == "aeo" {
                    delete_method = duplicate::DeleteMethod::AllExceptOldest;
                } else if argument_name == "on" {
                    delete_method = duplicate::DeleteMethod::OneNewest;
                } else if argument_name == "oo" {
                    delete_method = duplicate::DeleteMethod::OneOldest;
                } else if argument_name == "" {
                    // Nothing to do choosing default one
                } else {
                    println!(
                        "Invalid argument {} for command -delete, available arguments - aen(All except newest one), aeo(All except oldest one), on(Only one newest), oo(Only one oldest)",
                        argument_name
                    );
                    process::exit(1);
                }
            }

            df.find_duplicates(&check_method, &delete_method);

            print_infos(df.get_infos());
        }
        "--h" | "--help" => {
            print_help();
        }
        "--e" => {
            let mut ef = empty_folder::EmptyFolder::new();
            let mut delete_folders: bool = false;

            if ArgumentsPair::has_command(&arguments, "-i") {
                ef.set_include_directory(ArgumentsPair::get_argument(&arguments, "-i", false));
            } else {
                println!("FATAL ERROR: Parameter -i with set of included files is required.");
                process::exit(1);
            }
            if ArgumentsPair::has_command(&arguments, "-e") {
                ef.set_exclude_directory(ArgumentsPair::get_argument(&arguments, "-e", false));
            }

            if ArgumentsPair::has_command(&arguments, "-delete") {
                delete_folders = true;
            }

            ef.find_empty_folders(delete_folders);
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
Usage of Czkawka:

## Main arguments:
  --h / --help - prints help, also works without any arguments
    Usage example:
      czkawka --help
      czkawka

  --d <-i directory_to_search> [-e exclude_directories = ""] [-k excluded_items = ""] [-s min_size = 1024] [-x allowed_extension = ""] [-l type_of_search = "hash"] [-o] [-delete = "aeo"] - search for duplicates files
    -i directory_to_search - list of directories which should will be searched like /home/rafal
    -e exclude_directories - list of directories which will be excluded from search.
    -k excluded_items - list of excluded items which contains * wildcard(may be slow)
    -o non_recursive - this options prevents from recursive check of folders
    -s min_size - minimum size of checked files in bytes, assigning bigger value may speed up searching.
    -x allowed_extension - list of checked extension, e.g. "jpg,mp4" will allow to check "book.jpg" and "car.mp4" but not roman.png. There are also helpful macros which allow to easy use a typcal extension like IMAGE("jpg,kra,gif,png,bmp,tiff,webp,hdr,svg") or TEXT("txt,doc,docx,odt,rtf")
    -l type_of_search - allows to use fastest which takes into account only size, and more accurate which check if file contnet is same(hashes).
    -delete - delete found files, by default remove all except the most oldest one, it can take arguments: aen(All except newest one), aeo(All except oldest one), on(Only one newest), oo(Only one oldest)
    Usage example:
      czkawka --d -i "/home/rafal/,/home/szczekacz" -e "/home/rafal/Pulpit,/home/rafal/Obrazy" -s 25 -x "7z,rar,IMAGE" -l "size" -delete
      czkawka --d -i "/etc/,/mnt/Miecz" -s 1000 -x "VIDEO" -l "hash" -o
      czkawka --d -i "/var/" -k "/var/l*b/,/var/lo*,*tmp"
      czkawka --d -i "/etc/" -delete "aeo"

  --e <-i directory_to_search> [-e exclude_directories = ""] [-delete] - option to find and delete empty folders
    -i directory_to_search - list of directories which should will be searched like /home/rafal
    -e exclude_directories - list of directories which will be excluded from search.
    -delete - delete found empty folders
      czkawka --e -i "/home/rafal/rr, /home/gateway" -e "/home/rafal/rr/2" -delete
    "###
    );
}
/// Printing infos about warnings, messages and errors
fn print_infos(infos: &Info) {
    if !infos.messages.is_empty() {
        println!("-------------------------------MESSAGES--------------------------------");
    }
    for i in &infos.messages {
        println!("{}", i);
    }
    if !infos.messages.is_empty() {
        println!("---------------------------END OF MESSAGES-----------------------------");
    }

    if !infos.warnings.is_empty() {
        println!("-------------------------------WARNINGS--------------------------------");
    }
    for i in &infos.warnings {
        println!("{}", i);
    }
    if !infos.warnings.is_empty() {
        println!("---------------------------END OF WARNINGS-----------------------------");
    }

    if !infos.errors.is_empty() {
        println!("--------------------------------ERRORS---------------------------------");
    }
    for i in &infos.errors {
        println!("{}", i);
    }
    if !infos.errors.is_empty() {
        println!("----------------------------END OF ERRORS------------------------------");
    }
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
