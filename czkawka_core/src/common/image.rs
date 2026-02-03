#![allow(unused_imports)]
// I don't want to fight with unused(heif) imports in this file, so simply ignore it to avoid too much complexity

use std::fs::File;
use std::path::Path;
use std::sync::{Arc, atomic};
use std::thread::{JoinHandle, sleep};
use std::time::{Duration, Instant, SystemTime};
use std::{fs, panic, thread};

use anyhow::anyhow;
use fun_time::fun_time;
use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use jxl_oxide::image::BitDepth;
use jxl_oxide::integration::JxlDecoder;
use jxl_oxide::{JxlImage, PixelFormat};
#[cfg(feature = "heif")]
use libheif_rs::{ColorSpace, HeifContext, RgbChroma};
#[cfg(feature = "libraw")]
use libraw::Processor;
use log::{LevelFilter, Record, debug, error, info, trace, warn};
use nom_exif::{ExifIter, ExifTag, MediaParser, MediaSource};
use rawler::RawLoader;
use rawler::decoders::RawDecodeParams;
use rawler::imgop::develop::RawDevelop;
use rawler::rawsource::RawSource;
use symphonia::core::conv::IntoSample;

use crate::common;
use crate::common::consts::{HEIC_EXTENSIONS, IMAGE_RS_EXTENSIONS, IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS, JXL_IMAGE_EXTENSIONS, RAW_IMAGE_EXTENSIONS};
use crate::common::create_crash_message;
use crate::helpers::debug_timer::Timer;
// #[cfg(feature = "heif")]
// use libheif_rs::LibHeif;

pub(crate) fn get_jxl_image(path: &str) -> anyhow::Result<DynamicImage> {
    let file = File::open(path)?;
    let decoder = JxlDecoder::new(file)?;

    let image = DynamicImage::from_decoder(decoder)?;

    Ok(image)
}

pub fn get_dynamic_image_from_path(path: &str) -> Result<DynamicImage, String> {
    let path_lower = Path::new(path).extension().unwrap_or_default().to_string_lossy().to_lowercase();

    trace!("decoding file {path}");
    let res = panic::catch_unwind(|| {
        if HEIC_EXTENSIONS.iter().any(|ext| path_lower.ends_with(ext)) {
            #[cfg(feature = "heif")]
            {
                get_dynamic_image_from_heic(path).map_err(|e| format!("Cannot open heic file \"{path}\": {e}"))
            }
            #[cfg(not(feature = "heif"))]
            {
                image::open(path).map_err(|e| format!("Cannot open image file \"{path}\": {e}"))
            }
        } else if JXL_IMAGE_EXTENSIONS.iter().any(|ext| path_lower.ends_with(ext)) {
            get_jxl_image(path).map_err(|e| format!("Cannot open jxl image file \"{path}\": {e}"))
        } else if RAW_IMAGE_EXTENSIONS.iter().any(|ext| path_lower.ends_with(ext)) {
            get_raw_image(path).map_err(|e| format!("Cannot open raw image file \"{path}\": {e}"))
        } else {
            image::open(path).map_err(|e| format!("Cannot open image file \"{path}\": {e}"))
        }
    });

    if let Ok(res) = res {
        match res {
            Ok(t) => {
                let rotation = get_rotation_from_exif(path).unwrap_or(None);
                match rotation {
                    Some(ExifOrientation::Normal) | None => Ok(t),
                    Some(ExifOrientation::MirrorHorizontal) => Ok(t.fliph()),
                    Some(ExifOrientation::Rotate180) => Ok(t.rotate180()),
                    Some(ExifOrientation::MirrorVertical) => Ok(t.flipv()),
                    Some(ExifOrientation::MirrorHorizontalAndRotate270CW) => Ok(t.fliph().rotate270()),
                    Some(ExifOrientation::Rotate90CW) => Ok(t.rotate90()),
                    Some(ExifOrientation::MirrorHorizontalAndRotate90CW) => Ok(t.fliph().rotate90()),
                    Some(ExifOrientation::Rotate270CW) => Ok(t.rotate270()),
                }
            }
            Err(e) => Err(format!("Cannot open image file \"{path}\": {e}")),
        }
    } else {
        let message = create_crash_message("Image-rs or libraw-rs or jxl-oxide", path, "https://github.com/image-rs/image/issues");
        error!("{message}");
        Err(message)
    }
}

#[cfg(feature = "heif")]
pub(crate) fn get_dynamic_image_from_heic(path: &str) -> anyhow::Result<DynamicImage> {
    // let libheif = LibHeif::new();
    let im = HeifContext::read_from_file(path)?;
    let handle = im.primary_image_handle()?;
    // let image = libheif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?; // Enable when using libheif 0.19
    let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    let width = image.width();
    let height = image.height();
    let planes = image.planes();
    let interleaved_plane = planes.interleaved.ok_or_else(|| anyhow::anyhow!("Failed to get interleaved plane"))?;
    ImageBuffer::from_raw(width, height, interleaved_plane.data.to_owned())
        .map(DynamicImage::ImageRgb8)
        .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))
}

#[cfg(feature = "libraw")]
pub(crate) fn get_raw_image(path: impl AsRef<Path>) -> anyhow::Result<DynamicImage> {
    let buf = fs::read(path.as_ref())?;

    let processor = Processor::new();
    let processed = processor.process_8bit(&buf)?;

    let width = processed.width();
    let height = processed.height();

    let data = processed.to_vec();
    let data_len = data.len();

    let buffer = ImageBuffer::from_raw(width, height, data).ok_or(anyhow::anyhow!(format!(
        "Cannot create ImageBuffer from raw image with width: {width} and height: {height} and data length: {data_len}",
    )))?;

    Ok(DynamicImage::ImageRgb8(buffer))
}

#[cfg(not(feature = "libraw"))]
pub(crate) fn get_raw_image(path: impl AsRef<Path> + std::fmt::Debug) -> Result<DynamicImage, String> {
    let mut timer = Timer::new("Rawler");

    let raw_source = RawSource::new(path.as_ref()).map_err(|err| format!("Failed to create RawSource from path {path:?}: {err}"))?;

    timer.checkpoint("Created RawSource");

    let decoder = rawler::get_decoder(&raw_source).map_err(|e| e.to_string())?;

    timer.checkpoint("Got decoder");
    let raw_image = decoder.raw_image(&raw_source, &RawDecodeParams::default(), false).map_err(|e| e.to_string())?;

    timer.checkpoint("Decoded raw image");

    let developer = RawDevelop::default();
    let developed_image = developer.develop_intermediate(&raw_image).map_err(|e| e.to_string())?;

    timer.checkpoint("Developed raw image");

    let dynamic_image = developed_image.to_dynamic_image().ok_or("Failed to convert image to DynamicImage".to_string())?;

    timer.checkpoint("Converted to DynamicImage");

    let rgb_image = DynamicImage::from(dynamic_image.to_rgb8());

    timer.checkpoint("Reconverted to RGB");

    trace!("{}", timer.report("Everything", false));

    Ok(rgb_image)
}

pub fn check_if_can_display_image(path: &str) -> bool {
    let Some(extension) = Path::new(path).extension() else {
        return false;
    };
    let extension_str = extension.to_string_lossy().to_lowercase();
    #[cfg(feature = "heif")]
    let allowed_extensions = &[IMAGE_RS_EXTENSIONS, RAW_IMAGE_EXTENSIONS, JXL_IMAGE_EXTENSIONS, HEIC_EXTENSIONS].concat();

    #[cfg(not(feature = "heif"))]
    let allowed_extensions = &[IMAGE_RS_EXTENSIONS, RAW_IMAGE_EXTENSIONS, JXL_IMAGE_EXTENSIONS].concat();

    allowed_extensions.iter().any(|ext| extension_str.ends_with(ext))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExifOrientation {
    Normal,
    MirrorHorizontal,
    Rotate180,
    MirrorVertical,
    MirrorHorizontalAndRotate270CW,
    Rotate90CW,
    MirrorHorizontalAndRotate90CW,
    Rotate270CW,
}

pub(crate) fn get_rotation_from_exif(path: &str) -> Result<Option<ExifOrientation>, nom_exif::Error> {
    let res = panic::catch_unwind(|| {
        let mut parser = MediaParser::new();
        let ms = MediaSource::file_path(path)?;
        if !ms.has_exif() {
            return Ok(None);
        }
        let exif_iter: ExifIter = parser.parse(ms)?;
        for exif_entry in exif_iter {
            if exif_entry.tag() == Some(ExifTag::Orientation)
                && let Some(value) = exif_entry.get_value()
            {
                return match value.to_string().as_str() {
                    "1" => Ok(Some(ExifOrientation::Normal)),
                    "2" => Ok(Some(ExifOrientation::MirrorHorizontal)),
                    "3" => Ok(Some(ExifOrientation::Rotate180)),
                    "4" => Ok(Some(ExifOrientation::MirrorVertical)),
                    "5" => Ok(Some(ExifOrientation::MirrorHorizontalAndRotate270CW)),
                    "6" => Ok(Some(ExifOrientation::Rotate90CW)),
                    "7" => Ok(Some(ExifOrientation::MirrorHorizontalAndRotate90CW)),
                    "8" => Ok(Some(ExifOrientation::Rotate270CW)),
                    _ => Ok(None),
                };
            }
        }
        Ok(None)
    });

    res.unwrap_or_else(|_| {
        let message = create_crash_message("nom-exif", path, "https://github.com/mindeng/nom-exif");
        error!("{message}");
        Err(nom_exif::Error::IOError(std::io::Error::other("Panic in get_rotation_from_exif")))
    })
}
