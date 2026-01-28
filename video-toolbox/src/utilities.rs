use std::ptr::{null, null_mut};

use core_foundation::{
    base::{CFType, OSStatus, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    string::CFString,
};
use core_graphics2::image::{CGImage, CGImageRef};
use core_video::pixel_buffer::{CVPixelBuffer, CVPixelBufferRef};

extern "C" {
    pub fn VTCreateCGImageFromCVPixelBuffer(pixelBuffer: CVPixelBufferRef, options: CFDictionaryRef, imageOut: *mut CGImageRef) -> OSStatus;
}

pub fn create_cg_image_from_cv_pixel_buffer(
    pixel_buffer: &CVPixelBuffer,
    options: Option<&CFDictionary<CFString, CFType>>,
) -> Result<CGImage, OSStatus> {
    let mut image_out: CGImageRef = null_mut();
    let status = unsafe {
        VTCreateCGImageFromCVPixelBuffer(
            pixel_buffer.as_concrete_TypeRef(),
            options.map(|opts| opts.as_concrete_TypeRef()).unwrap_or(null()),
            &mut image_out,
        )
    };
    if status == 0 {
        Ok(unsafe { CGImage::wrap_under_create_rule(image_out) })
    } else {
        Err(status)
    }
}
