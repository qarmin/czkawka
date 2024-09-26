use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use fun_time::fun_time;
use serde::Serialize;

pub trait DebugPrint {
    fn debug_print(&self);
}

pub trait PrintResults {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;

    #[fun_time(message = "print_results_to_output", level = "debug")]
    fn print_results_to_output(&self) {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        // Panics here are allowed, because it is used only in CLI
        self.write_results(&mut handle).expect("Error while writing to stdout");
        handle.flush().expect("Error while flushing stdout");
    }

    #[fun_time(message = "print_results_to_file", level = "debug")]
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

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()>;

    fn save_results_to_file_as_json_internal<T: Serialize + std::fmt::Debug>(&self, file_name: &str, item_to_serialize: &T, pretty_print: bool) -> std::io::Result<()> {
        if pretty_print {
            self.save_results_to_file_as_json_pretty(file_name, item_to_serialize)
        } else {
            self.save_results_to_file_as_json_compact(file_name, item_to_serialize)
        }
    }

    #[fun_time(message = "save_results_to_file_as_json_pretty", level = "debug")]
    fn save_results_to_file_as_json_pretty<T: Serialize + std::fmt::Debug>(&self, file_name: &str, item_to_serialize: &T) -> std::io::Result<()> {
        let file_handler = File::create(file_name)?;
        let mut writer = BufWriter::new(file_handler);
        serde_json::to_writer_pretty(&mut writer, item_to_serialize)?;
        Ok(())
    }

    #[fun_time(message = "save_results_to_file_as_json_compact", level = "debug")]
    fn save_results_to_file_as_json_compact<T: Serialize + std::fmt::Debug>(&self, file_name: &str, item_to_serialize: &T) -> std::io::Result<()> {
        let file_handler = File::create(file_name)?;
        let mut writer = BufWriter::new(file_handler);
        serde_json::to_writer(&mut writer, item_to_serialize)?;
        Ok(())
    }

    fn save_all_in_one(&self, file_name: &str) -> std::io::Result<()> {
        self.save_results_to_file_as_json(&format!("{file_name}_pretty.json"), true)?;
        self.save_results_to_file_as_json(&format!("{file_name}_compact.json"), false)?;
        self.print_results_to_file(&format!("{file_name}.txt"))?;
        Ok(())
    }
}

pub trait ResultEntry {
    fn get_path(&self) -> &Path;
    fn get_modified_date(&self) -> u64;
    fn get_size(&self) -> u64;
}
