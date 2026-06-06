use crate::flc;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VideoCodec {
    H264,
    H265,
    Av1,
    Vp9,
}

impl VideoCodec {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::H264 => "libx264",
            Self::H265 => "libx265",
            Self::Av1 => "libsvtav1",
            Self::Vp9 => "libvpx-vp9",
        }
    }

    pub const fn as_ffprobe_codec_name(self) -> &'static str {
        match self {
            Self::H264 => "h264",
            Self::H265 => "h265",
            Self::Av1 => "av1",
            Self::Vp9 => "vp9",
        }
    }
}

impl std::str::FromStr for VideoCodec {
    type Err = String;

    fn from_str(codec: &str) -> Result<Self, Self::Err> {
        match codec.to_lowercase().as_str() {
            "h264" | "libx264" => Ok(Self::H264),
            "h265" | "hevc" | "libx265" => Ok(Self::H265),
            "av1" | "libaom-av1" | "libsvtav1" | "svtav1" => Ok(Self::Av1),
            "vp9" | "libvpx-vp9" => Ok(Self::Vp9),
            _ => Err(flc!("core_unknown_codec", codec = codec)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum HardwareEncoder {
    #[default]
    None,
    Nvenc,
    Vaapi,
    Qsv,
    VideoToolbox,
    Amf,
}

impl HardwareEncoder {
    pub const fn as_config_name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Nvenc => "nvenc",
            Self::Vaapi => "vaapi",
            Self::Qsv => "qsv",
            Self::VideoToolbox => "videotoolbox",
            Self::Amf => "amf",
        }
    }

    pub const fn as_display_name(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Nvenc => "NVENC",
            Self::Vaapi => "VAAPI",
            Self::Qsv => "QSV",
            Self::VideoToolbox => "VideoToolbox",
            Self::Amf => "AMF",
        }
    }

    pub const fn all_non_none() -> &'static [Self] {
        &[Self::Nvenc, Self::Vaapi, Self::Qsv, Self::VideoToolbox, Self::Amf]
    }

    /// Returns encoder-specific quality arguments for the given quality value.
    ///
    /// Each hardware encoder family uses a different rate-control mechanism:
    /// - NVENC: VBR with a constant quality target (`-rc:v vbr -cq:v`)
    /// - VAAPI / QSV: Intelligent Constant Quality (`-global_quality`)
    /// - VideoToolbox: quality scale 1-100 (`-q:v`)
    /// - AMF: constant QP for I- and P-frames (`-rc cqp -qp_i -qp_p`)
    pub fn quality_args(self, quality: u32) -> Vec<String> {
        let q = quality.to_string();
        match self {
            Self::None => vec!["-crf".into(), q],
            Self::Nvenc => vec!["-rc:v".into(), "vbr".into(), "-cq:v".into(), q],
            Self::Vaapi | Self::Qsv => vec!["-global_quality".into(), q],
            Self::VideoToolbox => vec!["-q:v".into(), q],
            Self::Amf => vec!["-rc".into(), "cqp".into(), "-qp_i".into(), quality.to_string(), "-qp_p".into(), quality.to_string()],
        }
    }

    /// Returns the ffmpeg hardware encoder name for the given codec, or None if unsupported.
    pub const fn encoder_name_for_codec(self, codec: VideoCodec) -> Option<&'static str> {
        match (self, codec) {
            (Self::Nvenc, VideoCodec::H264) => Some("h264_nvenc"),
            (Self::Nvenc, VideoCodec::H265) => Some("hevc_nvenc"),
            (Self::Nvenc, VideoCodec::Av1) => Some("av1_nvenc"),
            (Self::Vaapi, VideoCodec::H264) => Some("h264_vaapi"),
            (Self::Vaapi, VideoCodec::H265) => Some("hevc_vaapi"),
            (Self::Qsv, VideoCodec::H264) => Some("h264_qsv"),
            (Self::Qsv, VideoCodec::H265) => Some("hevc_qsv"),
            (Self::Qsv, VideoCodec::Av1) => Some("av1_qsv"),
            (Self::VideoToolbox, VideoCodec::H264) => Some("h264_videotoolbox"),
            (Self::VideoToolbox, VideoCodec::H265) => Some("hevc_videotoolbox"),
            (Self::Amf, VideoCodec::H264) => Some("h264_amf"),
            (Self::Amf, VideoCodec::H265) => Some("hevc_amf"),
            (Self::Amf, VideoCodec::Av1) => Some("av1_amf"),
            // VP9 has no widely available hardware encoders; AV1+VideoToolbox not supported
            _ => None,
        }
    }
}

impl std::str::FromStr for HardwareEncoder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "nvenc" => Ok(Self::Nvenc),
            "vaapi" => Ok(Self::Vaapi),
            "qsv" => Ok(Self::Qsv),
            "videotoolbox" => Ok(Self::VideoToolbox),
            "amf" => Ok(Self::Amf),
            _ => Err(format!("Unknown hardware encoder: {s}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum NoiseReductionMethod {
    #[default]
    None,
    Hqdn3d,
}

impl NoiseReductionMethod {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Hqdn3d => "hqdn3d",
        }
    }

    pub fn to_ffmpeg_filter(&self, strength: u32) -> Option<String> {
        let s = strength.clamp(1, 10) as f32;
        match self {
            Self::None => None,
            Self::Hqdn3d => Some(format!("hqdn3d={:.1}:{:.1}:{:.1}:{:.1}", s * 0.8, s * 0.6, s * 1.2, s * 0.9)),
        }
    }
}

impl std::str::FromStr for NoiseReductionMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "hqdn3d" => Ok(Self::Hqdn3d),
            _ => Err(format!("Unknown noise reduction method: {s}")),
        }
    }
}
