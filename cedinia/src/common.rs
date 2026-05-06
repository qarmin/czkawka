pub const STR_IDX_NAME: usize = 0;
pub const STR_IDX_PATH: usize = 1;
pub const STR_IDX_SIZE: usize = 2;
pub const STR_IDX_MODIFIED: usize = 3;
pub const STR_BASE_COUNT: usize = 4;

pub const INT_IDX_MOD_HI: usize = 0;
pub const INT_IDX_MOD_LO: usize = 1;
pub const INT_IDX_SIZE_HI: usize = 2;
pub const INT_IDX_SIZE_LO: usize = 3;
pub const INT_BASE_COUNT: usize = 4;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StrDataSimilarImages {
    DimsDisplay = STR_BASE_COUNT,
}
pub const MAX_STR_DATA_SIMILAR_IMAGES: usize = StrDataSimilarImages::DimsDisplay as usize + 1;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IntDataSimilarImages {
    Width = INT_BASE_COUNT,
    Height,
    Diff,
}
pub const MAX_INT_DATA_SIMILAR_IMAGES: usize = IntDataSimilarImages::Diff as usize + 1;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StrDataBrokenFiles {
    ErrorString = STR_BASE_COUNT,
}
pub const MAX_STR_DATA_BROKEN_FILES: usize = StrDataBrokenFiles::ErrorString as usize + 1;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StrDataBadExtensions {
    Display = STR_BASE_COUNT,
    ProperExtension,
}
pub const MAX_STR_DATA_BAD_EXTENSIONS: usize = StrDataBadExtensions::ProperExtension as usize + 1;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StrDataSameMusic {
    Display = STR_BASE_COUNT,
    Title,
}
pub const MAX_STR_DATA_SAME_MUSIC: usize = StrDataSameMusic::Title as usize + 1;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StrDataBadNames {
    NewName = STR_BASE_COUNT,
}
pub const MAX_STR_DATA_BAD_NAMES: usize = StrDataBadNames::NewName as usize + 1;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IntDataExifRemover {
    ExifTagCount = INT_BASE_COUNT,
}
pub const MAX_INT_DATA_EXIF_REMOVER: usize = IntDataExifRemover::ExifTagCount as usize + 1;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StrDataSimilarVideos {
    Duration = STR_BASE_COUNT,
}
pub const MAX_STR_DATA_SIMILAR_VIDEOS: usize = StrDataSimilarVideos::Duration as usize + 1;
