pub mod common;
pub mod helpers;
pub mod localizer_core;
pub mod tools;

pub mod re_exported {
    pub use image_hasher::{FilterType, HashAlg};
    pub use vid_dup_finder_lib::Cropdetect;
}

pub const CZKAWKA_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TOOLS_NUMBER: usize = 13;
