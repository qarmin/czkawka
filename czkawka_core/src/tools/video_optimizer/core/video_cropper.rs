use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use image::RgbImage;
use log::error;

use crate::common::consts::VIDEO_RESOLUTION_LIMIT;
use crate::common::process_utils::run_command_interruptible;
use crate::common::video_utils::{VideoMetadata, extract_frame_ffmpeg};
use crate::flc;
use crate::tools::video_optimizer::{VideoCropEntry, VideoCropParams, VideoCropSingleFixParams, VideoCroppingMechanism};

const MIN_SAMPLES: usize = 3;
const MIN_SAMPLE_INTERVAL: f32 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rectangle {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

impl Rectangle {
    fn new(top: u32, bottom: u32, left: u32, right: u32) -> Self {
        let s = Self { top, bottom, left, right };
        s.validate();
        s
    }

    fn union(&self, other: &Self) -> Self {
        let s = Self {
            top: self.top.min(other.top),
            bottom: self.bottom.max(other.bottom),
            left: self.left.min(other.left),
            right: self.right.max(other.right),
        };
        s.validate();
        s
    }

    fn validate(&self) {
        assert!(
            self.left <= self.right && self.top <= self.bottom,
            "Invalid rectangle coordinates: top={}, bottom={}, left={}, right={}. Expected: left <= right && top <= bottom (critical algorithm error, please report an issue)",
            self.top,
            self.bottom,
            self.left,
            self.right
        );
    }
    fn validate_image_size(&self, width: u32, height: u32) {
        assert!(
            self.right <= width && self.bottom <= height,
            "Rectangle exceeds image dimensions: image_width={}, image_height={}, rectangle_right={}, rectangle_bottom={}. Expected: right <= image_width && bottom <= image_height (critical algorithm error, please report an issue)",
            width,
            height,
            self.right,
            self.bottom
        );
    }

    fn is_cropping_needed(&self, width: u32, height: u32, min_crop_size: u32) -> bool {
        let right_margin = width - self.right;
        let bottom_margin = height - self.bottom;
        self.left > min_crop_size || right_margin > min_crop_size || self.top > min_crop_size || bottom_margin > min_crop_size
    }
}

fn is_pixel_black(img: &image::RgbImage, x: u32, y: u32, black_pixel_threshold: u8) -> bool {
    let pixel = img.get_pixel(x, y);
    pixel.0.iter().all(|&channel| channel <= black_pixel_threshold)
}

#[derive(Debug)]
enum BlackBarResult {
    NoBlackBars,
    BlackBarsDetected(Rectangle),
    FullBlackImage,
}

fn detect_black_bars(rgb_img: &RgbImage, params: &VideoCropParams) -> BlackBarResult {
    let (width, height) = rgb_img.dimensions();
    let min_percentage = params.black_bar_min_percentage as f32 / 100.0;

    let mut left_crop = 0u32;
    for x in 0..width {
        let black_pixels = (0..height).filter(|&y| is_pixel_black(rgb_img, x, y, params.black_pixel_threshold)).count();
        if (black_pixels as f32 / height as f32) < min_percentage {
            break;
        }
        left_crop = x + 1;
    }

    let mut right_pos = width;
    for x in (0..width).rev() {
        let black_pixels = (0..height).filter(|&y| is_pixel_black(rgb_img, x, y, params.black_pixel_threshold)).count();
        if (black_pixels as f32 / height as f32) < min_percentage {
            right_pos = x + 1;
            break;
        }
    }

    if left_crop >= right_pos {
        return BlackBarResult::FullBlackImage;
    }

    let mut top_crop = 0u32;
    for y in 0..height {
        let black_pixels = (0..width).filter(|&x| is_pixel_black(rgb_img, x, y, params.black_pixel_threshold)).count();
        if (black_pixels as f32 / width as f32) < min_percentage {
            break;
        }
        top_crop = y + 1;
    }

    let mut bottom_pos = height;
    for y in (0..height).rev() {
        let black_pixels = (0..width).filter(|&x| is_pixel_black(rgb_img, x, y, params.black_pixel_threshold)).count();
        if (black_pixels as f32 / width as f32) < min_percentage {
            bottom_pos = y + 1;
            break;
        }
    }

    if top_crop >= bottom_pos {
        return BlackBarResult::FullBlackImage;
    }

    let rect = Rectangle::new(top_crop, bottom_pos, left_crop, right_pos);
    if rect.is_cropping_needed(width, height, params.min_crop_size) {
        BlackBarResult::BlackBarsDetected(rect)
    } else {
        BlackBarResult::NoBlackBars
    }
}

fn analyze_black_bars<F>(
    duration: f32,
    get_frame: &F,
    stop_flag: &Arc<AtomicBool>,
    first_frame: &RgbImage,
    params: &VideoCropParams,
    path: &Path,
) -> Option<Result<Option<Rectangle>, String>>
where
    F: Fn(f32) -> Result<RgbImage, String>,
{
    if stop_flag.load(Ordering::Relaxed) {
        return None;
    }

    let mut rectangle = match detect_black_bars(first_frame, params) {
        BlackBarResult::BlackBarsDetected(rect) => Some(rect),
        BlackBarResult::NoBlackBars => {
            return Some(Ok(None));
        }
        BlackBarResult::FullBlackImage => None,
    };

    let num_samples = ((duration / MIN_SAMPLE_INTERVAL).floor() as usize).clamp(MIN_SAMPLES, params.max_samples);

    for i in 1..num_samples {
        if stop_flag.load(Ordering::Relaxed) {
            return None;
        }

        let timestamp = (i as f32 / num_samples as f32) * duration;

        let tmp_frame = match get_frame(timestamp) {
            Ok(frame) => frame,
            Err(e) => {
                return Some(Err(flc!(
                    "core_failed_get_frame_at_timestamp",
                    file = path.to_string_lossy().to_string(),
                    timestamp = timestamp,
                    reason = e
                )));
            }
        };
        if tmp_frame.dimensions() != first_frame.dimensions() {
            return Some(Err(flc!(
                "core_frame_dimensions_mismatch",
                timestamp = timestamp,
                first_w = first_frame.width(),
                first_h = first_frame.height()
            )));
        }

        match detect_black_bars(&tmp_frame, params) {
            BlackBarResult::BlackBarsDetected(tmp_rect) => {
                rectangle = match rectangle {
                    Some(current_rect) => Some(current_rect.union(&tmp_rect)),
                    None => Some(tmp_rect),
                };
            }
            BlackBarResult::NoBlackBars => {
                return Some(Ok(None));
            }
            BlackBarResult::FullBlackImage => {
                // Do nothing - leave the current rectangle as is
            }
        }
    }
    if let Some(rectangle) = rectangle {
        rectangle.validate();
        // Rectangle may extend step by step to full image size, so that is why previous checks are not enough
        if !rectangle.is_cropping_needed(first_frame.width(), first_frame.height(), params.min_crop_size) {
            return Some(Ok(None));
        }
        Some(Ok(Some(rectangle)))
    } else {
        Some(Ok(None)) // All frames were fully black
    }
}

fn diff_between_dynamic_images(img_original: &RgbImage, mut consumed_temp_img: RgbImage) -> RgbImage {
    assert_eq!(
        img_original.dimensions(),
        consumed_temp_img.dimensions(),
        "Image dimensions do not match for diffing (critical algorithm error, please report an issue)"
    );
    img_original.pixels().zip(consumed_temp_img.pixels_mut()).for_each(|(img_original_pixel, consumed_pixel)| {
        consumed_pixel
            .0
            .iter_mut()
            .zip(img_original_pixel.0.iter())
            .for_each(|(consumed_channel, &original_channel)| {
                *consumed_channel = original_channel.abs_diff(*consumed_channel);
            });
    });
    consumed_temp_img
}

fn analyze_static_image_parts<F>(
    duration: f32,
    get_frame: &F,
    stop_flag: &Arc<AtomicBool>,
    first_frame: &RgbImage,
    params: &VideoCropParams,
    path: &Path,
) -> Option<Result<Option<Rectangle>, String>>
where
    F: Fn(f32) -> Result<RgbImage, String>,
{
    if stop_flag.load(Ordering::Relaxed) {
        return None;
    }
    // Initial rectangle is empty, because with only one frame we cannot determine static parts
    let mut rectangle: Option<Rectangle> = None;

    let num_samples = ((duration / MIN_SAMPLE_INTERVAL).floor() as usize).clamp(MIN_SAMPLES, params.max_samples);

    for i in 1..num_samples {
        if stop_flag.load(Ordering::Relaxed) {
            return None;
        }

        let timestamp = (i as f32 / num_samples as f32) * duration;

        let tmp_frame = match get_frame(timestamp) {
            Ok(frame) => frame,
            Err(e) => {
                return Some(Err(flc!(
                    "core_failed_get_frame_from_file",
                    file = path.to_string_lossy().to_string(),
                    timestamp = timestamp,
                    reason = e
                )));
            }
        };
        if tmp_frame.dimensions() != first_frame.dimensions() {
            return Some(Err(flc!(
                "core_frame_dimensions_mismatch",
                timestamp = timestamp,
                first_w = first_frame.width(),
                first_h = first_frame.height()
            )));
        }
        let dynamic_image_diff: RgbImage = diff_between_dynamic_images(first_frame, tmp_frame);

        match detect_black_bars(&dynamic_image_diff, params) {
            BlackBarResult::FullBlackImage => {
                // Do nothing - leave the current rectangle as is
            }
            BlackBarResult::NoBlackBars => {
                return Some(Ok(None));
            }
            BlackBarResult::BlackBarsDetected(tmp_rect) => {
                rectangle = match rectangle {
                    Some(current_rect) => Some(current_rect.union(&tmp_rect)),
                    None => Some(tmp_rect),
                };
            }
        }
    }

    if let Some(rectangle) = rectangle {
        rectangle.validate();
        // Rectangle may extend step by step to full image size, so that is why previous checks are not enough
        if !rectangle.is_cropping_needed(first_frame.width(), first_frame.height(), params.min_crop_size) {
            return Some(Ok(None));
        }
        Some(Ok(Some(rectangle)))
    } else {
        Some(Ok(None)) // All frames were fully static
    }
}

fn extract_video_metadata_for_crop(entry: &mut VideoCropEntry) -> Result<(u32, u32, f64, f64), ()> {
    let metadata = match VideoMetadata::from_path(&entry.path) {
        Ok(metadata) => metadata,
        Err(e) => {
            entry.error = Some(format!("Failed to get video metadata for file \"{}\": {}", entry.path.to_string_lossy(), e));
            return Err(());
        }
    };

    let Some(current_codec) = metadata.codec.clone() else {
        entry.error = Some(format!("Failed to get video codec from metadata for file \"{}\"", entry.path.to_string_lossy()));
        return Err(());
    };

    entry.codec = current_codec;

    let (width, height) = match (metadata.width, metadata.height) {
        (Some(width), Some(height)) => {
            entry.width = width;
            entry.height = height;
            (width, height)
        }
        _ => {
            entry.error = Some(format!("Failed to get video dimensions from metadata for file \"{}\"", entry.path.to_string_lossy()));
            return Err(());
        }
    };

    let Some(duration) = metadata.duration else {
        entry.error = Some(format!("Failed to get video duration from metadata, for file \"{}\"", entry.path.to_string_lossy()));
        return Err(());
    };

    entry.duration = duration;

    let fps = metadata.fps.unwrap_or(25.0);

    Ok((width, height, duration, fps))
}

pub fn check_video_crop(mut entry: VideoCropEntry, params: &VideoCropParams, stop_flag: &Arc<AtomicBool>) -> Option<VideoCropEntry> {
    let Ok((_width, _height, duration, _fps)) = extract_video_metadata_for_crop(&mut entry) else {
        return Some(entry);
    };

    let video_path = entry.path.clone();
    let get_frame = |timestamp: f32| -> Result<RgbImage, String> { extract_frame_ffmpeg(&video_path, timestamp, None) };

    // TODO - metadata are broken? Not proper?
    // Metadata shows different dimensions than actual frames extracted - quite strange, probably rotated data -
    let first_frame = match get_frame(0.0) {
        Ok(frame) => frame,
        Err(e) => {
            entry.error = Some(format!("Failed to extract first frame for video \"{}\": {}", entry.path.to_string_lossy(), e));
            return Some(entry);
        }
    };

    let (width, height) = first_frame.dimensions();
    entry.height = height;
    entry.width = width;

    if entry.width > VIDEO_RESOLUTION_LIMIT || entry.height > VIDEO_RESOLUTION_LIMIT {
        entry.error = Some(format!(
            "Image dimensions for video \"{}\" exceed the limit: {}x{} > {}x{}",
            entry.path.to_string_lossy(),
            entry.width,
            entry.height,
            VIDEO_RESOLUTION_LIMIT,
            VIDEO_RESOLUTION_LIMIT
        ));
        return Some(entry);
    }

    match params.crop_detect {
        VideoCroppingMechanism::BlackBars => match analyze_black_bars(duration as f32, &get_frame, stop_flag, &first_frame, params, &entry.path) {
            Some(Ok(Some(rectangle))) => {
                rectangle.validate_image_size(width, height);
                entry.new_image_dimensions = (rectangle.left, rectangle.top, rectangle.right, rectangle.bottom);
            }
            Some(Ok(None)) => { // No black bars
            }
            Some(Err(e)) => {
                entry.error = Some(e);
                return Some(entry);
            }
            None => return None,
        },
        VideoCroppingMechanism::StaticContent => match analyze_static_image_parts(duration as f32, &get_frame, stop_flag, &first_frame, params, &entry.path) {
            Some(Ok(Some(rectangle))) => {
                rectangle.validate_image_size(width, height);
                entry.new_image_dimensions = (rectangle.left, rectangle.top, rectangle.right, rectangle.bottom);
            }
            Some(Ok(None)) => {}
            Some(Err(e)) => {
                entry.error = Some(e);
                return Some(entry);
            }
            None => return None,
        },
    }

    Some(entry)
}

pub fn fix_video_crop(video_path: &Path, params: &VideoCropSingleFixParams, stop_flag: &Arc<AtomicBool>, current_codec: &str) -> Result<(), String> {
    if stop_flag.load(Ordering::Relaxed) {
        return Err("Video processing was stopped by user".to_string());
    }

    let (left, top, right, bottom) = params.crop_rectangle;

    if left >= right || top >= bottom {
        return Err(flc!("core_invalid_crop_rectangle", left = left, top = top, right = right, bottom = bottom));
    }

    let crop_width = right - left;
    let crop_height = bottom - top;

    let crop_type_suffix = match params.crop_mechanism {
        VideoCroppingMechanism::BlackBars => "blackbars",
        VideoCroppingMechanism::StaticContent => "staticcontent",
    };

    let extension = video_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let temp_output = video_path.with_extension(format!("czkawka_cropped_{crop_type_suffix}.{extension}"));

    let mut command = Command::new("ffmpeg");
    command.arg("-i").arg(video_path).arg("-vf").arg(format!("crop={crop_width}:{crop_height}:{left}:{top}"));

    match (params.target_codec, params.quality) {
        (None, None) => {
            // Do nothing, do not convert video to different codec
        }
        (Some(target_codec), Some(quality)) => {
            command.arg("-c:v").arg(target_codec.as_str()).arg("-crf").arg(quality.to_string());
        }
        _ => {
            return Err("Both target_codec and quality must be specified together".to_string());
        }
    }

    command.arg("-c:a").arg("copy");
    command.arg("-y").arg(&temp_output);

    match run_command_interruptible(command, stop_flag) {
        None => {
            let _ = std::fs::remove_file(&temp_output);
            return Err(String::from("Video cropping was stopped by user"));
        }
        Some(Err(e)) => {
            let _ = std::fs::remove_file(&temp_output);
            return Err(flc!("core_failed_to_crop_video_file", file = video_path.to_string_lossy(), reason = e));
        }
        Some(Ok(output)) => {
            if !output.status.success() {
                let connected = format!("{} - {}", output.stdout, output.stderr);
                if connected.to_lowercase().contains("unknown encoder") {
                    let missing_codec = match params.target_codec {
                        Some(target_codec) => target_codec.as_ffprobe_codec_name(),
                        None => current_codec,
                    };
                    return Err(flc!("core_ffmpeg_unknown_encoder", file = video_path.to_string_lossy(), encoder = missing_codec));
                }
                error!(
                    "FFmpeg failed to crop video \"{}\" with status {}. Stdout: {}, Stderr: {}",
                    video_path.to_string_lossy(),
                    output.status,
                    output.stdout,
                    output.stderr
                );
                return Err(flc!(
                    "core_ffmpeg_error",
                    file = video_path.to_string_lossy(),
                    code = output.status.to_string(),
                    reason = output.stderr
                ));
            }
        }
    }

    if !temp_output.exists() {
        error!("Cropped video file was not created: {temp_output:?}");
        return Err(flc!("core_cropped_video_not_created", temp = format!("{:?}", temp_output)));
    }

    if params.overwrite_original {
        std::fs::rename(&temp_output, video_path).map_err(|e| format!("Failed to replace original file: {e}"))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use image::RgbImage;

    use super::*;

    fn default_test_params() -> VideoCropParams {
        VideoCropParams {
            crop_detect: VideoCroppingMechanism::BlackBars,
            black_pixel_threshold: 20,
            black_bar_min_percentage: 90,
            max_samples: 60,
            min_crop_size: 5,
            generate_thumbnails: false,
            thumbnail_video_percentage_from_start: 0,
            generate_thumbnail_grid_instead_of_single: false,
            thumbnail_grid_tiles_per_side: 2,
        }
    }

    fn create_colored_frame(width: u32, height: u32, r: u8, g: u8, b: u8) -> RgbImage {
        let mut img = RgbImage::new(width, height);
        for pixel in img.pixels_mut() {
            *pixel = image::Rgb([r, g, b]);
        }
        img
    }

    fn create_frame_with_black_bars(width: u32, height: u32, bar_size: u32) -> RgbImage {
        let mut img = RgbImage::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            if x < bar_size || x >= width - bar_size || y < bar_size || y >= height - bar_size {
                *pixel = image::Rgb([0, 0, 0]);
            } else {
                *pixel = image::Rgb([100, 150, 200]);
            }
        }
        img
    }

    #[test]
    fn test_is_pixel_black() {
        let params = default_test_params();

        let black_img = RgbImage::from_pixel(10, 10, image::Rgb([0, 0, 0]));
        assert!(is_pixel_black(&black_img, 5, 5, params.black_pixel_threshold));

        let light_gray_img = RgbImage::from_pixel(10, 10, image::Rgb([20, 20, 20]));
        assert!(is_pixel_black(&light_gray_img, 5, 5, params.black_pixel_threshold));

        let dark_gray_img = RgbImage::from_pixel(10, 10, image::Rgb([21, 21, 21]));
        assert!(!is_pixel_black(&dark_gray_img, 5, 5, params.black_pixel_threshold));

        let white_img = RgbImage::from_pixel(10, 10, image::Rgb([255, 255, 255]));
        assert!(!is_pixel_black(&white_img, 5, 5, params.black_pixel_threshold));
    }

    #[test]
    fn test_detect_black_bars_no_bars() {
        let params = default_test_params();
        let img = create_colored_frame(100, 100, 100, 150, 200);
        let result = detect_black_bars(&img, &params);
        assert!(matches!(result, BlackBarResult::NoBlackBars));
    }

    #[test]
    fn test_detect_black_bars_with_bars() {
        let params = default_test_params();
        let img = create_frame_with_black_bars(200, 200, 20);
        let result = detect_black_bars(&img, &params);
        if let BlackBarResult::BlackBarsDetected(rect) = result {
            assert!(rect.left >= 15 && rect.left <= 25, "Left crop: {}", rect.left);
            assert!(rect.top >= 15 && rect.top <= 25, "Top crop: {}", rect.top);
            assert!(rect.right >= 175 && rect.right <= 185, "Right position: {}", rect.right);
            assert!(rect.bottom >= 175 && rect.bottom <= 185, "Bottom position: {}", rect.bottom);
        } else {
            panic!("Expected BlackBarsDetected, got {result:?}");
        }
    }

    #[test]
    fn test_detect_black_bars_small_bars() {
        let params = default_test_params();
        let img = create_frame_with_black_bars(200, 200, 3);
        let result = detect_black_bars(&img, &params);
        assert!(matches!(result, BlackBarResult::NoBlackBars));
    }

    #[test]
    fn test_rectangle_union() {
        let rect1 = Rectangle::new(10, 10, 10, 10);
        let rect2 = Rectangle::new(5, 15, 8, 12);
        let union = rect1.union(&rect2);

        assert_eq!(union.top, 5);
        assert_eq!(union.bottom, 15);
        assert_eq!(union.left, 8);
        assert_eq!(union.right, 12);
    }

    #[test]
    fn test_rectangle_is_cropping_needed() {
        let params = default_test_params();

        // Image 100x100, cropped to (10, 10) -> (90, 90), so 10px margin on each side
        let cropping_needed = Rectangle::new(10, 90, 10, 90);
        assert!(cropping_needed.is_cropping_needed(100, 100, params.min_crop_size));

        // Image 100x100, no cropping: (0, 0) -> (100, 100)
        let no_cropping_needed = Rectangle::new(0, 100, 0, 100);
        assert!(!no_cropping_needed.is_cropping_needed(100, 100, params.min_crop_size));

        // Image 100x100, small crop (3px on each side) - below threshold
        let small_crop = Rectangle::new(3, 97, 3, 97);
        assert!(!small_crop.is_cropping_needed(100, 100, params.min_crop_size));
    }

    #[test]
    fn test_analyze_black_bars_consistent_bars() {
        let params = default_test_params();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |_timestamp: f32| -> Result<RgbImage, String> { Ok(create_frame_with_black_bars(200, 200, 20)) };

        let result = analyze_black_bars(
            duration,
            &get_frame,
            &stop_flag,
            &create_frame_with_black_bars(200, 200, 20),
            &params,
            Path::new("text.txt"),
        );
        assert!(result.expect("Expected Result").unwrap().is_some());
    }

    #[test]
    fn test_analyze_black_bars_no_bars() {
        let params = default_test_params();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |_timestamp: f32| -> Result<RgbImage, String> { Ok(create_colored_frame(200, 200, 100, 150, 200)) };

        let result = analyze_black_bars(
            duration,
            &get_frame,
            &stop_flag,
            &create_colored_frame(200, 200, 100, 150, 200),
            &params,
            Path::new("text.txt"),
        );
        assert!(result.expect("Expected Result").unwrap().is_none());
    }

    #[test]
    fn test_analyze_black_bars_inconsistent_bars() {
        let params = default_test_params();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |timestamp: f32| -> Result<RgbImage, String> {
            if timestamp < 5.0 {
                Ok(create_frame_with_black_bars(200, 200, 20))
            } else {
                Ok(create_colored_frame(200, 200, 100, 150, 200))
            }
        };

        let result = analyze_black_bars(
            duration,
            &get_frame,
            &stop_flag,
            &create_frame_with_black_bars(200, 200, 20),
            &params,
            Path::new("text.txt"),
        );
        assert!(result.expect("Expected Result").unwrap().is_none());
    }

    #[test]
    fn test_analyze_black_bars_variable_rectangles() {
        let params = default_test_params();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |timestamp: f32| -> Result<RgbImage, String> {
            if timestamp < 3.0 {
                Ok(create_frame_with_black_bars(200, 200, 20))
            } else if timestamp < 7.0 {
                Ok(create_frame_with_black_bars(200, 200, 18))
            } else {
                Ok(create_frame_with_black_bars(200, 200, 22))
            }
        };

        let result = analyze_black_bars(
            duration,
            &get_frame,
            &stop_flag,
            &create_frame_with_black_bars(200, 200, 20),
            &params,
            Path::new("text.txt"),
        );
        let rect = result.expect("Expected Result").unwrap().unwrap();
        assert_eq!(rect.left, 18);
        assert_eq!(rect.top, 18);
        assert_eq!(rect.right, 200 - 18);
        assert_eq!(rect.bottom, 200 - 18);
    }

    #[test]
    fn test_detect_black_bars_fuzzer() {
        let params = default_test_params();
        let test_cases = vec![
            (1, 1, "1x1 image"),
            (1, 100, "1 pixel wide"),
            (100, 1, "1 pixel tall"),
            (2, 2, "2x2 minimum"),
            (10, 10, "10x10 small"),
            (100, 100, "100x100 medium"),
            (1920, 1080, "1920x1080 Full HD"),
            (3840, 2160, "3840x2160 4K"),
        ];

        for (width, height, desc) in test_cases {
            // Test 1: All black image
            let mut all_black = RgbImage::new(width, height);
            for pixel in all_black.pixels_mut() {
                *pixel = image::Rgb([0, 0, 0]);
            }
            let result = detect_black_bars(&all_black, &params);
            assert!(matches!(result, BlackBarResult::FullBlackImage), "All black image should return FullBlackImage for {desc}");

            // Test 2: All white image
            let mut all_white = RgbImage::new(width, height);
            for pixel in all_white.pixels_mut() {
                *pixel = image::Rgb([255, 255, 255]);
            }
            let result = detect_black_bars(&all_white, &params);
            assert!(matches!(result, BlackBarResult::NoBlackBars), "All white image should return NoBlackBars for {desc}");

            // Test 4: Checkerboard pattern (no black bars)
            if width > 4 && height > 4 {
                let mut checkerboard = RgbImage::new(width, height);
                for (x, y, pixel) in checkerboard.enumerate_pixels_mut() {
                    let color = if (x + y) % 2 == 0 { 255 } else { 0 };
                    *pixel = image::Rgb([color, color, color]);
                }
                let result = detect_black_bars(&checkerboard, &params);
                assert!(matches!(result, BlackBarResult::NoBlackBars), "Checkerboard should return NoBlackBars for {desc}");
            }
        }
    }
}
