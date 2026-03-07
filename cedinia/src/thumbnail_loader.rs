use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use czkawka_core::common::image::ImgResizeOptions;

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
            if line.starts_with("MemTotal:") {
                if let Some(kb_str) = line.split_whitespace().nth(1) {
                    if let Ok(kb) = kb_str.parse::<u64>() {
                        return kb / 1024;
                    }
                }
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
        PathBuf::from("/data/data/io.github.qarmin.cedinia/cache/img_thumbnails")
    }
    #[cfg(not(target_os = "android"))]
    {
        let base = std::env::var("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".cache"));
        base.join("cedinia").join("img_thumbnails")
    }
}

fn cache_key(path: &str, mtime_secs: u64, file_size: u64) -> String {
    let mut h = DefaultHasher::new();
    path.hash(&mut h);
    mtime_secs.hash(&mut h);
    file_size.hash(&mut h);
    format!("{:016x}.qoi", h.finish())
}

fn try_read_qoi_cache(cache_path: &Path) -> Option<(Vec<u8>, u32, u32)> {
    use image::ImageDecoder;
    let f = std::fs::File::open(cache_path).ok()?;
    let decoder = image::codecs::qoi::QoiDecoder::new(BufReader::new(f)).ok()?;
    let (w, h) = decoder.dimensions();
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    decoder.read_image(&mut rgba).ok()?;
    Some((rgba, w, h))
}

fn try_write_qoi_cache(cache_path: &Path, rgba: &[u8], w: u32, h: u32) {
    use image::ImageEncoder;
    let tmp = cache_path.with_extension("tmp");
    let write = || -> image::ImageResult<()> {
        let f = std::fs::File::create(&tmp).map_err(image::ImageError::IoError)?;
        image::codecs::qoi::QoiEncoder::new(f).write_image(rgba, w, h, image::ExtendedColorType::Rgba8)
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
            let v = if ((x / CELL) + (y / CELL)) % 2 == 0 { 160u8 } else { 80u8 };
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
    use czkawka_core::common::image::get_dynamic_image_from_path;
    use fast_image_resize::FilterType;

    let meta = std::fs::metadata(path).ok();
    let (mtime_secs, file_size) = meta
        .as_ref()
        .map(|m| {
            let mtime = m
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            (mtime, m.len())
        })
        .unwrap_or((0, 0));

    let cache_file = cache_dir.join(cache_key(path, mtime_secs, file_size));

    if let Some(cached) = try_read_qoi_cache(&cache_file) {
        return Some(cached);
    }

    let rgba = get_dynamic_image_from_path(
        path,
        Some(ImgResizeOptions {
            max_width: 256,
            max_height: 256,
            filter: FilterType::Lanczos3,
        }),
    )
    .ok()?
    .into_rgba8();

    let orig_w = rgba.width();
    let orig_h = rgba.height();

    let should_cache = orig_w >= 256 || orig_h >= 256;

    let w = rgba.width();
    let h = rgba.height();
    let raw = rgba.into_raw();

    if should_cache {
        try_write_qoi_cache(&cache_file, &raw, w, h);
    }

    Some((raw, w, h))
}

pub fn collect_thumb_tasks(items: &[FileItem]) -> Vec<(usize, usize, String)> {
    let mut tasks = Vec::new();
    let mut group_idx: i32 = -1;
    let mut item_idx = 0usize;
    for item in items {
        if item.is_header {
            group_idx += 1;
            item_idx = 0;
        } else if group_idx >= 0 {
            let path = if item.path.is_empty() {
                item.name.clone()
            } else {
                format!("{}/{}", item.path, item.name)
            };
            tasks.push((group_idx as usize, item_idx, path));
            item_idx += 1;
        }
    }
    tasks
}

pub fn spawn_thumbnail_loader(tasks: Vec<(usize, usize, String)>, tx: std::sync::mpsc::Sender<ThumbnailResult>, cancel: Arc<AtomicBool>, scan_id: u32) {
    let cache_dir = thumbnail_cache_dir();
    let _ = std::fs::create_dir_all(&cache_dir);
    let cache_dir = Arc::new(cache_dir);

    std::thread::spawn(move || {
        if tasks.is_empty() {
            return;
        }

        let num_workers = std::thread::available_parallelism().map(|n| n.get().min(4)).unwrap_or(2);

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
            h.join().ok();
        }
    });
}
