use glib::Bytes;
use gtk4::gdk_pixbuf::{InterpType, Pixbuf};
use image::{DynamicImage, GenericImageView, RgbaImage};
use resvg::tiny_skia;
use resvg::usvg::{Options, Tree};

use crate::utils::widget_utils::get_custom_image_from_widget;

pub const SIZE_OF_ICON: i32 = 18;
pub const TYPE_OF_INTERPOLATION: InterpType = InterpType::Tiles;

pub fn svg_to_dynamic_image(svg_data: &[u8]) -> Option<DynamicImage> {
    let opt = Options::default();
    let tree = Tree::from_data(svg_data, &opt).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(tree.size().width() as u32, tree.size().height() as u32)?;
    resvg::render(&tree, tiny_skia::Transform::default(), &mut (pixmap.as_mut()));
    let rgba = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())?;
    Some(DynamicImage::ImageRgba8(rgba))
}

pub fn dynamic_image_to_pixbuf(img: DynamicImage) -> Pixbuf {
    let (width, height) = img.dimensions();
    let rgba = img.into_rgba8();
    let bytes = Bytes::from(&rgba.into_raw());
    let pixbuf = Pixbuf::from_bytes(&bytes, gtk4::gdk_pixbuf::Colorspace::Rgb, true, 8, width as i32, height as i32, (4 * width) as i32);
    pixbuf.scale_simple(SIZE_OF_ICON, SIZE_OF_ICON, TYPE_OF_INTERPOLATION).expect("Failed to scale pixbuf")
}

pub fn set_icon_of_button<P: gtk4::prelude::IsA<gtk4::Widget>>(button: &P, data: &'static [u8]) {
    let image = get_custom_image_from_widget(&button.clone());
    let dynamic_image = svg_to_dynamic_image(data).expect("Failed to convert SVG data to DynamicImage");
    let pixbuf = dynamic_image_to_pixbuf(dynamic_image);
    image.set_from_pixbuf(Some(&pixbuf));
}

pub fn get_pixbuf_from_dynamic_image(dynamic_image: &DynamicImage) -> Result<Pixbuf, String> {
    use std::io::{BufReader, Cursor};

    use image::codecs::jpeg::JpegEncoder;
    let mut output = Vec::new();
    JpegEncoder::new(&mut output)
        .encode_image(dynamic_image)
        .map_err(|e| format!("Failed to encode image: {e}"))?;
    Pixbuf::from_read(BufReader::new(Cursor::new(output))).map_err(|e| format!("Failed to create Pixbuf from DynamicImage: {e}"))
}

pub fn resize_pixbuf_dimension(pixbuf: &Pixbuf, requested_size: (i32, i32), interp_type: InterpType) -> Option<Pixbuf> {
    use std::cmp::Ordering;
    let current_ratio = pixbuf.width() as f32 / pixbuf.height() as f32;
    let mut new_size;
    match current_ratio.total_cmp(&(requested_size.0 as f32 / requested_size.1 as f32)) {
        Ordering::Greater => {
            new_size = (requested_size.0, (pixbuf.height() * requested_size.0) / pixbuf.width());
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
        Ordering::Less => {
            new_size = ((pixbuf.width() * requested_size.1) / pixbuf.height(), requested_size.1);
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
        Ordering::Equal => {
            new_size = requested_size;
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
    }
    pixbuf.scale_simple(new_size.0, new_size.1, interp_type)
}
