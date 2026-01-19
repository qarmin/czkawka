use std::{fs, thread};

use czkawka_core::common::config_cache_path::get_config_cache_path;
use czkawka_core::common::video_utils::VIDEO_THUMBNAILS_SUBFOLDER;
use humansize::{BINARY, format_size};
use log::{debug, error, info};

use crate::MainWindow;
use crate::settings::collect_settings;

const DURATION_BEFORE_CLEANING_SECS: u64 = 7 * 24 * 60 * 60; // 7 days

pub fn clear_outdated_video_thumbnails(app: &MainWindow) {
    let settings_custom = collect_settings(app);
    if settings_custom.video_thumbnails_unused_thumbnails {
        thread::spawn(|| {
            let Some(config_cache_path) = get_config_cache_path() else {
                return;
            };

            let thumbnails_dir = config_cache_path.cache_folder.join(VIDEO_THUMBNAILS_SUBFOLDER);

            if !thumbnails_dir.exists() {
                return;
            }

            let files = fs::read_dir(&thumbnails_dir).and_then(|e| e.collect::<Result<Vec<_>, std::io::Error>>()).map(|e| {
                e.into_iter()
                    .filter(|entry| entry.path().is_file())
                    .map(|e| e.path().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
            });
            let files = match files {
                Ok(files) => files,
                Err(e) => {
                    error!("Failed to read video thumbnails directory(\"{}\") - {}", thumbnails_dir.to_string_lossy(), e);
                    return;
                }
            };

            let mut removed_files = 0;
            let mut removed_size = 0u64;
            for file in files {
                let metadata = fs::metadata(&file);
                let metadata = match metadata {
                    Ok(metadata) => metadata,
                    Err(e) => {
                        error!("Failed to get metadata for file(\"{file}\") - {e}");
                        continue;
                    }
                };

                let modified = metadata.modified();
                let modified = match modified {
                    Ok(modified) => modified,
                    Err(e) => {
                        error!("Failed to get modified time for file(\"{file}\") - {e}");
                        continue;
                    }
                };

                let age = std::time::SystemTime::now().duration_since(modified);
                let age = match age {
                    Ok(age) => age,
                    Err(e) => {
                        error!("Failed to calculate age for file(\"{file}\") - {e}");
                        continue;
                    }
                };

                if age.as_secs() > DURATION_BEFORE_CLEANING_SECS {
                    let file_size = metadata.len();
                    let result = fs::remove_file(&file);
                    if let Err(e) = result {
                        error!("Failed to remove outdated thumbnail file(\"{file}\") - {e}");
                        continue;
                    }
                    removed_files += 1;
                    removed_size += file_size;
                }
            }

            if removed_files > 0 {
                info!(
                    "Cleared outdated video thumbnails: removed {} files, freed {}",
                    removed_files,
                    format_size(removed_size, BINARY),
                );
            } else {
                debug!("No outdated video thumbnails to clear.");
            }
        });
    }
}
