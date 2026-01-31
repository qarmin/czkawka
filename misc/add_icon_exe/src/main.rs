use std::env;
use std::path::Path;
use std::process::exit;

use editpe::Image;

fn print_usage(program: &str) {
    eprintln!("Usage: {} <exe-file> <icon-png>", program);
    eprintln!("Example: {} damocles.exe sword.png", program);
}

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "add_icon_exe".to_string());
    let exe_path = match args.next() {
        Some(v) => v,
        None => {
            print_usage(&program);
            exit(1);
        }
    };
    let icon_path = match args.next() {
        Some(v) => v,
        None => {
            print_usage(&program);
            exit(1);
        }
    };

    if !Path::new(&exe_path).exists() {
        eprintln!("Executable not found: {}", exe_path);
        exit(2);
    }
    if !Path::new(&icon_path).exists() {
        eprintln!("Icon file not found: {}", icon_path);
        exit(3);
    }

    // Try to parse the PE image
    match Image::parse_file(&exe_path) {
        Ok(mut image) => {
            // get the resource directory (or default)
            let mut resources = image.resource_directory().cloned().unwrap_or_default();

            // set the main icon file - this will add/replace the icon resource
            if let Err(e) = resources.set_main_icon_file(&icon_path) {
                eprintln!("Failed to set icon file: {}", e);
                exit(4);
            }

            if let Err(e) = image.set_resource_directory(resources) {
                eprintln!("Failed to set resource directory: {}", e);
                exit(5);
            }

            if let Err(e) = image.write_file(&exe_path) {
                eprintln!("Failed to write executable back: {}", e);
                exit(6);
            }

            println!("Successfully embedded '{}' into '{}'", icon_path, exe_path);
        }
        Err(e) => {
            eprintln!("Failed to parse executable '{}': {}", exe_path, e);
            exit(7);
        }
    }
}

