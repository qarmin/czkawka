use std::path::Path;
use std::process::{Command, Stdio};

use image::{DynamicImage, GenericImageView};
use log::debug;

use crate::common::video_metadata::VideoMetadata;
use crate::tools::video_optimizer::{VideoCropEntry, VideoCropParams};

// Zaimplementuj cropopwanie, jako przyjemny do czytania i testów funkcjonalność - NIE USUWAJ TEGO OPISU
//
// To ma:
// - znajdywać w przybliżeniu początek i koniec sensowny wideo(jeśli np. przez 5 sekund jest ten sam obraz, to znaczy że to intro/outro i można usunąć)
// - ma znajdywać czarne pasy na górze/dole/lewej prawje strony(w przyszłości tobędą statyczne części video, ale obecnie tylko czarne pasy)
//
// Algorythm ma działać tak
// - masz duration i działasz na nim
// Pobierasz z grubsza pierwszą klatkę, ostatnią klatkę - w sensie z grubsza, bo nie chcesz dekodować całego wideo klatka po klatce
// Jeśli tutaj nie ma ani czarnych pasów to odpuszczas krok sprawdzania czarnych pasów
// Ale dalej musisz sprawdzic czy pocztek i koniec mogą być przycięte
// Przycięcie początku i końca, powinno być przycięte do około 0.5 sekundy dokładności - czyli 0s - obraz A, 0.5s - obraz A, 1s - obraz B, to znaczy że można przyciąć do 0.5s
// Analogicznie na końcu wideo
// W obu przypadkach, najlepiej skorzystać z wyszukiwania binarnego - ale tylko po początkowych krokach
// A początkowe kroki to(dla dużych video, bo dla małych to będzie przetwarzane szybkeij):
// Sprawdź pierwszą i ostantią klatkę, sprawdź potem w krokach 5s, 30s, 100s, 300s - jeśli znajdziemy rożnicę, to suzkamy binarnie pomiedzy np. 30 - 100, czyli 65 etc.
// Komentarze (inne niż ten opis co robić) pousuwaj
// Dodaj testy - test ma sobie określać jakie wideo ma być tworzone (np. 10s czarnego, 20s kolorowego, 10s czarnego), i dla określonego timestampu, ma generować określone obrazy z czarnym etc. a potem weryfikować czy jest ok rezultaa
// To jest operacja długotrwała, więc ma być dodany stop_flag do przerwania, po każdej wyciągniętej klatce sprawdzamy czy stop_flag jest ustawiony i jeśli tak to przerywamy działanie
// Limituj ffmpeg do max 1 wątku
// Zmniejsz liczbę wyciągnięć klatek - pierwsza, ostatnia i 1 bliska początku i końca są wspólne, potem przy pomocy jakiś flag pomocniczych, sprawdzaj czy black bars potrzebuje gdzieś klatki czy i czy ten drugi tryb potrzebuje, by nie robić tego podwójnie

// Constants for detection
const BLACK_PIXEL_THRESHOLD: u8 = 20; // Pixels below this value are considered black
const BLACK_BAR_MIN_PERCENTAGE: f32 = 0.9; // 90% of row/column must be black
const STATIC_FRAME_SIMILARITY_THRESHOLD: f32 = 0.98; // 98% similarity means static frame
const BINARY_SEARCH_PRECISION: f32 = 0.5; // Search precision in seconds
const INITIAL_CHECK_INTERVALS: &[f32] = &[0.0, 5.0, 30.0, 100.0, 300.0]; // Initial sampling points in seconds

/// Extract a single frame from video at given timestamp
fn extract_frame_at_timestamp(video_path: &Path, timestamp: f32) -> Result<DynamicImage, String> {
    let output = Command::new("ffmpeg")
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
        .map_err(|e| format!("Failed to execute ffmpeg: {e}"))?;

    if !output.status.success() {
        return Err("ffmpeg failed to extract frame".to_string());
    }

    let img = image::load_from_memory(&output.stdout).map_err(|e| format!("Failed to decode frame: {e}"))?;
    Ok(img)
}

/// Calculate similarity between two images (0.0 = completely different, 1.0 = identical)
fn calculate_frame_similarity(img1: &DynamicImage, img2: &DynamicImage) -> f32 {
    let (w1, h1) = img1.dimensions();
    let (w2, h2) = img2.dimensions();

    if w1 != w2 || h1 != h2 {
        return 0.0;
    }

    let img1_rgb = img1.to_rgb8();
    let img2_rgb = img2.to_rgb8();

    let total_pixels = (w1 * h1) as usize;
    let mut similar_pixels = 0usize;

    for y in 0..h1 {
        for x in 0..w1 {
            let p1 = img1_rgb.get_pixel(x, y);
            let p2 = img2_rgb.get_pixel(x, y);

            let diff_r = (p1[0] as i32 - p2[0] as i32).abs();
            let diff_g = (p1[1] as i32 - p2[1] as i32).abs();
            let diff_b = (p1[2] as i32 - p2[2] as i32).abs();

            // Allow small differences (up to 10 per channel)
            if diff_r <= 10 && diff_g <= 10 && diff_b <= 10 {
                similar_pixels += 1;
            }
        }
    }

    similar_pixels as f32 / total_pixels as f32
}

/// Detect black bars in an image (returns: left, top, right, bottom crop amounts)
fn detect_black_bars(img: &DynamicImage) -> Option<(u32, u32, u32, u32)> {
    let (width, height) = img.dimensions();
    let rgb_img = img.to_rgb8();

    let mut left_crop = 0u32;
    let mut right_crop = 0u32;
    let mut top_crop = 0u32;
    let mut bottom_crop = 0u32;

    // Check left edge
    for x in 0..width {
        let black_pixels = (0..height).filter(|&y| is_pixel_black(&rgb_img, x, y)).count();
        if (black_pixels as f32 / height as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        left_crop = x + 1;
    }

    // Check right edge
    for x in (0..width).rev() {
        let black_pixels = (0..height).filter(|&y| is_pixel_black(&rgb_img, x, y)).count();
        if (black_pixels as f32 / height as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        right_crop = width - x;
    }

    // Check top edge
    for y in 0..height {
        let black_pixels = (0..width).filter(|&x| is_pixel_black(&rgb_img, x, y)).count();
        if (black_pixels as f32 / width as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        top_crop = y + 1;
    }

    // Check bottom edge
    for y in (0..height).rev() {
        let black_pixels = (0..width).filter(|&x| is_pixel_black(&rgb_img, x, y)).count();
        if (black_pixels as f32 / width as f32) < BLACK_BAR_MIN_PERCENTAGE {
            break;
        }
        bottom_crop = height - y;
    }

    // Only return if we found significant black bars (at least 5 pixels)
    if left_crop > 5 || right_crop > 5 || top_crop > 5 || bottom_crop > 5 {
        Some((left_crop, top_crop, right_crop, bottom_crop))
    } else {
        None
    }
}

/// Check if a pixel is considered black
fn is_pixel_black(img: &image::RgbImage, x: u32, y: u32) -> bool {
    let pixel = img.get_pixel(x, y);
    pixel[0] < BLACK_PIXEL_THRESHOLD && pixel[1] < BLACK_PIXEL_THRESHOLD && pixel[2] < BLACK_PIXEL_THRESHOLD
}

/// Find the point where video content actually starts (after static intro frames)
fn find_content_start(video_path: &Path, duration: f32) -> Result<Option<f32>, String> {
    // Don't bother with very short videos
    if duration < 10.0 {
        return Ok(None);
    }

    let first_frame = extract_frame_at_timestamp(video_path, 0.0)?;

    // Initial coarse search
    let mut last_static_time = 0.0;
    let mut search_intervals = INITIAL_CHECK_INTERVALS
        .iter()
        .copied()
        .filter(|&t| t < duration)
        .collect::<Vec<_>>();

    // Add last frame check
    search_intervals.push(duration - 1.0);

    for &check_time in &search_intervals {
        if check_time <= last_static_time {
            continue;
        }

        let frame = extract_frame_at_timestamp(video_path, check_time)?;
        let similarity = calculate_frame_similarity(&first_frame, &frame);

        if similarity >= STATIC_FRAME_SIMILARITY_THRESHOLD {
            last_static_time = check_time;
        } else {
            // Found difference, now binary search between last_static_time and check_time
            if check_time - last_static_time > BINARY_SEARCH_PRECISION * 2.0 {
                return binary_search_content_boundary(video_path, &first_frame, last_static_time, check_time, true);
            }
            return Ok(if last_static_time > 0.5 {
                Some(last_static_time)
            } else {
                None
            });
        }
    }

    // If we got here, the entire video might be static (shouldn't happen in practice)
    Ok(None)
}

/// Find the point where video content actually ends (before static outro frames)
fn find_content_end(video_path: &Path, duration: f32) -> Result<Option<f32>, String> {
    // Don't bother with very short videos
    if duration < 10.0 {
        return Ok(None);
    }

    let last_frame = extract_frame_at_timestamp(video_path, duration - 1.0)?;

    // Initial coarse search (going backwards)
    let mut first_static_time = duration;
    let mut search_intervals = INITIAL_CHECK_INTERVALS
        .iter()
        .copied()
        .filter(|&t| t < duration)
        .map(|t| duration - t)
        .collect::<Vec<_>>();

    search_intervals.sort_by(|a, b| b.partial_cmp(a).unwrap());

    for &check_time in &search_intervals {
        if check_time >= first_static_time {
            continue;
        }

        let frame = extract_frame_at_timestamp(video_path, check_time)?;
        let similarity = calculate_frame_similarity(&last_frame, &frame);

        if similarity >= STATIC_FRAME_SIMILARITY_THRESHOLD {
            first_static_time = check_time;
        } else {
            // Found difference, now binary search between check_time and first_static_time
            if first_static_time - check_time > BINARY_SEARCH_PRECISION * 2.0 {
                return binary_search_content_boundary(video_path, &last_frame, check_time, first_static_time, false);
            }
            return Ok(if duration - first_static_time > 0.5 {
                Some(first_static_time)
            } else {
                None
            });
        }
    }

    // If we got here, the entire video might be static (shouldn't happen in practice)
    Ok(None)
}

/// Binary search to find exact boundary between static and dynamic content
fn binary_search_content_boundary(
    video_path: &Path,
    reference_frame: &DynamicImage,
    mut left: f32,
    mut right: f32,
    _is_start: bool,
) -> Result<Option<f32>, String> {
    while right - left > BINARY_SEARCH_PRECISION {
        let mid = (left + right) / 2.0;
        let frame = extract_frame_at_timestamp(video_path, mid)?;
        let similarity = calculate_frame_similarity(reference_frame, &frame);

        if similarity >= STATIC_FRAME_SIMILARITY_THRESHOLD {
            left = mid;
        } else {
            right = mid;
        }
    }

    Ok(if left > 0.5 { Some(left) } else { None })
}

pub fn check_video_crop(mut entry: VideoCropEntry, _params: &VideoCropParams) -> VideoCropEntry {
    debug!("Checking video for crop: {}", entry.path.display());

    // Extract basic metadata
    let metadata = match VideoMetadata::from_path(&entry.path) {
        Ok(metadata) => metadata,
        Err(e) => {
            entry.error = Some(e);
            return entry;
        }
    };

    let Some(current_codec) = metadata.codec.clone() else {
        entry.error = Some("Failed to get video codec".to_string());
        return entry;
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
            return entry;
        }
    };

    let Some(duration) = metadata.duration else {
        entry.error = Some("Failed to get video duration".to_string());
        return entry;
    };

    let fps = metadata.fps.unwrap_or(25.0); // Default to 25 fps if not available

    debug!(
        "Video metadata: {}x{}, duration: {:.2}s, fps: {:.2}, codec: {}",
        width, height, duration, fps, entry.codec
    );

    // Step 1: Check for black bars by examining first and last frames
    let black_bars = match extract_frame_at_timestamp(&entry.path, 1.0) {
        Ok(first_frame) => {
            if let Some(bars) = detect_black_bars(&first_frame) {
                // Verify with last frame
                if let Ok(last_frame) = extract_frame_at_timestamp(&entry.path, (duration - 1.0) as f32) {
                    if let Some(bars_last) = detect_black_bars(&last_frame) {
                        // Use the average of both detections for better accuracy
                        Some((
                            (bars.0 + bars_last.0) / 2,
                            (bars.1 + bars_last.1) / 2,
                            (bars.2 + bars_last.2) / 2,
                            (bars.3 + bars_last.3) / 2,
                        ))
                    } else {
                        Some(bars)
                    }
                } else {
                    Some(bars)
                }
            } else {
                None
            }
        }
        Err(e) => {
            debug!("Failed to extract frame for black bar detection: {}", e);
            None
        }
    };

    if let Some((left, top, right, bottom)) = black_bars {
        debug!(
            "Detected black bars - Left: {}, Top: {}, Right: {}, Bottom: {}",
            left, top, right, bottom
        );
        entry.new_image_dimensions = Some((left, top, right, bottom));
    }

    // Step 2: Check for static frames at start and end
    let start_crop_time = match find_content_start(&entry.path, duration as f32) {
        Ok(Some(time)) => {
            debug!("Detected static intro ending at {:.2}s", time);
            // Convert time to frame number
            Some((time * fps as f32) as u32)
        }
        Ok(None) => None,
        Err(e) => {
            debug!("Failed to detect content start: {}", e);
            None
        }
    };

    let end_crop_time = match find_content_end(&entry.path, duration as f32) {
        Ok(Some(time)) => {
            debug!("Detected static outro starting at {:.2}s", time);
            // Convert time to frame number
            Some((time * fps as f32) as u32)
        }
        Ok(None) => None,
        Err(e) => {
            debug!("Failed to detect content end: {}", e);
            None
        }
    };

    entry.start_crop_frame = start_crop_time;
    entry.end_crop_frame = end_crop_time;

    if entry.new_image_dimensions.is_some() || entry.start_crop_frame.is_some() || entry.end_crop_frame.is_some() {
        debug!("Video can be cropped - found optimization opportunities");
    } else {
        debug!("No cropping opportunities found for this video");
    }

    entry
}
