use crate::help_functions::{
    ColumnsBadExtensions, ColumnsBigFiles, ColumnsBrokenFiles, ColumnsDuplicates, ColumnsEmptyFiles, ColumnsEmptyFolders, ColumnsInvalidSymlinks, ColumnsSameMusic,
    ColumnsSimilarImages, ColumnsSimilarVideos, ColumnsTemporaryFiles, PopoverTypes,
};
use crate::notebook_enums::{NotebookMainEnum, NUMBER_OF_NOTEBOOK_MAIN_TABS};

pub struct NotebookObject {
    pub notebook_type: NotebookMainEnum,
    pub available_modes: &'static [PopoverTypes],
    pub column_activatable_button: Option<i32>,
    pub column_path: i32,
    pub column_name: i32,
    pub column_selection: i32,
    pub column_header: Option<i32>,
    pub column_dimensions: Option<i32>,
    pub column_size: Option<i32>,
    pub column_size_as_bytes: Option<i32>,
    pub column_modification_as_secs: Option<i32>,
    pub columns_types: &'static [glib::types::Type],
}

pub static NOTEBOOKS_INFO: [NotebookObject; NUMBER_OF_NOTEBOOK_MAIN_TABS] = [
    NotebookObject {
        notebook_type: NotebookMainEnum::Duplicate,
        available_modes: &[
            PopoverTypes::All,
            PopoverTypes::Reverse,
            PopoverTypes::Custom,
            PopoverTypes::Date,
            PopoverTypes::Size,
            PopoverTypes::All,
        ],
        column_activatable_button: Some(ColumnsDuplicates::ActivatableSelectButton as i32),
        column_path: ColumnsDuplicates::Path as i32,
        column_name: ColumnsDuplicates::Name as i32,
        column_selection: ColumnsDuplicates::SelectionButton as i32,
        column_header: Some(ColumnsDuplicates::IsHeader as i32),
        column_dimensions: None,
        column_size: Some(ColumnsDuplicates::Size as i32), // Useless with duplicates by hash or size, but needed by sorting by name
        column_size_as_bytes: Some(ColumnsDuplicates::SizeAsBytes as i32),
        column_modification_as_secs: Some(ColumnsDuplicates::ModificationAsSecs as i32),
        columns_types: &[
            glib::types::Type::BOOL,   // ActivatableSelectButton
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Size
            glib::types::Type::U64,    // SizeAsBytes
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
            glib::types::Type::STRING, // Color
            glib::types::Type::BOOL,   // IsHeader
            glib::types::Type::STRING, // TextColor
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::EmptyDirectories,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom],
        column_activatable_button: None,
        column_path: ColumnsEmptyFolders::Path as i32,
        column_name: ColumnsEmptyFolders::Name as i32,
        column_selection: ColumnsEmptyFolders::SelectionButton as i32,
        column_header: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
        columns_types: &[
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::BigFiles,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom],
        column_activatable_button: None,
        column_path: ColumnsBigFiles::Path as i32,
        column_name: ColumnsBigFiles::Name as i32,
        column_selection: ColumnsBigFiles::SelectionButton as i32,
        column_header: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
        columns_types: &[
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Size
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // SizeAsBytes
            glib::types::Type::U64,    // ModificationAsSecs
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::EmptyFiles,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom],
        column_activatable_button: None,
        column_path: ColumnsEmptyFiles::Path as i32,
        column_name: ColumnsEmptyFiles::Name as i32,
        column_selection: ColumnsEmptyFiles::SelectionButton as i32,
        column_header: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
        columns_types: &[
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::Temporary,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom],
        column_activatable_button: None,
        column_path: ColumnsTemporaryFiles::Path as i32,
        column_name: ColumnsTemporaryFiles::Name as i32,
        column_selection: ColumnsTemporaryFiles::SelectionButton as i32,
        column_header: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
        columns_types: &[
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::SimilarImages,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date, PopoverTypes::Size],
        column_activatable_button: Some(ColumnsSimilarImages::ActivatableSelectButton as i32),
        column_path: ColumnsSimilarImages::Path as i32,
        column_name: ColumnsSimilarImages::Name as i32,
        column_selection: ColumnsSimilarImages::SelectionButton as i32,
        column_header: Some(ColumnsSimilarImages::IsHeader as i32),
        column_dimensions: Some(ColumnsSimilarImages::Dimensions as i32),
        column_size: Some(ColumnsSimilarImages::Size as i32),
        column_size_as_bytes: Some(ColumnsSimilarImages::SizeAsBytes as i32),
        column_modification_as_secs: Some(ColumnsSimilarImages::ModificationAsSecs as i32),
        columns_types: &[
            glib::types::Type::BOOL,   // ActivatableSelectButton
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Similarity
            glib::types::Type::STRING, // Size
            glib::types::Type::U64,    // SizeAsBytes
            glib::types::Type::STRING, // Dimensions
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
            glib::types::Type::STRING, // Color
            glib::types::Type::BOOL,   // IsHeader
            glib::types::Type::STRING, // TextColor
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::SimilarVideos,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date, PopoverTypes::Size],
        column_activatable_button: Some(ColumnsSimilarVideos::ActivatableSelectButton as i32),
        column_path: ColumnsSimilarVideos::Path as i32,
        column_name: ColumnsSimilarVideos::Name as i32,
        column_selection: ColumnsSimilarVideos::SelectionButton as i32,
        column_header: Some(ColumnsSimilarVideos::IsHeader as i32),
        column_dimensions: None,
        column_size: Some(ColumnsSimilarVideos::Size as i32),
        column_size_as_bytes: Some(ColumnsSimilarVideos::SizeAsBytes as i32),
        column_modification_as_secs: Some(ColumnsSimilarVideos::ModificationAsSecs as i32),
        columns_types: &[
            glib::types::Type::BOOL,   // ActivatableSelectButton
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Size
            glib::types::Type::U64,    // SizeAsBytes
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
            glib::types::Type::STRING, // Color
            glib::types::Type::BOOL,   // IsHeader
            glib::types::Type::STRING, // TextColor
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::SameMusic,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date, PopoverTypes::Size],
        column_activatable_button: Some(ColumnsSameMusic::ActivatableSelectButton as i32),
        column_path: ColumnsSameMusic::Path as i32,
        column_name: ColumnsSameMusic::Name as i32,
        column_selection: ColumnsSameMusic::SelectionButton as i32,
        column_header: Some(ColumnsSameMusic::IsHeader as i32),
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: Some(ColumnsSameMusic::SizeAsBytes as i32),
        column_modification_as_secs: Some(ColumnsSameMusic::ModificationAsSecs as i32),
        columns_types: &[
            glib::types::Type::BOOL,   // ActivatableSelectButton
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Size
            glib::types::Type::U64,    // SizeAsBytes
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // Title
            glib::types::Type::STRING, // Artist
            glib::types::Type::STRING, // Year
            glib::types::Type::STRING, // Bitrate
            glib::types::Type::U64,    // BitrateAsNumber
            glib::types::Type::STRING, // Length
            glib::types::Type::STRING, // Genre
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
            glib::types::Type::STRING, // Color
            glib::types::Type::BOOL,   // IsHeader
            glib::types::Type::STRING, // TextColor
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::Symlinks,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom],
        column_activatable_button: None,
        column_path: ColumnsInvalidSymlinks::Path as i32,
        column_name: ColumnsInvalidSymlinks::Name as i32,
        column_selection: ColumnsInvalidSymlinks::SelectionButton as i32,
        column_header: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
        columns_types: &[
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // DestinationPath
            glib::types::Type::STRING, // TypeOfError
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::BrokenFiles,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom],
        column_activatable_button: None,
        column_path: ColumnsBrokenFiles::Path as i32,
        column_name: ColumnsBrokenFiles::Name as i32,
        column_selection: ColumnsBrokenFiles::SelectionButton as i32,
        column_header: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
        columns_types: &[
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // ErrorType
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
        ],
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::BadExtensions,
        available_modes: &[PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom],
        column_activatable_button: None,
        column_path: ColumnsBadExtensions::Path as i32,
        column_name: ColumnsBadExtensions::Name as i32,
        column_selection: ColumnsBadExtensions::SelectionButton as i32,
        column_header: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
        columns_types: &[
            glib::types::Type::BOOL,   // SelectionButton
            glib::types::Type::STRING, // Name
            glib::types::Type::STRING, // Path
            glib::types::Type::STRING, // CurrentExtension
            glib::types::Type::STRING, // ProperExtensions
            glib::types::Type::STRING, // Modification
            glib::types::Type::U64,    // ModificationAsSecs
        ],
    },
];
