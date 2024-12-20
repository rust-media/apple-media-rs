#![allow(non_upper_case_globals)]

use core_foundation_sys::base::OSStatus;

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
