use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
};
use libc::c_int;

pub const kVTUnlimitedFrameDelayCount: c_int = -1;

extern "C" {
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
    pub static kVTProfileLevel_HEVC_Main42210_AutoLevel: CFStringRef;
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
    pub static kVTProfileLevel_H264_ConstrainedHigh_AutoLevel: CFStringRef;
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
    pub static kVTCompressionPropertyKey_OutputBitDepth: CFStringRef;
    pub static kVTCompressionPropertyKey_HDRMetadataInsertionMode: CFStringRef;
    pub static kVTHDRMetadataInsertionMode_None: CFStringRef;
    pub static kVTHDRMetadataInsertionMode_Auto: CFStringRef;
    pub static kVTHDRMetadataInsertionMode_RequestSDRRangePreservation: CFStringRef;
    pub static kVTCompressionPropertyKey_H264EntropyMode: CFStringRef;
    pub static kVTH264EntropyMode_CAVLC: CFStringRef;
    pub static kVTH264EntropyMode_CABAC: CFStringRef;
    pub static kVTCompressionPropertyKey_Depth: CFStringRef;
    pub static kVTCompressionPropertyKey_PreserveAlphaChannel: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxFrameDelayCount: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxH264SliceBytes: CFStringRef;
    pub static kVTCompressionPropertyKey_RealTime: CFStringRef;
    pub static kVTCompressionPropertyKey_MaximizePowerEfficiency: CFStringRef;
    pub static kVTCompressionPropertyKey_SourceFrameCount: CFStringRef;
    pub static kVTCompressionPropertyKey_ExpectedFrameRate: CFStringRef;
    pub static kVTCompressionPropertyKey_MaximumRealTimeFrameRate: CFStringRef;
    pub static kVTCompressionPropertyKey_BaseLayerFrameRateFraction: CFStringRef;
    pub static kVTCompressionPropertyKey_BaseLayerBitRateFraction: CFStringRef;
    pub static kVTCompressionPropertyKey_ExpectedDuration: CFStringRef;
    pub static kVTCompressionPropertyKey_BaseLayerFrameRate: CFStringRef;
    pub static kVTCompressionPropertyKey_ReferenceBufferCount: CFStringRef;
    pub static kVTCompressionPropertyKey_CalculateMeanSquaredError: CFStringRef;
    pub static kVTSampleAttachmentKey_QualityMetrics: CFStringRef;
    pub static kVTSampleAttachmentQualityMetricsKey_LumaMeanSquaredError: CFStringRef;
    pub static kVTSampleAttachmentQualityMetricsKey_ChromaBlueMeanSquaredError: CFStringRef;
    pub static kVTSampleAttachmentQualityMetricsKey_ChromaRedMeanSquaredError: CFStringRef;
    pub static kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder: CFStringRef;
    pub static kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder: CFStringRef;
    pub static kVTCompressionPropertyKey_UsingHardwareAcceleratedVideoEncoder: CFStringRef;
    pub static kVTVideoEncoderSpecification_RequiredEncoderGPURegistryID: CFStringRef;
    pub static kVTVideoEncoderSpecification_PreferredEncoderGPURegistryID: CFStringRef;
    pub static kVTCompressionPropertyKey_UsingGPURegistryID: CFStringRef;
    pub static kVTCompressionPropertyKey_SupportsBaseFrameQP: CFStringRef;
    pub static kVTEncodeFrameOptionKey_ForceKeyFrame: CFStringRef;
    pub static kVTEncodeFrameOptionKey_BaseFrameQP: CFStringRef;
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
    pub static kVTCompressionPropertyKey_GammaLevel: CFStringRef;
    pub static kVTCompressionPropertyKey_PixelTransferProperties: CFStringRef;
    pub static kVTCompressionPropertyKey_MultiPassStorage: CFStringRef;
    pub static kVTCompressionPropertyKey_EncoderID: CFStringRef;
    pub static kVTCompressionPropertyKey_PreserveDynamicHDRMetadata: CFStringRef;
    pub static kVTVideoEncoderSpecification_EnableLowLatencyRateControl: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxAllowedFrameQP: CFStringRef;
    pub static kVTCompressionPropertyKey_MinAllowedFrameQP: CFStringRef;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompressionPropertyKey {
    NumberOfPendingFrames,
    PixelBufferPoolIsShared,
    VideoEncoderPixelBufferAttributes,
    MaxKeyFrameInterval,
    MaxKeyFrameIntervalDuration,
    AllowTemporalCompression,
    AllowFrameReordering,
    AverageBitRate,
    DataRateLimits,
    Quality,
    MoreFramesBeforeStart,
    MoreFramesAfterEnd,
    ProfileLevel,
    OutputBitDepth,
    HDRMetadataInsertionMode,
    H264EntropyMode,
    Depth,
    PreserveAlphaChannel,
    MaxFrameDelayCount,
    MaxH264SliceBytes,
    RealTime,
    MaximizePowerEfficiency,
    SourceFrameCount,
    ExpectedFrameRate,
    MaximumRealTimeFrameRate,
    BaseLayerFrameRateFraction,
    BaseLayerBitRateFraction,
    ExpectedDuration,
    BaseLayerFrameRate,
    ReferenceBufferCount,
    CalculateMeanSquaredError,
    UsingHardwareAcceleratedVideoEncoder,
    UsingGPURegistryID,
    SupportsBaseFrameQP,
    CleanAperture,
    PixelAspectRatio,
    FieldCount,
    FieldDetail,
    AspectRatio16x9,
    ProgressiveScan,
    ColorPrimaries,
    TransferFunction,
    YCbCrMatrix,
    ICCProfile,
    MasteringDisplayColorVolume,
    ContentLightLevelInfo,
    GammaLevel,
    PixelTransferProperties,
    MultiPassStorage,
    EncoderID,
    PreserveDynamicHDRMetadata,
    MaxAllowedFrameQP,
    MinAllowedFrameQP,
}

impl From<CompressionPropertyKey> for CFStringRef {
    fn from(key: CompressionPropertyKey) -> Self {
        unsafe {
            match key {
                CompressionPropertyKey::NumberOfPendingFrames => kVTCompressionPropertyKey_NumberOfPendingFrames,
                CompressionPropertyKey::PixelBufferPoolIsShared => kVTCompressionPropertyKey_PixelBufferPoolIsShared,
                CompressionPropertyKey::VideoEncoderPixelBufferAttributes => kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes,
                CompressionPropertyKey::MaxKeyFrameInterval => kVTCompressionPropertyKey_MaxKeyFrameInterval,
                CompressionPropertyKey::MaxKeyFrameIntervalDuration => kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration,
                CompressionPropertyKey::AllowTemporalCompression => kVTCompressionPropertyKey_AllowTemporalCompression,
                CompressionPropertyKey::AllowFrameReordering => kVTCompressionPropertyKey_AllowFrameReordering,
                CompressionPropertyKey::AverageBitRate => kVTCompressionPropertyKey_AverageBitRate,
                CompressionPropertyKey::DataRateLimits => kVTCompressionPropertyKey_DataRateLimits,
                CompressionPropertyKey::Quality => kVTCompressionPropertyKey_Quality,
                CompressionPropertyKey::MoreFramesBeforeStart => kVTCompressionPropertyKey_MoreFramesBeforeStart,
                CompressionPropertyKey::MoreFramesAfterEnd => kVTCompressionPropertyKey_MoreFramesAfterEnd,
                CompressionPropertyKey::ProfileLevel => kVTCompressionPropertyKey_ProfileLevel,
                CompressionPropertyKey::OutputBitDepth => kVTCompressionPropertyKey_OutputBitDepth,
                CompressionPropertyKey::HDRMetadataInsertionMode => kVTCompressionPropertyKey_HDRMetadataInsertionMode,
                CompressionPropertyKey::H264EntropyMode => kVTCompressionPropertyKey_H264EntropyMode,
                CompressionPropertyKey::Depth => kVTCompressionPropertyKey_Depth,
                CompressionPropertyKey::PreserveAlphaChannel => kVTCompressionPropertyKey_PreserveAlphaChannel,
                CompressionPropertyKey::MaxFrameDelayCount => kVTCompressionPropertyKey_MaxFrameDelayCount,
                CompressionPropertyKey::MaxH264SliceBytes => kVTCompressionPropertyKey_MaxH264SliceBytes,
                CompressionPropertyKey::RealTime => kVTCompressionPropertyKey_RealTime,
                CompressionPropertyKey::MaximizePowerEfficiency => kVTCompressionPropertyKey_MaximizePowerEfficiency,
                CompressionPropertyKey::SourceFrameCount => kVTCompressionPropertyKey_SourceFrameCount,
                CompressionPropertyKey::ExpectedFrameRate => kVTCompressionPropertyKey_ExpectedFrameRate,
                CompressionPropertyKey::MaximumRealTimeFrameRate => kVTCompressionPropertyKey_MaximumRealTimeFrameRate,
                CompressionPropertyKey::BaseLayerFrameRateFraction => kVTCompressionPropertyKey_BaseLayerFrameRateFraction,
                CompressionPropertyKey::BaseLayerBitRateFraction => kVTCompressionPropertyKey_BaseLayerBitRateFraction,
                CompressionPropertyKey::ExpectedDuration => kVTCompressionPropertyKey_ExpectedDuration,
                CompressionPropertyKey::BaseLayerFrameRate => kVTCompressionPropertyKey_BaseLayerFrameRate,
                CompressionPropertyKey::ReferenceBufferCount => kVTCompressionPropertyKey_ReferenceBufferCount,
                CompressionPropertyKey::CalculateMeanSquaredError => kVTCompressionPropertyKey_CalculateMeanSquaredError,
                CompressionPropertyKey::UsingHardwareAcceleratedVideoEncoder => kVTCompressionPropertyKey_UsingHardwareAcceleratedVideoEncoder,
                CompressionPropertyKey::UsingGPURegistryID => kVTCompressionPropertyKey_UsingGPURegistryID,
                CompressionPropertyKey::SupportsBaseFrameQP => kVTCompressionPropertyKey_SupportsBaseFrameQP,
                CompressionPropertyKey::CleanAperture => kVTCompressionPropertyKey_CleanAperture,
                CompressionPropertyKey::PixelAspectRatio => kVTCompressionPropertyKey_PixelAspectRatio,
                CompressionPropertyKey::FieldCount => kVTCompressionPropertyKey_FieldCount,
                CompressionPropertyKey::FieldDetail => kVTCompressionPropertyKey_FieldDetail,
                CompressionPropertyKey::AspectRatio16x9 => kVTCompressionPropertyKey_AspectRatio16x9,
                CompressionPropertyKey::ProgressiveScan => kVTCompressionPropertyKey_ProgressiveScan,
                CompressionPropertyKey::ColorPrimaries => kVTCompressionPropertyKey_ColorPrimaries,
                CompressionPropertyKey::TransferFunction => kVTCompressionPropertyKey_TransferFunction,
                CompressionPropertyKey::YCbCrMatrix => kVTCompressionPropertyKey_YCbCrMatrix,
                CompressionPropertyKey::ICCProfile => kVTCompressionPropertyKey_ICCProfile,
                CompressionPropertyKey::MasteringDisplayColorVolume => kVTCompressionPropertyKey_MasteringDisplayColorVolume,
                CompressionPropertyKey::ContentLightLevelInfo => kVTCompressionPropertyKey_ContentLightLevelInfo,
                CompressionPropertyKey::GammaLevel => kVTCompressionPropertyKey_GammaLevel,
                CompressionPropertyKey::PixelTransferProperties => kVTCompressionPropertyKey_PixelTransferProperties,
                CompressionPropertyKey::MultiPassStorage => kVTCompressionPropertyKey_MultiPassStorage,
                CompressionPropertyKey::EncoderID => kVTCompressionPropertyKey_EncoderID,
                CompressionPropertyKey::PreserveDynamicHDRMetadata => kVTCompressionPropertyKey_PreserveDynamicHDRMetadata,
                CompressionPropertyKey::MaxAllowedFrameQP => kVTCompressionPropertyKey_MaxAllowedFrameQP,
                CompressionPropertyKey::MinAllowedFrameQP => kVTCompressionPropertyKey_MinAllowedFrameQP,
            }
        }
    }
}

impl From<CompressionPropertyKey> for CFString {
    fn from(key: CompressionPropertyKey) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileLevel {
    HEVCMainAutoLevel,
    HEVCMain10AutoLevel,
    HEVCMain42210AutoLevel,
    H264Baseline13,
    H264Baseline30,
    H264Baseline31,
    H264Baseline32,
    H264Baseline40,
    H264Baseline41,
    H264Baseline42,
    H264Baseline50,
    H264Baseline51,
    H264Baseline52,
    H264BaselineAutoLevel,
    H264Main30,
    H264Main31,
    H264Main32,
    H264Main40,
    H264Main41,
    H264Main42,
    H264Main50,
    H264Main51,
    H264Main52,
    H264MainAutoLevel,
    H264Extended50,
    H264ExtendedAutoLevel,
    H264High30,
    H264High31,
    H264High32,
    H264High40,
    H264High41,
    H264High42,
    H264High50,
    H264High51,
    H264High52,
    H264HighAutoLevel,
    H264ConstrainedHighAutoLevel,
    MP4VSimpleL0,
    MP4VSimpleL1,
    MP4VSimpleL2,
    MP4VSimpleL3,
    MP4VMainL2,
    MP4VMainL3,
    MP4VMainL4,
    MP4VAdvancedSimpleL0,
    MP4VAdvancedSimpleL1,
    MP4VAdvancedSimpleL2,
    MP4VAdvancedSimpleL3,
    MP4VAdvancedSimpleL4,
    H263Profile0Level10,
    H263Profile0Level45,
    H263Profile3Level45,
}

impl From<ProfileLevel> for CFStringRef {
    fn from(level: ProfileLevel) -> Self {
        unsafe {
            match level {
                ProfileLevel::HEVCMainAutoLevel => kVTProfileLevel_HEVC_Main_AutoLevel,
                ProfileLevel::HEVCMain10AutoLevel => kVTProfileLevel_HEVC_Main10_AutoLevel,
                ProfileLevel::HEVCMain42210AutoLevel => kVTProfileLevel_HEVC_Main42210_AutoLevel,
                ProfileLevel::H264Baseline13 => kVTProfileLevel_H264_Baseline_1_3,
                ProfileLevel::H264Baseline30 => kVTProfileLevel_H264_Baseline_3_0,
                ProfileLevel::H264Baseline31 => kVTProfileLevel_H264_Baseline_3_1,
                ProfileLevel::H264Baseline32 => kVTProfileLevel_H264_Baseline_3_2,
                ProfileLevel::H264Baseline40 => kVTProfileLevel_H264_Baseline_4_0,
                ProfileLevel::H264Baseline41 => kVTProfileLevel_H264_Baseline_4_1,
                ProfileLevel::H264Baseline42 => kVTProfileLevel_H264_Baseline_4_2,
                ProfileLevel::H264Baseline50 => kVTProfileLevel_H264_Baseline_5_0,
                ProfileLevel::H264Baseline51 => kVTProfileLevel_H264_Baseline_5_1,
                ProfileLevel::H264Baseline52 => kVTProfileLevel_H264_Baseline_5_2,
                ProfileLevel::H264BaselineAutoLevel => kVTProfileLevel_H264_Baseline_AutoLevel,
                ProfileLevel::H264Main30 => kVTProfileLevel_H264_Main_3_0,
                ProfileLevel::H264Main31 => kVTProfileLevel_H264_Main_3_1,
                ProfileLevel::H264Main32 => kVTProfileLevel_H264_Main_3_2,
                ProfileLevel::H264Main40 => kVTProfileLevel_H264_Main_4_0,
                ProfileLevel::H264Main41 => kVTProfileLevel_H264_Main_4_1,
                ProfileLevel::H264Main42 => kVTProfileLevel_H264_Main_4_2,
                ProfileLevel::H264Main50 => kVTProfileLevel_H264_Main_5_0,
                ProfileLevel::H264Main51 => kVTProfileLevel_H264_Main_5_1,
                ProfileLevel::H264Main52 => kVTProfileLevel_H264_Main_5_2,
                ProfileLevel::H264MainAutoLevel => kVTProfileLevel_H264_Main_AutoLevel,
                ProfileLevel::H264Extended50 => kVTProfileLevel_H264_Extended_5_0,
                ProfileLevel::H264ExtendedAutoLevel => kVTProfileLevel_H264_Extended_AutoLevel,
                ProfileLevel::H264High30 => kVTProfileLevel_H264_High_3_0,
                ProfileLevel::H264High31 => kVTProfileLevel_H264_High_3_1,
                ProfileLevel::H264High32 => kVTProfileLevel_H264_High_3_2,
                ProfileLevel::H264High40 => kVTProfileLevel_H264_High_4_0,
                ProfileLevel::H264High41 => kVTProfileLevel_H264_High_4_1,
                ProfileLevel::H264High42 => kVTProfileLevel_H264_High_4_2,
                ProfileLevel::H264High50 => kVTProfileLevel_H264_High_5_0,
                ProfileLevel::H264High51 => kVTProfileLevel_H264_High_5_1,
                ProfileLevel::H264High52 => kVTProfileLevel_H264_High_5_2,
                ProfileLevel::H264HighAutoLevel => kVTProfileLevel_H264_High_AutoLevel,
                ProfileLevel::H264ConstrainedHighAutoLevel => kVTProfileLevel_H264_ConstrainedHigh_AutoLevel,
                ProfileLevel::MP4VSimpleL0 => kVTProfileLevel_MP4V_Simple_L0,
                ProfileLevel::MP4VSimpleL1 => kVTProfileLevel_MP4V_Simple_L1,
                ProfileLevel::MP4VSimpleL2 => kVTProfileLevel_MP4V_Simple_L2,
                ProfileLevel::MP4VSimpleL3 => kVTProfileLevel_MP4V_Simple_L3,
                ProfileLevel::MP4VMainL2 => kVTProfileLevel_MP4V_Main_L2,
                ProfileLevel::MP4VMainL3 => kVTProfileLevel_MP4V_Main_L3,
                ProfileLevel::MP4VMainL4 => kVTProfileLevel_MP4V_Main_L4,
                ProfileLevel::MP4VAdvancedSimpleL0 => kVTProfileLevel_MP4V_AdvancedSimple_L0,
                ProfileLevel::MP4VAdvancedSimpleL1 => kVTProfileLevel_MP4V_AdvancedSimple_L1,
                ProfileLevel::MP4VAdvancedSimpleL2 => kVTProfileLevel_MP4V_AdvancedSimple_L2,
                ProfileLevel::MP4VAdvancedSimpleL3 => kVTProfileLevel_MP4V_AdvancedSimple_L3,
                ProfileLevel::MP4VAdvancedSimpleL4 => kVTProfileLevel_MP4V_AdvancedSimple_L4,
                ProfileLevel::H263Profile0Level10 => kVTProfileLevel_H263_Profile0_Level10,
                ProfileLevel::H263Profile0Level45 => kVTProfileLevel_H263_Profile0_Level45,
                ProfileLevel::H263Profile3Level45 => kVTProfileLevel_H263_Profile3_Level45,
            }
        }
    }
}

impl From<ProfileLevel> for CFString {
    fn from(level: ProfileLevel) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(level)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HDRMetadataInsertionMode {
    None,
    Auto,
    RequestSDRRangePreservation,
}

impl From<HDRMetadataInsertionMode> for CFStringRef {
    fn from(mode: HDRMetadataInsertionMode) -> Self {
        unsafe {
            match mode {
                HDRMetadataInsertionMode::None => kVTHDRMetadataInsertionMode_None,
                HDRMetadataInsertionMode::Auto => kVTHDRMetadataInsertionMode_Auto,
                HDRMetadataInsertionMode::RequestSDRRangePreservation => kVTHDRMetadataInsertionMode_RequestSDRRangePreservation,
            }
        }
    }
}

impl From<HDRMetadataInsertionMode> for CFString {
    fn from(mode: HDRMetadataInsertionMode) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(mode)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum H264EntropyMode {
    CAVLC,
    CABAC,
}

impl From<H264EntropyMode> for CFStringRef {
    fn from(mode: H264EntropyMode) -> Self {
        unsafe {
            match mode {
                H264EntropyMode::CAVLC => kVTH264EntropyMode_CAVLC,
                H264EntropyMode::CABAC => kVTH264EntropyMode_CABAC,
            }
        }
    }
}

impl From<H264EntropyMode> for CFString {
    fn from(mode: H264EntropyMode) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(mode)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoEncoderSpecification {
    EnableHardwareAcceleratedVideoEncoder,
    RequireHardwareAcceleratedVideoEncoder,
    RequiredEncoderGPURegistryID,
    PreferredEncoderGPURegistryID,
    EnableLowLatencyRateControl,
}

impl From<VideoEncoderSpecification> for CFStringRef {
    fn from(spec: VideoEncoderSpecification) -> Self {
        unsafe {
            match spec {
                VideoEncoderSpecification::EnableHardwareAcceleratedVideoEncoder => {
                    kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder
                }
                VideoEncoderSpecification::RequireHardwareAcceleratedVideoEncoder => {
                    kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder
                }
                VideoEncoderSpecification::RequiredEncoderGPURegistryID => kVTVideoEncoderSpecification_RequiredEncoderGPURegistryID,
                VideoEncoderSpecification::PreferredEncoderGPURegistryID => kVTVideoEncoderSpecification_PreferredEncoderGPURegistryID,
                VideoEncoderSpecification::EnableLowLatencyRateControl => kVTVideoEncoderSpecification_EnableLowLatencyRateControl,
            }
        }
    }
}

impl From<VideoEncoderSpecification> for CFString {
    fn from(spec: VideoEncoderSpecification) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(spec)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SampleAttachmentKey {
    QualityMetrics,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SampleAttachmentQualityMetricsKey {
    LumaMeanSquaredError,
    ChromaBlueMeanSquaredError,
    ChromaRedMeanSquaredError,
}

impl From<SampleAttachmentKey> for CFStringRef {
    fn from(key: SampleAttachmentKey) -> Self {
        unsafe {
            match key {
                SampleAttachmentKey::QualityMetrics => kVTSampleAttachmentKey_QualityMetrics,
            }
        }
    }
}

impl From<SampleAttachmentKey> for CFString {
    fn from(key: SampleAttachmentKey) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

impl From<SampleAttachmentQualityMetricsKey> for CFStringRef {
    fn from(key: SampleAttachmentQualityMetricsKey) -> Self {
        unsafe {
            match key {
                SampleAttachmentQualityMetricsKey::LumaMeanSquaredError => kVTSampleAttachmentQualityMetricsKey_LumaMeanSquaredError,
                SampleAttachmentQualityMetricsKey::ChromaBlueMeanSquaredError => kVTSampleAttachmentQualityMetricsKey_ChromaBlueMeanSquaredError,
                SampleAttachmentQualityMetricsKey::ChromaRedMeanSquaredError => kVTSampleAttachmentQualityMetricsKey_ChromaRedMeanSquaredError,
            }
        }
    }
}

impl From<SampleAttachmentQualityMetricsKey> for CFString {
    fn from(key: SampleAttachmentQualityMetricsKey) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodeFrameOptionKey {
    ForceKeyFrame,
    BaseFrameQP,
}

impl From<EncodeFrameOptionKey> for CFStringRef {
    fn from(key: EncodeFrameOptionKey) -> Self {
        unsafe {
            match key {
                EncodeFrameOptionKey::ForceKeyFrame => kVTEncodeFrameOptionKey_ForceKeyFrame,
                EncodeFrameOptionKey::BaseFrameQP => kVTEncodeFrameOptionKey_BaseFrameQP,
            }
        }
    }
}

impl From<EncodeFrameOptionKey> for CFString {
    fn from(key: EncodeFrameOptionKey) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}
