use crate::{Callabler, GuiState, MainWindow};
use czkawka_core::common::{get_dynamic_image_from_raw_image, IMAGE_RS_EXTENSIONS, RAW_IMAGE_EXTENSIONS};
use image::DynamicImage;
use log::{debug, error};
use slint::ComponentHandle;
use std::path::Path;
use std::time::{Duration, Instant};

pub type ImageBufferRgba = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn connect_show_preview(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_load_image_preview(move |image_path| {
        let app = a.upgrade().unwrap();

        let path = Path::new(image_path.as_str());

        let res = load_image(path);
        if let Some((load_time, img)) = res {
            let start_timer_convert_time = Instant::now();
            let slint_image = convert_into_slint_image(img);
            let convert_time = start_timer_convert_time.elapsed();

            let start_set_time = Instant::now();
            app.global::<GuiState>().set_preview_image(slint_image);
            let set_time = start_set_time.elapsed();

            debug!(
                "Loading image took: {:?}, converting image took: {:?}, setting image took: {:?}",
                load_time, convert_time, set_time
            );
            app.global::<GuiState>().set_preview_visible(true);
        } else {
            app.global::<GuiState>().set_preview_visible(false);
        }
    });
}

fn convert_into_slint_image(img: DynamicImage) -> slint::Image {
    let image_buffer: ImageBufferRgba = img.to_rgba8();
    let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(image_buffer.as_raw(), image_buffer.width(), image_buffer.height());
    slint::Image::from_rgba8(buffer)
}

fn load_image(image_path: &Path) -> Option<(Duration, image::DynamicImage)> {
    if !image_path.is_file() {
        return None;
    }
    let image_name = image_path.to_string_lossy().to_string();
    let image_extension = image_path.extension()?.to_string_lossy().to_lowercase();

    let is_raw_image = RAW_IMAGE_EXTENSIONS.contains(&image_extension.as_str());
    let is_normal_image = IMAGE_RS_EXTENSIONS.contains(&image_extension.as_str());

    if !is_raw_image && !is_normal_image {
        return None;
    }
    let load_img_start_timer = Instant::now();

    // TODO this needs to be run inside closure
    let img = if is_normal_image {
        match image::open(image_name) {
            Ok(img) => img,
            Err(e) => {
                error!("Error while loading image: {}", e);
                return None;
            }
        }
    } else if is_raw_image {
        if let Some(img) = get_dynamic_image_from_raw_image(image_name) {
            img
        } else {
            error!("Error while loading raw image - not sure why - try to guess");
            return None;
        }
    } else {
        panic!("Used not supported image extension");
    };

    Some((load_img_start_timer.elapsed(), img))
}
