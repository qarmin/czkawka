use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use image::RgbImage;
use log::debug;

use crate::common::video_metadata::VideoMetadata;
use crate::tools::video_optimizer::{VideoCropEntry, VideoCropFixParams, VideoCropParams, VideoCroppingMechanism};

const BLACK_PIXEL_THRESHOLD: u8 = 20;
const BLACK_BAR_MIN_PERCENTAGE: f32 = 0.95;
const MIN_SAMPLE_INTERVAL: f32 = 0.25;
const MAX_SAMPLES: usize = 60;
const MIN_CROP_SIZE: u32 = 5;

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
            "Invalid rectangle coordinates: top={}, bottom={}, left={}, right={}. Expected: left <= right && top <= bottom",
            self.top,
            self.bottom,
            self.left,
            self.right
        );
    }

    fn is_cropping_needed(&self, width: u32, height: u32) -> bool {
        let right_margin = width - self.right;
        let bottom_margin = height - self.bottom;
        self.left > MIN_CROP_SIZE || right_margin > MIN_CROP_SIZE || self.top > MIN_CROP_SIZE || bottom_margin > MIN_CROP_SIZE
    }
}

fn extract_frame_ffmpeg(video_path: &Path, timestamp: f32) -> Option<RgbImage> {
    let output = Command::new("ffmpeg")
        .arg("-threads")
        .arg("1")
        .arg("-ss")
        .arg(timestamp.to_string())
        .arg("-i")
        .arg(video_path)
        .arg("-vframes")
        .arg("1")
        .arg("-f")
        .arg("image2pipe")
        .arg("-pix_fmt") // TODO - newly added - may be broken
        .arg("rgb24") // TODO
        .arg("-vcodec")
        .arg("png")
        .arg("pipe:1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    image::load_from_memory(&output.stdout).ok().map(|img| img.into_rgb8())
}

fn is_pixel_black(img: &image::RgbImage, x: u32, y: u32) -> bool {
    let pixel = img.get_pixel(x, y);
    pixel.0.iter().all(|&channel| channel < BLACK_PIXEL_THRESHOLD)
}

fn detect_black_bars(rgb_img: &RgbImage) -> Option<Rectangle> {
    let (width, height) = rgb_img.dimensions();

    let mut left_crop = 0u32;
    for x in 0..width {
        let black_pixels = (0..height).filter(|&y| is_pixel_black(rgb_img, x, y)).count();
        if (black_pixels as f32 / height as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        left_crop = x + 1;
    }

    let mut right_pos = width;
    for x in (0..width).rev() {
        let black_pixels = (0..height).filter(|&y| is_pixel_black(rgb_img, x, y)).count();
        if (black_pixels as f32 / height as f32) < BLACK_BAR_MIN_PERCENTAGE {
            right_pos = x + 1;
            break;
        }
    }

    if left_crop >= right_pos {
        return None;
    }

    let mut top_crop = 0u32;
    for y in 0..height {
        let black_pixels = (0..width).filter(|&x| is_pixel_black(rgb_img, x, y)).count();
        if (black_pixels as f32 / width as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        top_crop = y + 1;
    }

    let mut bottom_pos = height;
    for y in (0..height).rev() {
        let black_pixels = (0..width).filter(|&x| is_pixel_black(rgb_img, x, y)).count();
        if (black_pixels as f32 / width as f32) < BLACK_BAR_MIN_PERCENTAGE {
            bottom_pos = y + 1;
            break;
        }
    }

    if top_crop >= bottom_pos {
        return None;
    }

    let rect = Rectangle::new(top_crop, bottom_pos, left_crop, right_pos);
    if rect.is_cropping_needed(width, height) { Some(rect) } else { None }
}

fn analyze_black_bars<F>(duration: f32, get_frame: &F, stop_flag: &Arc<AtomicBool>) -> Result<Option<Rectangle>, String>
where
    F: Fn(f32) -> Option<RgbImage>,
{
    if stop_flag.load(Ordering::Relaxed) {
        return Err("Operation cancelled".to_string());
    }

    let first_frame = get_frame(0.0).ok_or("Failed to extract first frame")?;

    let Some(mut rectangle) = detect_black_bars(&first_frame) else {
        return Ok(None);
    };

    let num_samples = (duration / MIN_SAMPLE_INTERVAL).min(MAX_SAMPLES as f32).floor() as usize;
    let num_samples = num_samples.max(1);

    for i in 1..num_samples {
        if stop_flag.load(Ordering::Relaxed) {
            return Err("Operation cancelled".to_string());
        }

        let timestamp = (i as f32 / num_samples as f32) * duration;

        let Some(tmp_frame) = get_frame(timestamp) else {
            return Ok(None);
        };

        if let Some(tmp_rect) = detect_black_bars(&tmp_frame) {
            rectangle = rectangle.union(&tmp_rect);
        } else {
            return Ok(None);
        }
    }

    Ok(Some(rectangle))
}

fn diff_between_dynamic_images(img_original: &RgbImage, mut consumed_temp_img: RgbImage) -> RgbImage {
    assert_eq!(img_original.dimensions(), consumed_temp_img.dimensions(), "Image dimensions do not match for diffing");
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

fn analyze_static_image_parts<F>(duration: f32, get_frame: &F, stop_flag: &Arc<AtomicBool>) -> Result<Option<Rectangle>, String>
where
    F: Fn(f32) -> Option<RgbImage>,
{
    if stop_flag.load(Ordering::Relaxed) {
        return Err("Operation cancelled".to_string());
    }

    let first_frame = get_frame(0.0).ok_or("Failed to extract first frame")?;
    let mut rectangle = Rectangle::new(0, first_frame.height(), 0, first_frame.width());

    let num_samples = (duration / MIN_SAMPLE_INTERVAL).min(MAX_SAMPLES as f32).floor() as usize;
    let num_samples = num_samples.max(1);

    for i in 1..num_samples {
        if stop_flag.load(Ordering::Relaxed) {
            return Err("Operation cancelled".to_string());
        }

        let timestamp = (i as f32 / num_samples as f32) * duration;

        let Some(tmp_frame) = get_frame(timestamp) else {
            return Ok(None);
        };
        let dynamic_image_diff: RgbImage = diff_between_dynamic_images(&first_frame, tmp_frame);

        if let Some(tmp_rect) = detect_black_bars(&dynamic_image_diff) {
            rectangle = rectangle.union(&tmp_rect);
        } else {
            return Ok(None);
        }
    }

    Ok(Some(rectangle))
}

fn extract_video_metadata_for_crop(entry: &mut VideoCropEntry) -> Result<(u32, u32, f64, f64), ()> {
    let metadata = match VideoMetadata::from_path(&entry.path) {
        Ok(metadata) => metadata,
        Err(e) => {
            entry.error = Some(e);
            return Err(());
        }
    };

    let Some(current_codec) = metadata.codec.clone() else {
        entry.error = Some("Failed to get video codec".to_string());
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
            entry.error = Some("Failed to get video dimensions".to_string());
            return Err(());
        }
    };

    let Some(duration) = metadata.duration else {
        entry.error = Some("Failed to get video duration".to_string());
        return Err(());
    };

    let fps = metadata.fps.unwrap_or(25.0);

    Ok((width, height, duration, fps))
}

pub fn check_video_crop(mut entry: VideoCropEntry, params: &VideoCropParams, stop_flag: &Arc<AtomicBool>) -> VideoCropEntry {
    debug!("Checking video for crop: {}", entry.path.display());

    let Ok((width, height, duration, fps)) = extract_video_metadata_for_crop(&mut entry) else {
        return entry;
    };

    debug!("Video metadata: {}x{}, duration: {:.2}s, fps: {:.2}, codec: {}", width, height, duration, fps, entry.codec);

    let video_path = entry.path.clone();
    let get_frame = |timestamp: f32| -> Option<RgbImage> { extract_frame_ffmpeg(&video_path, timestamp) };

    match params.crop_detect {
        VideoCroppingMechanism::BlackBars => match analyze_black_bars(duration as f32, &get_frame, stop_flag) {
            Ok(Some(rectangle)) => {
                entry.new_image_dimensions = Some((rectangle.left, rectangle.top, rectangle.right, rectangle.bottom));
            }
            Ok(None) => {
                debug!("No black bars detected");
            }
            Err(e) => {
                entry.error = Some(e);
                return entry;
            }
        },
        VideoCroppingMechanism::StaticContent => match analyze_static_image_parts(duration as f32, &get_frame, stop_flag) {
            Ok(Some(rectangle)) => {
                entry.new_image_dimensions = Some((rectangle.left, rectangle.top, rectangle.right, rectangle.bottom));
            }
            Ok(None) => {
                debug!("No static content detected");
            }
            Err(e) => {
                entry.error = Some(e);
                return entry;
            }
        },
    }

    entry
}

pub fn fix_video_crop(entry: &VideoCropEntry, params: &VideoCropFixParams, stop_flag: &Arc<AtomicBool>) -> Result<(), String> {
    if stop_flag.load(Ordering::Relaxed) {
        return Err("Operation cancelled".to_string());
    }

    let (left, top, right, bottom) = params.crop_rectangle;

    if left >= right || top >= bottom {
        return Err(format!("Invalid crop rectangle: left={left}, top={top}, right={right}, bottom={bottom}"));
    }

    let crop_width = right - left;
    let crop_height = bottom - top;

    if crop_width == 0 || crop_height == 0 {
        return Err("Crop dimensions cannot be zero".to_string());
    }

    let output_path = if params.overwrite_original {
        entry.path.with_extension("tmp.mp4")
    } else {
        let stem = entry.path.file_stem().ok_or("Cannot get file stem")?;
        let parent = entry.path.parent().ok_or("Cannot get parent directory")?;
        let extension = entry.path.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
        parent.join(format!("{}_cropped.{}", stem.to_string_lossy(), extension))
    };

    debug!(
        "Cropping video: {} -> {}, crop: {}x{}+{}+{}",
        entry.path.display(),
        output_path.display(),
        crop_width,
        crop_height,
        left,
        top
    );

    let mut ffmpeg_cmd = Command::new("ffmpeg");
    ffmpeg_cmd
        .arg("-i")
        .arg(&entry.path)
        .arg("-vf")
        .arg(format!("crop={crop_width}:{crop_height}:{left}:{top}"));

    // Add codec parameters if target codec is specified
    if let Some(target_codec) = params.target_codec {
        ffmpeg_cmd.arg("-c:v").arg(target_codec.as_str());

        // Add quality parameter if specified
        if let Some(quality) = params.quality {
            ffmpeg_cmd.arg("-crf").arg(quality.to_string());
        }

        // Copy audio stream
        ffmpeg_cmd.arg("-c:a").arg("copy");
    } else {
        // Copy both video and audio streams
        ffmpeg_cmd.arg("-c").arg("copy");
    }

    ffmpeg_cmd
        .arg("-y") // Overwrite output file
        .arg(&output_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let output = ffmpeg_cmd.output().map_err(|e| format!("Failed to execute ffmpeg: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FFmpeg failed: {stderr}"));
    }

    // If overwriting, move temp file to original
    if params.overwrite_original {
        std::fs::rename(&output_path, &entry.path).map_err(|e| format!("Failed to replace original file: {e}"))?;
    }

    debug!("Successfully cropped video: {}", entry.path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use image::RgbImage;

    use super::*;

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
        let black_img = RgbImage::from_pixel(10, 10, image::Rgb([0, 0, 0]));
        assert!(is_pixel_black(&black_img, 5, 5));

        let dark_gray_img = RgbImage::from_pixel(10, 10, image::Rgb([19, 19, 19]));
        assert!(is_pixel_black(&dark_gray_img, 5, 5));

        let light_gray_img = RgbImage::from_pixel(10, 10, image::Rgb([20, 20, 20]));
        assert!(!is_pixel_black(&light_gray_img, 5, 5));

        let white_img = RgbImage::from_pixel(10, 10, image::Rgb([255, 255, 255]));
        assert!(!is_pixel_black(&white_img, 5, 5));
    }

    #[test]
    fn test_detect_black_bars_no_bars() {
        let img = create_colored_frame(100, 100, 100, 150, 200);
        let result = detect_black_bars(&img);
        assert!(result.is_none());
    }

    #[test]
    fn test_detect_black_bars_with_bars() {
        let img = create_frame_with_black_bars(200, 200, 20);
        let result = detect_black_bars(&img);
        assert!(result.is_some());

        let rect = result.unwrap();
        assert!(rect.left >= 15 && rect.left <= 25, "Left crop: {}", rect.left);
        assert!(rect.top >= 15 && rect.top <= 25, "Top crop: {}", rect.top);
        assert!(rect.right >= 175 && rect.right <= 185, "Right position: {}", rect.right);
        assert!(rect.bottom >= 175 && rect.bottom <= 185, "Bottom position: {}", rect.bottom);
    }

    #[test]
    fn test_detect_black_bars_small_bars() {
        let img = create_frame_with_black_bars(200, 200, 3);
        let result = detect_black_bars(&img);
        assert!(result.is_none());
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
        // Image 100x100, cropped to (10, 10) -> (90, 90), so 10px margin on each side
        let cropping_needed = Rectangle::new(10, 90, 10, 90);
        assert!(cropping_needed.is_cropping_needed(100, 100));

        // Image 100x100, no cropping: (0, 0) -> (100, 100)
        let no_cropping_needed = Rectangle::new(0, 100, 0, 100);
        assert!(!no_cropping_needed.is_cropping_needed(100, 100));

        // Image 100x100, small crop (3px on each side) - below threshold
        let small_crop = Rectangle::new(3, 97, 3, 97);
        assert!(!small_crop.is_cropping_needed(100, 100));
    }

    #[test]
    fn test_analyze_black_bars_consistent_bars() {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |_timestamp: f32| -> Option<RgbImage> { Some(create_frame_with_black_bars(200, 200, 20)) };

        let result = analyze_black_bars(duration, &get_frame, &stop_flag);
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_analyze_black_bars_no_bars() {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |_timestamp: f32| -> Option<RgbImage> { Some(create_colored_frame(200, 200, 100, 150, 200)) };

        let result = analyze_black_bars(duration, &get_frame, &stop_flag);
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_analyze_black_bars_inconsistent_bars() {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |timestamp: f32| -> Option<RgbImage> {
            if timestamp < 5.0 {
                Some(create_frame_with_black_bars(200, 200, 20))
            } else {
                Some(create_colored_frame(200, 200, 100, 150, 200))
            }
        };

        let result = analyze_black_bars(duration, &get_frame, &stop_flag);
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_analyze_black_bars_with_stop_flag() {
        let stop_flag = Arc::new(AtomicBool::new(true));
        let duration = 10.0;

        let get_frame = |_timestamp: f32| -> Option<RgbImage> { Some(create_frame_with_black_bars(200, 200, 20)) };

        let result = analyze_black_bars(duration, &get_frame, &stop_flag);
        assert!(result.unwrap_err().contains("cancelled"));
    }

    #[test]
    fn test_analyze_black_bars_variable_rectangles() {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |timestamp: f32| -> Option<RgbImage> {
            if timestamp < 3.0 {
                Some(create_frame_with_black_bars(200, 200, 20))
            } else if timestamp < 7.0 {
                Some(create_frame_with_black_bars(200, 200, 18))
            } else {
                Some(create_frame_with_black_bars(200, 200, 22))
            }
        };

        let result = analyze_black_bars(duration, &get_frame, &stop_flag);
        let rectangle = result.unwrap();
        let rect = rectangle.unwrap();
        assert_eq!(rect.left, 18);
        assert_eq!(rect.top, 18);
        assert_eq!(rect.right, 200 - 18);
        assert_eq!(rect.bottom, 200 - 18);
    }

    #[test]
    fn test_detect_black_bars_fuzzer() {
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
            let result = detect_black_bars(&all_black);
            assert!(result.is_none(), "All black image should return None for {desc}");

            // Test 2: All white image
            let mut all_white = RgbImage::new(width, height);
            for pixel in all_white.pixels_mut() {
                *pixel = image::Rgb([255, 255, 255]);
            }
            let result = detect_black_bars(&all_white);
            assert!(result.is_none(), "All white image should return None for {desc}");

            // Test 3: Single white pixel in center
            if width > 2 && height > 2 {
                let mut single_pixel = RgbImage::new(width, height);
                for pixel in single_pixel.pixels_mut() {
                    *pixel = image::Rgb([0, 0, 0]);
                }
                single_pixel.put_pixel(width / 2, height / 2, image::Rgb([255, 255, 255]));
                let result = detect_black_bars(&single_pixel);
                if let Some(rect) = result {
                    assert!(rect.left < rect.right, "Invalid rectangle for single pixel in {desc}: left >= right");
                    assert!(rect.top < rect.bottom, "Invalid rectangle for single pixel in {desc}: top >= bottom");
                    assert!(rect.right <= width, "Right exceeds width in {desc}");
                    assert!(rect.bottom <= height, "Bottom exceeds height in {desc}");
                }
            }

            // Test 4: Checkerboard pattern (no black bars)
            if width > 4 && height > 4 {
                let mut checkerboard = RgbImage::new(width, height);
                for (x, y, pixel) in checkerboard.enumerate_pixels_mut() {
                    let color = if (x + y) % 2 == 0 { 255 } else { 0 };
                    *pixel = image::Rgb([color, color, color]);
                }
                let result = detect_black_bars(&checkerboard);
                assert!(result.is_none(), "Checkerboard should return None for {desc}");
            }

            // Test 5: Black bars on all sides
            if width > 40 && height > 40 {
                let bar_size = 10;
                let mut with_bars = RgbImage::new(width, height);
                for (x, y, pixel) in with_bars.enumerate_pixels_mut() {
                    if x < bar_size || x >= width - bar_size || y < bar_size || y >= height - bar_size {
                        *pixel = image::Rgb([0, 0, 0]);
                    } else {
                        *pixel = image::Rgb([128, 128, 128]);
                    }
                }
                let result = detect_black_bars(&with_bars);
                if let Some(rect) = result {
                    assert!(rect.left > 0, "Should detect left black bar in {desc}");
                    assert!(rect.top > 0, "Should detect top black bar in {desc}");
                    assert!(rect.right < width, "Should detect right black bar in {desc}");
                    assert!(rect.bottom < height, "Should detect bottom black bar in {desc}");
                    assert!(rect.left < rect.right, "Invalid rectangle in {desc}: left >= right");
                    assert!(rect.top < rect.bottom, "Invalid rectangle in {desc}: top >= bottom");
                }
            }

            // Test 6: Only left and right black bars (letterbox)
            if width > 40 && height > 20 {
                let bar_size = 10;
                let mut letterbox = RgbImage::new(width, height);
                for (x, _y, pixel) in letterbox.enumerate_pixels_mut() {
                    if x < bar_size || x >= width - bar_size {
                        *pixel = image::Rgb([0, 0, 0]);
                    } else {
                        *pixel = image::Rgb([128, 128, 128]);
                    }
                }
                let result = detect_black_bars(&letterbox);
                if let Some(rect) = result {
                    assert!(rect.left < rect.right, "Invalid letterbox rectangle in {desc}");
                    assert!(rect.top < rect.bottom, "Invalid letterbox rectangle in {desc}");
                }
            }

            // Test 7: Only top and bottom black bars (pillarbox)
            if width > 20 && height > 40 {
                let bar_size = 10;
                let mut pillarbox = RgbImage::new(width, height);
                for (_x, y, pixel) in pillarbox.enumerate_pixels_mut() {
                    if y < bar_size || y >= height - bar_size {
                        *pixel = image::Rgb([0, 0, 0]);
                    } else {
                        *pixel = image::Rgb([128, 128, 128]);
                    }
                }
                let result = detect_black_bars(&pillarbox);
                if let Some(rect) = result {
                    assert!(rect.left < rect.right, "Invalid pillarbox rectangle in {desc}");
                    assert!(rect.top < rect.bottom, "Invalid pillarbox rectangle in {desc}");
                }
            }

            // Test 8: Asymmetric black bars
            if width > 50 && height > 50 {
                let mut asymmetric = RgbImage::new(width, height);
                let left_bar = 5;
                let right_bar = 15;
                let top_bar = 10;
                let bottom_bar = 20;
                for (x, y, pixel) in asymmetric.enumerate_pixels_mut() {
                    if x < left_bar || x >= width - right_bar || y < top_bar || y >= height - bottom_bar {
                        *pixel = image::Rgb([0, 0, 0]);
                    } else {
                        *pixel = image::Rgb([200, 200, 200]);
                    }
                }
                let result = detect_black_bars(&asymmetric);
                if let Some(rect) = result {
                    assert!(rect.left < rect.right, "Invalid asymmetric rectangle in {desc}");
                    assert!(rect.top < rect.bottom, "Invalid asymmetric rectangle in {desc}");
                    assert!(rect.left >= left_bar, "Left bar not detected properly in {desc}");
                    assert!(rect.right <= width - right_bar, "Right bar not detected properly in {desc}");
                }
            }
        }
    }
}
