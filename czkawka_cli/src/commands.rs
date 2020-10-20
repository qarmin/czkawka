use czkawka_core::duplicate::{CheckingMethod, DeleteMethod};
use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "czkawka", help_message = HELP_MESSAGE, template = HELP_TEMPLATE)]
pub enum Commands {
    #[structopt(name = "dup", about = "Finds duplicate files", help_message = HELP_MESSAGE, after_help = "EXAMPLE:\n    czkawka dup -d /home/rafal -e /home/rafal/Obrazy  -m 25 -x 7z rar IMAGE -s hashmb -f results.txt -D aeo")]
    Duplicates {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(flatten)]
        excluded_directories: ExcludedDirectories,
        #[structopt(flatten)]
        excluded_items: ExcludedItems,
        #[structopt(short, long, parse(try_from_str = parse_minimal_file_size), default_value = "1024", help = "Minimum size in bytes", long_help = "Minimum size of checked files in bytes, assigning bigger value may speed up searching")]
        minimal_file_size: u64,
        #[structopt(flatten)]
        allowed_extensions: AllowedExtensions,
        #[structopt(short, long, default_value = "HASH", parse(try_from_str = parse_checking_method), help = "Search method (SIZE, HASH, HASHMB)", long_help = "Methods to search files.\nSIZE - The fastest method, checking by the file's size,\nHASHMB - More accurate but slower, checking by the hash of the file's first mibibyte or\nHASH - The slowest method, checking by the hash of the entire file")]
        search_method: CheckingMethod,
        #[structopt(short = "D", long, default_value = "NONE", parse(try_from_str = parse_delete_method), help = "Delete method (AEN, AEO, ON, OO)", long_help = "Methods to delete the files.\nAEN - All files except the newest,\nAEO - All files except the oldest,\nON - Only 1 file, the newest,\nOO - Only 1 file, the oldest\nNONE - not delete files")]
        delete_method: DeleteMethod,
        #[structopt(flatten)]
        file_to_save: FileToSave,
        #[structopt(flatten)]
        not_recursive: NotRecursive,
    },
    #[structopt(name = "empty-folders", about = "Finds empty folders", help_message = HELP_MESSAGE, after_help = "EXAMPLE:\n    czkawka empty-folders -d /home/rafal/rr /home/gateway -f results.txt")]
    EmptyFolders {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(short = "D", long, help = "Delete found folders")]
        delete_folders: bool,
        #[structopt(flatten)]
        file_to_save: FileToSave,
    },
    #[structopt(name = "big", about = "Finds big files", help_message = HELP_MESSAGE, after_help = "EXAMPLE:\n    czkawka big -d /home/rafal/ /home/piszczal -e /home/rafal/Roman -n 25 -x VIDEO -f results.txt")]
    BiggestFiles {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(flatten)]
        excluded_directories: ExcludedDirectories,
        #[structopt(flatten)]
        excluded_items: ExcludedItems,
        #[structopt(flatten)]
        allowed_extensions: AllowedExtensions,
        #[structopt(short, long, default_value = "50", help = "Number of files to be shown")]
        number_of_files: usize,
        #[structopt(flatten)]
        file_to_save: FileToSave,
        #[structopt(flatten)]
        not_recursive: NotRecursive,
    },
    #[structopt(name = "empty-files", about = "Finds emtpy files", help_message = HELP_MESSAGE, after_help = "EXAMPLE:\n    czkawka empty-files -d /home/rafal /home/szczekacz -e /home/rafal/Pulpit -R -f results.txt")]
    EmptyFiles {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(flatten)]
        excluded_directories: ExcludedDirectories,
        #[structopt(flatten)]
        excluded_items: ExcludedItems,
        #[structopt(flatten)]
        allowed_extensions: AllowedExtensions,
        #[structopt(short = "D", long, help = "Delete found files")]
        delete_files: bool,
        #[structopt(flatten)]
        file_to_save: FileToSave,
        #[structopt(flatten)]
        not_recursive: NotRecursive,
    },
    #[structopt(name = "temp", about = "Finds temporary files", help_message = HELP_MESSAGE, after_help = "EXAMPLE:\n    czkawka temp -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt -D")]
    Temporary {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(flatten)]
        excluded_directories: ExcludedDirectories,
        #[structopt(flatten)]
        excluded_items: ExcludedItems,
        #[structopt(short = "D", long, help = "Delete found files")]
        delete_files: bool,
        #[structopt(flatten)]
        file_to_save: FileToSave,
        #[structopt(flatten)]
        not_recursive: NotRecursive,
    },
    #[structopt(name = "ima", about = "Finds similar images", help_message = HELP_MESSAGE, after_help = "EXAMPLE:\n    czkawka ima -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt")]
    SimilarImages {
        #[structopt(flatten)]
        directories: Directories,
        #[structopt(flatten)]
        excluded_directories: ExcludedDirectories,
        #[structopt(short, long, parse(try_from_str = parse_minimal_file_size), default_value = "16384", help = "Minimum size in bytes", long_help = "Minimum size of checked files in bytes, assigning bigger value may speed up searching")]
        minimal_file_size: u64,
        #[structopt(flatten)]
        excluded_items: ExcludedItems,
        #[structopt(flatten)]
        file_to_save: FileToSave,
        #[structopt(flatten)]
        not_recursive: NotRecursive,
    },
}

#[derive(Debug, StructOpt)]
pub struct Directories {
    #[structopt(short, long, parse(from_os_str), required = true, help = "Directorie(s) to search", long_help = "List of directorie(s) which will be searched(absolute path)")]
    pub directories: Vec<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct ExcludedDirectories {
    #[structopt(short, long, parse(from_os_str), help = "Excluded directorie(s)", long_help = "List of directorie(s) which will be excluded from search(absolute path)")]
    pub excluded_directories: Vec<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct ExcludedItems {
    #[structopt(short = "E", long, parse(from_os_str), help = "Excluded item(s)", long_help = "List of excluded item(s) which contains * wildcard(may be slow, so use -e where possible)")]
    pub excluded_items: Vec<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct AllowedExtensions {
    #[structopt(
        short = "x",
        long,
        help = "Allowed file extension(s)",
        long_help = "List of checked files with provided extension(s). There are also helpful macros which allow to easy use a typical extensions like:\nIMAGE(\"jpg,kra,gif,png,bmp,tiff,webp,hdr,svg\"),\nTEXT(\"txt,doc,docx,odt,rtf\"),\nVIDEO(\"mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp\") or\nMUSIC(\"mp3,flac,ogg,tta,wma,webm\")\n "
    )]
    pub allowed_extensions: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct NotRecursive {
    #[structopt(short = "R", long, help = "Prevents from recursive check of folders")]
    pub not_recursive: bool,
}

#[derive(Debug, StructOpt)]
pub struct FileToSave {
    #[structopt(short, long, value_name = "file-name", help = "Saves the results into the file")]
    pub file_to_save: Option<PathBuf>,
}

impl FileToSave {
    pub fn file_name(&self) -> Option<&str> {
        if let Some(file_name) = &self.file_to_save {
            return file_name.to_str();
        }

        None
    }
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
        "none" => Ok(DeleteMethod::None),
        "aen" => Ok(DeleteMethod::AllExceptNewest),
        "aeo" => Ok(DeleteMethod::AllExceptOldest),
        "on" => Ok(DeleteMethod::OneNewest),
        "oo" => Ok(DeleteMethod::OneOldest),
        _ => Err("Couldn't parse the delete method (allowed: AEN, AEO, ON, OO)"),
    }
}

fn parse_minimal_file_size(src: &str) -> Result<u64, String> {
    match src.parse::<u64>() {
        Ok(minimal_file_size) => {
            if minimal_file_size > 0 {
                Ok(minimal_file_size)
            } else {
                Err("Minimum file size must be at least 1 byte".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

static HELP_MESSAGE: &str = "Prints help information (--help will give more information)";

const HELP_TEMPLATE: &str = r#"
{bin} {version}

USAGE:
    {usage} [SCFLAGS] [SCOPTIONS]

FLAGS:
{flags}

SUBCOMMANDS:
{subcommands}

    try "{usage} -h" to get more info about a specific tool

EXAMPLES:
    {bin} dup -d /home/rafal -e /home/rafal/Obrazy  -m 25 -x 7z rar IMAGE -s hashmb -f results.txt -D aeo
    {bin} empty-folders -d /home/rafal/rr /home/gateway -f results.txt
    {bin} big -d /home/rafal/ /home/piszczal -e /home/rafal/Roman -n 25 -x VIDEO -f results.txt
    {bin} empty-files -d /home/rafal /home/szczekacz -e /home/rafal/Pulpit -R -f results.txt
    {bin} temp -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt -D"#;
