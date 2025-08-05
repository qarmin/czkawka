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
use crate::tools::similar_images::core::get_string_from_similarity;
use crate::tools::similar_images::{Info, SimilarImages, SimilarImagesParameters};

impl DebugPrint for SimilarImages {
    #[allow(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for SimilarImages {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.similar_vectors.is_empty() {
            write!(writer, "{} images which have similar friends\n\n", self.similar_vectors.len())?;

            for struct_similar in &self.similar_vectors {
                writeln!(writer, "Found {} images which have similar friends", struct_similar.len())?;
                for file_entry in struct_similar {
                    writeln!(
                        writer,
                        "\"{}\" - {}x{} - {} - {}",
                        file_entry.path.to_string_lossy(),
                        file_entry.width,
                        file_entry.height,
                        format_size(file_entry.size, BINARY),
                        get_string_from_similarity(&file_entry.similarity, self.get_params().hash_size)
                    )?;
                }
                writeln!(writer)?;
            }
        } else if !self.similar_referenced_vectors.is_empty() {
            writeln!(writer, "{} images which have similar friends\n\n", self.similar_referenced_vectors.len())?;

            for (file_entry, vec_file_entry) in &self.similar_referenced_vectors {
                writeln!(writer, "Found {} images which have similar friends", vec_file_entry.len())?;
                writeln!(writer)?;
                writeln!(
                    writer,
                    "\"{}\" - {}x{} - {} - {}",
                    file_entry.path.to_string_lossy(),
                    file_entry.width,
                    file_entry.height,
                    format_size(file_entry.size, BINARY),
                    get_string_from_similarity(&file_entry.similarity, self.get_params().hash_size)
                )?;
                for file_entry in vec_file_entry {
                    writeln!(
                        writer,
                        "{:?} - {}x{} - {} - {}",
                        file_entry.path,
                        file_entry.width,
                        file_entry.height,
                        format_size(file_entry.size, BINARY),
                        get_string_from_similarity(&file_entry.similarity, self.get_params().hash_size)
                    )?;
                }
                writeln!(writer)?;
            }
        } else {
            write!(writer, "Not found any similar images.")?;
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
impl CommonData for SimilarImages {
    type Info = Info;
    type Parameters = SimilarImagesParameters;

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

impl DeletingItems for SimilarImages {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.get_cd().delete_method == DeleteMethod::None {
            return WorkContinueStatus::Continue;
        }
        let files_to_delete = self.similar_vectors.clone();
        self.delete_advanced_elements_and_add_to_messages(stop_flag, progress_sender, files_to_delete)
    }
}
