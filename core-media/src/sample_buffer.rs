use std::ptr::{null, null_mut};

use block::{Block, ConcreteBlock};
use core_audio_types::base_types::{AudioBufferList, AudioStreamPacketDescription};
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, CFRange, CFType, CFTypeID, OSStatus, TCFType},
    declare_TCFType,
    dictionary::CFDictionary,
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use core_video::image_buffer::{CVImageBuffer, CVImageBufferRef};
use libc::{c_void, size_t};
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

use crate::{
    base::CMItemCount,
    block_buffer::{CMBlockBuffer, CMBlockBufferRef},
    format_description::{CMFormatDescription, CMFormatDescriptionRef, CMVideoFormatDescription, CMVideoFormatDescriptionRef},
    time::CMTime,
};

pub const kCMSampleBufferError_AllocationFailed: OSStatus = -12730;
pub const kCMSampleBufferError_RequiredParameterMissing: OSStatus = -12731;
pub const kCMSampleBufferError_AlreadyHasDataBuffer: OSStatus = -12732;
pub const kCMSampleBufferError_BufferNotReady: OSStatus = -12733;
pub const kCMSampleBufferError_SampleIndexOutOfRange: OSStatus = -12734;
pub const kCMSampleBufferError_BufferHasNoSampleSizes: OSStatus = -12735;
pub const kCMSampleBufferError_BufferHasNoSampleTimingInfo: OSStatus = -12736;
pub const kCMSampleBufferError_ArrayTooSmall: OSStatus = -12737;
pub const kCMSampleBufferError_InvalidEntryCount: OSStatus = -12738;
pub const kCMSampleBufferError_CannotSubdivide: OSStatus = -12739;
pub const kCMSampleBufferError_SampleTimingInfoInvalid: OSStatus = -12740;
pub const kCMSampleBufferError_InvalidMediaTypeForOperation: OSStatus = -12741;
pub const kCMSampleBufferError_InvalidSampleData: OSStatus = -12742;
pub const kCMSampleBufferError_InvalidMediaFormat: OSStatus = -12743;
pub const kCMSampleBufferError_Invalidated: OSStatus = -12744;
pub const kCMSampleBufferError_DataFailed: OSStatus = -16750;
pub const kCMSampleBufferError_DataCanceled: OSStatus = -16751;

pub const kCMSampleBufferFlag_AudioBufferList_Assure16ByteAlignment: u32 = 1 << 0;

#[repr(C)]
pub struct opaqueCMSampleBuffer(c_void);

pub type CMSampleBufferRef = *mut opaqueCMSampleBuffer;

#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSampleTimingInfo {
    pub duration: CMTime,
    pub presentationTimeStamp: CMTime,
    pub decodeTimeStamp: CMTime,
}

extern "C" {
    pub static kCMTimingInfoInvalid: CMSampleTimingInfo;
}

pub type CMSampleBufferMakeDataReadyCallback = extern "C" fn(CMSampleBufferRef, *mut c_void) -> OSStatus;
pub type CMSampleBufferMakeDataReadyHandler = *const Block<(CMSampleBufferRef,), OSStatus>;

extern "C" {
    pub fn CMSampleBufferCreate(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: Boolean,
        makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
        makeDataReadyRefcon: *mut c_void,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        numSampleTimingEntries: CMItemCount,
        sampleTimingArray: *const CMSampleTimingInfo,
        numSampleSizeEntries: CMItemCount,
        sampleSizeArray: *const size_t,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferCreateWithMakeDataReadyHandler(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: Boolean,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        numSampleTimingEntries: CMItemCount,
        sampleTimingArray: *const CMSampleTimingInfo,
        numSampleSizeEntries: CMItemCount,
        sampleSizeArray: *const size_t,
        sampleBufferOut: *mut CMSampleBufferRef,
        makeDataReadyHandler: CMSampleBufferMakeDataReadyHandler,
    ) -> OSStatus;
    pub fn CMSampleBufferCreateReady(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        numSampleTimingEntries: CMItemCount,
        sampleTimingArray: *const CMSampleTimingInfo,
        numSampleSizeEntries: CMItemCount,
        sampleSizeArray: *const size_t,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMAudioSampleBufferCreateWithPacketDescriptions(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: Boolean,
        makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
        makeDataReadyRefcon: *mut c_void,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        presentationTimeStamp: CMTime,
        packetDescriptions: *const AudioStreamPacketDescription,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMAudioSampleBufferCreateWithPacketDescriptionsAndMakeDataReadyHandler(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: Boolean,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        presentationTimeStamp: CMTime,
        packetDescriptions: *const AudioStreamPacketDescription,
        sampleBufferOut: *mut CMSampleBufferRef,
        makeDataReadyHandler: CMSampleBufferMakeDataReadyHandler,
    ) -> OSStatus;
    pub fn CMAudioSampleBufferCreateReadyWithPacketDescriptions(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        presentationTimeStamp: CMTime,
        packetDescriptions: *const AudioStreamPacketDescription,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferCreateForImageBuffer(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        dataReady: Boolean,
        makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
        makeDataReadyRefcon: *mut c_void,
        formatDescription: CMVideoFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        dataReady: Boolean,
        formatDescription: CMVideoFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
        makeDataReadyHandler: CMSampleBufferMakeDataReadyHandler,
    ) -> OSStatus;
    pub fn CMSampleBufferCreateReadyWithImageBuffer(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        formatDescription: CMVideoFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferCreateCopy(allocator: CFAllocatorRef, sbuf: CMSampleBufferRef, sampleBufferOut: *mut CMSampleBufferRef) -> OSStatus;
    pub fn CMSampleBufferCreateCopyWithNewTiming(
        allocator: CFAllocatorRef,
        originalSBuf: CMSampleBufferRef,
        numSampleTimingEntries: CMItemCount,
        sampleTimingArray: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferCopySampleBufferForRange(
        allocator: CFAllocatorRef,
        sbuf: CMSampleBufferRef,
        sampleRange: CFRange,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferGetTypeID() -> CFTypeID;
    pub fn CMSampleBufferSetDataBuffer(sbuf: CMSampleBufferRef, dataBuffer: CMBlockBufferRef) -> OSStatus;
    pub fn CMSampleBufferGetDataBuffer(sbuf: CMSampleBufferRef) -> CMBlockBufferRef;
    pub fn CMSampleBufferGetImageBuffer(sbuf: CMSampleBufferRef) -> CVImageBufferRef;
    pub fn CMSampleBufferSetDataBufferFromAudioBufferList(
        sbuf: CMSampleBufferRef,
        blockBufferStructureAllocator: CFAllocatorRef,
        blockBufferBlockAllocator: CFAllocatorRef,
        flags: u32,
        bufferList: *const AudioBufferList,
    ) -> OSStatus;
    pub fn CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer(
        sbuf: CMSampleBufferRef,
        bufferListSizeNeededOut: *mut size_t,
        bufferListOut: *mut AudioBufferList,
        bufferListSize: size_t,
        blockBufferStructureAllocator: CFAllocatorRef,
        blockBufferBlockAllocator: CFAllocatorRef,
        flags: u32,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferGetAudioStreamPacketDescriptions(
        sbuf: CMSampleBufferRef,
        packetDescriptionsSize: size_t,
        packetDescriptionsOut: *mut AudioStreamPacketDescription,
        packetDescriptionsSizeNeededOut: *mut size_t,
    ) -> OSStatus;
    pub fn CMSampleBufferGetAudioStreamPacketDescriptionsPtr(
        sbuf: CMSampleBufferRef,
        packetDescriptionsPointerOut: *mut *mut AudioStreamPacketDescription,
        packetDescriptionsSizeOut: *mut size_t,
    ) -> OSStatus;
    pub fn CMSampleBufferCopyPCMDataIntoAudioBufferList(
        sbuf: CMSampleBufferRef,
        frameOffset: i32,
        numFrames: i32,
        bufferList: *mut AudioBufferList,
    ) -> OSStatus;
    pub fn CMSampleBufferSetDataReady(sbuf: CMSampleBufferRef) -> OSStatus;
    pub fn CMSampleBufferDataIsReady(sbuf: CMSampleBufferRef) -> Boolean;
    pub fn CMSampleBufferSetDataFailed(sbuf: CMSampleBufferRef, status: OSStatus) -> OSStatus;
    pub fn CMSampleBufferHasDataFailed(sbuf: CMSampleBufferRef, statusOut: *mut OSStatus) -> Boolean;
    pub fn CMSampleBufferMakeDataReady(sbuf: CMSampleBufferRef) -> OSStatus;
    pub fn CMSampleBufferTrackDataReadiness(sbuf: CMSampleBufferRef, sampleBufferToTrack: CMSampleBufferRef) -> OSStatus;
    pub fn CMSampleBufferInvalidate(sbuf: CMSampleBufferRef) -> OSStatus;
}

pub type CMSampleBufferInvalidateCallback = extern "C" fn(CMSampleBufferRef, u64);
pub type CMSampleBufferInvalidateHandler = *const Block<(CMSampleBufferRef,), ()>;

extern "C" {
    pub fn CMSampleBufferSetInvalidateCallback(
        sbuf: CMSampleBufferRef,
        invalidateCallback: CMSampleBufferInvalidateCallback,
        invalidateRefCon: u64,
    ) -> OSStatus;
    pub fn CMSampleBufferSetInvalidateHandler(sbuf: CMSampleBufferRef, invalidateHandler: CMSampleBufferInvalidateHandler) -> OSStatus;
    pub fn CMSampleBufferIsValid(sbuf: CMSampleBufferRef) -> Boolean;

    pub static kCMSampleBufferNotification_DataBecameReady: CFStringRef;
    pub static kCMSampleBufferNotification_DataFailed: CFStringRef;
    pub static kCMSampleBufferNotificationParameter_OSStatus: CFStringRef;
    pub static kCMSampleBufferConduitNotification_InhibitOutputUntil: CFStringRef;
    pub static kCMSampleBufferConduitNotificationParameter_ResumeTag: CFStringRef;
    pub static kCMSampleBufferConduitNotification_ResetOutput: CFStringRef;
    pub static kCMSampleBufferConduitNotification_UpcomingOutputPTSRangeChanged: CFStringRef;
    pub static kCMSampleBufferConduitNotificationParameter_UpcomingOutputPTSRangeMayOverlapQueuedOutputPTSRange: CFStringRef;
    pub static kCMSampleBufferConduitNotificationParameter_MinUpcomingOutputPTS: CFStringRef;
    pub static kCMSampleBufferConduitNotificationParameter_MaxUpcomingOutputPTS: CFStringRef;
    pub static kCMSampleBufferConsumerNotification_BufferConsumed: CFStringRef;

    pub fn CMSampleBufferGetNumSamples(sbuf: CMSampleBufferRef) -> CMItemCount;
    pub fn CMSampleBufferGetDuration(sbuf: CMSampleBufferRef) -> CMTime;
    pub fn CMSampleBufferGetPresentationTimeStamp(sbuf: CMSampleBufferRef) -> CMTime;
    pub fn CMSampleBufferGetDecodeTimeStamp(sbuf: CMSampleBufferRef) -> CMTime;
    pub fn CMSampleBufferGetOutputDuration(sbuf: CMSampleBufferRef) -> CMTime;
    pub fn CMSampleBufferGetOutputPresentationTimeStamp(sbuf: CMSampleBufferRef) -> CMTime;
    pub fn CMSampleBufferSetOutputPresentationTimeStamp(sbuf: CMSampleBufferRef, outputPresentationTimeStamp: CMTime) -> OSStatus;
    pub fn CMSampleBufferGetOutputDecodeTimeStamp(sbuf: CMSampleBufferRef) -> CMTime;
    pub fn CMSampleBufferGetSampleTimingInfoArray(
        sbuf: CMSampleBufferRef,
        numSampleTimingEntries: CMItemCount,
        timingArrayOut: *mut CMSampleTimingInfo,
        timingArrayEntriesNeededOut: *mut CMItemCount,
    ) -> OSStatus;
    pub fn CMSampleBufferGetSampleTimingInfo(sbuf: CMSampleBufferRef, sampleIndex: CMItemCount, timingInfoOut: *mut CMSampleTimingInfo) -> OSStatus;
    pub fn CMSampleBufferGetSampleSizeArray(
        sbuf: CMSampleBufferRef,
        sizeArrayEntries: CMItemCount,
        sizeArrayOut: *mut size_t,
        sizeArrayEntriesNeededOut: *mut CMItemCount,
    ) -> OSStatus;
    pub fn CMSampleBufferGetSampleSize(sbuf: CMSampleBufferRef, sampleIndex: CMItemCount) -> size_t;
    pub fn CMSampleBufferGetTotalSampleSize(sbuf: CMSampleBufferRef) -> size_t;
    pub fn CMSampleBufferGetFormatDescription(sbuf: CMSampleBufferRef) -> CMFormatDescriptionRef;
    pub fn CMSampleBufferGetSampleAttachmentsArray(sbuf: CMSampleBufferRef, createIfNecessary: Boolean) -> CFArrayRef;

    pub static kCMSampleAttachmentKey_NotSync: CFStringRef;
    pub static kCMSampleAttachmentKey_PartialSync: CFStringRef;
    pub static kCMSampleAttachmentKey_HasRedundantCoding: CFStringRef;
    pub static kCMSampleAttachmentKey_IsDependedOnByOthers: CFStringRef;
    pub static kCMSampleAttachmentKey_DependsOnOthers: CFStringRef;
    pub static kCMSampleAttachmentKey_EarlierDisplayTimesAllowed: CFStringRef;
    pub static kCMSampleAttachmentKey_DisplayImmediately: CFStringRef;
    pub static kCMSampleAttachmentKey_DoNotDisplay: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_ResetDecoderBeforeDecoding: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_DrainAfterDecoding: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_PostNotificationWhenConsumed: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_ResumeOutput: CFStringRef;
    pub static kCMSampleAttachmentKey_HEVCTemporalLevelInfo: CFStringRef;
    pub static kCMHEVCTemporalLevelInfoKey_TemporalLevel: CFStringRef;
    pub static kCMHEVCTemporalLevelInfoKey_ProfileSpace: CFStringRef;
    pub static kCMHEVCTemporalLevelInfoKey_TierFlag: CFStringRef;
    pub static kCMHEVCTemporalLevelInfoKey_ProfileIndex: CFStringRef;
    pub static kCMHEVCTemporalLevelInfoKey_ProfileCompatibilityFlags: CFStringRef;
    pub static kCMHEVCTemporalLevelInfoKey_ConstraintIndicatorFlags: CFStringRef;
    pub static kCMHEVCTemporalLevelInfoKey_LevelIndex: CFStringRef;
    pub static kCMSampleAttachmentKey_HEVCTemporalSubLayerAccess: CFStringRef;
    pub static kCMSampleAttachmentKey_HEVCStepwiseTemporalSubLayerAccess: CFStringRef;
    pub static kCMSampleAttachmentKey_HEVCSyncSampleNALUnitType: CFStringRef;
    pub static kCMSampleAttachmentKey_AudioIndependentSampleDecoderRefreshCount: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_TransitionID: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_TrimDurationAtStart: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_TrimDurationAtEnd: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_SpeedMultiplier: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_Reverse: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_FillDiscontinuitiesWithSilence: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_EmptyMedia: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_PermanentEmptyMedia: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_DisplayEmptyMediaImmediately: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_EndsPreviousSampleDuration: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_SampleReferenceURL: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_SampleReferenceByteOffset: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_GradualDecoderRefresh: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_DroppedFrameReason: CFStringRef;
    pub static kCMSampleBufferDroppedFrameReason_FrameWasLate: CFStringRef;
    pub static kCMSampleBufferDroppedFrameReason_OutOfBuffers: CFStringRef;
    pub static kCMSampleBufferDroppedFrameReason_Discontinuity: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_DroppedFrameReasonInfo: CFStringRef;
    pub static kCMSampleBufferDroppedFrameReasonInfo_CameraModeSwitch: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_StillImageLensStabilizationInfo: CFStringRef;
    pub static kCMSampleBufferLensStabilizationInfo_Active: CFStringRef;
    pub static kCMSampleBufferLensStabilizationInfo_OutOfRange: CFStringRef;
    pub static kCMSampleBufferLensStabilizationInfo_Unavailable: CFStringRef;
    pub static kCMSampleBufferLensStabilizationInfo_Off: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_CameraIntrinsicMatrix: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_ForceKeyFrame: CFStringRef;
    pub static kCMSampleAttachmentKey_CryptorSubsampleAuxiliaryData: CFStringRef;
    pub static kCMSampleAttachmentKey_HDR10PlusPerFrameData: CFStringRef;

    pub fn CMSampleBufferCallForEachSample(
        sbuf: CMSampleBufferRef,
        callback: extern "C" fn(CMSampleBufferRef, CMItemCount, *mut c_void) -> OSStatus,
        refcon: *mut c_void,
    ) -> OSStatus;
    pub fn CMSampleBufferCallBlockForEachSample(sbuf: CMSampleBufferRef, block: *const Block<(CMSampleBufferRef, CMItemCount), OSStatus>)
        -> OSStatus;
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for opaqueCMSampleBuffer {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("opaqueCMSampleBuffer", &[]));
}

declare_TCFType!(CMSampleBuffer, CMSampleBufferRef);
impl_TCFType!(CMSampleBuffer, CMSampleBufferRef, CMSampleBufferGetTypeID);
impl_CFTypeDescription!(CMSampleBuffer);

impl CMSampleBuffer {
    #[inline]
    pub unsafe fn new(
        data_buffer: Option<&CMBlockBuffer>,
        data_ready: bool,
        make_data_ready_callback: CMSampleBufferMakeDataReadyCallback,
        make_data_ready_refcon: *mut c_void,
        format_description: Option<&CMFormatDescription>,
        num_samples: CMItemCount,
        sample_timing_array: Option<&[CMSampleTimingInfo]>,
        sample_size_array: Option<&[size_t]>,
    ) -> Result<CMSampleBuffer, OSStatus> {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let status = unsafe {
            CMSampleBufferCreate(
                kCFAllocatorDefault,
                data_buffer.map_or(null_mut(), |b| b.as_concrete_TypeRef()),
                data_ready as Boolean,
                make_data_ready_callback,
                make_data_ready_refcon,
                format_description.map_or(null_mut(), |f| f.as_concrete_TypeRef()),
                num_samples,
                sample_timing_array.map_or(0, |a| a.len() as CMItemCount),
                sample_timing_array.map_or(null(), |a| a.as_ptr()),
                sample_size_array.map_or(0, |a| a.len() as CMItemCount),
                sample_size_array.map_or(null(), |a| a.as_ptr()),
                &mut sample_buffer,
            )
        };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn new_with_make_data_ready_closure<F>(
        data_buffer: Option<&CMBlockBuffer>,
        data_ready: bool,
        format_description: Option<&CMFormatDescription>,
        num_samples: CMItemCount,
        sample_timing_array: Option<&[CMSampleTimingInfo]>,
        sample_size_array: Option<&[size_t]>,
        make_data_ready_closure: Option<F>,
    ) -> Result<CMSampleBuffer, OSStatus>
    where
        F: Fn(CMSampleBuffer) -> OSStatus + 'static,
    {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let handler = make_data_ready_closure.map(|closure| {
            ConcreteBlock::new(move |sbuf: CMSampleBufferRef| -> OSStatus {
                let sbuf = unsafe { CMSampleBuffer::wrap_under_get_rule(sbuf) };
                closure(sbuf)
            })
            .copy()
        });
        let status = unsafe {
            CMSampleBufferCreateWithMakeDataReadyHandler(
                kCFAllocatorDefault,
                data_buffer.map_or(null_mut(), |b| b.as_concrete_TypeRef()),
                data_ready as Boolean,
                format_description.map_or(null_mut(), |f| f.as_concrete_TypeRef()),
                num_samples,
                sample_timing_array.map_or(0, |a| a.len() as CMItemCount),
                sample_timing_array.map_or(null(), |a| a.as_ptr()),
                sample_size_array.map_or(0, |a| a.len() as CMItemCount),
                sample_size_array.map_or(null(), |a| a.as_ptr()),
                &mut sample_buffer,
                handler.as_ref().map_or(null(), |h| &**h),
            )
        };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn new_ready(
        data_buffer: &CMBlockBuffer,
        format_description: Option<&CMFormatDescription>,
        num_samples: CMItemCount,
        sample_timing_array: Option<&[CMSampleTimingInfo]>,
        sample_size_array: Option<&[size_t]>,
    ) -> Result<CMSampleBuffer, OSStatus> {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let status = unsafe {
            CMSampleBufferCreateReady(
                kCFAllocatorDefault,
                data_buffer.as_concrete_TypeRef(),
                format_description.map_or(null_mut(), |f| f.as_concrete_TypeRef()),
                num_samples,
                sample_timing_array.map_or(0, |a| a.len() as CMItemCount),
                sample_timing_array.map_or(null(), |a| a.as_ptr()),
                sample_size_array.map_or(0, |a| a.len() as CMItemCount),
                sample_size_array.map_or(null(), |a| a.as_ptr()),
                &mut sample_buffer,
            )
        };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    pub unsafe fn new_audio_sample_buffer_with_packet_descriptions(
        data_buffer: Option<&CMBlockBuffer>,
        data_ready: bool,
        make_data_ready_callback: CMSampleBufferMakeDataReadyCallback,
        make_data_ready_refcon: *mut c_void,
        format_description: &CMFormatDescription,
        num_samples: CMItemCount,
        presentation_time_stamp: CMTime,
        packet_descriptions: Option<&[AudioStreamPacketDescription]>,
    ) -> Result<CMSampleBuffer, OSStatus> {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let status = unsafe {
            CMAudioSampleBufferCreateWithPacketDescriptions(
                kCFAllocatorDefault,
                data_buffer.map_or(null_mut(), |b| b.as_concrete_TypeRef()),
                data_ready as Boolean,
                make_data_ready_callback,
                make_data_ready_refcon,
                format_description.as_concrete_TypeRef(),
                num_samples,
                presentation_time_stamp,
                packet_descriptions.map_or(null(), |a| a.as_ptr()),
                &mut sample_buffer,
            )
        };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    pub fn new_audio_sample_buffer_with_packet_descriptions_and_make_data_ready_closure<F>(
        data_buffer: Option<&CMBlockBuffer>,
        data_ready: bool,
        format_description: &CMFormatDescription,
        num_samples: CMItemCount,
        presentation_time_stamp: CMTime,
        packet_descriptions: Option<&[AudioStreamPacketDescription]>,
        make_data_ready_closure: Option<F>,
    ) -> Result<CMSampleBuffer, OSStatus>
    where
        F: Fn(CMSampleBuffer) -> OSStatus + 'static,
    {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let handler = make_data_ready_closure.map(|closure| {
            ConcreteBlock::new(move |sbuf: CMSampleBufferRef| -> OSStatus {
                let sbuf = unsafe { CMSampleBuffer::wrap_under_get_rule(sbuf) };
                closure(sbuf)
            })
            .copy()
        });
        let status = unsafe {
            CMAudioSampleBufferCreateWithPacketDescriptionsAndMakeDataReadyHandler(
                kCFAllocatorDefault,
                data_buffer.map_or(null_mut(), |b| b.as_concrete_TypeRef()),
                data_ready as Boolean,
                format_description.as_concrete_TypeRef(),
                num_samples,
                presentation_time_stamp,
                packet_descriptions.map_or(null(), |a| a.as_ptr()),
                &mut sample_buffer,
                handler.as_ref().map_or(null(), |h| &**h),
            )
        };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn new_audio_sample_buffer_ready_with_packet_descriptions(
        data_buffer: &CMBlockBuffer,
        format_description: &CMFormatDescription,
        num_samples: CMItemCount,
        presentation_time_stamp: CMTime,
        packet_descriptions: Option<&[AudioStreamPacketDescription]>,
    ) -> Result<CMSampleBuffer, OSStatus> {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let status = unsafe {
            CMAudioSampleBufferCreateReadyWithPacketDescriptions(
                kCFAllocatorDefault,
                data_buffer.as_concrete_TypeRef(),
                format_description.as_concrete_TypeRef(),
                num_samples,
                presentation_time_stamp,
                packet_descriptions.map_or(null(), |a| a.as_ptr()),
                &mut sample_buffer,
            )
        };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    pub unsafe fn from_image_buffer(
        image_buffer: &CVImageBuffer,
        data_ready: bool,
        make_data_ready_callback: CMSampleBufferMakeDataReadyCallback,
        make_data_ready_refcon: *mut c_void,
        format_description: &CMVideoFormatDescription,
        sample_timing: &CMSampleTimingInfo,
    ) -> Result<CMSampleBuffer, OSStatus> {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let status = CMSampleBufferCreateForImageBuffer(
            kCFAllocatorDefault,
            image_buffer.as_concrete_TypeRef(),
            data_ready as Boolean,
            make_data_ready_callback,
            make_data_ready_refcon,
            format_description.as_concrete_TypeRef(),
            sample_timing,
            &mut sample_buffer,
        );
        if status == 0 {
            Ok(CMSampleBuffer::wrap_under_create_rule(sample_buffer))
        } else {
            Err(status)
        }
    }

    pub fn from_image_buffer_with_make_data_ready_closure<F>(
        image_buffer: &CVImageBuffer,
        data_ready: bool,
        format_description: &CMVideoFormatDescription,
        sample_timing: &CMSampleTimingInfo,
        make_data_ready_closure: Option<F>,
    ) -> Result<CMSampleBuffer, OSStatus>
    where
        F: Fn(CMSampleBuffer) -> OSStatus + 'static,
    {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let handler = make_data_ready_closure.map(|closure| {
            ConcreteBlock::new(move |sbuf: CMSampleBufferRef| -> OSStatus {
                let sbuf = unsafe { CMSampleBuffer::wrap_under_get_rule(sbuf) };
                closure(sbuf)
            })
            .copy()
        });
        let status = unsafe {
            CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler(
                kCFAllocatorDefault,
                image_buffer.as_concrete_TypeRef(),
                data_ready as Boolean,
                format_description.as_concrete_TypeRef(),
                sample_timing,
                &mut sample_buffer,
                handler.as_ref().map_or(null(), |h| &**h),
            )
        };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn from_image_buffer_ready(
        image_buffer: &CVImageBuffer,
        format_description: &CMVideoFormatDescription,
        sample_timing: &CMSampleTimingInfo,
    ) -> Result<CMSampleBuffer, OSStatus> {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let status = unsafe {
            CMSampleBufferCreateReadyWithImageBuffer(
                kCFAllocatorDefault,
                image_buffer.as_concrete_TypeRef(),
                format_description.as_concrete_TypeRef(),
                sample_timing,
                &mut sample_buffer,
            )
        };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn copy(&self) -> Result<CMSampleBuffer, OSStatus> {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let status = unsafe { CMSampleBufferCreateCopy(kCFAllocatorDefault, self.as_concrete_TypeRef(), &mut sample_buffer) };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn copy_with_new_timing(&self, sample_timing_array: Option<&[CMSampleTimingInfo]>) -> Result<CMSampleBuffer, OSStatus> {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let status = unsafe {
            CMSampleBufferCreateCopyWithNewTiming(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                sample_timing_array.map_or(0, |a| a.len() as CMItemCount),
                sample_timing_array.map_or(null(), |a| a.as_ptr()),
                &mut sample_buffer,
            )
        };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn copy_for_range(&self, sample_range: CFRange) -> Result<CMSampleBuffer, OSStatus> {
        let mut sample_buffer: CMSampleBufferRef = null_mut();
        let status =
            unsafe { CMSampleBufferCopySampleBufferForRange(kCFAllocatorDefault, self.as_concrete_TypeRef(), sample_range, &mut sample_buffer) };
        if status == 0 {
            Ok(unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn set_data_buffer(&self, data_buffer: &CMBlockBuffer) -> Result<(), OSStatus> {
        let status = unsafe { CMSampleBufferSetDataBuffer(self.as_concrete_TypeRef(), data_buffer.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_data_buffer(&self) -> Option<CMBlockBuffer> {
        unsafe {
            let data_buffer = CMSampleBufferGetDataBuffer(self.as_concrete_TypeRef());
            if data_buffer.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(data_buffer))
            }
        }
    }

    #[inline]
    pub fn get_image_buffer(&self) -> Option<CVImageBuffer> {
        unsafe {
            let image_buffer = CMSampleBufferGetImageBuffer(self.as_concrete_TypeRef());
            if image_buffer.is_null() {
                None
            } else {
                Some(CVImageBuffer::wrap_under_get_rule(image_buffer))
            }
        }
    }

    #[inline]
    pub fn set_data_buffer_from_audio_buffer_list(&self, flags: u32, buffer_list: &AudioBufferList) -> Result<(), OSStatus> {
        let status = unsafe {
            CMSampleBufferSetDataBufferFromAudioBufferList(self.as_concrete_TypeRef(), kCFAllocatorDefault, kCFAllocatorDefault, flags, buffer_list)
        };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn set_data_ready(&self) -> Result<(), OSStatus> {
        let status = unsafe { CMSampleBufferSetDataReady(self.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn is_data_ready(&self) -> bool {
        unsafe { CMSampleBufferDataIsReady(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn set_data_failed(&self, status: OSStatus) -> Result<(), OSStatus> {
        let status = unsafe { CMSampleBufferSetDataFailed(self.as_concrete_TypeRef(), status) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn has_data_failed(&self) -> (OSStatus, bool) {
        let mut status = 0;
        let has_failed = unsafe { CMSampleBufferHasDataFailed(self.as_concrete_TypeRef(), &mut status) != 0 };
        (status, has_failed)
    }

    #[inline]
    pub fn make_data_ready(&self) -> Result<(), OSStatus> {
        let status = unsafe { CMSampleBufferMakeDataReady(self.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn track_data_readiness(&self, sample_buffer_to_track: &CMSampleBuffer) -> Result<(), OSStatus> {
        let status = unsafe { CMSampleBufferTrackDataReadiness(self.as_concrete_TypeRef(), sample_buffer_to_track.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn invalidate(&self) -> Result<(), OSStatus> {
        let status = unsafe { CMSampleBufferInvalidate(self.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub unsafe fn set_invalidate_callback(
        &self,
        invalidate_callback: CMSampleBufferInvalidateCallback,
        invalidate_ref_con: u64,
    ) -> Result<(), OSStatus> {
        let status = unsafe { CMSampleBufferSetInvalidateCallback(self.as_concrete_TypeRef(), invalidate_callback, invalidate_ref_con) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn set_invalidate_closure<F>(&self, invalidate_closure: Option<F>) -> Result<(), OSStatus>
    where
        F: Fn(CMSampleBuffer) + 'static,
    {
        let status = unsafe {
            CMSampleBufferSetInvalidateHandler(
                self.as_concrete_TypeRef(),
                invalidate_closure
                    .map(|closure| {
                        ConcreteBlock::new(move |sbuf: CMSampleBufferRef| {
                            let sbuf = CMSampleBuffer::wrap_under_get_rule(sbuf);
                            closure(sbuf);
                        })
                        .copy()
                    })
                    .as_ref()
                    .map_or(null(), |h| &**h),
            )
        };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CMSampleBufferIsValid(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn get_num_samples(&self) -> CMItemCount {
        unsafe { CMSampleBufferGetNumSamples(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_duration(&self) -> CMTime {
        unsafe { CMSampleBufferGetDuration(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_presentation_time_stamp(&self) -> CMTime {
        unsafe { CMSampleBufferGetPresentationTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_decode_time_stamp(&self) -> CMTime {
        unsafe { CMSampleBufferGetDecodeTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_output_duration(&self) -> CMTime {
        unsafe { CMSampleBufferGetOutputDuration(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_output_presentation_time_stamp(&self) -> CMTime {
        unsafe { CMSampleBufferGetOutputPresentationTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn set_output_presentation_time_stamp(&self, output_presentation_time_stamp: CMTime) -> Result<(), OSStatus> {
        let status = unsafe { CMSampleBufferSetOutputPresentationTimeStamp(self.as_concrete_TypeRef(), output_presentation_time_stamp) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_output_decode_time_stamp(&self) -> CMTime {
        unsafe { CMSampleBufferGetOutputDecodeTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_sample_timing_info_array(&self) -> Option<Vec<CMSampleTimingInfo>> {
        unsafe {
            let mut timing_array_entries_needed = 0;
            let status = CMSampleBufferGetSampleTimingInfoArray(self.as_concrete_TypeRef(), 0, null_mut(), &mut timing_array_entries_needed);
            if status != 0 {
                return None;
            }
            let mut timing_array = vec![CMSampleTimingInfo::default(); timing_array_entries_needed as usize];
            let status = CMSampleBufferGetSampleTimingInfoArray(
                self.as_concrete_TypeRef(),
                timing_array_entries_needed,
                timing_array.as_mut_ptr(),
                &mut timing_array_entries_needed,
            );
            if status != 0 {
                return None;
            }
            Some(timing_array)
        }
    }

    #[inline]
    pub fn get_sample_timing_info(&self, sample_index: CMItemCount) -> Option<CMSampleTimingInfo> {
        unsafe {
            let mut timing_info = CMSampleTimingInfo::default();
            let status = CMSampleBufferGetSampleTimingInfo(self.as_concrete_TypeRef(), sample_index, &mut timing_info);
            if status == 0 {
                Some(timing_info)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn get_sample_size_array(&self) -> Option<Vec<size_t>> {
        unsafe {
            let mut size_array_entries_needed = 0;
            let status = CMSampleBufferGetSampleSizeArray(self.as_concrete_TypeRef(), 0, null_mut(), &mut size_array_entries_needed);
            if status != 0 {
                return None;
            }
            let mut size_array = vec![0; size_array_entries_needed as usize];
            let status = CMSampleBufferGetSampleSizeArray(
                self.as_concrete_TypeRef(),
                size_array_entries_needed,
                size_array.as_mut_ptr(),
                &mut size_array_entries_needed,
            );
            if status != 0 {
                return None;
            }
            Some(size_array)
        }
    }

    #[inline]
    pub fn get_sample_size(&self, sample_index: CMItemCount) -> size_t {
        unsafe { CMSampleBufferGetSampleSize(self.as_concrete_TypeRef(), sample_index) }
    }

    #[inline]
    pub fn get_total_sample_size(&self) -> usize {
        unsafe { CMSampleBufferGetTotalSampleSize(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_format_description(&self) -> Option<CMFormatDescription> {
        unsafe {
            let format_description = CMSampleBufferGetFormatDescription(self.as_concrete_TypeRef());
            if format_description.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(format_description))
            }
        }
    }

    #[inline]
    pub fn get_sample_attachments_array(&self, create_if_necessary: bool) -> Option<CFArray<CFDictionary<CFString, CFType>>> {
        unsafe {
            let attachments = CMSampleBufferGetSampleAttachmentsArray(self.as_concrete_TypeRef(), create_if_necessary as Boolean);
            if attachments.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(attachments))
            }
        }
    }

    pub unsafe fn call_for_each_sample(
        &self,
        callback: extern "C" fn(CMSampleBufferRef, CMItemCount, *mut c_void) -> OSStatus,
        refcon: *mut c_void,
    ) -> Result<(), OSStatus> {
        let status = unsafe { CMSampleBufferCallForEachSample(self.as_concrete_TypeRef(), callback, refcon) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn call_closure_for_each_sample<F>(&self, closure: Option<F>) -> Result<(), OSStatus>
    where
        F: Fn(CMSampleBuffer, CMItemCount) -> OSStatus + 'static,
    {
        let status = unsafe {
            CMSampleBufferCallBlockForEachSample(
                self.as_concrete_TypeRef(),
                closure
                    .map(|closure| {
                        ConcreteBlock::new(move |sbuf: CMSampleBufferRef, sample_index: CMItemCount| -> OSStatus {
                            let sbuf = CMSampleBuffer::wrap_under_get_rule(sbuf);
                            closure(sbuf, sample_index)
                        })
                        .copy()
                    })
                    .as_ref()
                    .map_or(null(), |h| &**h),
            )
        };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }
}
