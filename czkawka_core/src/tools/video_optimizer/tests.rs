#[cfg(test)]
mod tests2 {
    use crate::tools::video_optimizer::{VideoOptimizer, VideoOptimizerParameters, VideoTranscodeParams};

    #[test]
    fn test_video_optimizer_creation() {
        let params = VideoOptimizerParameters::VideoTranscode(VideoTranscodeParams { excluded_codecs: Vec::new() });
        let optimizer = VideoOptimizer::new(params);

        assert_eq!(optimizer.get_information().number_of_processed_files, 0);
        assert_eq!(optimizer.get_information().number_of_failed_files, 0);
    }
}
