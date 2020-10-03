use czkawka_core::duplicate::{CheckingMethod, DeleteMethod};
use std::{path::PathBuf, process};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "czkawka")]
pub enum Commands {
    #[structopt(name = "dup", about = "Finds duplicate files")]
    Duplicates {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(flatten)]
        excluded_directories: ExludedDirectories,
        #[structopt(flatten)]
        excluded_items: ExludedItems,
        #[structopt(short, long, default_value = "1024", help = "Minimum size in bytes", long_help = "Minimum size of checked files in bytes, assigning bigger value may speed up searching")]
        min_size: u64,
        #[structopt(flatten)]
        allowed_extensions: AllowedExtensions,
        #[structopt(short, long, default_value = "HASH", parse(try_from_str = parse_checking_method), help = "Search method (SIZE, HASH, HASHMB)", long_help = "Methods to search files.\nSIZE - The fastest method, checking by the file's size,\nHASHMB - More accurate but slower, checking by the hash of the file's first mibibyte or\nHASH - The slowest method, checking by the hash of the entire file")]
        search_method: CheckingMethod,
        #[structopt(short = "D", long, default_value = "AEO", parse(try_from_str = parse_delete_method), help = "Delete method (AEN, AEO, ON, OO)", long_help = "Methods to delete the files.\nAEN - All files except the newest,\nAEO - All files except the oldest,\nON - Only 1 file, the newest,\nOO - Only 1 file, the oldest")]
        delete_method: DeleteMethod,
        #[structopt(flatten)]
        not_recursive: NotRecursive,
    },
    #[structopt(name = "empty-folders", about = "Finds emtpty folders")]
    EmptyFolders {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(short = "D", long, help = "Delete found folders")]
        delete_folders: bool,
    },
    #[structopt(name = "big", about = "Finds big files")]
    BiggestFiles {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(flatten)]
        excluded_directories: ExludedDirectories,
        #[structopt(flatten)]
        excluded_items: ExludedItems,
        #[structopt(flatten)]
        allowed_extensions: AllowedExtensions,
        #[structopt(short, long, default_value = "50", help = "Number of files to be shown")]
        number_of_files: usize,
        #[structopt(flatten)]
        not_recursive: NotRecursive,
    },
    #[structopt(name = "empty-files", about = "Finds emtpy files")]
    EmptyFiles {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(flatten)]
        excluded_directories: ExludedDirectories,
        #[structopt(flatten)]
        excluded_items: ExludedItems,
        #[structopt(flatten)]
        allowed_extensions: AllowedExtensions,
        #[structopt(short = "D", long, help = "Delete found files")]
        delete_files: bool,
        #[structopt(flatten)]
        not_recursive: NotRecursive,
    },
    #[structopt(name = "temp", about = "Finds temporary files")]
    Temporary {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(flatten)]
        excluded_directories: ExludedDirectories,
        #[structopt(flatten)]
        excluded_items: ExludedItems,
        #[structopt(short = "D", long, help = "Delete found files")]
        delete_files: bool,
        #[structopt(flatten)]
        not_recursive: NotRecursive,
    },
}

#[derive(Debug, StructOpt)]
pub struct Directories {
    #[structopt(short, long, parse(from_os_str), help = "Directorie(s) to search", long_help = "List of directorie(s) which will be searched(absolute path)")]
    pub directories: Vec<PathBuf>,
}

impl Directories {
    pub fn not_empty(&self) {
        if self.directories.is_empty() {
            eprintln!("error: At least one directory should be provided.");
            process::exit(1);
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct ExludedDirectories {
    #[structopt(short, long, parse(from_os_str), help = "Exluded directorie(s)", long_help = "List of directorie(s) which will be excluded from search(absolute path)")]
    pub excluded_directories: Vec<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct ExludedItems {
    #[structopt(short = "E", long, parse(from_os_str), help = "Exluded item(s)", long_help = "List of excluded item(s) which contains * wildcard(may be slow, so use -e where possible)")]
    pub excluded_items: Vec<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct AllowedExtensions {
    #[structopt(
        short = "x",
        long,
        help = "Allowed file extension(s)",
        long_help = "List of checked files with provided extension(s). There are also helpful macros which allow to easy use a typcal extensions like:\nIMAGE(\"jpg,kra,gif,png,bmp,tiff,webp,hdr,svg\"),\nTEXT(\"txt,doc,docx,odt,rtf\"),\nVIDEO(\"mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp\") or\nMUSIC(\"mp3,flac,ogg,tta,wma,webm\")"
    )]
    pub allowed_extensions: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct NotRecursive {
    #[structopt(short = "R", long, help = "Prevents from recursive check of folders")]
    pub not_recursive: bool,
}

fn parse_checking_method(src: &str) -> Result<CheckingMethod, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "size" => Ok(CheckingMethod::Size),
        "hash" => Ok(CheckingMethod::Hash),
        "hashmb" => Ok(CheckingMethod::HashMB),
        _ => Err("Couldn't parse the search method (allowed: SIZE, HASH, HASHMB)"),
    }
}

fn parse_delete_method(src: &str) -> Result<DeleteMethod, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "aen" => Ok(DeleteMethod::AllExceptNewest),
        "aeo" => Ok(DeleteMethod::AllExceptOldest),
        "on" => Ok(DeleteMethod::OneNewest),
        "oo" => Ok(DeleteMethod::OneOldest),
        _ => Err("Couldn't parse the delete method (allowed: AEN, AEO, ON, OO)"),
    }
}
