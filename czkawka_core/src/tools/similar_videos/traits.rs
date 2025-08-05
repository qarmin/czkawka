use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteMethod};
use crate::common::traits::{DebugPrint, DeletingItems, PrintResults};
use crate::tools::similar_videos::{Info, SimilarVideos, SimilarVideosParameters};

impl DeletingItems for SimilarVideos {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.get_cd().delete_method == DeleteMethod::None {
            return WorkContinueStatus::Continue;
        }
        let files_to_delete = self.similar_vectors.clone();
        self.delete_advanced_elements_and_add_to_messages(stop_flag, progress_sender, files_to_delete)
    }
}

impl DebugPrint for SimilarVideos {
    #[allow(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        println!("Included directories - {:?}", self.common_data.directories.included_directories);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for SimilarVideos {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.similar_vectors.is_empty() {
            write!(writer, "{} videos which have similar friends\n\n", self.similar_vectors.len())?;

            for struct_similar in &self.similar_vectors {
                writeln!(writer, "Found {} videos which have similar friends", struct_similar.len())?;
                for file_entry in struct_similar {
                    writeln!(writer, "\"{}\" - {}", file_entry.path.to_string_lossy(), format_size(file_entry.size, BINARY))?;
                }
                writeln!(writer)?;
            }
        } else if !self.similar_referenced_vectors.is_empty() {
            write!(writer, "{} videos which have similar friends\n\n", self.similar_referenced_vectors.len())?;

            for (fe, struct_similar) in &self.similar_referenced_vectors {
                writeln!(writer, "Found {} videos which have similar friends", struct_similar.len())?;
                writeln!(writer)?;
                writeln!(writer, "\"{}\" - {}", fe.path.to_string_lossy(), format_size(fe.size, BINARY))?;
                for file_entry in struct_similar {
                    writeln!(writer, "\"{}\" - {}", file_entry.path.to_string_lossy(), format_size(file_entry.size, BINARY))?;
                }
                writeln!(writer)?;
            }
        } else {
            write!(writer, "Not found any similar videos.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        if self.get_use_reference() {
            self.save_results_to_file_as_json_internal(file_name, &self.similar_referenced_vectors, pretty_print)
        } else {
            self.save_results_to_file_as_json_internal(file_name, &self.similar_vectors, pretty_print)
        }
    }
}

impl CommonData for SimilarVideos {
    type Info = Info;
    type Parameters = SimilarVideosParameters;

    fn get_information(&self) -> Self::Info {
        self.information.clone()
    }
    fn get_params(&self) -> Self::Parameters {
        self.params.clone()
    }
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn found_any_broken_files(&self) -> bool {
        self.information.number_of_duplicates > 0
    }
}
