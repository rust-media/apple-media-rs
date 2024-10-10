#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]

extern crate core_foundation;
extern crate core_graphics2 as core_graphics;
extern crate core_media;
extern crate core_video;
extern crate dispatch2;
extern crate libc;
#[macro_use]
extern crate objc2;
extern crate objc2_foundation;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}

#[cfg(feature = "capture")]
pub mod capture_device;
#[cfg(feature = "capture")]
pub mod capture_input;
#[cfg(feature = "capture")]
pub mod capture_output_base;
#[cfg(feature = "capture")]
pub mod capture_session;
#[cfg(feature = "capture")]
pub mod capture_session_preset;
#[cfg(feature = "capture")]
pub mod capture_video_data_output;
pub mod media_format;
pub mod video_settings;
