pub const NUMBER_OF_NOTEBOOK_MAIN_TABS: usize = 10;
pub const NUMBER_OF_NOTEBOOK_UPPER_TABS: usize = 4;

// Needs to be updated when changed order of notebook tabs
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum NotebookMainEnum {
    Duplicate = 0,
    EmptyDirectories,
    BigFiles,
    EmptyFiles,
    Temporary,
    SimilarImages,
    SameMusic,
    Zeroed,
    Symlinks,
    BrokenFiles,
}
pub fn to_notebook_main_enum(notebook_number: u32) -> NotebookMainEnum {
    match notebook_number {
        0 => NotebookMainEnum::Duplicate,
        1 => NotebookMainEnum::EmptyDirectories,
        2 => NotebookMainEnum::BigFiles,
        3 => NotebookMainEnum::EmptyFiles,
        4 => NotebookMainEnum::Temporary,
        5 => NotebookMainEnum::SimilarImages,
        6 => NotebookMainEnum::SameMusic,
        7 => NotebookMainEnum::Zeroed,
        8 => NotebookMainEnum::Symlinks,
        9 => NotebookMainEnum::BrokenFiles,
        _ => panic!("Invalid Notebook Tab"),
    }
}
pub fn get_all_main_tabs() -> [NotebookMainEnum; NUMBER_OF_NOTEBOOK_MAIN_TABS] {
    [
        to_notebook_main_enum(0),
        to_notebook_main_enum(1),
        to_notebook_main_enum(2),
        to_notebook_main_enum(3),
        to_notebook_main_enum(4),
        to_notebook_main_enum(5),
        to_notebook_main_enum(6),
        to_notebook_main_enum(7),
        to_notebook_main_enum(8),
        to_notebook_main_enum(9),
    ]
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum NotebookUpperEnum {
    IncludedDirectories = 0,
    ExcludedDirectories,
    ExcludedItems,
    AllowedExtensions,
}
pub fn to_notebook_upper_enum(notebook_number: u32) -> NotebookUpperEnum {
    match notebook_number {
        0 => NotebookUpperEnum::IncludedDirectories,
        1 => NotebookUpperEnum::ExcludedDirectories,
        2 => NotebookUpperEnum::ExcludedItems,
        3 => NotebookUpperEnum::AllowedExtensions,
        _ => panic!("Invalid Upper Notebook Tab"),
    }
}
pub fn get_all_upper_tabs() -> [NotebookUpperEnum; NUMBER_OF_NOTEBOOK_UPPER_TABS] {
    [to_notebook_upper_enum(0), to_notebook_upper_enum(1), to_notebook_upper_enum(2), to_notebook_upper_enum(3)]
}
