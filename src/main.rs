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
                println!("Duplicate Finder must be executed with exactly 3 arguments");
                process::exit(1);
            }

            let mut df = duplicate::DuplicateFinder::new();
            df.set_exclude_directory(arguments[3].to_string());
            df.set_include_directory(arguments[2].to_string());
            df.optimize_directories();
            df.debug_print();
            df.find_duplicates();
        }
        argum => println!("{} argument is not supported, check help for more info.", argum),
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
