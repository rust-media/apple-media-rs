use core_foundation::base::{Boolean, CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::CFStringRef;
use core_media::base::CMItemCount;
use core_media::format_description::CMVideoCodecType;
use core_media::sample_buffer::CMSampleBufferRef;
use core_media::time::CMTime;
use core_media::time_range::CMTimeRange;
use core_video::image_buffer::CVImageBufferRef;
use core_video::pixel_buffer_pool::CVPixelBufferPoolRef;
use libc::{c_int, c_void};

pub const kVTUnlimitedFrameDelayCount: c_int = -1;

pub type VTEncodeInfoFlags = u32;
pub type VTCompressionSessionOptionFlags = u32;

pub type VTCompressionSessionRef = CFTypeRef;
pub type VTCompressionOutputCallback = extern "C" fn(
    outputCallbackRefCon: *mut c_void,
    sourceFrameRefCon: *mut c_void,
    status: OSStatus,
    infoFlags: VTEncodeInfoFlags,
    sampleBuffer: CMSampleBufferRef,
);
pub type VTCompressionOutputHandler = extern "C" fn(status: OSStatus, infoFlags: VTEncodeInfoFlags, sampleBuffer: CMSampleBufferRef);

// VTEncodeInfoFlags
//
// Informational status for encoding -- non-error flags
pub const kVTEncodeInfo_Asynchronous: VTEncodeInfoFlags = 1 << 0;
pub const kVTEncodeInfo_FrameDropped: VTEncodeInfoFlags = 1 << 1;

// VTCompressionSessionOptionFlags
//
pub const kVTCompressionSessionBeginFinalPass: VTCompressionSessionOptionFlags = 1 << 0;

#[link(name = "VideoToolBox", kind = "framework")]
extern "C" {
    pub static kVTVideoEncoderSpecification_EncoderID: CFStringRef;

    // Creating Sessions
    pub fn VTCompressionSessionCreate(
        allocator: CFAllocatorRef,
        width: i32,
        height: i32,
        codecType: CMVideoCodecType,
        encoderSpecification: CFDictionaryRef,
        sourceImageBufferAttributes: CFDictionaryRef,
        compressedDataAllocator: CFAllocatorRef,
        outputCallback: VTCompressionOutputCallback,
        outputCallbackRefCon: *mut c_void,
        compressionSessionOut: *mut VTCompressionSessionRef,
    ) -> OSStatus;
    // Configuring Sessions
    pub static kVTCompressionPropertyKey_NumberOfPendingFrames: CFStringRef;
    pub static kVTCompressionPropertyKey_PixelBufferPoolIsShared: CFStringRef;
    pub static kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxKeyFrameInterval: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration: CFStringRef;
    pub static kVTCompressionPropertyKey_AllowTemporalCompression: CFStringRef;
    pub static kVTCompressionPropertyKey_AllowFrameReordering: CFStringRef;
    pub static kVTCompressionPropertyKey_AverageBitRate: CFStringRef;
    pub static kVTCompressionPropertyKey_DataRateLimits: CFStringRef;
    pub static kVTCompressionPropertyKey_Quality: CFStringRef;
    pub static kVTCompressionPropertyKey_MoreFramesBeforeStart: CFStringRef;
    pub static kVTCompressionPropertyKey_MoreFramesAfterEnd: CFStringRef;
    pub static kVTCompressionPropertyKey_ProfileLevel: CFStringRef;
    pub static kVTProfileLevel_HEVC_Main_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_HEVC_Main10_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_1_3: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_3_0: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_3_1: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_3_2: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_4_0: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_4_1: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_4_2: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_5_0: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_5_1: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_5_2: CFStringRef;
    pub static kVTProfileLevel_H264_Baseline_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_H264_Main_3_0: CFStringRef;
    pub static kVTProfileLevel_H264_Main_3_1: CFStringRef;
    pub static kVTProfileLevel_H264_Main_3_2: CFStringRef;
    pub static kVTProfileLevel_H264_Main_4_0: CFStringRef;
    pub static kVTProfileLevel_H264_Main_4_1: CFStringRef;
    pub static kVTProfileLevel_H264_Main_4_2: CFStringRef;
    pub static kVTProfileLevel_H264_Main_5_0: CFStringRef;
    pub static kVTProfileLevel_H264_Main_5_1: CFStringRef;
    pub static kVTProfileLevel_H264_Main_5_2: CFStringRef;
    pub static kVTProfileLevel_H264_Main_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_H264_Extended_5_0: CFStringRef;
    pub static kVTProfileLevel_H264_Extended_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_H264_High_3_0: CFStringRef;
    pub static kVTProfileLevel_H264_High_3_1: CFStringRef;
    pub static kVTProfileLevel_H264_High_3_2: CFStringRef;
    pub static kVTProfileLevel_H264_High_4_0: CFStringRef;
    pub static kVTProfileLevel_H264_High_4_1: CFStringRef;
    pub static kVTProfileLevel_H264_High_4_2: CFStringRef;
    pub static kVTProfileLevel_H264_High_5_0: CFStringRef;
    pub static kVTProfileLevel_H264_High_5_1: CFStringRef;
    pub static kVTProfileLevel_H264_High_5_2: CFStringRef;
    pub static kVTProfileLevel_H264_High_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_MP4V_Simple_L0: CFStringRef;
    pub static kVTProfileLevel_MP4V_Simple_L1: CFStringRef;
    pub static kVTProfileLevel_MP4V_Simple_L2: CFStringRef;
    pub static kVTProfileLevel_MP4V_Simple_L3: CFStringRef;
    pub static kVTProfileLevel_MP4V_Main_L2: CFStringRef;
    pub static kVTProfileLevel_MP4V_Main_L3: CFStringRef;
    pub static kVTProfileLevel_MP4V_Main_L4: CFStringRef;
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L0: CFStringRef;
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L1: CFStringRef;
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L2: CFStringRef;
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L3: CFStringRef;
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L4: CFStringRef;
    pub static kVTProfileLevel_H263_Profile0_Level10: CFStringRef;
    pub static kVTProfileLevel_H263_Profile0_Level45: CFStringRef;
    pub static kVTProfileLevel_H263_Profile3_Level45: CFStringRef;
    pub static kVTCompressionPropertyKey_H264EntropyMode: CFStringRef;
    pub static kVTH264EntropyMode_CAVLC: CFStringRef;
    pub static kVTH264EntropyMode_CABAC: CFStringRef;
    pub static kVTCompressionPropertyKey_Depth: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxFrameDelayCount: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxH264SliceBytes: CFStringRef;
    pub static kVTCompressionPropertyKey_RealTime: CFStringRef;
    pub static kVTCompressionPropertyKey_SourceFrameCount: CFStringRef;
    pub static kVTCompressionPropertyKey_ExpectedFrameRate: CFStringRef;
    pub static kVTCompressionPropertyKey_ExpectedDuration: CFStringRef;
    pub static kVTCompressionPropertyKey_BaseLayerFrameRate: CFStringRef;
    pub static kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder: CFStringRef;
    pub static kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder: CFStringRef;
    pub static kVTVideoEncoderSpecification_EnableLowLatencyRateControl: CFStringRef;
    pub static kVTCompressionPropertyKey_UsingHardwareAcceleratedVideoEncoder: CFStringRef;
    pub static kVTEncodeFrameOptionKey_ForceKeyFrame: CFStringRef;
    pub static kVTCompressionPropertyKey_CleanAperture: CFStringRef;
    pub static kVTCompressionPropertyKey_PixelAspectRatio: CFStringRef;
    pub static kVTCompressionPropertyKey_FieldCount: CFStringRef;
    pub static kVTCompressionPropertyKey_FieldDetail: CFStringRef;
    pub static kVTCompressionPropertyKey_AspectRatio16x9: CFStringRef;
    pub static kVTCompressionPropertyKey_ProgressiveScan: CFStringRef;
    pub static kVTCompressionPropertyKey_ColorPrimaries: CFStringRef;
    pub static kVTCompressionPropertyKey_TransferFunction: CFStringRef;
    pub static kVTCompressionPropertyKey_YCbCrMatrix: CFStringRef;
    pub static kVTCompressionPropertyKey_ICCProfile: CFStringRef;
    pub static kVTCompressionPropertyKey_MasteringDisplayColorVolume: CFStringRef;
    pub static kVTCompressionPropertyKey_ContentLightLevelInfo: CFStringRef;
    pub static kVTCompressionPropertyKey_PixelTransferProperties: CFStringRef;
    pub static kVTCompressionPropertyKey_MultiPassStorage: CFStringRef;
    pub static kVTCompressionPropertyKey_EncoderID: CFStringRef;

    // Encoding Frames
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

    // Inspecting Sessions
    pub fn VTCompressionSessionGetPixelBufferPool(session: VTCompressionSessionRef) -> CVPixelBufferPoolRef;
    pub fn VTCompressionSessionGetTypeID() -> CFTypeID;

    // Performing Multipass Compression
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

    // Ending Sessions
    pub fn VTCompressionSessionInvalidate(session: VTCompressionSessionRef);

}
