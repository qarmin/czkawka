use czkawka_core::big_file::SearchMode;
use czkawka_core::common_dir_traversal::CheckingMethod;
use czkawka_core::duplicate::HashType;
use image_hasher::{FilterType, HashAlg};

pub struct HashTypeStruct {
    pub eng_name: &'static str,
    pub hash_type: HashType,
}

pub const DUPLICATES_HASH_TYPE_COMBO_BOX: [HashTypeStruct; 3] = [
    HashTypeStruct {
        eng_name: "Blake3",
        hash_type: HashType::Blake3,
    },
    HashTypeStruct {
        eng_name: "CRC32",
        hash_type: HashType::Crc32,
    },
    HashTypeStruct {
        eng_name: "XXH3",
        hash_type: HashType::Xxh3,
    },
];

pub struct CheckMethodStruct {
    pub eng_name: &'static str,
    pub check_method: CheckingMethod,
}

pub const DUPLICATES_CHECK_METHOD_COMBO_BOX: [CheckMethodStruct; 4] = [
    CheckMethodStruct {
        eng_name: "Hash",
        check_method: CheckingMethod::Hash,
    },
    CheckMethodStruct {
        eng_name: "Size",
        check_method: CheckingMethod::Size,
    },
    CheckMethodStruct {
        eng_name: "Name",
        check_method: CheckingMethod::Name,
    },
    CheckMethodStruct {
        eng_name: "Size and Name",
        check_method: CheckingMethod::SizeName,
    },
];

#[derive(Copy, Clone)]
pub struct AudioTypeStruct {
    #[allow(unused)]
    pub eng_name: &'static str,
    pub check_method: CheckingMethod,
}

pub const AUDIO_TYPE_CHECK_METHOD_COMBO_BOX: [AudioTypeStruct; 2] = [
    AudioTypeStruct {
        eng_name: "Tags",
        check_method: CheckingMethod::AudioTags,
    },
    AudioTypeStruct {
        eng_name: "Content",
        check_method: CheckingMethod::AudioContent,
    },
];

#[derive(Copy, Clone)]
pub struct SearchModeStruct {
    #[allow(unused)]
    pub eng_name: &'static str,
    pub check_method: SearchMode,
}

pub const BIG_FILES_CHECK_METHOD_COMBO_BOX: [SearchModeStruct; 2] = [
    SearchModeStruct {
        eng_name: "Biggest",
        check_method: SearchMode::BiggestFiles,
    },
    SearchModeStruct {
        eng_name: "Smallest",
        check_method: SearchMode::SmallestFiles,
    },
];

pub struct ImageResizeAlgStruct {
    pub eng_name: &'static str,
    pub filter: FilterType,
}

pub const IMAGES_RESIZE_ALGORITHM_COMBO_BOX: [ImageResizeAlgStruct; 5] = [
    ImageResizeAlgStruct {
        eng_name: "Lanczos3",
        filter: FilterType::Lanczos3,
    },
    ImageResizeAlgStruct {
        eng_name: "Nearest",
        filter: FilterType::Nearest,
    },
    ImageResizeAlgStruct {
        eng_name: "Triangle",
        filter: FilterType::Triangle,
    },
    ImageResizeAlgStruct {
        eng_name: "Gaussian",
        filter: FilterType::Gaussian,
    },
    ImageResizeAlgStruct {
        eng_name: "CatmullRom",
        filter: FilterType::CatmullRom,
    },
];

pub struct ImageHashTypeStruct {
    pub eng_name: &'static str,
    pub hash_alg: HashAlg,
}

pub const IMAGES_HASH_TYPE_COMBO_BOX: &[ImageHashTypeStruct] = &[
    ImageHashTypeStruct {
        eng_name: "Gradient",
        hash_alg: HashAlg::Gradient,
    },
    ImageHashTypeStruct {
        eng_name: "Mean",
        hash_alg: HashAlg::Mean,
    },
    ImageHashTypeStruct {
        eng_name: "VertGradient",
        hash_alg: HashAlg::VertGradient,
    },
    ImageHashTypeStruct {
        eng_name: "Blockhash",
        hash_alg: HashAlg::Blockhash,
    },
    ImageHashTypeStruct {
        eng_name: "DoubleGradient",
        hash_alg: HashAlg::DoubleGradient,
    },
    ImageHashTypeStruct {
        eng_name: "Median",
        hash_alg: HashAlg::Median,
    },
];

pub const IMAGES_HASH_SIZE_COMBO_BOX: [i32; 4] = [8, 16, 32, 64];
