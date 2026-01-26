#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]

extern crate core_graphics2 as core_graphics;

#[cfg(target_os = "macos")]
#[link(name = "ScreenCaptureKit", kind = "framework")]
extern "C" {}

pub mod encode;
pub mod error;
pub mod shareable_content;
pub mod stream;
