#![allow(clippy::collapsible_else_if)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]
#![warn(clippy::unwrap_used)]

#[macro_use]
extern crate bitflags;
extern crate core;

pub mod common;
pub mod common_cache;
pub mod common_dir_traversal;
pub mod common_directory;
pub mod common_extensions;
pub mod common_image;
pub mod common_items;
pub mod common_messages;
pub mod common_tool;
pub mod common_traits;
pub mod localizer_core;
pub mod progress_data;
pub mod tools;

pub const CZKAWKA_VERSION: &str = env!("CARGO_PKG_VERSION");
