use std::path::Path;

use czkawka_core::common::image::{check_if_can_display_image, get_dynamic_image_from_path};
use czkawka_core::helpers::debug_timer::Timer;
use fast_image_resize::{FilterType, ResizeAlg, ResizeOptions, Resizer};
use image::DynamicImage;
use log::{debug, error};
use slint::ComponentHandle;

use crate::{ActiveTab, Callabler, GuiState, MainWindow, Settings};
pub type ImageBufferRgba = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub(crate) fn connect_show_preview(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_load_image_preview(move |image_path| {
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let settings = app.global::<Settings>();
        let gui_state = app.global::<GuiState>();

        let active_tab = gui_state.get_active_tab();

        if !((active_tab == ActiveTab::SimilarImages && settings.get_similar_images_show_image_preview())
            || (active_tab == ActiveTab::DuplicateFiles && settings.get_duplicate_image_preview())
            || (active_tab == ActiveTab::SimilarVideos && settings.get_video_thumbnails_preview()))
        {
            set_preview_visible(&gui_state, None);
            return;
        }

        if !check_if_can_display_image(&image_path) {
            set_preview_visible(&gui_state, None);
            return;
        }

        // Do not load the same image again
        if image_path == gui_state.get_preview_image_path() {
            return;
        }

        let path = Path::new(image_path.as_str());

        // Looks that resizing image before sending it to GUI is faster than resizing it in Slint
        // Additionally it fixes issues with
        if let Some((mut timer, img)) = load_image(path) {
            let img_to_use = if img.width() > 1024 || img.height() > 1024 {
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
                        dst_img
                    }
                    Err(_) => {
                        let r = img.resize(new_width, new_height, image::imageops::Lanczos3);
                        timer.checkpoint("resizing image with image-rs");
                        r
                    }
                }
            } else {
                img
            };

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

fn convert_into_slint_image(img: &DynamicImage) -> slint::Image {
    let image_buffer: ImageBufferRgba = img.to_rgba8();
    let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(image_buffer.as_raw(), image_buffer.width(), image_buffer.height());
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
