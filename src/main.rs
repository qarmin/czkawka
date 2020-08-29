use std::{env, process};

mod duplicate;

fn main() {
    // Parse argument
    //
    let arguments: Vec<String> = env::args().collect();
    println!("Number of arguments - {}", arguments.len());

    for (index, argument) in arguments.iter().enumerate() {
        println!("Argument number {} - {}", index, argument);
    }
    if arguments.len() == 1 {
        print_help();
        process::exit(0);
    }
    match arguments[1].as_ref() {
        "-d" | "-duplicate_finder" => {
            if arguments.len() != 4 {
                println!("FATAL ERROR: Duplicate Finder must be executed with exactly 3 arguments");
                process::exit(1);
            }

            let mut df = duplicate::DuplicateFinder::new();
            df.set_include_directory(arguments[2].clone());

            if arguments.len() > 3 {
                df.set_exclude_directory(arguments[3].clone());
            }
            if arguments.len() > 4 {
                let min_size = match arguments[4].parse::<u64>() {
                    Ok(t) => t,
                    Err(_) => {
                        println!("FATAL ERROR: Cannot parse \"{}\" to u64", arguments[4]);
                        process::exit(1);
                    }
                };
                df.set_min_file_size(min_size);
            }
            if arguments.len() > 5 {
                df.set_allowed_extensions(arguments[5].clone());
            }
            if arguments.len() > 6 {
                df.set_excluded_items(arguments[6].clone());
            }

            df.find_duplicates(duplicate::CheckingMethod::SIZE);
        }
        argum => println!("\"{}\" argument is not supported, check help for more info.", argum),
    };
}

fn print_help() {
    println!();
    println!("Usage of Czkawka:");
    println!("# Arguments:");
    println!("  -h - prints help, also works without any arguments");
    println!("    -help");
    println!("  -d \"include,include2\" \"exclude,exclude2\" [--delete] - search for duplicate files in `include` directories separated by comma inside qutes and exclue selected files from search");
    println!("    -duplicate_finder");
    println!();
}
