use crate::file_fixer::engine::Repairer;
use crate::file_fixer::format::Format;

pub mod avi;
pub mod avi_rebuild;
pub mod avi_scan;
pub mod bmp;
pub mod deflate;
pub mod ebml;
pub mod ffmpeg;
pub mod flac;
pub mod flac_frames;
pub mod gif;
pub mod gif_blocks;
pub mod jpeg;
pub mod jpeg_assemble;
pub mod jpeg_geometry;
pub mod jpeg_integrity;
pub mod jpeg_markers;
pub mod jpeg_rebuild;
pub mod jpeg_recover;
pub mod jpeg_scan;
pub mod jpeg_tables;
pub mod mkv;
pub mod mp4;
pub mod mp4_boxes;
pub mod mp4_elementary;
pub mod mp4_fragments;
pub mod mp4_profile;
pub mod mp4_profile_store;
pub mod mp4_rebuild;
pub mod mp4_relink;
pub mod mp4_scan;
pub mod mp4_trim;
pub mod mpeg;
pub mod mpeg_packs;
pub mod pdf;
pub mod pdf_objects;
pub mod png;
pub mod png_chunks;
pub mod png_image;
pub mod png_rebuild;
pub mod riff;
pub mod tiff;
pub mod tiff_ifd;
pub mod wav;
pub mod webp;
pub mod zip;
pub mod zip_entries;

/// The repairer able to handle `format`. Every format in [`Format`] has one, and every one of
/// them proposes real strategies; what varies is how much the format can prove about its own
/// content, which decides how much of a repair may be called faithful. See the README's
/// "Recovering content, not inventing it".
pub fn repairer_for(format: Format) -> Box<dyn Repairer> {
    match format {
        Format::Jpeg => Box::new(jpeg::JpegRepairer),
        Format::Png => Box::new(png::PngRepairer),
        Format::Mp4 => Box::new(mp4::Mp4Repairer),
        Format::Avi => Box::new(avi::AviRepairer),
        Format::Gif => Box::new(gif::GifRepairer),
        Format::Bmp => Box::new(bmp::BmpRepairer),
        Format::Wav => Box::new(wav::WavRepairer),
        Format::Flac => Box::new(flac::FlacRepairer),
        Format::WebP => Box::new(webp::WebPRepairer),
        Format::Tiff => Box::new(tiff::TiffRepairer),
        Format::Zip => Box::new(zip::ZipRepairer),
        Format::Mkv => Box::new(mkv::MkvRepairer),
        Format::Pdf => Box::new(pdf::PdfRepairer),
        Format::Mpeg => Box::new(mpeg::MpegRepairer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_repairer_reports_the_format_it_was_asked_for() {
        for format in Format::ALL {
            assert_eq!(repairer_for(format).format(), format);
        }
    }
}
