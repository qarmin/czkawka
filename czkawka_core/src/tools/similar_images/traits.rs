use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};

use crate::common::consts::{HEIC_EXTENSIONS, IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS, JXL_IMAGE_EXTENSIONS, RAW_IMAGE_EXTENSIONS};
use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::tools::similar_images::core::get_string_from_similarity;
use crate::tools::similar_images::{Info, SimilarImages, SimilarImagesParameters};

impl AllTraits for SimilarImages {}

impl Search for SimilarImages {
    #[fun_time(message = "find_similar_images", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            let extensions = if cfg!(feature = "heif") {
                [IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS, RAW_IMAGE_EXTENSIONS, JXL_IMAGE_EXTENSIONS, HEIC_EXTENSIONS].concat()
            } else {
                [IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS, RAW_IMAGE_EXTENSIONS, JXL_IMAGE_EXTENSIONS].concat()
            };

            if self.prepare_items(Some(&extensions)).is_err() {
                return;
            }
            self.common_data.use_reference_folders = !self.common_data.directories.reference_directories.is_empty() || !self.common_data.directories.reference_files.is_empty();
            if self.check_for_similar_images(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            if self.hash_images(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            if self.find_similar_hashes(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            if self.delete_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
            }
        })();

        self.information.scanning_time = start_time.elapsed();

        if !self.common_data.stopped_search {
            self.debug_print();
        }
    }
}

impl DebugPrint for SimilarImages {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
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
                        get_string_from_similarity(file_entry.similarity, self.get_params().hash_size)
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
                    get_string_from_similarity(file_entry.similarity, self.get_params().hash_size)
                )?;
                for file_entry in vec_file_entry {
                    writeln!(
                        writer,
                        "\"{}\" - {}x{} - {} - {}",
                        file_entry.path.to_string_lossy(),
                        file_entry.width,
                        file_entry.height,
                        format_size(file_entry.size, BINARY),
                        get_string_from_similarity(file_entry.similarity, self.get_params().hash_size)
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
