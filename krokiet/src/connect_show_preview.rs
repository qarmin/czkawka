use std::panic;
use std::path::Path;
use std::time::{Duration, Instant};

use image::DynamicImage;
use log::{debug, error};
use slint::ComponentHandle;

use czkawka_core::common::{get_dynamic_image_from_raw_image, IMAGE_RS_EXTENSIONS, RAW_IMAGE_EXTENSIONS};

use crate::{Callabler, CurrentTab, GuiState, MainWindow, Settings};

pub type ImageBufferRgba = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn connect_show_preview(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_load_image_preview(move |image_path| {
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let settings = app.global::<Settings>();
        let gui_state = app.global::<GuiState>();

        let active_tab = gui_state.get_active_tab();

        if (active_tab == CurrentTab::SimilarImages && !settings.get_similar_images_show_image_preview())
            || (active_tab == CurrentTab::DuplicateFiles && !settings.get_duplicate_image_preview())
        {
            set_preview_visible(&gui_state, None);
            return;
        }

        // Do not load the same image again
        if image_path == gui_state.get_preview_image_path() {
            return;
        }

        let path = Path::new(image_path.as_str());

        let res = load_image(path);
        if let Some((load_time, img)) = res {
            let start_timer_convert_time = Instant::now();
            let slint_image = convert_into_slint_image(&img);
            let convert_time = start_timer_convert_time.elapsed();

            let start_set_time = Instant::now();
            gui_state.set_preview_image(slint_image);
            let set_time = start_set_time.elapsed();

            debug!(
                "Loading image took: {:?}, converting image took: {:?}, setting image took: {:?}",
                load_time, convert_time, set_time
            );
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

fn load_image(image_path: &Path) -> Option<(Duration, DynamicImage)> {
    if !image_path.is_file() {
        return None;
    }
    let image_name = image_path.to_string_lossy().to_string();
    let image_extension = image_path.extension()?.to_string_lossy().to_lowercase();

    let is_raw_image = RAW_IMAGE_EXTENSIONS.contains(&image_extension.as_str());
    let is_normal_image = IMAGE_RS_EXTENSIONS.contains(&image_extension.as_str());

    let load_img_start_timer = Instant::now();

    let img = panic::catch_unwind(|| {
        let int_img = if is_normal_image {
            match image::open(image_name) {
                Ok(img) => img,
                Err(e) => {
                    error!("Error while loading image: {}", e);
                    return None;
                }
            }
        } else if is_raw_image {
            match get_dynamic_image_from_raw_image(image_name) {
                Ok(img) => img,
                Err(e) => {
                    error!("Error while loading raw image: {}", e);
                    return None;
                }
            }
        } else {
            return None;
        };
        Some(int_img)
    })
    .unwrap_or_else(|e| {
        error!("Error while loading image: {e:?}");
        None
    })?;
    Some((load_img_start_timer.elapsed(), img))
}
