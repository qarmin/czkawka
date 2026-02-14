use std::fmt::Debug;
use std::sync::{Arc, Mutex, MutexGuard};

use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::re_exported::{Cropdetect, HashAlg};
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::video_optimizer::{VideoCodec, VideoCroppingMechanism, VideoOptimizerMode};
use image::imageops::FilterType;
use log::warn;
use slint::SharedString;

use crate::connect_translation::LANGUAGE_LIST;

#[derive(Debug, Clone)]
pub struct StringComboBoxItem<T>
where
    T: Clone + Debug,
{
    pub config_name: String,
    pub display_name: String,
    pub value: T,
}

pub struct StringComboBoxItems {
    pub languages: Vec<StringComboBoxItem<String>>,
    pub hash_size: Vec<StringComboBoxItem<u8>>,
    pub resize_algorithm: Vec<StringComboBoxItem<FilterType>>,
    pub image_hash_alg: Vec<StringComboBoxItem<HashAlg>>,
    pub duplicates_hash_type: Vec<StringComboBoxItem<HashType>>,
    pub biggest_files_method: Vec<StringComboBoxItem<SearchMode>>,
    pub audio_check_type: Vec<StringComboBoxItem<CheckingMethod>>,
    pub duplicates_check_method: Vec<StringComboBoxItem<CheckingMethod>>,
    pub videos_crop_detect: Vec<StringComboBoxItem<Cropdetect>>,
    pub video_optimizer_crop_type: Vec<StringComboBoxItem<VideoCroppingMechanism>>,
    pub video_optimizer_mode: Vec<StringComboBoxItem<VideoOptimizerMode>>,
    pub video_optimizer_video_codec: Vec<StringComboBoxItem<VideoCodec>>,
}

pub static STRING_COMBO_BOX_ITEMS: std::sync::LazyLock<Arc<Mutex<StringComboBoxItems>>> = std::sync::LazyLock::new(|| {
    let l = StringComboBoxItems::regenerate_items();
    Arc::new(Mutex::new(l))
});

impl StringComboBoxItems {
    pub(crate) fn get_item_and_idx_from_config_name<T>(config_name: &str, items: &Vec<StringComboBoxItem<T>>) -> (usize, Vec<SharedString>)
    where
        T: Clone + Debug,
    {
        let position = items.iter().position(|e| e.config_name == config_name).unwrap_or_else(|| {
            warn!("Trying to get non existent item - \"{config_name}\" from {items:?}");
            0
        });
        let display_names = items.iter().map(|e| e.display_name.clone().into()).collect::<Vec<_>>();
        (position, display_names)
    }

    pub(crate) fn regenerate_items() -> Self {
        let languages = LANGUAGE_LIST
            .iter()
            .map(|e| StringComboBoxItem {
                config_name: e.short_name.to_string(),
                display_name: e.long_name.to_string(),
                value: e.short_name.to_string(),
            })
            .collect();

        let hash_size = Self::convert_to_combobox_items(&[("8", "8", 8), ("16", "16", 16), ("32", "32", 32), ("64", "64", 64)]);
        let resize_algorithm = Self::convert_to_combobox_items(&[
            ("lanczos3", "Lanczos3", FilterType::Lanczos3),
            ("gaussian", "Gaussian", FilterType::Gaussian),
            ("catmullrom", "CatmullRom", FilterType::CatmullRom),
            ("triangle", "Triangle", FilterType::Triangle),
            ("nearest", "Nearest", FilterType::Nearest),
        ]);

        let image_hash_alg = Self::convert_to_combobox_items(&[
            ("mean", "Mean", HashAlg::Mean),
            ("gradient", "Gradient", HashAlg::Gradient),
            ("blockhash", "BlockHash", HashAlg::Blockhash),
            ("vertgradient", "VertGradient", HashAlg::VertGradient),
            ("doublegradient", "DoubleGradient", HashAlg::DoubleGradient),
            ("median", "Median", HashAlg::Median),
        ]);

        let duplicates_hash_type = Self::convert_to_combobox_items(&[
            ("blake3", "Blake3", HashType::Blake3),
            ("crc32", "CRC32", HashType::Crc32),
            ("xxh3", "XXH3", HashType::Xxh3),
        ]);

        let biggest_files_method = Self::convert_to_combobox_items(&[
            ("biggest", "The Biggest", SearchMode::BiggestFiles),
            ("smallest", "The Smallest", SearchMode::SmallestFiles),
        ]);

        let audio_check_type = Self::convert_to_combobox_items(&[("tags", "Tags", CheckingMethod::AudioTags), ("fingerprint", "Fingerprint", CheckingMethod::AudioContent)]);

        let duplicates_check_method = Self::convert_to_combobox_items(&[
            ("hash", "Hash", CheckingMethod::Hash),
            ("size", "Size", CheckingMethod::Size),
            ("name", "Name", CheckingMethod::Name),
            ("size_and_name", "Size and Name", CheckingMethod::SizeName),
        ]);

        let videos_crop_detect = Self::convert_to_combobox_items(&[
            ("letterbox", "LetterBox", Cropdetect::Letterbox),
            ("motion", "Motion", Cropdetect::Motion),
            ("none", "None", Cropdetect::None),
        ]);

        let video_optimizer_crop_type = Self::convert_to_combobox_items(&[
            ("blackbars", "Black Bars", VideoCroppingMechanism::BlackBars),
            ("staticcontent", "Static Content", VideoCroppingMechanism::StaticContent),
        ]);

        let video_optimizer_mode = Self::convert_to_combobox_items(&[
            ("crop", "Crop", VideoOptimizerMode::VideoCrop),
            ("transcode", "Transcode", VideoOptimizerMode::VideoTranscode),
        ]);

        let video_optimizer_video_codec = Self::convert_to_combobox_items(&[
            ("h265", "HEVC/H265", VideoCodec::H265),
            ("h264", "H264", VideoCodec::H264),
            ("vp9", "VP9", VideoCodec::Vp9),
            ("av1", "AV1", VideoCodec::Av1),
        ]);

        Self {
            languages,
            hash_size,
            resize_algorithm,
            image_hash_alg,
            duplicates_hash_type,
            biggest_files_method,
            audio_check_type,
            duplicates_check_method,
            videos_crop_detect,
            video_optimizer_crop_type,
            video_optimizer_mode,
            video_optimizer_video_codec,
        }
    }

    fn convert_to_combobox_items<T>(input: &[(&str, &str, T)]) -> Vec<StringComboBoxItem<T>>
    where
        T: Clone + Debug,
    {
        input
            .iter()
            .map(|(config_name, display_name, value)| StringComboBoxItem {
                config_name: config_name.to_string(),
                display_name: display_name.to_string(),
                value: value.clone(),
            })
            .collect()
    }

    pub(crate) fn get_items() -> MutexGuard<'static, Self> {
        STRING_COMBO_BOX_ITEMS.lock().expect("Can't lock string combobox items")
    }

    pub(crate) fn regenerate_and_set() {
        *STRING_COMBO_BOX_ITEMS.lock().expect("Can't lock string combobox items") = Self::regenerate_items();
    }

    pub(crate) fn get_display_names<T: Debug + Clone>(items: &[StringComboBoxItem<T>]) -> Vec<SharedString> {
        items.iter().map(|e| e.display_name.clone().into()).collect()
    }
}
