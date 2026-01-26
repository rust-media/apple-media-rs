#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]

extern crate core_graphics2 as core_graphics;

pub type OSType = u32;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[cfg_attr(feature = "link", link(name = "CoreMedia", kind = "framework"))]
extern "C" {}

pub mod attachment;
#[cfg(target_os = "ios")]
pub mod audio_clock;
pub mod audio_device_clock;
pub mod base;
pub mod block_buffer;
pub mod buffer_queue;
pub mod format_description;
pub mod format_description_bridge;
pub mod sample_buffer;
pub mod sample_queue;
pub mod sync;
pub mod time;
pub mod time_range;
