use std::fs::metadata;
use std::path::Path;
use std::sync::{Arc, Mutex};

use czkawka_core::common::image::{check_if_can_display_image, get_dynamic_image_from_path};
use czkawka_core::helpers::debug_timer::Timer;
use fast_image_resize::{FilterType, ResizeAlg, ResizeOptions, Resizer};
use image::{DynamicImage, Rgba, RgbaImage};
use log::{debug, error};
use slint::ComponentHandle;

use crate::shared_models::SharedModels;
use crate::{ActiveTab, Callabler, GuiState, MainWindow, Settings};

pub type ImageBufferRgba = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub(crate) fn connect_show_preview(app: &MainWindow, shared_models: Arc<Mutex<SharedModels>>) {
    let a = app.as_weak();
    app.global::<Callabler>()
        .on_load_image_preview(move |image_path, crop_left, crop_top, crop_right, crop_bottom, orig_width, orig_height| {
            let app = a.upgrade().expect("Failed to upgrade app :(");

            let settings = app.global::<Settings>();
            let gui_state = app.global::<GuiState>();

            let active_tab = gui_state.get_active_tab();

            if !((active_tab == ActiveTab::SimilarImages && settings.get_similar_images_show_image_preview())
                || (active_tab == ActiveTab::DuplicateFiles && settings.get_duplicate_image_preview())
                || ((active_tab == ActiveTab::SimilarVideos || active_tab == ActiveTab::VideoOptimizer) && settings.get_video_thumbnails_preview()))
            {
                set_preview_visible(&gui_state, None);
                return;
            }

            if !check_if_can_display_image(&image_path) {
                set_preview_visible(&gui_state, None);
                return;
            }

            // Video Thumbnails files can be empty if generation failed or thumbnails are disabled
            if metadata(&image_path).is_ok_and(|m| m.len() == 0) {
                set_preview_visible(&gui_state, None);
                return;
            }

            // Do not load the same image again
            if image_path == gui_state.get_preview_image_path() {
                return;
            }

            let path = Path::new(image_path.as_str());

            let images_in_thumbnails_line = if active_tab == ActiveTab::VideoOptimizer {
                shared_models
                    .lock()
                    .expect("Failed to lock model mutex")
                    .shared_video_optimizer_state
                    .as_ref()
                    .map_or(1, |state| state.get_params().get_generate_number_of_items_in_thumbnail_grid())
            } else {
                1
            };

            // Looks that resizing image before sending it to GUI works better that letting Slint do it automatically
            if let Some((mut timer, img)) = load_image(path) {
                let mut img_to_use = if img.width() > 1024 || img.height() > 1024 {
                    let bigger_side = img.width().max(img.height());
                    let scale_factor = bigger_side as f32 / 1024.0;
                    let new_width = (img.width() as f32 / scale_factor) as u32;
                    let new_height = (img.height() as f32 / scale_factor) as u32;

                    let mut dst_img = DynamicImage::new(new_width, new_height, img.color());
                    timer.checkpoint("creating new image buffer");

                    let resize_options = ResizeOptions::new().resize_alg(ResizeAlg::Interpolation(FilterType::Lanczos3));
                    match Resizer::new().resize(&img, &mut dst_img, Some(&resize_options)) {
                        Ok(()) => {
                            timer.checkpoint("resizing image with fast-image-resize");
                            dst_img.into_rgba8()
                        }
                        Err(_) => {
                            let r = img.resize(new_width, new_height, image::imageops::Lanczos3);
                            timer.checkpoint("resizing image with image-rs");
                            r.into_rgba8()
                        }
                    }
                } else {
                    img.into_rgba8()
                };

                if crop_left != -1 && crop_top != -1 && crop_right != -1 && crop_bottom != -1 && orig_width > 0 && orig_height > 0 {
                    img_to_use = draw_crop_rectangle_on_image(
                        img_to_use,
                        crop_left,
                        crop_top,
                        crop_right,
                        crop_bottom,
                        orig_width as u32,
                        orig_height as u32,
                        images_in_thumbnails_line as u32,
                    );
                    timer.checkpoint("cropping image");
                }

                let slint_image = convert_into_slint_image(&img_to_use);
                timer.checkpoint("converting image to Slint image");

                gui_state.set_preview_image(slint_image);
                timer.checkpoint("setting image in GUI");

                debug!("{}", timer.report("total", true));
                set_preview_visible(&gui_state, Some(image_path.as_str()));
            } else {
                set_preview_visible(&gui_state, None);
            }
        });
}

fn set_preview_visible(gui_state: &GuiState, preview: Option<&str>) {
    if let Some(preview) = preview {
        gui_state.set_preview_image_path(preview.into());
        gui_state.set_preview_visible(true);
    } else {
        gui_state.set_preview_image_path("".into());
        gui_state.set_preview_visible(false);
    }
}

fn convert_into_slint_image(img: &RgbaImage) -> slint::Image {
    let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(img.as_raw(), img.width(), img.height());
    slint::Image::from_rgba8(buffer)
}

fn load_image(image_path: &Path) -> Option<(Timer, DynamicImage)> {
    if !image_path.is_file() {
        return None;
    }

    let mut debug_timer = Timer::new("Loading and converting image in slint");

    let img = match get_dynamic_image_from_path(&image_path.to_string_lossy()) {
        Ok(img) => img,
        Err(e) => {
            error!("Failed to load image \"{}\": {e}", image_path.to_string_lossy());
            return None;
        }
    };

    debug_timer.checkpoint("loading image");

    Some((debug_timer, img))
}

fn draw_crop_rectangle_on_image(
    mut buf: ImageBufferRgba,
    crop_left: i32,
    crop_top: i32,
    crop_right: i32,
    crop_bottom: i32,
    original_width: u32,
    _original_height: u32,
    images_in_thumbnails_line: u32,
) -> ImageBufferRgba {
    let width = buf.width() / images_in_thumbnails_line;
    let height = buf.height() / images_in_thumbnails_line;

    let scale_factor = original_width as f32 / width as f32;

    let crop_left = (crop_left as f32 / scale_factor).round() as i32;
    let crop_top = (crop_top as f32 / scale_factor).round() as i32;
    let crop_right = (crop_right as f32 / scale_factor).round() as i32;
    let crop_bottom = (crop_bottom as f32 / scale_factor).round() as i32;

    let l = (crop_left.max(0) as u32).min(width.saturating_sub(1));
    let t = (crop_top.max(0) as u32).min(height.saturating_sub(1));
    let r = (crop_right.max(0) as u32).min(width.saturating_sub(1));
    let b = (crop_bottom.max(0) as u32).min(height.saturating_sub(1));

    if l > r || t > b {
        return buf;
    }

    let thickness = (width.max(height) / 100 * images_in_thumbnails_line).max(2);

    for x_im in 0..images_in_thumbnails_line {
        for y_im in 0..images_in_thumbnails_line {
            for side in [-1, 1] {
                for th in 0..(thickness as i32 / 2) {
                    let th_val = side * th;

                    let top_y = (t as i32 + th_val) as u32;
                    let bottom_y = (b as i32 - th_val) as u32;
                    let left_x = (l as i32) as u32;
                    let right_x = (r as i32) as u32;

                    for x in left_x..=right_x {
                        for y in [top_y, bottom_y] {
                            if (0..height).contains(&y) && (0..width).contains(&x) {
                                buf.put_pixel(x + x_im * width, y + y_im * height, get_pixel_color(x, y));
                            }
                        }
                    }

                    let top_y = (t as i32) as u32;
                    let bottom_y = (b as i32) as u32;
                    let left_x = (l as i32 + th_val) as u32;
                    let right_x = (r as i32 - th_val) as u32;

                    for y in top_y..=bottom_y {
                        for x in [left_x, right_x] {
                            if (0..height).contains(&y) && (0..width).contains(&x) {
                                buf.put_pixel(x + x_im * width, y + y_im * height, get_pixel_color(x, y));
                            }
                        }
                    }
                }
            }
        }
    }

    buf
}

#[inline]
fn get_pixel_color(x: u32, y: u32) -> Rgba<u8> {
    match (x + y) % 9 {
        0 => Rgba([127u8, 0u8, 0u8, 255u8]),
        1 => Rgba([0u8, 127u8, 0u8, 255u8]),
        2 => Rgba([0u8, 0u8, 127u8, 255u8]),
        3 => Rgba([255u8, 255u8, 0u8, 255u8]),
        4 => Rgba([0u8, 255u8, 255u8, 255u8]),
        5 => Rgba([255u8, 0u8, 255u8, 255u8]),
        6 => Rgba([255u8, 255u8, 255u8, 255u8]),
        7 => Rgba([128u8, 0u8, 128u8, 255u8]),
        8 => Rgba([0u8, 0u8, 0u8, 255u8]),
        _ => unreachable!("Modulo 9 should always be in 0..8"),
    }
}
