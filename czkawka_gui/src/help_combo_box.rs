use img_hash::{FilterType, HashAlg};

use czkawka_core::duplicate::{CheckingMethod, HashType};

pub struct HashTypeStruct {
    pub eng_name: &'static str,
    pub hash_type: HashType,
}
pub const DUPLICATES_HASH_TYPE_COMBO_BOX: [HashTypeStruct; 3] = [
    HashTypeStruct {
        eng_name: "Blake3",
        hash_type: HashType::Blake3,
    },
    HashTypeStruct { eng_name: "CRC32", hash_type: HashType::Crc32 },
    HashTypeStruct { eng_name: "XXH3", hash_type: HashType::Xxh3 },
];
pub struct CheckMethodStruct {
    pub eng_name: &'static str,
    pub check_method: CheckingMethod,
}

pub const DUPLICATES_CHECK_METHOD_COMBO_BOX: [CheckMethodStruct; 3] = [
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

pub const IMAGES_HASH_TYPE_COMBO_BOX: [ImageHashTypeStruct; 5] = [
    ImageHashTypeStruct {
        eng_name: "Gradient",
        hash_alg: HashAlg::Gradient,
    },
    ImageHashTypeStruct { eng_name: "Mean", hash_alg: HashAlg::Mean },
    ImageHashTypeStruct {
        eng_name: "VertGradient",
        hash_alg: HashAlg::VertGradient,
    },
    ImageHashTypeStruct {
        eng_name: "Blackhash",
        hash_alg: HashAlg::Blockhash,
    },
    ImageHashTypeStruct {
        eng_name: "DoubleGradient",
        hash_alg: HashAlg::DoubleGradient,
    },
];

pub const IMAGES_HASH_SIZE_COMBO_BOX: [i32; 4] = [8, 16, 32, 64];
