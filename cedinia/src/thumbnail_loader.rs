use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, SystemTime};

use czkawka_core::common::image::{ImgResizeOptions, LoadedImage, get_dynamic_image_from_path};
use czkawka_core::re_exported::FirFilterType;
use log::trace;

use crate::scan_runner::FileItem;

pub enum ThumbnailData {
    Loaded(Vec<u8>, u32, u32),
    Placeholder,
}

pub struct ThumbnailResult {
    pub scan_id: u32,
    pub group_idx: usize,
    pub item_idx: usize,
    pub data: ThumbnailData,
}

fn get_total_ram_mb() -> u64 {
    if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            if line.starts_with("MemTotal:")
                && let Some(kb_str) = line.split_whitespace().nth(1)
                && let Ok(kb) = kb_str.parse::<u64>()
            {
                return kb / 1024;
            }
        }
    }
    4096
}

pub fn cache_limit_bytes() -> u64 {
    let ram_mb = get_total_ram_mb();
    let limit_mb: u64 = if ram_mb <= 2048 {
        256
    } else if ram_mb <= 4096 {
        1024
    } else if ram_mb <= 8192 {
        2048
    } else {
        4096
    };
    limit_mb * 1024 * 1024
}

pub fn thumbnail_cache_dir() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        let base = crate::android_cache_path().unwrap_or("/data/data/io.github.qarmin.cedinia/cache");
        PathBuf::from(base).join("img_thumbnails")
    }
    #[cfg(not(target_os = "android"))]
    {
        let base = std::env::var("XDG_CACHE_HOME").map_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".cache"), PathBuf::from);
        base.join("cedinia").join("img_thumbnails")
    }
}

fn cache_key(path: &str, mtime_secs: u64, file_size: u64) -> String {
    let mut h = xxhash_rust::xxh3::Xxh3::new();
    path.hash(&mut h);
    mtime_secs.hash(&mut h);
    file_size.hash(&mut h);
    format!("{:016x}.png", h.finish())
}

fn try_read_png_cache(cache_path: &Path) -> Option<(Vec<u8>, u32, u32)> {
    let data = std::fs::read(cache_path).ok()?;
    let img = image::load_from_memory_with_format(&data, image::ImageFormat::Png).ok()?;
    let rgba = img.into_rgba8();
    let w = rgba.width();
    let h = rgba.height();
    Some((rgba.into_raw(), w, h))
}

fn try_write_png_cache(cache_path: &Path, rgba: &[u8], w: u32, h: u32) {
    let tmp = cache_path.with_extension("tmp");
    let write = || -> image::ImageResult<()> {
        use image::ImageEncoder;
        let f = std::fs::File::create(&tmp).map_err(image::ImageError::IoError)?;
        image::codecs::png::PngEncoder::new(f).write_image(rgba, w, h, image::ExtendedColorType::Rgba8)
    };
    if write().is_ok() {
        let _ = std::fs::rename(&tmp, cache_path);
    } else {
        let _ = std::fs::remove_file(&tmp);
    }
}

pub fn make_placeholder_image() -> slint::Image {
    const W: u32 = 32;
    const H: u32 = 32;
    const CELL: u32 = 16;
    let mut rgba = vec![0u8; (W * H * 4) as usize];
    for y in 0..H {
        for x in 0..W {
            let off = ((y * W + x) * 4) as usize;
            let v = if ((x / CELL) + (y / CELL)).is_multiple_of(2) { 160u8 } else { 80u8 };
            rgba[off] = v;
            rgba[off + 1] = v;
            rgba[off + 2] = v;
            rgba[off + 3] = 255;
        }
    }
    rgba_to_slint_image(&rgba, W, H)
}

pub fn rgba_to_slint_image(rgba: &[u8], width: u32, height: u32) -> slint::Image {
    let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(rgba, width, height);
    slint::Image::from_rgba8(buffer)
}

pub fn load_and_resize_thumbnail(path: &str, cache_dir: &Path) -> Option<(Vec<u8>, u32, u32)> {
    let meta = std::fs::metadata(path).ok();
    let (mtime_secs, file_size) = meta.as_ref().map_or((0, 0), |m| {
        let mtime = m.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs());
        (mtime, m.len())
    });

    let cache_file = cache_dir.join(cache_key(path, mtime_secs, file_size));

    if let Some(cached) = try_read_png_cache(&cache_file) {
        let now = filetime::FileTime::now();
        let _ = filetime::set_file_mtime(&cache_file, now);
        trace!("Loaded thumbnail from cache for {path} ({file_size} bytes)");
        return Some(cached);
    }
    trace!("Generating thumbnail for {path} ({file_size} bytes)");
    let loaded_data = get_dynamic_image_from_path(
        path,
        Some(ImgResizeOptions {
            max_width: 256,
            max_height: 256,
            filter: FirFilterType::Lanczos3,
        }),
    )
    .ok()?;

    let LoadedImage {
        image,
        original_width,
        original_height,
    } = loaded_data;

    let should_cache = original_width >= 256 || original_height >= 256;

    let rgba = image.into_rgba8();
    let w = rgba.width();
    let h = rgba.height();
    let raw = rgba.into_raw();

    if should_cache {
        trace!("Caching thumbnail for {path} at {w}x{h}");
        try_write_png_cache(&cache_file, &raw, w, h);
    } else {
        trace!("Not caching thumbnail for {path} since it's smaller than 256x256 ({original_width}x{original_height})");
    }

    Some((raw, w, h))
}

pub fn collect_thumb_tasks(items: &[FileItem]) -> Vec<(usize, usize, String)> {
    use crate::common::{STR_IDX_NAME, STR_IDX_PATH};
    let mut tasks = Vec::new();
    let mut group_idx: i32 = -1;
    let mut item_idx = 0usize;
    for item in items {
        if item.is_header {
            group_idx += 1;
            item_idx = 0;
        } else if group_idx >= 0 {
            let name = &item.val_str[STR_IDX_NAME];
            let path = &item.val_str[STR_IDX_PATH];
            let full = if path.is_empty() { name.clone() } else { format!("{path}/{name}") };
            tasks.push((group_idx as usize, item_idx, full));
            item_idx += 1;
        }
    }
    tasks
}

pub fn cleanup_old_thumbnails() {
    let cache_dir = thumbnail_cache_dir();
    let cutoff = SystemTime::now().checked_sub(Duration::from_secs(30 * 24 * 3600)).unwrap_or(SystemTime::UNIX_EPOCH);
    if let Ok(entries) = std::fs::read_dir(&cache_dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata()
                && meta.modified().is_ok_and(|t| t < cutoff)
            {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
}

pub fn spawn_thumbnail_loader(tasks: Vec<(usize, usize, String)>, tx: std::sync::mpsc::Sender<ThumbnailResult>, cancel: Arc<AtomicBool>, scan_id: u32) {
    let cache_dir = thumbnail_cache_dir();
    let _ = std::fs::create_dir_all(&cache_dir);
    let cache_dir = Arc::new(cache_dir);

    std::thread::spawn(move || {
        if tasks.is_empty() {
            return;
        }

        let num_workers = std::thread::available_parallelism().map_or(2, |n| n.get().min(4));

        let limit = cache_limit_bytes();
        let used_bytes = Arc::new(AtomicU64::new(0));
        let next_idx = Arc::new(AtomicUsize::new(0));
        let tasks = Arc::new(tasks);
        let mut handles = Vec::with_capacity(num_workers);

        for _ in 0..num_workers {
            let tasks = tasks.clone();
            let tx = tx.clone();
            let cancel = cancel.clone();
            let used_bytes = used_bytes.clone();
            let next_idx = next_idx.clone();
            let cache_dir = cache_dir.clone();

            handles.push(std::thread::spawn(move || {
                loop {
                    let idx = next_idx.fetch_add(1, Ordering::Relaxed);
                    if idx >= tasks.len() || cancel.load(Ordering::Relaxed) {
                        break;
                    }

                    let (group_idx, item_idx, ref path) = tasks[idx];
                    let cur = used_bytes.load(Ordering::Relaxed);
                    let data = if cur >= limit {
                        ThumbnailData::Placeholder
                    } else {
                        match load_and_resize_thumbnail(path, &cache_dir) {
                            Some((rgba, w, h)) => {
                                let size = rgba.len() as u64;
                                let prev = used_bytes.fetch_add(size, Ordering::SeqCst);
                                if prev + size <= limit {
                                    ThumbnailData::Loaded(rgba, w, h)
                                } else {
                                    used_bytes.fetch_sub(size, Ordering::SeqCst);
                                    ThumbnailData::Placeholder
                                }
                            }
                            None => ThumbnailData::Placeholder,
                        }
                    };

                    if tx
                        .send(ThumbnailResult {
                            scan_id,
                            group_idx,
                            item_idx,
                            data,
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            }));
        }

        for h in handles {
            h.join().expect("Thumbnail loader panicked");
        }
    });
}
