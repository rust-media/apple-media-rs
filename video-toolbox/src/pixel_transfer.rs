use core_foundation_sys::base::{CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use core_foundation_sys::string::CFStringRef;
use core_video_sys::CVPixelBufferRef;

pub type VTPixelTransferSessionRef = CFTypeRef;

#[link(name = "VideoToolBox", kind = "framework")]
extern "C" {
    /// Scaling mode for images during transfer between source and destination buffers.
    pub static kVTPixelTransferPropertyKey_ScalingMode: CFStringRef;
    pub static kVTScalingMode_Normal: CFStringRef;
    pub static kVTScalingMode_CropSourceToCleanAperture: CFStringRef;
    pub static kVTScalingMode_Letterbox: CFStringRef;
    pub static kVTScalingMode_Trim: CFStringRef;
    pub static kVTPixelTransferPropertyKey_DestinationPixelAspectRatio: CFStringRef;
    pub static kVTPixelTransferPropertyKey_DownsamplingMode: CFStringRef;
    pub static kVTDownsamplingMode_Decimate: CFStringRef;
    pub static kVTDownsamplingMode_Average: CFStringRef;
    pub static kVTPixelTransferPropertyKey_DestinationColorPrimaries: CFStringRef;
    pub static kVTPixelTransferPropertyKey_DestinationTransferFunction: CFStringRef;
    pub static kVTPixelTransferPropertyKey_DestinationICCProfile: CFStringRef;
    pub static kVTPixelTransferPropertyKey_DestinationYCbCrMatrix: CFStringRef;

    pub fn VTPixelTransferSessionCreate(
        allocator: CFAllocatorRef,
        pixelTransferSessionOut: VTPixelTransferSessionRef,
    ) -> OSStatus;
    pub fn VTPixelTransferSessionTransferImage(
        session: VTPixelTransferSessionRef,
        sourceBuffer: CVPixelBufferRef,
        destinationBuffer: CVPixelBufferRef,
    ) -> OSStatus;
    pub fn VTPixelTransferSessionGetTypeID() -> CFTypeID;
    pub fn VTPixelTransferSessionInvalidate(session: VTPixelTransferSessionRef);

}
