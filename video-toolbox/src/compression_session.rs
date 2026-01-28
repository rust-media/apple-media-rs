use std::ptr::{null, null_mut};

use bitflags::bitflags;
use block::{Block, ConcreteBlock};
use core_foundation::{
    base::{Boolean, CFAllocator, CFAllocatorRef, CFType, CFTypeID, OSStatus, TCFType},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use core_media::{base::CMItemCount, format_description::CMVideoCodecType, sample_buffer::CMSampleBufferRef, time::CMTime, time_range::CMTimeRange};
use core_video::{
    image_buffer::{CVImageBuffer, CVImageBufferRef},
    pixel_buffer_pool::{CVPixelBufferPool, CVPixelBufferPoolRef},
};
use libc::c_void;

use crate::{
    errors::VTEncodeInfoFlags,
    session::{TVTSession, VTSessionRef},
};

pub type VTCompressionSessionRef = VTSessionRef;

pub type VTCompressionOutputCallback = extern "C" fn(
    outputCallbackRefCon: *mut c_void,
    sourceFrameRefCon: *mut c_void,
    status: OSStatus,
    infoFlags: VTEncodeInfoFlags,
    sampleBuffer: CMSampleBufferRef,
);

pub type VTCompressionOutputHandler = *const Block<(OSStatus, VTEncodeInfoFlags, CMSampleBufferRef), ()>;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct VTCompressionSessionOptionFlags: u32 {
        #[doc(alias = "kVTCompressionSessionBeginFinalPass")]
        const BeginFinalPass = 1 << 0;
    }
}

extern "C" {
    pub static kVTVideoEncoderSpecification_EncoderID: CFStringRef;

    pub fn VTCompressionSessionCreate(
        allocator: CFAllocatorRef,
        width: i32,
        height: i32,
        codecType: CMVideoCodecType,
        encoderSpecification: CFDictionaryRef,
        sourceImageBufferAttributes: CFDictionaryRef,
        compressedDataAllocator: CFAllocatorRef,
        outputCallback: *const VTCompressionOutputCallback,
        outputCallbackRefCon: *mut c_void,
        compressionSessionOut: *mut VTCompressionSessionRef,
    ) -> OSStatus;
    pub fn VTCompressionSessionInvalidate(session: VTCompressionSessionRef) -> ();
    pub fn VTCompressionSessionGetTypeID() -> CFTypeID;
    pub fn VTCompressionSessionGetPixelBufferPool(session: VTCompressionSessionRef) -> CVPixelBufferPoolRef;
    pub fn VTCompressionSessionPrepareToEncodeFrames(session: VTCompressionSessionRef) -> OSStatus;
    pub fn VTCompressionSessionEncodeFrame(
        session: VTCompressionSessionRef,
        imageBuffer: CVImageBufferRef,
        presentationTimeStamp: CMTime,
        duration: CMTime,
        frameProperties: CFDictionaryRef,
        sourceFrameRefcon: *mut c_void,
        infoFlagsOut: *mut VTEncodeInfoFlags,
    ) -> OSStatus;
    pub fn VTCompressionSessionEncodeFrameWithOutputHandler(
        session: VTCompressionSessionRef,
        imageBuffer: CVImageBufferRef,
        presentationTimeStamp: CMTime,
        duration: CMTime,
        frameProperties: CFDictionaryRef,
        infoFlagsOut: *mut VTEncodeInfoFlags,
        outputHandler: VTCompressionOutputHandler,
    ) -> OSStatus;
    pub fn VTCompressionSessionCompleteFrames(session: VTCompressionSessionRef, completeUntilPresentationTimeStamp: CMTime) -> OSStatus;
    pub fn VTCompressionSessionBeginPass(
        session: VTCompressionSessionRef,
        beginPassFlags: VTCompressionSessionOptionFlags,
        reserved: *mut u32,
    ) -> OSStatus;
    pub fn VTCompressionSessionEndPass(session: VTCompressionSessionRef, furtherPassesRequestedOut: *mut Boolean, reserved: *mut u32) -> OSStatus;
    pub fn VTCompressionSessionGetTimeRangesForNextPass(
        session: VTCompressionSessionRef,
        timeRangeCountOut: *mut CMItemCount,
        timeRangeArrayOut: *const CMTimeRange,
    ) -> OSStatus;
}

declare_TCFType!(VTCompressionSession, VTCompressionSessionRef);
impl_TCFType!(VTCompressionSession, VTCompressionSessionRef, VTCompressionSessionGetTypeID);
impl_CFTypeDescription!(VTCompressionSession);

impl TVTSession for VTCompressionSession {}

impl VTCompressionSession {
    pub fn new(
        width: i32,
        height: i32,
        codec_type: CMVideoCodecType,
        encoder_specification: CFDictionary<CFString, CFType>,
        source_image_buffer_attributes: CFDictionary<CFString, CFType>,
        compressed_data_allocator: CFAllocator,
    ) -> Result<VTCompressionSession, OSStatus> {
        unsafe {
            Self::new_with_callback(
                width,
                height,
                codec_type,
                encoder_specification,
                source_image_buffer_attributes,
                compressed_data_allocator,
                None,
                None,
            )
        }
    }
    pub unsafe fn new_with_callback(
        width: i32,
        height: i32,
        codec_type: CMVideoCodecType,
        encoder_specification: CFDictionary<CFString, CFType>,
        source_image_buffer_attributes: CFDictionary<CFString, CFType>,
        compressed_data_allocator: CFAllocator,
        output_callback: Option<*const VTCompressionOutputCallback>,
        output_callback_ref_con: Option<*mut c_void>,
    ) -> Result<VTCompressionSession, OSStatus> {
        let mut session: VTCompressionSessionRef = null_mut();
        let status = VTCompressionSessionCreate(
            compressed_data_allocator.as_concrete_TypeRef(),
            width,
            height,
            codec_type,
            encoder_specification.as_concrete_TypeRef(),
            source_image_buffer_attributes.as_concrete_TypeRef(),
            compressed_data_allocator.as_concrete_TypeRef(),
            output_callback.unwrap_or(null()),
            output_callback_ref_con.unwrap_or(null_mut()),
            &mut session,
        );
        if status == 0 {
            Ok(unsafe { VTCompressionSession::wrap_under_create_rule(session) })
        } else {
            Err(status)
        }
    }

    pub fn invalidate(&self) {
        unsafe { VTCompressionSessionInvalidate(self.as_concrete_TypeRef()) }
    }

    pub fn get_pixel_buffer_pool(&self) -> CVPixelBufferPool {
        unsafe {
            let pool_ref = VTCompressionSessionGetPixelBufferPool(self.as_concrete_TypeRef());
            CVPixelBufferPool::wrap_under_get_rule(pool_ref)
        }
    }

    pub fn prepare_to_encode_frames(&self) -> Result<(), OSStatus> {
        let status = unsafe { VTCompressionSessionPrepareToEncodeFrames(self.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub unsafe fn encode_frame(
        &self,
        image_buffer: CVImageBuffer,
        presentation_time_stamp: CMTime,
        duration: CMTime,
        frame_properties: CFDictionary<CFString, CFType>,
        source_frame_refcon: *mut c_void,
    ) -> Result<VTEncodeInfoFlags, OSStatus> {
        let mut info_flags_out: VTEncodeInfoFlags = VTEncodeInfoFlags::empty();

        let status = VTCompressionSessionEncodeFrame(
            self.as_concrete_TypeRef(),
            image_buffer.as_concrete_TypeRef(),
            presentation_time_stamp,
            duration,
            frame_properties.as_concrete_TypeRef(),
            source_frame_refcon,
            &mut info_flags_out,
        );

        if status == 0 {
            Ok(info_flags_out)
        } else {
            Err(status)
        }
    }

    pub fn encode_frame_with_closure<F>(
        &self,
        image_buffer: CVImageBuffer,
        presentation_time_stamp: CMTime,
        duration: CMTime,
        frame_properties: CFDictionary<CFString, CFType>,
        closure: F,
    ) -> Result<VTEncodeInfoFlags, OSStatus>
    where
        F: Fn(OSStatus, VTEncodeInfoFlags, CMSampleBufferRef) + 'static,
    {
        let handler = ConcreteBlock::new(move |status: OSStatus, info_flags: VTEncodeInfoFlags, sample_buffer: CMSampleBufferRef| {
            closure(status, info_flags, sample_buffer)
        })
        .copy();

        let mut info_flags_out: VTEncodeInfoFlags = VTEncodeInfoFlags::empty();

        let status = unsafe {
            VTCompressionSessionEncodeFrameWithOutputHandler(
                self.as_concrete_TypeRef(),
                image_buffer.as_concrete_TypeRef(),
                presentation_time_stamp,
                duration,
                frame_properties.as_concrete_TypeRef(),
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

    pub fn complete_frames(&self, complete_until_presentation_time_stamp: CMTime) -> Result<(), OSStatus> {
        let status = unsafe { VTCompressionSessionCompleteFrames(self.as_concrete_TypeRef(), complete_until_presentation_time_stamp) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn begin_pass(&self, begin_pass_flags: VTCompressionSessionOptionFlags) -> Result<(), OSStatus> {
        let status = unsafe { VTCompressionSessionBeginPass(self.as_concrete_TypeRef(), begin_pass_flags, null_mut()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn end_pass(&self) -> Result<Boolean, OSStatus> {
        let mut further_passes_requested_out: Boolean = 0;
        let status = unsafe { VTCompressionSessionEndPass(self.as_concrete_TypeRef(), &mut further_passes_requested_out, null_mut()) };
        if status == 0 {
            Ok(further_passes_requested_out)
        } else {
            Err(status)
        }
    }

    pub fn get_time_ranges_for_next_pass(&self) -> Result<Vec<CMTimeRange>, OSStatus> {
        let mut time_range_count_out: CMItemCount = 0;

        let status = unsafe { VTCompressionSessionGetTimeRangesForNextPass(self.as_concrete_TypeRef(), &mut time_range_count_out, null()) };
        if status != 0 {
            return Err(status);
        }

        let mut time_range_array: Vec<CMTimeRange> = Vec::with_capacity(time_range_count_out as usize);
        let status = unsafe {
            let result =
                VTCompressionSessionGetTimeRangesForNextPass(self.as_concrete_TypeRef(), &mut time_range_count_out, time_range_array.as_mut_ptr());
            if result == 0 {
                time_range_array.set_len(time_range_count_out as usize);
            }
            result
        };

        if status == 0 {
            Ok(time_range_array)
        } else {
            Err(status)
        }
    }
}
