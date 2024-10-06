#![allow(unused_imports)]
// I don't wanna fight with unused(heif) imports in this file, so simply ignore it to avoid too much complexity

use std::cmp::Ordering;
use std::ffi::OsString;
use std::fs::{DirEntry, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{atomic, Arc};
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, Instant, SystemTime};
use std::{fs, panic, thread};

use anyhow::anyhow;
use crossbeam_channel::Sender;
use directories_next::ProjectDirs;
use fun_time::fun_time;
use handsome_logger::{ColorChoice, ConfigBuilder, TerminalMode};
use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use imagepipe::{ImageSource, Pipeline};
use jxl_oxide::image::BitDepth;
use jxl_oxide::{JxlImage, PixelFormat};
#[cfg(feature = "heif")]
use libheif_rs::{ColorSpace, HeifContext, RgbChroma};
#[cfg(feature = "libraw")]
use libraw::Processor;
use log::{debug, error, info, warn, LevelFilter, Record};
use rawloader::RawLoader;
use symphonia::core::conv::IntoSample;

use crate::common;
use crate::common::{create_crash_message, HEIC_EXTENSIONS, IMAGE_RS_EXTENSIONS, IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS, JXL_IMAGE_EXTENSIONS, RAW_IMAGE_EXTENSIONS};
// #[cfg(feature = "heif")]
// use libheif_rs::LibHeif;

// TODO this code is ugly - this should exists in image-rs or be taken from official example of jxl-oxide
// Its presence offends everything good in this world
pub fn get_jxl_image(path: &str) -> anyhow::Result<DynamicImage> {
    let buf_reader = std::io::BufReader::new(File::open(path)?);

    let decoder = JxlImage::builder().read(buf_reader).map_err(|e| anyhow::anyhow!("Failed to read jxl file {e}"))?;
    let width = decoder.width();
    let height = decoder.height();
    let frame = decoder.render_frame(0).map_err(|e| anyhow::anyhow!("Failed to render jxl frame {e}"))?;
    let planar = &frame.image_planar();
    let pixfmt = decoder.pixel_format();
    let bits_per_sample = decoder.image_header().metadata.bit_depth;

    if bits_per_sample.bits_per_sample() == 8 && pixfmt == PixelFormat::Rgb && planar.len() == 3 {
        let zips = planar[0].buf().iter().zip(planar[1].buf().iter()).zip(planar[2].buf().iter());
        let pixels = zips.flat_map(|((r, g), b)| [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]).collect::<Vec<_>>();
        let data = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(width, height, pixels).ok_or_else(|| anyhow::anyhow!("Failed to create rgb image buffer from jxl data"))?;
        Ok(DynamicImage::ImageRgb8(data))
    } else if bits_per_sample.bits_per_sample() == 8 && pixfmt == PixelFormat::Rgba && planar.len() == 4 {
        let zips = planar[0].buf().iter().zip(planar[1].buf().iter()).zip(planar[2].buf().iter()).zip(planar[3].buf().iter());
        let pixels = zips
            .flat_map(|(((r, g), b), a)| [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (a * 255.0) as u8])
            .collect::<Vec<_>>();
        let data = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(width, height, pixels).ok_or_else(|| anyhow::anyhow!("Failed to create rgba image buffer from jxl data"))?;
        Ok(DynamicImage::ImageRgba8(data))
    } else {
        return Err(anyhow::anyhow!("Unsupported number of planes: {}", planar.len()));
    }
}

pub fn get_dynamic_image_from_path(path: &str) -> Result<DynamicImage, String> {
    let path_lower = Path::new(path).extension().unwrap_or_default().to_string_lossy().to_lowercase();

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
            Ok(t) => Ok(t),
            Err(e) => Err(format!("Cannot open image file \"{path}\": {e}")),
        }
    } else {
        let message = create_crash_message("Image-rs or libraw-rs or jxl-oxide", path, "https://github.com/image-rs/image/issues");
        println!("{message}");
        Err(message)
    }
}

#[cfg(feature = "heif")]
pub fn get_dynamic_image_from_heic(path: &str) -> anyhow::Result<DynamicImage> {
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
pub fn get_raw_image(path: impl AsRef<Path>) -> anyhow::Result<DynamicImage> {
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
pub fn get_raw_image(path: impl AsRef<Path> + std::fmt::Debug) -> Result<DynamicImage, String> {
    let mut start_timer = Instant::now();
    let mut times = Vec::new();

    let loader = RawLoader::new();
    let raw = loader.decode_file(path.as_ref()).map_err(|e| format!("Error decoding file: {e:?}"))?;

    times.push(("After decoding", start_timer.elapsed()));
    start_timer = Instant::now();

    let source = ImageSource::Raw(raw);

    times.push(("After creating source", start_timer.elapsed()));
    start_timer = Instant::now();

    let mut pipeline = Pipeline::new_from_source(source).map_err(|e| format!("Error creating pipeline: {e:?}"))?;

    times.push(("After creating pipeline", start_timer.elapsed()));
    start_timer = Instant::now();

    pipeline.run(None);
    let image = pipeline.output_8bit(None).map_err(|e| format!("Error running pipeline: {e:?}"))?;

    times.push(("After creating image", start_timer.elapsed()));
    start_timer = Instant::now();

    let image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(image.width as u32, image.height as u32, image.data).ok_or_else(|| "Failed to create image buffer".to_string())?;

    times.push(("After creating image buffer", start_timer.elapsed()));
    start_timer = Instant::now();
    let res = DynamicImage::ImageRgb8(image);
    times.push(("After creating dynamic image", start_timer.elapsed()));

    let str_timer = times.into_iter().map(|(name, time)| format!("{name}: {time:?}")).collect::<Vec<_>>().join(", ");
    debug!("Loading raw image --- {str_timer}");
    Ok(res)
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
