use core_foundation::base::{Boolean, CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::CFStringRef;
use core_media::format_description::{CMFormatDescriptionRef, CMVideoFormatDescriptionRef};
use core_media::{sample_buffer::CMSampleBufferRef, time::CMTime};
use core_video::{image_buffer::CVImageBufferRef, pixel_buffer::CVPixelBufferRef};
use libc::c_void;

pub type VTDecodeInfoFlags = u32;
pub type VTDecodeFrameFlags = u32;

// #[allow(missing_copy_implementations)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTDecompressionOutputCallbackRecord {
    pub decompressionOutputCallback: VTDecompressionOutputCallback,
    pub decompressionOutputRefCon: *mut c_void,
}

pub type VTDecompressionSessionRef = CFTypeRef;

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct OpaqueVTDecompressionSession;
// pub type VTDecompressionSessionRef = *mut OpaqueVTDecompressionSession;

pub type VTDecompressionOutputCallback = extern "C" fn(
    decompressionOutputRefCon: *mut c_void,
    sourceFrameRefCon: *mut c_void,
    status: OSStatus,
    infoFlags: VTDecodeInfoFlags,
    imageBuffer: CVImageBufferRef,
    presentationTimeStamp: CMTime,
    presentationDuration: CMTime,
);
pub type VTDecompressionOutputHandler = extern "C" fn(
    status: OSStatus,
    infoFlags: VTDecodeInfoFlags,
    imageBuffer: CVImageBufferRef,
    presentationTimeStamp: CMTime,
    presentationDuration: CMTime,
);

// VTDecodeFrameFlags
//
/// With the kVTDecodeFrame_EnableAsynchronousDecompression bit clear,
/// the video decoder is compelled to emit every frame before it returns.
/// With the bit set, the decoder may process frames asynchronously,
/// but it is not compelled to do so.
pub const kVTDecodeFrame_EnableAsynchronousDecompression: VTDecodeFrameFlags = 1 << 0;
/// A hint to the decompression session and video decoder that a CVImageBuffer
/// should not be emitted for this frame.  NULL will be returned instead.
pub const kVTDecodeFrame_DoNotOutputFrame: VTDecodeFrameFlags = 1 << 1;
/// A hint to the video decoder that it would be OK to use a low-power mode
/// that can not decode faster than 1x realtime.
pub const kVTDecodeFrame_1xRealTimePlayback: VTDecodeFrameFlags = 1 << 2;
/// With the kVTDecodeFrame_EnableTemporalProcessing bit clear,
/// the video decoder should emit every frame once that frame's
/// decoding is done -- frames may not be delayed indefinitely.
/// With the bit set, it is legal for the decoder to delay frames
/// indefinitely -- at least until VTDecompressionSessionFinishDelayedFrames
/// or VTDecompressionSessionInvalidate is called.
pub const kVTDecodeFrame_EnableTemporalProcessing: VTDecodeFrameFlags = 1 << 3;

// VTDecodeInfoFlags
//
// Informational status for decoding -- non-error flags
pub const kVTDecodeInfo_Asynchronous: VTDecodeInfoFlags = 1 << 0;
pub const kVTDecodeInfo_FrameDropped: VTDecodeInfoFlags = 1 << 1;
pub const kVTDecodeInfo_ImageBufferModifiable: VTDecodeInfoFlags = 1 << 2;

#[link(name = "VideoToolBox", kind = "framework")]
extern "C" {
    pub static kVTDecompressionPropertyKey_PixelBufferPool: CFStringRef;
    pub static kVTDecompressionPropertyKey_PixelBufferPoolIsShared: CFStringRef;
    pub static kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount: CFStringRef;
    pub static kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded: CFStringRef;
    pub static kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded: CFStringRef;
    pub static kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded: CFStringRef;
    pub static kVTDecompressionPropertyKey_ContentHasInterframeDependencies: CFStringRef;
    pub static kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder: CFStringRef;
    pub static kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder: CFStringRef;
    pub static kVTDecompressionPropertyKey_UsingHardwareAcceleratedVideoDecoder: CFStringRef;
    pub static kVTDecompressionPropertyKey_RealTime: CFStringRef;
    pub static kVTDecompressionPropertyKey_ThreadCount: CFStringRef;
    pub static kVTDecompressionPropertyKey_FieldMode: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_BothFields: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_TopFieldOnly: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_BottomFieldOnly: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_SingleField: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_DeinterlaceFields: CFStringRef;
    pub static kVTDecompressionPropertyKey_DeinterlaceMode: CFStringRef;
    pub static kVTDecompressionProperty_DeinterlaceMode_VerticalFilter: CFStringRef;
    pub static kVTDecompressionProperty_DeinterlaceMode_Temporal: CFStringRef;
    pub static kVTDecompressionPropertyKey_ReducedResolutionDecode: CFStringRef;
    pub static kVTDecompressionResolutionKey_Width: CFStringRef;
    pub static kVTDecompressionResolutionKey_Height: CFStringRef;
    pub static kVTDecompressionPropertyKey_ReducedCoefficientDecode: CFStringRef;
    pub static kVTDecompressionPropertyKey_ReducedFrameDelivery: CFStringRef;
    pub static kVTDecompressionPropertyKey_OnlyTheseFrames: CFStringRef;
    pub static kVTDecompressionProperty_OnlyTheseFrames_AllFrames: CFStringRef;
    pub static kVTDecompressionProperty_OnlyTheseFrames_NonDroppableFrames: CFStringRef;
    pub static kVTDecompressionProperty_OnlyTheseFrames_IFrames: CFStringRef;
    pub static kVTDecompressionProperty_OnlyTheseFrames_KeyFrames: CFStringRef;
    pub static kVTDecompressionProperty_TemporalLevelLimit: CFStringRef;
    pub static kVTDecompressionPropertyKey_SuggestedQualityOfServiceTiers: CFStringRef;
    pub static kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByQuality: CFStringRef;
    pub static kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance: CFStringRef;
    pub static kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport: CFStringRef;
    pub static kVTDecompressionPropertyKey_PixelTransferProperties: CFStringRef;
    pub static kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID: CFStringRef;
    pub static kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID: CFStringRef;

    // Creating Sessions
    pub fn VTDecompressionSessionCreate(
        allocator: CFAllocatorRef,
        videoFormatDescription: CMVideoFormatDescriptionRef,
        videoDecoderSpecification: CFDictionaryRef,
        destinationImageBufferAttributes: CFDictionaryRef,
        outputCallback: *const VTDecompressionOutputCallbackRecord,
        decompressionSessionOut: *mut VTDecompressionSessionRef,
    ) -> OSStatus;

    // Decoding Sample Buffers
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
    pub fn VTDecompressionSessionWaitForAsynchronousFrames(session: VTDecompressionSessionRef) -> OSStatus;
    pub fn VTDecompressionSessionCopyBlackPixelBuffer(session: VTDecompressionSessionRef, pixelBufferOut: *mut CVPixelBufferRef) -> OSStatus;

    // Invalidating a Session
    pub fn VTDecompressionSessionInvalidate(session: VTDecompressionSessionRef);

    // Inspecting Sessions
    pub fn VTDecompressionSessionGetTypeID() -> CFTypeID;
    pub fn VTDecompressionSessionCanAcceptFormatDescription(session: VTDecompressionSessionRef, newFormatDesc: CMFormatDescriptionRef) -> Boolean;
}
