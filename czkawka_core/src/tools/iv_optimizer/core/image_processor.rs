use std::fs;
use std::path::Path;

use image::GenericImageView;
use log::{debug, error, info};

use crate::tools::iv_optimizer::{BoundingBox, ImageTrimEntry};

pub fn check_image(mut entry: ImageTrimEntry, threshold: u8) -> ImageTrimEntry {
    debug!("Checking image: {}", entry.path.display());

    let img = match image::open(&entry.path) {
        Ok(img) => img,
        Err(e) => {
            error!("Failed to open image {}: {}", entry.path.display(), e);
            entry.error = Some(format!("Failed to open image: {e}"));
            return entry;
        }
    };

    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    let (top, bottom, left, right) = find_trim_bounds(&rgb_img, threshold);

    if top == 0 && bottom == height && left == 0 && right == width {
        debug!("No trimming needed for {}", entry.path.display());
        return entry;
    }

    entry.bounding_box = Some(BoundingBox { top, bottom, left, right });
    entry
}

pub fn process_image(image_path: &Path, bounding_box: &BoundingBox) -> Result<u64, String> {
    debug!("Processing image: {}", image_path.display());

    let img = image::open(image_path).map_err(|e| format!("Failed to open image: {e}"))?;

    let (width, height) = img.dimensions();

    if bounding_box.top >= bounding_box.bottom || bounding_box.left >= bounding_box.right {
        return Err("Invalid bounding box".to_string());
    }

    let cropped = image::imageops::crop_imm(
        &img,
        bounding_box.left,
        bounding_box.top,
        bounding_box.right - bounding_box.left,
        bounding_box.bottom - bounding_box.top,
    )
    .to_image();

    let temp_output = image_path.with_extension(format!("trimmed.{}", image_path.extension().and_then(|e| e.to_str()).unwrap_or("jpg")));

    cropped.save(&temp_output).map_err(|e| format!("Failed to save image: {e}"))?;

    let metadata = fs::metadata(&temp_output).map_err(|e| {
        let _ = fs::remove_file(&temp_output);
        format!("Failed to get metadata: {e}")
    })?;

    let new_size = metadata.len();

    fs::rename(&temp_output, image_path).map_err(|e| {
        let _ = fs::remove_file(&temp_output);
        format!("Failed to replace file: {e}")
    })?;

    info!(
        "Successfully trimmed image: {} ({} bytes, {}x{} -> {}x{})",
        image_path.display(),
        new_size,
        width,
        height,
        bounding_box.right - bounding_box.left,
        bounding_box.bottom - bounding_box.top
    );

    Ok(new_size)
}

pub fn process_image_legacy(entry: &mut ImageTrimEntry) {
    let Some(bb) = &entry.bounding_box else {
        return;
    };

    match process_image(&entry.path, bb) {
        Ok(new_size) => {
            entry.new_size = Some(new_size);
        }
        Err(e) => {
            entry.error = Some(e);
        }
    }
}

fn find_trim_bounds(img: &image::RgbImage, threshold: u8) -> (u32, u32, u32, u32) {
    let (width, height) = img.dimensions();

    let mut top = 0;
    'outer_top: for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if !is_border_pixel(*pixel, threshold) {
                break 'outer_top;
            }
        }
        top = y + 1;
    }

    let mut bottom = height;
    'outer_bottom: for y in (0..height).rev() {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if !is_border_pixel(*pixel, threshold) {
                break 'outer_bottom;
            }
        }
        bottom = y;
    }

    let mut left = 0;
    'outer_left: for x in 0..width {
        for y in top..bottom {
            let pixel = img.get_pixel(x, y);
            if !is_border_pixel(*pixel, threshold) {
                break 'outer_left;
            }
        }
        left = x + 1;
    }

    let mut right = width;
    'outer_right: for x in (0..width).rev() {
        for y in top..bottom {
            let pixel = img.get_pixel(x, y);
            if !is_border_pixel(*pixel, threshold) {
                break 'outer_right;
            }
        }
        right = x;
    }

    if top >= bottom || left >= right {
        return (0, height, 0, width);
    }

    (top, bottom, left, right)
}

fn is_border_pixel(pixel: image::Rgb<u8>, threshold: u8) -> bool {
    let [r, g, b] = pixel.0;
    let high_threshold = 255 - threshold;

    (r < threshold && g < threshold && b < threshold) || (r > high_threshold && g > high_threshold && b > high_threshold)
}
