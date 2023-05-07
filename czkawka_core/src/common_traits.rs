use std::path::Path;

pub trait DebugPrint {
    fn debug_print(&self);
}

pub trait SaveResults {
    fn save_results_to_file(&mut self, file_name: &str) -> bool;
}

pub trait PrintResults {
    fn print_results(&self);
}

pub trait ResultEntry {
    fn get_path(&self) -> &Path;
}
