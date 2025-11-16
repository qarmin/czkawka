use std::cmp::Ordering;
use std::io::{BufReader, Cursor};

use gdk4::gdk_pixbuf::{InterpType, Pixbuf};
use glib::Bytes;
use gtk4::Widget;
use gtk4::gdk_pixbuf::Colorspace;
use gtk4::prelude::*;
use image::codecs::png::PngEncoder;
use image::{DynamicImage, GenericImageView, ImageEncoder, RgbaImage};
use resvg::tiny_skia;
use resvg::usvg::{Options, Tree};

use crate::gtk_traits::WidgetTraits;

const SIZE_OF_ICON: i32 = 18;
const TYPE_OF_INTERPOLATION: InterpType = InterpType::Tiles;

pub(crate) fn resize_pixbuf_dimension(pixbuf: &Pixbuf, requested_size: (i32, i32), interp_type: InterpType) -> Option<Pixbuf> {
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

fn svg_to_dynamic_image(svg_data: &[u8]) -> Option<DynamicImage> {
    let opt = Options::default();
    let tree = Tree::from_data(svg_data, &opt).ok()?;

    let mut pixmap = tiny_skia::Pixmap::new(tree.size().width() as u32, tree.size().height() as u32)?;
    resvg::render(&tree, tiny_skia::Transform::default(), &mut (pixmap.as_mut()));

    let rgba = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())?;

    Some(DynamicImage::ImageRgba8(rgba))
}

fn dynamic_image_to_pixbuf(img: DynamicImage) -> Pixbuf {
    let (width, height) = img.dimensions();
    let rgba = img.into_rgba8();
    let bytes = Bytes::from(&rgba.into_raw());

    let pixbuf = Pixbuf::from_bytes(&bytes, Colorspace::Rgb, true, 8, width as i32, height as i32, (4 * width) as i32);
    pixbuf.scale_simple(SIZE_OF_ICON, SIZE_OF_ICON, TYPE_OF_INTERPOLATION).expect("Failed to scale pixbuf")
}

pub(crate) fn set_icon_of_button<P: IsA<Widget>>(button: &P, data: &'static [u8]) {
    let image = button.get_custom_image();
    let dynamic_image = svg_to_dynamic_image(data).expect("Failed to convert SVG data to DynamicImage");
    let pixbuf = dynamic_image_to_pixbuf(dynamic_image);
    image.set_from_pixbuf(Some(&pixbuf));
}

pub(crate) fn get_pixbuf_from_dynamic_image(dynamic_image: DynamicImage) -> Result<Pixbuf, String> {
    let mut output = Vec::new();
    let width = dynamic_image.width();
    let height = dynamic_image.height();
    let rgba = dynamic_image.into_rgba8();
    let encoder = PngEncoder::new(&mut output);
    encoder
        .write_image(&rgba, width, height, image::ExtendedColorType::Rgba8)
        .map_err(|e| format!("Failed to encode image: {e}"))?;
    Pixbuf::from_read(BufReader::new(Cursor::new(output))).map_err(|e| format!("Failed to create Pixbuf from DynamicImage: {e}"))
}

#[cfg(test)]
mod test {
    use image::DynamicImage;

    use super::*;

    #[test]
    fn test_pixbuf_from_dynamic_image() {
        let dynamic_image = DynamicImage::new_rgb8(1, 1);
        get_pixbuf_from_dynamic_image(dynamic_image.clone()).expect("Failed to get pixbuf from dynamic image");
        get_pixbuf_from_dynamic_image(dynamic_image.clone()).expect("Failed to get pixbuf from dynamic image");
        get_pixbuf_from_dynamic_image(dynamic_image.clone()).expect("Failed to get pixbuf from dynamic image");
        get_pixbuf_from_dynamic_image(dynamic_image).expect("Failed to get pixbuf from dynamic image");
    }
}
