use std::io::prelude::*;
use std::io::{self};

use humansize::{BINARY, format_size};

use crate::common::model::CheckingMethod;
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::*;
use crate::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters, Info};

impl DebugPrint for DuplicateFinder {
    #[allow(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!(
            "Number of duplicated files by size(in groups) - {} ({})",
            self.information.number_of_duplicated_files_by_size, self.information.number_of_groups_by_size
        );
        println!(
            "Number of duplicated files by hash(in groups) - {} ({})",
            self.information.number_of_duplicated_files_by_hash, self.information.number_of_groups_by_hash
        );
        println!(
            "Number of duplicated files by name(in groups) - {} ({})",
            self.information.number_of_duplicated_files_by_name, self.information.number_of_groups_by_name
        );
        println!(
            "Lost space by size - {} ({} bytes)",
            format_size(self.information.lost_space_by_size, BINARY),
            self.information.lost_space_by_size
        );
        println!(
            "Lost space by hash - {} ({} bytes)",
            format_size(self.information.lost_space_by_hash, BINARY),
            self.information.lost_space_by_hash
        );

        println!("### Other");

        println!("Files list size - {}", self.files_with_identical_size.len());
        println!("Hashed Files list size - {}", self.files_with_identical_hashes.len());
        println!("Files with identical names - {}", self.files_with_identical_names.len());
        println!("Files with identical size names - {}", self.files_with_identical_size_names.len());
        println!("Files with identical names referenced - {}", self.files_with_identical_names_referenced.len());
        println!("Files with identical size names referenced - {}", self.files_with_identical_size_names_referenced.len());
        println!("Files with identical size referenced - {}", self.files_with_identical_size_referenced.len());
        println!("Files with identical hashes referenced - {}", self.files_with_identical_hashes_referenced.len());
        println!("Checking Method - {:?}", self.get_params().check_method);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for DuplicateFinder {
    fn write_results<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        writeln!(
            writer,
            "Results of searching {:?} (reference directories {:?}) with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories,
            self.common_data.directories.reference_directories,
            self.common_data.directories.excluded_directories,
            self.common_data.excluded_items.get_excluded_items()
        )?;

        match self.get_params().check_method {
            CheckingMethod::Name => {
                if !self.files_with_identical_names.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same names-------------------------------------------------"
                    )?;
                    writeln!(
                        writer,
                        "Found {} files in {} groups with same name(may have different content)",
                        self.information.number_of_duplicated_files_by_name, self.information.number_of_groups_by_name,
                    )?;
                    for (name, vector) in self.files_with_identical_names.iter().rev() {
                        writeln!(writer, "Name - {} - {} files ", name, vector.len())?;
                        for j in vector {
                            writeln!(writer, "\"{}\"", j.path.to_string_lossy())?;
                        }
                        writeln!(writer)?;
                    }
                } else if !self.files_with_identical_names_referenced.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same names in referenced folders-------------------------------------------------"
                    )?;
                    writeln!(
                        writer,
                        "Found {} files in {} groups with same name(may have different content)",
                        self.information.number_of_duplicated_files_by_name, self.information.number_of_groups_by_name,
                    )?;
                    for (name, (file_entry, vector)) in self.files_with_identical_names_referenced.iter().rev() {
                        writeln!(writer, "Name - {} - {} files ", name, vector.len())?;
                        writeln!(writer, "Reference file - {:?}", file_entry.path)?;
                        for j in vector {
                            writeln!(writer, "\"{}\"", j.path.to_string_lossy())?;
                        }
                        writeln!(writer)?;
                    }
                } else {
                    write!(writer, "Not found any files with same names.")?;
                }
            }
            CheckingMethod::SizeName => {
                if !self.files_with_identical_names.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same size and names-------------------------------------------------"
                    )?;
                    writeln!(
                        writer,
                        "Found {} files in {} groups with same size and name(may have different content)",
                        self.information.number_of_duplicated_files_by_size_name, self.information.number_of_groups_by_size_name,
                    )?;
                    for ((size, name), vector) in self.files_with_identical_size_names.iter().rev() {
                        writeln!(writer, "Name - {}, {} - {} files ", name, format_size(*size, BINARY), vector.len())?;
                        for j in vector {
                            writeln!(writer, "\"{}\"", j.path.to_string_lossy())?;
                        }
                        writeln!(writer)?;
                    }
                } else if !self.files_with_identical_names_referenced.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same size and names in referenced folders-------------------------------------------------"
                    )?;
                    writeln!(
                        writer,
                        "Found {} files in {} groups with same size and name(may have different content)",
                        self.information.number_of_duplicated_files_by_size_name, self.information.number_of_groups_by_size_name,
                    )?;
                    for ((size, name), (file_entry, vector)) in self.files_with_identical_size_names_referenced.iter().rev() {
                        writeln!(writer, "Name - {}, {} - {} files ", name, format_size(*size, BINARY), vector.len())?;
                        writeln!(writer, "Reference file - {:?}", file_entry.path)?;
                        for j in vector {
                            writeln!(writer, "\"{}\"", j.path.to_string_lossy())?;
                        }
                        writeln!(writer)?;
                    }
                } else {
                    write!(writer, "Not found any files with same size and names.")?;
                }
            }
            CheckingMethod::Size => {
                if !self.files_with_identical_size.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same size-------------------------------------------------"
                    )?;
                    writeln!(
                        writer,
                        "Found {} duplicated files which in {} groups which takes {}.",
                        self.information.number_of_duplicated_files_by_size,
                        self.information.number_of_groups_by_size,
                        format_size(self.information.lost_space_by_size, BINARY)
                    )?;
                    for (size, vector) in self.files_with_identical_size.iter().rev() {
                        write!(writer, "\n---- Size {} ({}) - {} files \n", format_size(*size, BINARY), size, vector.len())?;
                        for file_entry in vector {
                            writeln!(writer, "\"{}\"", file_entry.path.to_string_lossy())?;
                        }
                    }
                } else if !self.files_with_identical_size_referenced.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same size in referenced folders-------------------------------------------------"
                    )?;
                    writeln!(
                        writer,
                        "Found {} duplicated files which in {} groups which takes {}.",
                        self.information.number_of_duplicated_files_by_size,
                        self.information.number_of_groups_by_size,
                        format_size(self.information.lost_space_by_size, BINARY)
                    )?;
                    for (size, (file_entry, vector)) in self.files_with_identical_size_referenced.iter().rev() {
                        writeln!(writer, "\n---- Size {} ({}) - {} files", format_size(*size, BINARY), size, vector.len())?;
                        writeln!(writer, "Reference file - {:?}", file_entry.path)?;
                        for file_entry in vector {
                            writeln!(writer, "\"{}\"", file_entry.path.to_string_lossy())?;
                        }
                    }
                } else {
                    write!(writer, "Not found any duplicates.")?;
                }
            }
            CheckingMethod::Hash => {
                if !self.files_with_identical_hashes.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same hashes-------------------------------------------------"
                    )?;
                    writeln!(
                        writer,
                        "Found {} duplicated files which in {} groups which takes {}.",
                        self.information.number_of_duplicated_files_by_hash,
                        self.information.number_of_groups_by_hash,
                        format_size(self.information.lost_space_by_hash, BINARY)
                    )?;
                    for (size, vectors_vector) in self.files_with_identical_hashes.iter().rev() {
                        for vector in vectors_vector {
                            writeln!(writer, "\n---- Size {} ({}) - {} files", format_size(*size, BINARY), size, vector.len())?;
                            for file_entry in vector {
                                writeln!(writer, "\"{}\"", file_entry.path.to_string_lossy())?;
                            }
                        }
                    }
                } else if !self.files_with_identical_hashes_referenced.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same hashes in referenced folders-------------------------------------------------"
                    )?;
                    writeln!(
                        writer,
                        "Found {} duplicated files which in {} groups which takes {}.",
                        self.information.number_of_duplicated_files_by_hash,
                        self.information.number_of_groups_by_hash,
                        format_size(self.information.lost_space_by_hash, BINARY)
                    )?;
                    for (size, vectors_vector) in self.files_with_identical_hashes_referenced.iter().rev() {
                        for (file_entry, vector) in vectors_vector {
                            writeln!(writer, "\n---- Size {} ({}) - {} files", format_size(*size, BINARY), size, vector.len())?;
                            writeln!(writer, "Reference file - \"{}\"", file_entry.path.to_string_lossy())?;
                            for file_entry in vector {
                                writeln!(writer, "\"{}\"", file_entry.path.to_string_lossy())?;
                            }
                        }
                    }
                } else {
                    write!(writer, "Not found any duplicates.")?;
                }
            }
            _ => panic!(),
        }

        Ok(())
    }

    // TODO - check if is possible to save also data in header about size and name in SizeName mode - https://github.com/qarmin/czkawka/issues/1137
    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> io::Result<()> {
        if self.get_use_reference() {
            match self.get_params().check_method {
                CheckingMethod::Name => self.save_results_to_file_as_json_internal(file_name, &self.files_with_identical_names_referenced, pretty_print),
                CheckingMethod::SizeName => {
                    self.save_results_to_file_as_json_internal(file_name, &self.files_with_identical_size_names_referenced.values().collect::<Vec<_>>(), pretty_print)
                }
                CheckingMethod::Size => self.save_results_to_file_as_json_internal(file_name, &self.files_with_identical_size_referenced, pretty_print),
                CheckingMethod::Hash => self.save_results_to_file_as_json_internal(file_name, &self.files_with_identical_hashes_referenced, pretty_print),
                _ => panic!(),
            }
        } else {
            match self.get_params().check_method {
                CheckingMethod::Name => self.save_results_to_file_as_json_internal(file_name, &self.files_with_identical_names, pretty_print),
                CheckingMethod::SizeName => self.save_results_to_file_as_json_internal(file_name, &self.files_with_identical_size_names.values().collect::<Vec<_>>(), pretty_print),
                CheckingMethod::Size => self.save_results_to_file_as_json_internal(file_name, &self.files_with_identical_size, pretty_print),
                CheckingMethod::Hash => self.save_results_to_file_as_json_internal(file_name, &self.files_with_identical_hashes, pretty_print),
                _ => panic!(),
            }
        }
    }
}

impl CommonData for DuplicateFinder {
    type Info = Info;
    type Parameters = DuplicateFinderParameters;

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
    fn get_check_method(&self) -> CheckingMethod {
        self.get_params().check_method
    }
    fn found_any_broken_files(&self) -> bool {
        self.get_information().number_of_duplicated_files_by_hash > 0
            || self.get_information().number_of_duplicated_files_by_name > 0
            || self.get_information().number_of_duplicated_files_by_size > 0
            || self.get_information().number_of_duplicated_files_by_size_name > 0
    }
}
