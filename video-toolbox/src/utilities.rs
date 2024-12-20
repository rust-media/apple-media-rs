use core_foundation::array::CFArrayRef;
use core_foundation::base::{Boolean, CFTypeRef, OSStatus};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::CFStringRef;
use core_media::format_description::CMVideoCodecType;
use core_video::pixel_buffer::CVPixelBufferRef;

pub type CGImageRef = CFTypeRef;

#[link(name = "VideoToolBox", kind = "framework")]
extern "C" {

    pub fn VTRegisterProfessionalVideoWorkflowVideoDecoders();
    pub fn VTRegisterProfessionalVideoWorkflowVideoEncoders();

    pub fn VTCreateCGImageFromCVPixelBuffer(pixelBuffer: CVPixelBufferRef, options: CFDictionaryRef, imageOut: *mut CGImageRef) -> OSStatus;

    pub static kVTVideoEncoderList_CodecType: CFStringRef;
    pub static kVTVideoEncoderList_EncoderID: CFStringRef;

    pub static kVTVideoEncoderList_CodecName: CFStringRef;
    pub static kVTVideoEncoderList_EncoderName: CFStringRef;
    pub static kVTVideoEncoderList_DisplayName: CFStringRef;

    pub fn VTCopyVideoEncoderList(options: CFDictionaryRef, listOfVideoEncodersOut: *mut CFArrayRef) -> OSStatus;
    pub fn VTCopySupportedPropertyDictionaryForEncoder(
        width: i32,
        height: i32,
        codecType: CMVideoCodecType,
        encoderSpecification: CFDictionaryRef,
        outEncoderID: *mut CFStringRef,
        outSupportedProperties: *mut CFDictionaryRef,
    ) -> OSStatus;
    pub fn VTIsHardwareDecodeSupported(codecType: CMVideoCodecType) -> Boolean;
}
