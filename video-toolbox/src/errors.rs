use bitflags::bitflags;
use core_foundation::base::OSStatus;

pub const kVTPropertyNotSupportedErr: OSStatus = -12900;
pub const kVTPropertyReadOnlyErr: OSStatus = -12901;
pub const kVTParameterErr: OSStatus = -12902;
pub const kVTInvalidSessionErr: OSStatus = -12903;
pub const kVTAllocationFailedErr: OSStatus = -12904;
pub const kVTPixelTransferNotSupportedErr: OSStatus = -12905; // c.f. -8961
pub const kVTCouldNotFindVideoDecoderErr: OSStatus = -12906;
pub const kVTCouldNotCreateInstanceErr: OSStatus = -12907;
pub const kVTCouldNotFindVideoEncoderErr: OSStatus = -12908;
pub const kVTVideoDecoderBadDataErr: OSStatus = -12909; // c.f. -8969
pub const kVTVideoDecoderUnsupportedDataFormatErr: OSStatus = -12910; // c.f. -8970
pub const kVTVideoDecoderMalfunctionErr: OSStatus = -12911; // c.f. -8960
pub const kVTVideoEncoderMalfunctionErr: OSStatus = -12912;
pub const kVTVideoDecoderNotAvailableNowErr: OSStatus = -12913;
pub const kVTImageRotationNotSupportedErr: OSStatus = -12914;
pub const kVTVideoEncoderNotAvailableNowErr: OSStatus = -12915;
pub const kVTFormatDescriptionChangeNotSupportedErr: OSStatus = -12916;
pub const kVTInsufficientSourceColorDataErr: OSStatus = -12917;
pub const kVTCouldNotCreateColorCorrectionDataErr: OSStatus = -12918;
pub const kVTColorSyncTransformConvertFailedErr: OSStatus = -12919;
pub const kVTVideoDecoderAuthorizationErr: OSStatus = -12210;
pub const kVTVideoEncoderAuthorizationErr: OSStatus = -12211;
pub const kVTColorCorrectionPixelTransferFailedErr: OSStatus = -12212;
pub const kVTMultiPassStorageIdentifierMismatchErr: OSStatus = -12213;
pub const kVTMultiPassStorageInvalidErr: OSStatus = -12214;
pub const kVTFrameSiloInvalidTimeStampErr: OSStatus = -12215;
pub const kVTFrameSiloInvalidTimeRangeErr: OSStatus = -12216;
pub const kVTCouldNotFindTemporalFilterErr: OSStatus = -12217;
pub const kVTPixelTransferNotPermittedErr: OSStatus = -12218;
pub const kVTColorCorrectionImageRotationFailedErr: OSStatus = -12219;
pub const kVTVideoDecoderRemovedErr: OSStatus = -17690;
pub const kVTSessionMalfunctionErr: OSStatus = -17691;
pub const kVTVideoDecoderNeedsRosettaErr: OSStatus = -17692;
pub const kVTVideoEncoderNeedsRosettaErr: OSStatus = -17693;
pub const kVTVideoDecoderReferenceMissingErr: OSStatus = -17694;
pub const kVTVideoDecoderCallbackMessagingErr: OSStatus = -17695;
pub const kVTVideoDecoderUnknownErr: OSStatus = -17696;
pub const kVTExtensionDisabledErr: OSStatus = -17697;
pub const kVTVideoEncoderMVHEVCVideoLayerIDsMismatchErr: OSStatus = -17698;
pub const kVTCouldNotOutputTaggedBufferGroupErr: OSStatus = -17699;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct VTDecodeFrameFlags: u32 {
        #[doc(alias = "kVTDecodeFrame_EnableAsynchronousDecompression")]
        const Frame_EnableAsynchronousDecompression = 1 << 0;
        #[doc(alias = "kVTDecodeFrame_DoNotOutputFrame")]
        const Frame_DoNotOutputFrame                = 1 << 1;
        #[doc(alias = "kVTDecodeFrame_1xRealTimePlayback")]
        const Frame_1xRealTimePlayback              = 1 << 2;
        #[doc(alias = "kVTDecodeFrame_EnableTemporalProcessing")]
        const Frame_EnableTemporalProcessing        = 1 << 3;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct VTDecodeInfoFlags: u32 {
        #[doc(alias = "kVTDecodeInfo_Asynchronous")]
        const Asynchronous                  = 1 << 0;
        #[doc(alias = "kVTDecodeInfo_FrameDropped")]
        const FrameDropped                  = 1 << 1;
        #[doc(alias = "kVTDecodeInfo_ImageBufferModifiable")]
        const ImageBufferModifiable         = 1 << 2;
        #[doc(alias = "kVTDecodeInfo_SkippedLeadingFrameDropped")]
        const SkippedLeadingFrameDropped    = 1 << 3;
        #[doc(alias = "kVTDecodeInfo_FrameInterrupted")]
        const FrameInterrupted              = 1 << 4;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct VTEncodeInfoFlags: u32 {
        #[doc(alias = "kVTEncodeInfo_Asynchronous")]
        const Asynchronous          = 1 << 0;
        #[doc(alias = "kVTEncodeInfo_FrameDropped")]
        const FrameDropped          = 1 << 1;
    }
}
