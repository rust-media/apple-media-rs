#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]

pub mod base;
pub mod compression_properties;
pub mod compression_session;
pub mod decompression_properties;
pub mod decompression_session;
pub mod errors;
pub mod session;
pub mod utilities;

#[link(name = "VideoToolbox", kind = "framework")]
extern "C" {}
