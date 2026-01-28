use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
};

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
    pub static kVTDecompressionPropertyKey_UsingGPURegistryID: CFStringRef;
    pub static kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata: CFStringRef;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecompressionPropertyKey {
    PixelBufferPool,
    PixelBufferPoolIsShared,
    OutputPoolRequestedMinimumBufferCount,
    NumberOfFramesBeingDecoded,
    MinOutputPresentationTimeStampOfFramesBeingDecoded,
    MaxOutputPresentationTimeStampOfFramesBeingDecoded,
    ContentHasInterframeDependencies,
    VideoDecoderSpecification,
    UsingHardwareAcceleratedVideoDecoder,
    RealTime,
    ThreadCount,
    FieldMode,
    DeinterlaceMode,
    ReducedResolutionDecode,
    ReducedCoefficientDecode,
    ReducedFrameDelivery,
    OnlyTheseFrames,
    TemporalLevelLimit,
    SuggestedQualityOfServiceTiers,
    SupportedPixelFormatsOrderedByQuality,
    SupportedPixelFormatsOrderedByPerformance,
    PixelFormatsWithReducedResolutionSupport,
    PixelTransferProperties,
    UsingGPURegistryID,
    PropagatePerFrameHDRDisplayMetadata,
}

impl From<DecompressionPropertyKey> for CFStringRef {
    fn from(key: DecompressionPropertyKey) -> Self {
        unsafe {
            match key {
                DecompressionPropertyKey::PixelBufferPool => kVTDecompressionPropertyKey_PixelBufferPool,
                DecompressionPropertyKey::PixelBufferPoolIsShared => kVTDecompressionPropertyKey_PixelBufferPoolIsShared,
                DecompressionPropertyKey::OutputPoolRequestedMinimumBufferCount => kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount,
                DecompressionPropertyKey::NumberOfFramesBeingDecoded => kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded,
                DecompressionPropertyKey::MinOutputPresentationTimeStampOfFramesBeingDecoded => {
                    kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded
                }
                DecompressionPropertyKey::MaxOutputPresentationTimeStampOfFramesBeingDecoded => {
                    kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded
                }
                DecompressionPropertyKey::ContentHasInterframeDependencies => kVTDecompressionPropertyKey_ContentHasInterframeDependencies,
                DecompressionPropertyKey::VideoDecoderSpecification => panic!("Use VideoDecoderSpecification enum for this key"),
                DecompressionPropertyKey::UsingHardwareAcceleratedVideoDecoder => kVTDecompressionPropertyKey_UsingHardwareAcceleratedVideoDecoder,
                DecompressionPropertyKey::RealTime => kVTDecompressionPropertyKey_RealTime,
                DecompressionPropertyKey::ThreadCount => kVTDecompressionPropertyKey_ThreadCount,
                DecompressionPropertyKey::FieldMode => kVTDecompressionPropertyKey_FieldMode,
                DecompressionPropertyKey::DeinterlaceMode => kVTDecompressionPropertyKey_DeinterlaceMode,
                DecompressionPropertyKey::ReducedResolutionDecode => kVTDecompressionPropertyKey_ReducedResolutionDecode,
                DecompressionPropertyKey::ReducedCoefficientDecode => kVTDecompressionPropertyKey_ReducedCoefficientDecode,
                DecompressionPropertyKey::ReducedFrameDelivery => kVTDecompressionPropertyKey_ReducedFrameDelivery,
                DecompressionPropertyKey::OnlyTheseFrames => kVTDecompressionPropertyKey_OnlyTheseFrames,
                DecompressionPropertyKey::TemporalLevelLimit => kVTDecompressionProperty_TemporalLevelLimit,
                DecompressionPropertyKey::SuggestedQualityOfServiceTiers => kVTDecompressionPropertyKey_SuggestedQualityOfServiceTiers,
                DecompressionPropertyKey::SupportedPixelFormatsOrderedByQuality => kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByQuality,
                DecompressionPropertyKey::SupportedPixelFormatsOrderedByPerformance => {
                    kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance
                }
                DecompressionPropertyKey::PixelFormatsWithReducedResolutionSupport => {
                    kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport
                }
                DecompressionPropertyKey::PixelTransferProperties => kVTDecompressionPropertyKey_PixelTransferProperties,
                DecompressionPropertyKey::UsingGPURegistryID => kVTDecompressionPropertyKey_UsingGPURegistryID,
                DecompressionPropertyKey::PropagatePerFrameHDRDisplayMetadata => kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata,
            }
        }
    }
}

impl From<DecompressionPropertyKey> for CFString {
    fn from(key: DecompressionPropertyKey) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldMode {
    BothFields,
    TopFieldOnly,
    BottomFieldOnly,
    SingleField,
    DeinterlaceFields,
}

impl From<FieldMode> for CFStringRef {
    fn from(mode: FieldMode) -> Self {
        unsafe {
            match mode {
                FieldMode::BothFields => kVTDecompressionProperty_FieldMode_BothFields,
                FieldMode::TopFieldOnly => kVTDecompressionProperty_FieldMode_TopFieldOnly,
                FieldMode::BottomFieldOnly => kVTDecompressionProperty_FieldMode_BottomFieldOnly,
                FieldMode::SingleField => kVTDecompressionProperty_FieldMode_SingleField,
                FieldMode::DeinterlaceFields => kVTDecompressionProperty_FieldMode_DeinterlaceFields,
            }
        }
    }
}

impl From<FieldMode> for CFString {
    fn from(mode: FieldMode) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(mode)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeinterlaceMode {
    VerticalFilter,
    Temporal,
}

impl From<DeinterlaceMode> for CFStringRef {
    fn from(mode: DeinterlaceMode) -> Self {
        unsafe {
            match mode {
                DeinterlaceMode::VerticalFilter => kVTDecompressionProperty_DeinterlaceMode_VerticalFilter,
                DeinterlaceMode::Temporal => kVTDecompressionProperty_DeinterlaceMode_Temporal,
            }
        }
    }
}

impl From<DeinterlaceMode> for CFString {
    fn from(mode: DeinterlaceMode) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(mode)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnlyTheseFrames {
    AllFrames,
    NonDroppableFrames,
    IFrames,
    KeyFrames,
}

impl From<OnlyTheseFrames> for CFStringRef {
    fn from(val: OnlyTheseFrames) -> Self {
        unsafe {
            match val {
                OnlyTheseFrames::AllFrames => kVTDecompressionProperty_OnlyTheseFrames_AllFrames,
                OnlyTheseFrames::NonDroppableFrames => kVTDecompressionProperty_OnlyTheseFrames_NonDroppableFrames,
                OnlyTheseFrames::IFrames => kVTDecompressionProperty_OnlyTheseFrames_IFrames,
                OnlyTheseFrames::KeyFrames => kVTDecompressionProperty_OnlyTheseFrames_KeyFrames,
            }
        }
    }
}

impl From<OnlyTheseFrames> for CFString {
    fn from(val: OnlyTheseFrames) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(val)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoDecoderSpecification {
    EnableHardwareAcceleratedVideoDecoder,
    RequireHardwareAcceleratedVideoDecoder,
    RequiredDecoderGPURegistryID,
    PreferredDecoderGPURegistryID,
}

impl From<VideoDecoderSpecification> for CFStringRef {
    fn from(spec: VideoDecoderSpecification) -> Self {
        unsafe {
            match spec {
                VideoDecoderSpecification::EnableHardwareAcceleratedVideoDecoder => {
                    kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder
                }
                VideoDecoderSpecification::RequireHardwareAcceleratedVideoDecoder => {
                    kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder
                }
                VideoDecoderSpecification::RequiredDecoderGPURegistryID => kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID,
                VideoDecoderSpecification::PreferredDecoderGPURegistryID => kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID,
            }
        }
    }
}

impl From<VideoDecoderSpecification> for CFString {
    fn from(spec: VideoDecoderSpecification) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(spec)) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecompressionResolutionKey {
    Width,
    Height,
}

impl From<DecompressionResolutionKey> for CFStringRef {
    fn from(key: DecompressionResolutionKey) -> Self {
        unsafe {
            match key {
                DecompressionResolutionKey::Width => kVTDecompressionResolutionKey_Width,
                DecompressionResolutionKey::Height => kVTDecompressionResolutionKey_Height,
            }
        }
    }
}

impl From<DecompressionResolutionKey> for CFString {
    fn from(key: DecompressionResolutionKey) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}
