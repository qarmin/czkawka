use fun_time::fun_time;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub trait DebugPrint {
    fn debug_print(&self);
}

pub trait PrintResults {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;

    #[fun_time(message = "print_results_to_output")]
    fn print_results_to_output(&self) {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        self.write_results(&mut handle).unwrap();
        handle.flush().unwrap();
    }

    #[fun_time(message = "print_results_to_file")]
    fn print_results_to_file(&self, file_name: &str) -> std::io::Result<()> {
        let file_name: String = match file_name {
            "" => "results.txt".to_string(),
            k => k.to_string(),
        };

        let file_handler = File::create(file_name)?;
        let mut writer = BufWriter::new(file_handler);
        self.write_results(&mut writer)?;
        writer.flush()?;
        Ok(())
    }
}

pub trait ResultEntry {
    fn get_path(&self) -> &Path;
    fn get_modified_date(&self) -> u64;
    fn get_size(&self) -> u64;
}
