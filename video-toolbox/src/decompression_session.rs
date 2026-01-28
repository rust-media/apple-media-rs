use std::ptr::{null, null_mut};

use block::{Block, ConcreteBlock};
use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, CFType, CFTypeID, OSStatus, TCFType},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::CFString,
};
use core_media::{
    format_description::{CMFormatDescription, CMFormatDescriptionRef, CMVideoCodecType, CMVideoFormatDescription, CMVideoFormatDescriptionRef},
    sample_buffer::{CMSampleBuffer, CMSampleBufferRef},
    time::CMTime,
};
use core_video::{
    image_buffer::{CVImageBuffer, CVImageBufferRef},
    pixel_buffer::{CVPixelBuffer, CVPixelBufferRef},
};
use libc::c_void;

use crate::{
    errors::{VTDecodeFrameFlags, VTDecodeInfoFlags},
    session::{TVTSession, VTSessionRef},
};

pub type VTDecompressionSessionRef = VTSessionRef;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTDecompressionOutputCallbackRecord {
    pub decompressionOutputCallback: VTDecompressionOutputCallback,
    pub decompressionOutputRefCon: *mut c_void,
}

pub type VTDecompressionOutputHandler = *const Block<(OSStatus, VTDecodeInfoFlags, CVImageBufferRef, CMTime, CMTime), ()>;

pub type VTDecompressionOutputCallback = extern "C" fn(
    decompressionOutputRefCon: *mut c_void,
    sourceFrameRefCon: *mut c_void,
    status: OSStatus,
    infoFlags: VTDecodeInfoFlags,
    imageBuffer: CVImageBufferRef,
    presentationTimeStamp: CMTime,
    presentationDuration: CMTime,
);

extern "C" {
    pub fn VTDecompressionSessionCreate(
        allocator: CFAllocatorRef,
        videoFormatDescription: CMVideoFormatDescriptionRef,
        videoDecoderSpecification: CFDictionaryRef,
        destinationImageBufferAttributes: CFDictionaryRef,
        outputCallback: *const VTDecompressionOutputCallbackRecord,
        decompressionSessionOut: *mut VTDecompressionSessionRef,
    ) -> OSStatus;
    pub fn VTDecompressionSessionGetTypeID() -> CFTypeID;
    pub fn VTDecompressionSessionDecodeFrame(
        session: VTDecompressionSessionRef,
        sampleBuffer: CMSampleBufferRef,
        decodeFlags: VTDecodeFrameFlags,
        sourceFrameRefCon: *mut c_void,
        infoFlagsOut: *mut VTDecodeInfoFlags,
    ) -> OSStatus;
    pub fn VTDecompressionSessionDecodeFrameWithOutputHandler(
        session: VTDecompressionSessionRef,
        sampleBuffer: CMSampleBufferRef,
        decodeFlags: VTDecodeFrameFlags,
        infoFlagsOut: *mut VTDecodeInfoFlags,
        outputHandler: VTDecompressionOutputHandler,
    ) -> OSStatus;
    pub fn VTDecompressionSessionFinishDelayedFrames(session: VTDecompressionSessionRef) -> OSStatus;
    pub fn VTDecompressionSessionCanAcceptFormatDescription(session: VTDecompressionSessionRef, newFormatDesc: CMFormatDescriptionRef) -> Boolean;
    pub fn VTDecompressionSessionWaitForAsynchronousFrames(session: VTDecompressionSessionRef) -> OSStatus;
    pub fn VTDecompressionSessionCopyBlackPixelBuffer(session: VTDecompressionSessionRef, pixelBufferOut: *mut CVPixelBufferRef) -> OSStatus;
    pub fn VTIsHardwareDecodeSupported(codecType: CMVideoCodecType) -> Boolean;
}

declare_TCFType!(VTDecompressionSession, VTDecompressionSessionRef);
impl_TCFType!(VTDecompressionSession, VTDecompressionSessionRef, VTDecompressionSessionGetTypeID);
impl_CFTypeDescription!(VTDecompressionSession);

impl TVTSession for VTDecompressionSession {}

impl VTDecompressionSession {
    pub fn new(
        video_format_description: CMVideoFormatDescription,
        video_decoder_specification: Option<CFDictionary<CFString, CFType>>,
        destination_image_buffer_attributes: Option<CFDictionary<CFString, CFType>>,
    ) -> Result<Self, OSStatus> {
        unsafe { Self::new_with_callback(video_format_description, video_decoder_specification, destination_image_buffer_attributes, None) }
    }

    pub unsafe fn new_with_callback(
        video_format_description: CMVideoFormatDescription,
        video_decoder_specification: Option<CFDictionary<CFString, CFType>>,
        destination_image_buffer_attributes: Option<CFDictionary<CFString, CFType>>,
        output_callback: Option<*const VTDecompressionOutputCallbackRecord>,
    ) -> Result<Self, OSStatus> {
        let mut session: VTDecompressionSessionRef = null_mut();
        let status = VTDecompressionSessionCreate(
            kCFAllocatorDefault,
            video_format_description.as_concrete_TypeRef(),
            video_decoder_specification.as_ref().map_or(null(), |s| s.as_concrete_TypeRef()),
            destination_image_buffer_attributes.as_ref().map_or(null(), |a| a.as_concrete_TypeRef()),
            output_callback.unwrap_or(null()),
            &mut session,
        );
        if status == 0 {
            Ok(TCFType::wrap_under_create_rule(session))
        } else {
            Err(status)
        }
    }

    pub unsafe fn decode_frame(
        &self,
        sample_buffer: CMSampleBuffer,
        decode_flags: VTDecodeFrameFlags,
        source_frame_ref_con: *mut c_void,
    ) -> Result<VTDecodeInfoFlags, OSStatus> {
        let mut info_flags_out: VTDecodeInfoFlags = VTDecodeInfoFlags::empty();

        let status = VTDecompressionSessionDecodeFrame(
            self.as_concrete_TypeRef(),
            sample_buffer.as_concrete_TypeRef(),
            decode_flags,
            source_frame_ref_con,
            &mut info_flags_out,
        );

        if status == 0 {
            Ok(info_flags_out)
        } else {
            Err(status)
        }
    }

    pub fn decode_frame_with_closure<F>(
        &self,
        sample_buffer: CMSampleBuffer,
        decode_flags: VTDecodeFrameFlags,
        closure: F,
    ) -> Result<VTDecodeInfoFlags, OSStatus>
    where
        F: Fn(OSStatus, VTDecodeInfoFlags, CVImageBuffer, CMTime, CMTime) + 'static,
    {
        let handler = ConcreteBlock::new(
            move |status: OSStatus, info_flags: VTDecodeInfoFlags, image_buffer: CVImageBufferRef, pts: CMTime, duration: CMTime| {
                let image_buffer = unsafe { CVImageBuffer::wrap_under_get_rule(image_buffer) };
                closure(status, info_flags, image_buffer, pts, duration)
            },
        )
        .copy();

        let mut info_flags_out: VTDecodeInfoFlags = VTDecodeInfoFlags::empty();

        let status = unsafe {
            VTDecompressionSessionDecodeFrameWithOutputHandler(
                self.as_concrete_TypeRef(),
                sample_buffer.as_concrete_TypeRef(),
                decode_flags,
                &mut info_flags_out,
                &*handler,
            )
        };

        if status == 0 {
            Ok(info_flags_out)
        } else {
            Err(status)
        }
    }

    pub fn finish_delayed_frames(&self) -> Result<(), OSStatus> {
        let status = unsafe { VTDecompressionSessionFinishDelayedFrames(self.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn can_accept_format_description(&self, new_format_desc: CMFormatDescription) -> bool {
        unsafe { VTDecompressionSessionCanAcceptFormatDescription(self.as_concrete_TypeRef(), new_format_desc.as_concrete_TypeRef()) != 0 }
    }

    pub fn wait_for_asynchronous_frames(&self) -> Result<(), OSStatus> {
        let status = unsafe { VTDecompressionSessionWaitForAsynchronousFrames(self.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn copy_black_pixel_buffer(&self) -> Result<CVPixelBuffer, OSStatus> {
        let mut pixel_buffer_out: CVPixelBufferRef = null_mut();
        let status = unsafe { VTDecompressionSessionCopyBlackPixelBuffer(self.as_concrete_TypeRef(), &mut pixel_buffer_out) };
        if status == 0 {
            Ok(unsafe { CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out) })
        } else {
            Err(status)
        }
    }

    pub fn is_hardware_decode_supported(codec_type: CMVideoCodecType) -> bool {
        unsafe { VTIsHardwareDecodeSupported(codec_type) != 0 }
    }
}
