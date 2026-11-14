// The portal implementation replaces trash::delete on Linux, leaving the dependency unused there
#[cfg(all(feature = "xdg_portal_trash", target_os = "linux"))]
use trash as _;

pub mod common;
pub mod helpers;
pub mod localizer_core;
pub mod tools;

pub mod re_exported {
    pub use fast_image_resize::FilterType as FirFilterType;
    pub use image_hasher::{FilterType, HashAlg};
}

pub const CZKAWKA_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TOOLS_NUMBER: usize = 14;
