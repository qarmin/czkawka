use crate::{Callabler, GuiState, MainWindow};
use czkawka_core::common::IMAGE_RS_EXTENSIONS;
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
    let extension_with_dot = format!(".{}", image_extension);

    if !IMAGE_RS_EXTENSIONS.contains(&extension_with_dot.as_str()) {
        return None;
    }
    let load_img_start_timer = Instant::now();
    let img = image::open(image_name);

    match img {
        Ok(img) => Some((load_img_start_timer.elapsed(), img)),
        Err(e) => {
            error!("Error while loading image: {}", e);
            return None;
        }
    }
}
