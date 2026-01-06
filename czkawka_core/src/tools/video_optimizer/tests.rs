#[cfg(test)]
mod tests2 {
    use crate::tools::video_optimizer::{OptimizerMode, VideoCodec, VideoOptimizer, VideoOptimizerParameters, VideoOptimizerFixParams};

    #[test]
    fn test_video_optimizer_creation() {
        let params = VideoOptimizerParameters::default();
        let fix_params = VideoOptimizerFixParams::VideoTranscode {
            codec: VideoCodec::H265,
            quality: 23,
        };
        let optimizer = VideoOptimizer::new(params, fix_params);

        assert_eq!(optimizer.get_information().number_of_processed_files, 0);
        assert_eq!(optimizer.get_information().number_of_failed_files, 0);
    }
}
