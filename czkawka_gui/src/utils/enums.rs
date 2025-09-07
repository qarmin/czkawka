use czkawka_core::tools::bad_extensions::BadExtensions;
use czkawka_core::tools::big_file::BigFile;
use czkawka_core::tools::broken_files::BrokenFiles;
use czkawka_core::tools::duplicate::DuplicateFinder;
use czkawka_core::tools::empty_files::EmptyFiles;
use czkawka_core::tools::empty_folder::EmptyFolder;
use czkawka_core::tools::invalid_symlinks::InvalidSymlinks;
use czkawka_core::tools::same_music::SameMusic;
use czkawka_core::tools::similar_images::SimilarImages;
use czkawka_core::tools::similar_videos::SimilarVideos;
use czkawka_core::tools::temporary::Temporary;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PopoverTypes {
    All,
    Size,
    Reverse,
    Custom,
    Date,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum BottomButtonsEnum {
    Search,
    Select,
    Delete,
    Save,
    Symlink,
    Hardlink,
    Move,
    Compare,
    Sort,
}

pub enum Message {
    Duplicates(DuplicateFinder),
    EmptyFolders(EmptyFolder),
    EmptyFiles(EmptyFiles),
    BigFiles(BigFile),
    Temporary(Temporary),
    SimilarImages(SimilarImages),
    SimilarVideos(SimilarVideos),
    SameMusic(SameMusic),
    InvalidSymlinks(InvalidSymlinks),
    BrokenFiles(BrokenFiles),
    BadExtensions(BadExtensions),
}

#[derive(Clone, Copy)]
pub enum ColumnsDuplicates {
    ActivatableSelectButton = 0,
    SelectionButton,
    Size,
    SizeAsBytes,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
    Color,
    IsHeader,
    TextColor,
}

#[derive(Clone, Copy)]
pub enum ColumnsEmptyFolders {
    SelectionButton = 0,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsIncludedDirectory {
    Path = 0,
    ReferenceButton,
}

#[derive(Clone, Copy)]
pub enum ColumnsExcludedDirectory {
    Path = 0,
}

#[derive(Clone, Copy)]
pub enum ColumnsBigFiles {
    SelectionButton = 0,
    Size,
    Name,
    Path,
    Modification,
    SizeAsBytes,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsEmptyFiles {
    SelectionButton = 0,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsTemporaryFiles {
    SelectionButton = 0,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsSimilarImages {
    ActivatableSelectButton = 0,
    SelectionButton,
    Similarity,
    Size,
    SizeAsBytes,
    Dimensions,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
    Color,
    IsHeader,
    TextColor,
}

#[derive(Clone, Copy)]
pub enum ColumnsSimilarVideos {
    ActivatableSelectButton = 0,
    SelectionButton,
    Size,
    SizeAsBytes,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
    Color,
    IsHeader,
    TextColor,
}

#[derive(Clone, Copy)]
pub enum ColumnsSameMusic {
    ActivatableSelectButton = 0,
    SelectionButton,
    Size,
    SizeAsBytes,
    Name,
    Path,
    Title,
    Artist,
    Year,
    Bitrate,
    BitrateAsNumber,
    Length,
    Genre,
    Modification,
    ModificationAsSecs,
    Color,
    IsHeader,
    TextColor,
}

#[derive(Clone, Copy)]
pub enum ColumnsInvalidSymlinks {
    SelectionButton = 0,
    Name,
    Path,
    DestinationPath,
    TypeOfError,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsBrokenFiles {
    SelectionButton = 0,
    Name,
    Path,
    ErrorType,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsBadExtensions {
    SelectionButton = 0,
    Name,
    Path,
    CurrentExtension,
    ValidExtensions,
    Modification,
    ModificationAsSecs,
}

