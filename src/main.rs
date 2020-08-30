use std::{env, process};

mod duplicate;

fn main() {
    // Parse argument
    let mut all_arguments: Vec<String> = env::args().collect();
    let mut commands_arguments: Vec<String> = Vec::new();

    all_arguments.remove(0); // Removing program name from arguments

    // No arguments, so we print help to allow user to learn more about program
    if all_arguments.is_empty() {
        print_help();
        process::exit(0);
    }

    // Assigning commands with arguments
    let mut arguments: Vec<ArgumentsPair> = Vec::new();

    let mut can_pass_argument: bool = false;
    for argument in 0..all_arguments.len() {
        if all_arguments[argument].starts_with("--") {
            commands_arguments.push(all_arguments[argument].clone());
        } else if all_arguments[argument].starts_with('-') {
            if argument + 1 < all_arguments.len() {
                if all_arguments[argument + 1].starts_with("--") || all_arguments[argument + 1].starts_with('-') {
                    println!("FATAL ERROR: Missing argument for {}", all_arguments[argument]);
                    process::exit(1);
                } else {
                    let a: ArgumentsPair = ArgumentsPair {
                        command: all_arguments[argument].clone(),
                        argument: all_arguments[argument + 1].clone(),
                    };
                    arguments.push(a);
                    can_pass_argument = true;
                }
            } else {
                println!("FATAL ERROR: Missing argument for {}", all_arguments[argument]);
                process::exit(1);
            }
        } else {
            if !can_pass_argument {
                println!("FATAL ERROR: Argument \"{}\" is not linked to any command", all_arguments[argument]);
                process::exit(1);
            } else {
                can_pass_argument = false;
            }
        }
    }

    for a in &arguments {
        println!("Argument number {} - {}", a.command, a.argument);
    }

    if commands_arguments.is_empty() {
        println! {"FATAL ERROR: Missing type of app which you want to run, please read help for more info."};
        process::exit(0);
    }
    match commands_arguments[0].as_ref() {
        "--d" => {
            let mut df = duplicate::DuplicateFinder::new();

            if ArgumentsPair::has_command(&arguments, "-i") {
                df.set_include_directory(ArgumentsPair::get_argument(&arguments, "-i"));
            } else {
                println!("FATAL ERROR: Parameter -i with set of included files is required.");
                process::exit(1);
            }
            if ArgumentsPair::has_command(&arguments, "-e") {
                df.set_exclude_directory(ArgumentsPair::get_argument(&arguments, "-e"));
            }

            if ArgumentsPair::has_command(&arguments, "-s") {
                let min_size = match ArgumentsPair::get_argument(&arguments, "-s").parse::<u64>() {
                    Ok(t) => t,
                    Err(_) => {
                        println!("FATAL ERROR: \"{}\" is not valid file size(allowed range <0,u64::max>)", ArgumentsPair::get_argument(&arguments, "-s"));
                        process::exit(1);
                    }
                };
                df.set_min_file_size(min_size);
            }

            if ArgumentsPair::has_command(&arguments, "-x") {
                df.set_allowed_extensions(ArgumentsPair::get_argument(&arguments, "-x"));
            }
            if ArgumentsPair::has_command(&arguments, "-k") {
                df.set_excluded_items(ArgumentsPair::get_argument(&arguments, "-k"));
            }
            if ArgumentsPair::has_command(&arguments, "-l") {
                let check_method: duplicate::CheckingMethod;
                if ArgumentsPair::get_argument(&arguments, "-l").to_lowercase() == "size" {
                    check_method = duplicate::CheckingMethod::SIZE;
                } else if ArgumentsPair::get_argument(&arguments, "-l").to_lowercase() == "hash" {
                    check_method = duplicate::CheckingMethod::HASH;
                } else {
                    println!("-l can only have values hash or size");
                    process::exit(1);
                }
                df.find_duplicates(check_method, ArgumentsPair::has_command(&arguments, "--delete"));
            }
        }
        "--h" | "--help" => {
            print_help();
        }
        argum => {
            println!("FATAL ERROR: \"{}\" argument is not supported, check help for more info.", argum);
            process::exit(1);
        }
    };
}

fn print_help() {
    println!();
    println!("Usage of Czkawka:");
    println!("czkawka <option> <>");
    println!("# Main arguments:");
    println!("  --h / --help - prints help, also works without any arguments");
    println!("    Usage example:");
    println!("      czkawka --help");
    println!("      czkawka");
    println!("  --d - <-i directory_to_search> [-e exclude_directories = \"\"] [-s min_size = 1024] [-x allowed_extension = \"\"] [-l type_of_search = \"hash\"] [--delete] - search for duplicates files");
    println!("    -i directory_to_search - list of directories which should will be searched like /home/rafal");
    println!("    -e exclude_directories - list of directories which will be excluded from search.");
    println!("    -s min_size - minimum size of checked files in bytes, assigning bigger value may speed up searching.");
    println!("    -x allowed_extension - list of checked extension, e.g. \"jpg,mp4\" will allow to check \"book.jpg\" and \"car.mp4\" but not roman.png.There are also helpful macros which allow to easy use a typcal extension like IMAGE(\"jpg,kra,gif,png,bmp,tiff,webp,hdr,svg\") or TEXT(\"txt,doc,docx,odt,rtf\")");
    println!("    -k type_of_search - allows to use fastest which takes into account only size, and more accurate which check if file contnet is same(hashes).");
    println!("    --delete - removing file except one.");
    println!("    Usage example:");
    println!("      czkawka --d -i \"/home/rafal/,/home/szczekacz\" -e \"/home/rafal/Pulpit,/home/rafal/Obrazy\" -s 25 -x \"7z,rar,IMAGE\" -k \"size\" --delete");
    println!("      czkawka --d -i \"/etc/,/mnt/Miecz\" -s 1000 -x \"VIDEO\" -k \"hash\"");
    println!("      czkawka --d -i \"/etc/\" --delete");
    println!("  --e");
    println!();
}

struct ArgumentsPair {
    command: String,
    argument: String,
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
    pub fn get_argument(ar: &[ArgumentsPair], command: &str) -> String {
        for a in ar {
            if a.command == command {
                return a.argument.clone();
            }
        }
        panic!("FATAL ERROR: Get argument should always return value");
    }
}
