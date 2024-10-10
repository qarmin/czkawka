use glib::types::Type;

use crate::help_functions::{
    BottomButtonsEnum, ColumnsBadExtensions, ColumnsBigFiles, ColumnsBrokenFiles, ColumnsDuplicates, ColumnsEmptyFiles, ColumnsEmptyFolders, ColumnsInvalidSymlinks,
    ColumnsSameMusic, ColumnsSimilarImages, ColumnsSimilarVideos, ColumnsTemporaryFiles, PopoverTypes,
};
use crate::notebook_enums::{NotebookMainEnum, NUMBER_OF_NOTEBOOK_MAIN_TABS};

#[derive(Debug)]
pub struct NotebookObject {
    pub notebook_type: NotebookMainEnum,
    pub available_modes: &'static [PopoverTypes],
    #[allow(unused)]
    pub column_activatable_button: Option<i32>,
    pub column_path: i32,
    pub column_name: i32,
    pub column_selection: i32,
    pub column_header: Option<i32>,
    pub column_dimensions: Option<i32>,
    #[allow(unused)]
    pub column_size: Option<i32>,
    pub column_size_as_bytes: Option<i32>,
    pub column_modification_as_secs: Option<i32>,
    pub columns_types: &'static [Type],
    pub bottom_buttons: &'static [BottomButtonsEnum],
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
            Type::BOOL,   // ActivatableSelectButton
            Type::BOOL,   // SelectionButton
            Type::STRING, // Size
            Type::U64,    // SizeAsBytes
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
            Type::STRING, // Color
            Type::BOOL,   // IsHeader
            Type::STRING, // TextColor
        ],
        bottom_buttons: &[
            BottomButtonsEnum::Save,
            BottomButtonsEnum::Delete,
            BottomButtonsEnum::Select,
            BottomButtonsEnum::Sort,
            BottomButtonsEnum::Symlink,
            BottomButtonsEnum::Hardlink,
            BottomButtonsEnum::Move,
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
            Type::BOOL,   // SelectionButton
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
        ],
        bottom_buttons: &[BottomButtonsEnum::Save, BottomButtonsEnum::Delete, BottomButtonsEnum::Select, BottomButtonsEnum::Move],
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
            Type::BOOL,   // SelectionButton
            Type::STRING, // Size
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // Modification
            Type::U64,    // SizeAsBytes
            Type::U64,    // ModificationAsSecs
        ],
        bottom_buttons: &[BottomButtonsEnum::Save, BottomButtonsEnum::Delete, BottomButtonsEnum::Select, BottomButtonsEnum::Move],
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
            Type::BOOL,   // SelectionButton
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
        ],
        bottom_buttons: &[BottomButtonsEnum::Save, BottomButtonsEnum::Delete, BottomButtonsEnum::Select, BottomButtonsEnum::Move],
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
            Type::BOOL,   // SelectionButton
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
        ],
        bottom_buttons: &[BottomButtonsEnum::Save, BottomButtonsEnum::Delete, BottomButtonsEnum::Select, BottomButtonsEnum::Move],
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
            Type::BOOL,   // ActivatableSelectButton
            Type::BOOL,   // SelectionButton
            Type::STRING, // Similarity
            Type::STRING, // Size
            Type::U64,    // SizeAsBytes
            Type::STRING, // Dimensions
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
            Type::STRING, // Color
            Type::BOOL,   // IsHeader
            Type::STRING, // TextColor
        ],
        bottom_buttons: &[
            BottomButtonsEnum::Save,
            BottomButtonsEnum::Delete,
            BottomButtonsEnum::Select,
            BottomButtonsEnum::Sort,
            BottomButtonsEnum::Symlink,
            BottomButtonsEnum::Hardlink,
            BottomButtonsEnum::Move,
            BottomButtonsEnum::Compare,
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
            Type::BOOL,   // ActivatableSelectButton
            Type::BOOL,   // SelectionButton
            Type::STRING, // Size
            Type::U64,    // SizeAsBytes
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
            Type::STRING, // Color
            Type::BOOL,   // IsHeader
            Type::STRING, // TextColor
        ],
        bottom_buttons: &[
            BottomButtonsEnum::Save,
            BottomButtonsEnum::Delete,
            BottomButtonsEnum::Select,
            BottomButtonsEnum::Sort,
            BottomButtonsEnum::Symlink,
            BottomButtonsEnum::Hardlink,
            BottomButtonsEnum::Move,
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
            Type::BOOL,   // ActivatableSelectButton
            Type::BOOL,   // SelectionButton
            Type::STRING, // Size
            Type::U64,    // SizeAsBytes
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // Title
            Type::STRING, // Artist
            Type::STRING, // Year
            Type::STRING, // Bitrate
            Type::U64,    // BitrateAsNumber
            Type::STRING, // Length
            Type::STRING, // Genre
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
            Type::STRING, // Color
            Type::BOOL,   // IsHeader
            Type::STRING, // TextColor
        ],
        bottom_buttons: &[
            BottomButtonsEnum::Save,
            BottomButtonsEnum::Delete,
            BottomButtonsEnum::Select,
            BottomButtonsEnum::Sort,
            BottomButtonsEnum::Symlink,
            BottomButtonsEnum::Hardlink,
            BottomButtonsEnum::Move,
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
            Type::BOOL,   // SelectionButton
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // DestinationPath
            Type::STRING, // TypeOfError
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
        ],
        bottom_buttons: &[BottomButtonsEnum::Save, BottomButtonsEnum::Delete, BottomButtonsEnum::Select, BottomButtonsEnum::Move],
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
            Type::BOOL,   // SelectionButton
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // ErrorType
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
        ],
        bottom_buttons: &[BottomButtonsEnum::Save, BottomButtonsEnum::Delete, BottomButtonsEnum::Select, BottomButtonsEnum::Move],
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
            Type::BOOL,   // SelectionButton
            Type::STRING, // Name
            Type::STRING, // Path
            Type::STRING, // CurrentExtension
            Type::STRING, // ProperExtensions
            Type::STRING, // Modification
            Type::U64,    // ModificationAsSecs
        ],
        bottom_buttons: &[BottomButtonsEnum::Save, BottomButtonsEnum::Delete, BottomButtonsEnum::Select, BottomButtonsEnum::Move],
    },
];
