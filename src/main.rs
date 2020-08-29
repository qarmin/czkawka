use std::{env, process};

mod duplicate;

fn main() {
    // Parse argument
    //
    let mut all_arguments: Vec<String> = env::args().collect();
    let number_of_arguments: usize = all_arguments.len() - 1;
    let mut arguments: Vec<String> = Vec::new();
    let mut commands_arguments: Vec<String> = Vec::new();

    all_arguments.remove(0); // Removing program name from arguments

    for argument in all_arguments {
        if argument.starts_with('-') {
            commands_arguments.push(argument);
        } else {
            arguments.push(argument);
        }
    }

    println!("Number of arguments - {}", arguments.len());
    for (index, argument) in arguments.iter().enumerate() {
        println!("Argument number {} - {}", index, argument);
    }
    if number_of_arguments == 0 {
        print_help();
        process::exit(0);
    }
    if commands_arguments.is_empty() {
        println! {"Missing command, please read help for more info."};
        process::exit(0);
    }
    match commands_arguments[0].as_ref() {
        "-d" | "-duplicate_finder" => {
            let delete_files: bool = commands_arguments.contains(&"-delete".to_owned());

            if arguments.len() < 2 {
                println!("FATAL ERROR: Duplicate Finder must be executed with at least 1 argument");
                process::exit(1);
            }

            let mut df = duplicate::DuplicateFinder::new();
            df.set_include_directory(arguments[0].clone());

            if arguments.len() > 1 {
                df.set_exclude_directory(arguments[1].clone());
            }
            if arguments.len() > 2 {
                let min_size = match arguments[2].parse::<u64>() {
                    Ok(t) => t,
                    Err(_) => {
                        println!("FATAL ERROR: \"{}\" is not valid file size(allowed range <0,u64::max>)", arguments[2]);
                        process::exit(1);
                    }
                };
                df.set_min_file_size(min_size);
            }
            if arguments.len() > 3 {
                df.set_allowed_extensions(arguments[3].clone());
            }
            if arguments.len() > 4 {
                df.set_excluded_items(arguments[4].clone());
            }

            df.find_duplicates(duplicate::CheckingMethod::SIZE, delete_files);
        }
        "-h" | "-help" => {
            print_help();
        }
        argum => println!("\"{}\" argument is not supported, check help for more info.", argum),
    };
}

fn print_help() {
    println!();
    println!("Usage of Czkawka:");
    println!("czkawka <option> <>");
    println!("# Main arguments:");
    println!("  -h - prints help, also works without any arguments");
    println!("    -help");
    println!("  -d <directory_to_search> [exclude_directories = \"\"] [min_size = 10] [allowed_extension = \"\"] [-delete]  - search for duplicate files in choosen directories, minimum size(in bytes) and allowed extensions and avaibility to delete duplicates.");
    println!("    -duplicate_finder");
    println!("    e.g.");
    println!("    czkawka -d \"/home/rafal/,/home/szczekacz\" \"/home/rafal/Pulpit,/home/rafal/Obrazy\" 25 \"7z,rar,IMAGE\" -delete");
    println!();
}
