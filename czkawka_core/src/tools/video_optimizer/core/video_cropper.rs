use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use image::{DynamicImage, GenericImageView};
use log::debug;

use crate::common::video_metadata::VideoMetadata;
use crate::tools::video_optimizer::{VideoCropEntry, VideoCropParams};

const BLACK_PIXEL_THRESHOLD: u8 = 20;
const BLACK_BAR_MIN_PERCENTAGE: f32 = 0.9;
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
        Self { top, bottom, left, right }
    }

    fn union(&self, other: &Self) -> Self {
        Self {
            top: self.top.min(other.top),
            bottom: self.bottom.max(other.bottom),
            left: self.left.min(other.left),
            right: self.right.max(other.right),
        }
    }

    fn is_cropping_needed(&self, width: u32, height: u32) -> bool {
        let right_margin = width - self.right;
        let bottom_margin = height - self.bottom;
        self.left > MIN_CROP_SIZE || right_margin > MIN_CROP_SIZE || self.top > MIN_CROP_SIZE || bottom_margin > MIN_CROP_SIZE
    }
}

fn extract_frame_ffmpeg(video_path: &Path, timestamp: f32) -> Option<DynamicImage> {
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

    image::load_from_memory(&output.stdout).ok()
}

fn is_pixel_black(img: &image::RgbImage, x: u32, y: u32) -> bool {
    let pixel = img.get_pixel(x, y);
    pixel[0] < BLACK_PIXEL_THRESHOLD && pixel[1] < BLACK_PIXEL_THRESHOLD && pixel[2] < BLACK_PIXEL_THRESHOLD
}

fn detect_black_bars(img: &DynamicImage) -> Option<Rectangle> {
    let (width, height) = img.dimensions();
    let rgb_img = img.to_rgb8();

    let mut left_crop = 0u32;
    for x in 0..width {
        let black_pixels = (0..height).filter(|&y| is_pixel_black(&rgb_img, x, y)).count();
        if (black_pixels as f32 / height as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        left_crop = x + 1;
    }

    let mut right_margin = 0u32;
    for x in (0..width).rev() {
        let black_pixels = (0..height).filter(|&y| is_pixel_black(&rgb_img, x, y)).count();
        if (black_pixels as f32 / height as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        right_margin = width - x;
    }

    let mut top_crop = 0u32;
    for y in 0..height {
        let black_pixels = (0..width).filter(|&x| is_pixel_black(&rgb_img, x, y)).count();
        if (black_pixels as f32 / width as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        top_crop = y + 1;
    }

    let mut bottom_margin = 0u32;
    for y in (0..height).rev() {
        let black_pixels = (0..width).filter(|&x| is_pixel_black(&rgb_img, x, y)).count();
        if (black_pixels as f32 / width as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        bottom_margin = height - y;
    }

    let rect = Rectangle::new(top_crop, height - bottom_margin, left_crop, width - right_margin);
    if rect.is_cropping_needed(width, height) {
        Some(rect)
    } else {
        None
    }
}

fn analyze_black_bars<F>(duration: f32, get_frame: &F, stop_flag: &Arc<AtomicBool>) -> Result<Option<Rectangle>, String>
where
    F: Fn(f32) -> Option<DynamicImage>,
{
    if stop_flag.load(Ordering::Relaxed) {
        return Err("Operation cancelled".to_string());
    }

    let first_frame = get_frame(0.0).ok_or("Failed to extract first frame")?;

    let Some(mut rectangle) = detect_black_bars(&first_frame) else {
        return Ok(None);
    };

    let mut array_with_rectangles_and_timestamps = vec![(0.0, rectangle)];

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
            array_with_rectangles_and_timestamps.push((timestamp, tmp_rect));
        } else {
            return Ok(None);
        }
    }

    debug!(
        "Black bar analysis complete: {} samples, final rectangle: {:?}",
        array_with_rectangles_and_timestamps.len(),
        rectangle
    );

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

pub fn check_video_crop(mut entry: VideoCropEntry, _params: &VideoCropParams, stop_flag: &Arc<AtomicBool>) -> VideoCropEntry {
    debug!("Checking video for crop: {}", entry.path.display());

    let Ok((width, height, duration, fps)) = extract_video_metadata_for_crop(&mut entry) else { return entry };

    debug!("Video metadata: {}x{}, duration: {:.2}s, fps: {:.2}, codec: {}", width, height, duration, fps, entry.codec);

    let video_path = entry.path.clone();
    let get_frame = |timestamp: f32| -> Option<DynamicImage> { extract_frame_ffmpeg(&video_path, timestamp) };

    match analyze_black_bars(duration as f32, &get_frame, stop_flag) {
        Ok(Some(rectangle)) => {
            debug!(
                "Detected black bars - Left: {}, Top: {}, Right: {}, Bottom: {}",
                rectangle.left, rectangle.top, rectangle.right, rectangle.bottom
            );
            entry.new_image_dimensions = Some((rectangle.left, rectangle.top, rectangle.right, rectangle.bottom));
        }
        Ok(None) => {
            debug!("No black bars detected");
        }
        Err(e) => {
            entry.error = Some(e);
            return entry;
        }
    }

    entry
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use image::{DynamicImage, RgbImage};

    use super::*;

    fn create_colored_frame(width: u32, height: u32, r: u8, g: u8, b: u8) -> DynamicImage {
        let mut img = RgbImage::new(width, height);
        for pixel in img.pixels_mut() {
            *pixel = image::Rgb([r, g, b]);
        }
        DynamicImage::ImageRgb8(img)
    }

    fn create_frame_with_black_bars(width: u32, height: u32, bar_size: u32) -> DynamicImage {
        let mut img = RgbImage::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            if x < bar_size || x >= width - bar_size || y < bar_size || y >= height - bar_size {
                *pixel = image::Rgb([0, 0, 0]);
            } else {
                *pixel = image::Rgb([100, 150, 200]);
            }
        }
        DynamicImage::ImageRgb8(img)
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

        let get_frame = |_timestamp: f32| -> Option<DynamicImage> { Some(create_frame_with_black_bars(200, 200, 20)) };

        let result = analyze_black_bars(duration, &get_frame, &stop_flag);
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_analyze_black_bars_no_bars() {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |_timestamp: f32| -> Option<DynamicImage> { Some(create_colored_frame(200, 200, 100, 150, 200)) };

        let result = analyze_black_bars(duration, &get_frame, &stop_flag);
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_analyze_black_bars_inconsistent_bars() {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |timestamp: f32| -> Option<DynamicImage> {
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

        let get_frame = |_timestamp: f32| -> Option<DynamicImage> { Some(create_frame_with_black_bars(200, 200, 20)) };

        let result = analyze_black_bars(duration, &get_frame, &stop_flag);
        assert!(result.unwrap_err().contains("cancelled"));
    }

    #[test]
    fn test_analyze_black_bars_variable_rectangles() {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let duration = 10.0;

        let get_frame = |timestamp: f32| -> Option<DynamicImage> {
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
        dbg!(&rect);
        assert_eq!(rect.left, 18);
        assert_eq!(rect.top, 18);
        assert_eq!(rect.right, 200 - 18);
        assert_eq!(rect.bottom, 200 - 18);
    }
}
