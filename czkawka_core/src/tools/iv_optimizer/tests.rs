#[cfg(test)]
mod tests2 {
    use crate::tools::iv_optimizer::{IVOptimizer, IVOptimizerParameters, OptimizerMode, VideoCodec};

    #[test]
    fn test_iv_optimizer_creation() {
        let params = IVOptimizerParameters::default();
        let optimizer = IVOptimizer::new(params);

        assert_eq!(optimizer.get_information().number_of_processed_files, 0);
        assert_eq!(optimizer.get_information().number_of_failed_files, 0);
    }

    #[test]
    fn test_parameters_builder() {
        let params = IVOptimizerParameters::new(OptimizerMode::VideoTranscode {
            codec: VideoCodec::H265,
            quality: 25,
        });

        assert_eq!(
            params.mode,
            OptimizerMode::VideoTranscode {
                codec: VideoCodec::H265,
                quality: 25
            }
        );
    }

    #[test]
    fn test_video_codec_str() {
        assert_eq!(VideoCodec::H265.as_str(), "libx265");
        assert_eq!(VideoCodec::Av1.as_str(), "libaom-av1");
        assert_eq!(VideoCodec::Vp9.as_str(), "libvpx-vp9");
    }

    #[test]
    fn test_default_parameters() {
        let params = IVOptimizerParameters::default();

        assert_eq!(
            params.mode,
            OptimizerMode::VideoTranscode {
                codec: VideoCodec::H265,
                quality: 23
            }
        );
    }
}
