use std::env;

use file_rotate::compression::Compression;
use file_rotate::suffix::{AppendTimestamp, FileLimit};
use file_rotate::{ContentLimit, FileRotate};
use handsome_logger::{ColorChoice, CombinedLogger, ConfigBuilder, FormatText, SharedLogger, TermLogger, TerminalMode, TimeFormat, WriteLogger};
use log::{LevelFilter, Record, info, warn};

use crate::CZKAWKA_VERSION;
use crate::common::config_cache_path::get_config_cache_path;
use crate::common::get_all_available_threads;

pub fn setup_logger(disabled_terminal_printing: bool, app_name: &str, filtering_messages_func: fn(&Record) -> bool) {
    log_panics::init();

    let terminal_log_level = if disabled_terminal_printing && ![Ok("1"), Ok("true")].contains(&env::var("ENABLE_TERMINAL_LOGS_IN_CLI").as_deref()) {
        LevelFilter::Off
    } else {
        LevelFilter::Info
    };
    let file_log_level = LevelFilter::Debug;

    let term_config = ConfigBuilder::default()
        .set_level(terminal_log_level)
        .set_message_filtering(Some(filtering_messages_func))
        .build();
    let file_config = ConfigBuilder::default()
        .set_level(file_log_level)
        .set_write_once(true)
        .set_message_filtering(Some(filtering_messages_func))
        .set_time_format(TimeFormat::DateTimeWithMicro, None)
        .set_format_text(FormatText::DefaultWithThreadFile.get(), None)
        .build();

    let combined_logger = (|| {
        let Some(config_cache_path) = get_config_cache_path() else {
            // println!("No config cache path configured, using default config folder");
            return None;
        };

        let cache_logs_path = config_cache_path.cache_folder.join(format!("{app_name}.log"));

        let write_rotater = FileRotate::new(
            &cache_logs_path,
            AppendTimestamp::default(FileLimit::MaxFiles(3)),
            ContentLimit::BytesSurpassed(100 * 1024 * 1024),
            Compression::None,
            None,
        );

        let combined_logs: Vec<Box<dyn SharedLogger>> = if [Ok("1"), Ok("true")].contains(&env::var("DISABLE_FILE_LOGGING").as_deref()) {
            vec![TermLogger::new_from_config(term_config.clone())]
        } else {
            vec![TermLogger::new_from_config(term_config.clone()), WriteLogger::new(file_config, write_rotater)]
        };

        CombinedLogger::init(combined_logs).ok().inspect(|()| {
            info!("Logging to file \"{}\" and terminal", cache_logs_path.to_string_lossy());
        })
    })();

    if combined_logger.is_none() {
        TermLogger::init(term_config, TerminalMode::Mixed, ColorChoice::Always).expect("Cannot initialize logger");
        info!("Logging to terminal only, file logging is disabled");
    }
}

pub fn filtering_messages(record: &Record) -> bool {
    if let Some(module_path) = record.module_path() {
        // Printing not supported modules
        // if !["krokiet", "czkawka", "log_panics", "smithay_client_toolkit", "sctk_adwaita"]
        //     .iter()
        //     .any(|t| module_path.starts_with(t))
        // {
        //     println!("{:?}", module_path);
        //     return true;
        // } else {
        //     return false;
        // }

        ["krokiet", "czkawka", "log_panics"].iter().any(|t| module_path.starts_with(t))
    } else {
        true
    }
}

#[allow(clippy::allow_attributes)]
#[allow(unfulfilled_lint_expectations)] // Happens only on release build
#[expect(clippy::vec_init_then_push)]
#[expect(unused_mut)]
pub fn print_version_mode(app: &str) {
    let rust_version = env!("RUST_VERSION_INTERNAL");
    let debug_release = if cfg!(debug_assertions) { "debug" } else { "release" };

    let processors = get_all_available_threads();

    let info = os_info::get();

    let mut features: Vec<&str> = vec![];
    #[cfg(feature = "heif")]
    features.push("heif");
    #[cfg(feature = "libavif")]
    features.push("libavif");
    #[cfg(feature = "libraw")]
    features.push("libraw");

    let mut app_cpu_version = "Baseline";
    let mut os_cpu_version = "Baseline";
    if cfg!(target_feature = "sse2") {
        app_cpu_version = "x86-64-v1 (SSE2)";
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("sse2") {
        os_cpu_version = "x86-64-v1 (SSE2)";
    }

    if cfg!(target_feature = "popcnt") {
        app_cpu_version = "x86-64-v2 (SSE4.2 + POPCNT)";
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("popcnt") {
        os_cpu_version = "x86-64-v2 (SSE4.2 + POPCNT)";
    }

    if cfg!(target_feature = "avx2") {
        app_cpu_version = "x86-64-v3 (AVX2)";
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx2") {
        os_cpu_version = "x86-64-v3 (AVX2)";
    }

    if cfg!(target_feature = "avx512f") {
        app_cpu_version = "x86-64-v4 (AVX-512)";
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx512f") {
        os_cpu_version = "x86-64-v4 (AVX-512)";
    }

    let musl_or_glibc = if cfg!(target_os = "linux") {
        let libc_versions_str = match glibc_musl_version::get_os_libc_versions() {
            Ok(libc_versions) => {
                let libc_versions_str = libc_versions.to_string();

                match option_env!("CZKAWKA_LIBC_VERSIONS") {
                    Some(env) if env == libc_versions_str => {
                        format!(" [build + runtime ({libc_versions_str})]")
                    }
                    Some(env) => {
                        format!(" [build ({env}), runtime ({libc_versions_str})]")
                    }
                    None => {
                        format!(" [build (unknown), runtime ({libc_versions_str})]")
                    }
                }
            }
            Err(e) => {
                warn!("Cannot get libc version: {e}");
                "".to_string()
            }
        };
        format!(", libc {}{libc_versions_str}", option_env!("CZKAWKA_LIBC").unwrap_or("unknown(cross-compilation?)"))
    } else {
        "".to_string()
    };

    info!(
        "{app} version: {CZKAWKA_VERSION}, {debug_release} mode, rust {rust_version}, os {} {} ({} {}), {processors} cpu/threads, features({}): [{}], app cpu version: {app_cpu_version}, os cpu version: {os_cpu_version}{musl_or_glibc}",
        info.os_type(),
        info.version(),
        env::consts::ARCH,
        info.bitness(),
        features.len(),
        features.join(", "),
    );
    if cfg!(debug_assertions) {
        warn!("You are running debug version of app which is a lot of slower than release version.");
    }

    if option_env!("USING_CRANELIFT").is_some() {
        warn!("You are running app with cranelift which is intended only for fast compilation, not runtime performance.");
    }

    if cfg!(panic = "abort") {
        warn!("You are running app compiled with panic='abort', which may cause panics when processing untrusted data.");
    }
}
