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
