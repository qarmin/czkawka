use std::fmt::Debug;
use std::sync::{Arc, Mutex, MutexGuard};

use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::localizer_core::LANGUAGE_LIST;
use czkawka_core::re_exported::HashAlg;
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::similar_images::GeometricInvariance;
use czkawka_core::tools::video_optimizer::{NoiseReductionMethod, VideoCodec, VideoCroppingMechanism, VideoOptimizerMode};
use image::imageops::FilterType;
use log::warn;
use slint::SharedString;

use crate::localizer_krokiet::LANGUAGE_LOADER_KROKIET;

#[expect(dead_code)]
pub enum DisplaySpec {
    Const(&'static str),
    Translatable(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarVideosVisualPreset {
    Custom,
    NearIdentical,
    Similar,
    Movies,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarVideosAudioPreset {
    Custom,
    Identical,
    Clip,
    Similar,
}

#[derive(Debug, Clone)]
pub struct StringComboBoxItem<T>
where
    T: Clone + Debug,
{
    pub config_name: String,
    pub display_name: String,
    pub value: T,
}

// When item is not a number(like hash_size), but it is e.g. Language - pl, en etc.
// It cannot be set as:
// pub language: Vec<StringComboBoxItem<u8>>
// Where u8 is index in array - because this makes everything harder to work, than with enums
// and additionally it isn't safe to
pub struct StringComboBoxItems {
    pub languages: Vec<StringComboBoxItem<String>>,
    pub hash_size: Vec<StringComboBoxItem<u8>>,
    pub resize_algorithm: Vec<StringComboBoxItem<FilterType>>,
    pub image_hash_alg: Vec<StringComboBoxItem<HashAlg>>,
    pub image_geometric_invariance: Vec<StringComboBoxItem<GeometricInvariance>>,
    pub duplicates_hash_type: Vec<StringComboBoxItem<HashType>>,
    pub biggest_files_method: Vec<StringComboBoxItem<SearchMode>>,
    pub audio_check_type: Vec<StringComboBoxItem<CheckingMethod>>,
    pub duplicates_check_method: Vec<StringComboBoxItem<CheckingMethod>>,
    pub video_optimizer_crop_type: Vec<StringComboBoxItem<VideoCroppingMechanism>>,
    pub video_optimizer_mode: Vec<StringComboBoxItem<VideoOptimizerMode>>,
    pub video_optimizer_video_codec: Vec<StringComboBoxItem<VideoCodec>>,
    pub video_optimizer_noise_reduction: Vec<StringComboBoxItem<NoiseReductionMethod>>,
    pub similar_videos_visual_preset: Vec<StringComboBoxItem<SimilarVideosVisualPreset>>,
    pub similar_videos_audio_preset: Vec<StringComboBoxItem<SimilarVideosAudioPreset>>,
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

        let image_geometric_invariance = Self::convert_to_combobox_items(&[
            ("off", "Off", GeometricInvariance::Off),
            ("mirror_flip", "Mirror + Flip", GeometricInvariance::MirrorFlip),
            ("mirror_flip_rotate90", "Mirror + Flip + Rotate 90", GeometricInvariance::MirrorFlipRotate90),
        ]);

        let duplicates_hash_type = Self::convert_to_combobox_items(&[
            ("blake3", "Blake3", HashType::Blake3),
            ("crc32", "CRC32", HashType::Crc32),
            ("xxh3", "XXH3", HashType::Xxh3),
        ]);

        let biggest_files_method = Self::convert_to_combobox_items_i18n(&[
            ("biggest", SearchMode::BiggestFiles, DisplaySpec::Translatable("option_search_mode_biggest")),
            ("smallest", SearchMode::SmallestFiles, DisplaySpec::Translatable("option_search_mode_smallest")),
        ]);

        let audio_check_type = Self::convert_to_combobox_items_i18n(&[
            ("tags", CheckingMethod::AudioTags, DisplaySpec::Translatable("option_music_method_tags")),
            ("fingerprint", CheckingMethod::AudioContent, DisplaySpec::Translatable("option_music_method_fingerprint")),
        ]);

        let duplicates_check_method = Self::convert_to_combobox_items_i18n(&[
            ("hash", CheckingMethod::Hash, DisplaySpec::Translatable("option_check_method_hash")),
            ("size", CheckingMethod::Size, DisplaySpec::Translatable("option_check_method_size")),
            ("name", CheckingMethod::Name, DisplaySpec::Translatable("option_check_method_name")),
            ("size_and_name", CheckingMethod::SizeName, DisplaySpec::Translatable("option_check_method_size_and_name")),
        ]);

        let video_optimizer_crop_type = Self::convert_to_combobox_items_i18n(&[
            (
                "blackbars",
                VideoCroppingMechanism::BlackBars,
                DisplaySpec::Translatable("option_video_crop_type_black_bars"),
            ),
            (
                "staticcontent",
                VideoCroppingMechanism::StaticContent,
                DisplaySpec::Translatable("option_video_crop_type_static_content"),
            ),
        ]);

        let video_optimizer_mode = Self::convert_to_combobox_items_i18n(&[
            ("crop", VideoOptimizerMode::VideoCrop, DisplaySpec::Translatable("option_video_optimizer_mode_crop")),
            (
                "transcode",
                VideoOptimizerMode::VideoTranscode,
                DisplaySpec::Translatable("option_video_optimizer_mode_transcode"),
            ),
        ]);

        let video_optimizer_video_codec = Self::convert_to_combobox_items(&[
            ("h265", "HEVC/H265", VideoCodec::H265),
            ("h264", "H264", VideoCodec::H264),
            ("vp9", "VP9", VideoCodec::Vp9),
            ("av1", "AV1", VideoCodec::Av1),
        ]);

        let video_optimizer_noise_reduction = Self::convert_to_combobox_items_i18n(&[
            ("none", NoiseReductionMethod::None, DisplaySpec::Translatable("option_noise_reduction_none")),
            ("hqdn3d", NoiseReductionMethod::Hqdn3d, DisplaySpec::Translatable("option_noise_reduction_hqdn3d")),
        ]);

        let similar_videos_visual_preset = Self::convert_to_combobox_items_i18n(&[
            (
                "custom",
                SimilarVideosVisualPreset::Custom,
                DisplaySpec::Translatable("subsettings_videos_visual_preset_custom"),
            ),
            (
                "near_identical",
                SimilarVideosVisualPreset::NearIdentical,
                DisplaySpec::Translatable("subsettings_videos_visual_preset_near_identical"),
            ),
            (
                "similar",
                SimilarVideosVisualPreset::Similar,
                DisplaySpec::Translatable("subsettings_videos_visual_preset_similar"),
            ),
            (
                "movies",
                SimilarVideosVisualPreset::Movies,
                DisplaySpec::Translatable("subsettings_videos_visual_preset_movies"),
            ),
        ]);

        let similar_videos_audio_preset = Self::convert_to_combobox_items_i18n(&[
            (
                "custom",
                SimilarVideosAudioPreset::Custom,
                DisplaySpec::Translatable("subsettings_videos_audio_preset_custom"),
            ),
            (
                "identical",
                SimilarVideosAudioPreset::Identical,
                DisplaySpec::Translatable("subsettings_videos_audio_preset_identical"),
            ),
            ("clip", SimilarVideosAudioPreset::Clip, DisplaySpec::Translatable("subsettings_videos_audio_preset_clip")),
            (
                "similar",
                SimilarVideosAudioPreset::Similar,
                DisplaySpec::Translatable("subsettings_videos_audio_preset_similar"),
            ),
        ]);

        Self {
            languages,
            hash_size,
            resize_algorithm,
            image_hash_alg,
            image_geometric_invariance,
            duplicates_hash_type,
            biggest_files_method,
            audio_check_type,
            duplicates_check_method,
            video_optimizer_crop_type,
            video_optimizer_mode,
            video_optimizer_video_codec,
            video_optimizer_noise_reduction,
            similar_videos_visual_preset,
            similar_videos_audio_preset,
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

    fn convert_to_combobox_items_i18n<T>(input: &[(&str, T, DisplaySpec)]) -> Vec<StringComboBoxItem<T>>
    where
        T: Clone + Debug,
    {
        input
            .iter()
            .map(|(config_name, value, spec)| {
                let display_name = match spec {
                    DisplaySpec::Const(s) => s.to_string(),
                    DisplaySpec::Translatable(key) => LANGUAGE_LOADER_KROKIET.get(key),
                };
                StringComboBoxItem {
                    config_name: config_name.to_string(),
                    display_name,
                    value: value.clone(),
                }
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
