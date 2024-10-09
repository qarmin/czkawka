use std::path::Path;
use std::time::{Duration, Instant};

use czkawka_core::common_image::{check_if_can_display_image, get_dynamic_image_from_path};
use image::DynamicImage;
use log::{debug, error};
use slint::ComponentHandle;

use crate::{Callabler, CurrentTab, GuiState, MainWindow, Settings};

pub type ImageBufferRgba = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn connect_show_preview(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_load_image_preview(move |image_path| {
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let settings = app.global::<Settings>();
        let gui_state = app.global::<GuiState>();

        let active_tab = gui_state.get_active_tab();

        if !((active_tab == CurrentTab::SimilarImages && settings.get_similar_images_show_image_preview())
            || (active_tab == CurrentTab::DuplicateFiles && settings.get_duplicate_image_preview()))
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

    let load_img_start_timer = Instant::now();

    let img = match get_dynamic_image_from_path(&image_path.to_string_lossy()) {
        Ok(img) => img,
        Err(e) => {
            error!("{e}");
            return None;
        }
    };
    Some((load_img_start_timer.elapsed(), img))
}
