use std::{
    mem::{forget, size_of_val},
    ptr::{null, null_mut},
    slice::from_raw_parts,
};

use core_audio_types::base_types::{AudioChannelLayout, AudioFormatListItem, AudioStreamBasicDescription};
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, CFType, CFTypeID, CFTypeRef, OSStatus, TCFType, TCFTypeRef},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    propertylist::{CFPropertyList, CFPropertyListRef},
    string::{CFString, CFStringRef},
};
use core_graphics::{
    base::CGFloat,
    geometry::{CGRect, CGSize},
};
use core_video::image_buffer::{CVImageBuffer, CVImageBufferRef};
use libc::{c_int, c_void, size_t};
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

use crate::{time::CMTime, OSType};

pub const kCMFormatDescriptionError_InvalidParameter: OSStatus = -12710;
pub const kCMFormatDescriptionError_AllocationFailed: OSStatus = -12711;
pub const kCMFormatDescriptionError_ValueNotAvailable: OSStatus = -12718;

type FourCharCode = u32;

pub type CMMediaType = FourCharCode;

#[inline]
const fn fourcc(code: &[u8; 4]) -> u32 {
    ((code[0] as u32) << 24) | ((code[1] as u32) << 16) | ((code[2] as u32) << 8) | ((code[3] as u32) << 0)
}

pub const kCMMediaType_Video: CMMediaType = fourcc(b"vide");
pub const kCMMediaType_Audio: CMMediaType = fourcc(b"soun");
pub const kCMMediaType_Muxed: CMMediaType = fourcc(b"muxx");
pub const kCMMediaType_Text: CMMediaType = fourcc(b"text");
pub const kCMMediaType_ClosedCaption: CMMediaType = fourcc(b"clcp");
pub const kCMMediaType_Subtitle: CMMediaType = fourcc(b"sbtl");
pub const kCMMediaType_TimeCode: CMMediaType = fourcc(b"tmcd");
pub const kCMMediaType_Metadata: CMMediaType = fourcc(b"meta");
pub const kCMMediaType_TaggedBufferGroup: CMMediaType = fourcc(b"tbgr");

#[repr(C)]
pub struct opaqueCMFormatDescription(c_void);

pub type CMFormatDescriptionRef = *mut opaqueCMFormatDescription;

extern "C" {
    pub fn CMFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        mediaType: CMMediaType,
        mediaSubType: FourCharCode,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMFormatDescriptionGetTypeID() -> CFTypeID;
    pub fn CMFormatDescriptionEqual(formatDescription: CMFormatDescriptionRef, otherFormatDescription: CMFormatDescriptionRef) -> Boolean;
    pub fn CMFormatDescriptionEqualIgnoringExtensionKeys(
        formatDescription: CMFormatDescriptionRef,
        otherFormatDescription: CMFormatDescriptionRef,
        formatDescriptionExtensionKeysToIgnore: CFTypeRef,
        sampleDescriptionExtensionAtomKeysToIgnore: CFTypeRef,
    ) -> Boolean;
    pub fn CMFormatDescriptionGetMediaType(desc: CMFormatDescriptionRef) -> CMMediaType;
    pub fn CMFormatDescriptionGetMediaSubType(desc: CMFormatDescriptionRef) -> FourCharCode;
    pub fn CMFormatDescriptionGetExtensions(desc: CMFormatDescriptionRef) -> CFDictionaryRef;

    pub static kCMFormatDescriptionExtension_OriginalCompressionSettings: CFStringRef;
    pub static kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms: CFStringRef;
    pub static kCMFormatDescriptionExtension_VerbatimSampleDescription: CFStringRef;
    pub static kCMFormatDescriptionExtension_VerbatimISOSampleEntry: CFStringRef;

    pub fn CMFormatDescriptionGetExtension(desc: CMFormatDescriptionRef, extensionKey: CFStringRef) -> CFPropertyListRef;
}

pub type CMAudioCodecType = FourCharCode;

pub const kCMAudioCodecType_AAC_LCProtected: CMAudioCodecType = fourcc(b"paac");
pub const kCMAudioCodecType_AAC_AudibleProtected: CMAudioCodecType = fourcc(b"aaac");

pub type CMAudioFormatDescriptionRef = CMFormatDescriptionRef;

extern "C" {
    pub fn CMAudioFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        asbd: *const AudioStreamBasicDescription,
        layoutSize: size_t,
        layout: *const AudioChannelLayout,
        magicCookieSize: size_t,
        magicCookie: *const c_void,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMAudioFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMAudioFormatDescriptionGetStreamBasicDescription(desc: CMAudioFormatDescriptionRef) -> *const AudioStreamBasicDescription;
    pub fn CMAudioFormatDescriptionGetMagicCookie(desc: CMAudioFormatDescriptionRef, sizeOut: *mut size_t) -> *const c_void;
    pub fn CMAudioFormatDescriptionGetChannelLayout(desc: CMAudioFormatDescriptionRef, sizeOut: *mut size_t) -> *const AudioChannelLayout;
    pub fn CMAudioFormatDescriptionGetFormatList(desc: CMAudioFormatDescriptionRef, sizeOut: *mut size_t) -> *const AudioFormatListItem;
    pub fn CMAudioFormatDescriptionGetRichestDecodableFormat(desc: CMAudioFormatDescriptionRef) -> *const AudioFormatListItem;
    pub fn CMAudioFormatDescriptionGetMostCompatibleFormat(desc: CMAudioFormatDescriptionRef) -> *const AudioFormatListItem;
    pub fn CMAudioFormatDescriptionCreateSummary(
        allocator: CFAllocatorRef,
        formatDescriptionArray: CFArrayRef,
        flags: u32,
        formatDescriptionOut: *mut CMAudioFormatDescriptionRef,
    ) -> OSStatus;
}

pub type CMAudioFormatDescriptionMask = u32;

pub const kCMAudioFormatDescriptionMask_StreamBasicDescription: CMAudioFormatDescriptionMask = 1 << 0;
pub const kCMAudioFormatDescriptionMask_MagicCookie: CMAudioFormatDescriptionMask = 1 << 1;
pub const kCMAudioFormatDescriptionMask_ChannelLayout: CMAudioFormatDescriptionMask = 1 << 2;
pub const kCMAudioFormatDescriptionMask_Extensions: CMAudioFormatDescriptionMask = 1 << 3;
pub const kCMAudioFormatDescriptionMask_All: CMAudioFormatDescriptionMask = kCMAudioFormatDescriptionMask_StreamBasicDescription |
    kCMAudioFormatDescriptionMask_MagicCookie |
    kCMAudioFormatDescriptionMask_ChannelLayout |
    kCMAudioFormatDescriptionMask_Extensions;

extern "C" {
    pub fn CMAudioFormatDescriptionEqual(
        formatDescription: CMAudioFormatDescriptionRef,
        otherFormatDescription: CMAudioFormatDescriptionRef,
        equalityMask: CMAudioFormatDescriptionMask,
        equalityMaskOut: *mut CMAudioFormatDescriptionMask,
    ) -> Boolean;
}

pub type CMVideoFormatDescriptionRef = CMFormatDescriptionRef;

pub type CMPixelFormatType = FourCharCode;

pub const kCMPixelFormat_32ARGB: CMPixelFormatType = 32;
pub const kCMPixelFormat_32BGRA: CMPixelFormatType = fourcc(b"BGRA");
pub const kCMPixelFormat_24RGB: CMPixelFormatType = 24;
pub const kCMPixelFormat_16BE555: CMPixelFormatType = 16;
pub const kCMPixelFormat_16BE565: CMPixelFormatType = fourcc(b"B565");
pub const kCMPixelFormat_16LE555: CMPixelFormatType = fourcc(b"L555");
pub const kCMPixelFormat_16LE565: CMPixelFormatType = fourcc(b"L565");
pub const kCMPixelFormat_16LE5551: CMPixelFormatType = fourcc(b"5551");
pub const kCMPixelFormat_422YpCbCr8: CMPixelFormatType = fourcc(b"2vuy");
pub const kCMPixelFormat_422YpCbCr8_yuvs: CMPixelFormatType = fourcc(b"yuvs");
pub const kCMPixelFormat_444YpCbCr8: CMPixelFormatType = fourcc(b"v308");
pub const kCMPixelFormat_4444YpCbCrA8: CMPixelFormatType = fourcc(b"v408");
pub const kCMPixelFormat_422YpCbCr16: CMPixelFormatType = fourcc(b"v216");
pub const kCMPixelFormat_422YpCbCr10: CMPixelFormatType = fourcc(b"v210");
pub const kCMPixelFormat_444YpCbCr10: CMPixelFormatType = fourcc(b"v410");
pub const kCMPixelFormat_8IndexedGray_WhiteIsZero: CMPixelFormatType = 0x00000028;

pub type CMVideoCodecType = FourCharCode;

pub const kCMVideoCodecType_422YpCbCr8: CMVideoCodecType = kCMPixelFormat_422YpCbCr8;
pub const kCMVideoCodecType_Animation: CMVideoCodecType = fourcc(b"rle ");
pub const kCMVideoCodecType_Cinepak: CMVideoCodecType = fourcc(b"cvid");
pub const kCMVideoCodecType_JPEG: CMVideoCodecType = fourcc(b"jpeg");
pub const kCMVideoCodecType_JPEG_OpenDML: CMVideoCodecType = fourcc(b"dmb1");
pub const kCMVideoCodecType_SorensonVideo: CMVideoCodecType = fourcc(b"SVQ1");
pub const kCMVideoCodecType_SorensonVideo3: CMVideoCodecType = fourcc(b"SVQ3");
pub const kCMVideoCodecType_H263: CMVideoCodecType = fourcc(b"h263");
pub const kCMVideoCodecType_H264: CMVideoCodecType = fourcc(b"avc1");
pub const kCMVideoCodecType_HEVC: CMVideoCodecType = fourcc(b"hvc1");
pub const kCMVideoCodecType_HEVCWithAlpha: CMVideoCodecType = fourcc(b"muxa");
pub const kCMVideoCodecType_DolbyVisionHEVC: CMVideoCodecType = fourcc(b"dvh1");
pub const kCMVideoCodecType_MPEG4Video: CMVideoCodecType = fourcc(b"mp4v");
pub const kCMVideoCodecType_MPEG2Video: CMVideoCodecType = fourcc(b"mp2v");
pub const kCMVideoCodecType_MPEG1Video: CMVideoCodecType = fourcc(b"mp1v");
pub const kCMVideoCodecType_VP9: CMVideoCodecType = fourcc(b"vp09");
pub const kCMVideoCodecType_DVCNTSC: CMVideoCodecType = fourcc(b"dvc ");
pub const kCMVideoCodecType_DVCPAL: CMVideoCodecType = fourcc(b"dvcp");
pub const kCMVideoCodecType_DVCProPAL: CMVideoCodecType = fourcc(b"dvpp");
pub const kCMVideoCodecType_DVCPro50NTSC: CMVideoCodecType = fourcc(b"dv5n");
pub const kCMVideoCodecType_DVCPro50PAL: CMVideoCodecType = fourcc(b"dv5p");
pub const kCMVideoCodecType_DVCPROHD720p60: CMVideoCodecType = fourcc(b"dvhp");
pub const kCMVideoCodecType_DVCPROHD720p50: CMVideoCodecType = fourcc(b"dvhq");
pub const kCMVideoCodecType_DVCPROHD1080i60: CMVideoCodecType = fourcc(b"dvh6");
pub const kCMVideoCodecType_DVCPROHD1080i50: CMVideoCodecType = fourcc(b"dvh5");
pub const kCMVideoCodecType_DVCPROHD1080p30: CMVideoCodecType = fourcc(b"dvh3");
pub const kCMVideoCodecType_DVCPROHD1080p25: CMVideoCodecType = fourcc(b"dvh2");
pub const kCMVideoCodecType_AppleProRes4444XQ: CMVideoCodecType = fourcc(b"ap4x");
pub const kCMVideoCodecType_AppleProRes4444: CMVideoCodecType = fourcc(b"ap4h");
pub const kCMVideoCodecType_AppleProRes422HQ: CMVideoCodecType = fourcc(b"apch");
pub const kCMVideoCodecType_AppleProRes422: CMVideoCodecType = fourcc(b"apcn");
pub const kCMVideoCodecType_AppleProRes422LT: CMVideoCodecType = fourcc(b"apcs");
pub const kCMVideoCodecType_AppleProRes422Proxy: CMVideoCodecType = fourcc(b"apco");
pub const kCMVideoCodecType_AppleProResRAW: CMVideoCodecType = fourcc(b"aprn");
pub const kCMVideoCodecType_AppleProResRAWHQ: CMVideoCodecType = fourcc(b"aprh");
pub const kCMVideoCodecType_DisparityHEVC: CMVideoCodecType = fourcc(b"dish");
pub const kCMVideoCodecType_DepthHEVC: CMVideoCodecType = fourcc(b"deph");
pub const kCMVideoCodecType_AV1: CMVideoCodecType = fourcc(b"av01");

#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMVideoDimensions {
    pub width: i32,
    pub height: i32,
}

extern "C" {
    pub static kCMFormatDescriptionExtension_FormatName: CFStringRef;
    pub static kCMFormatDescriptionExtension_Depth: CFStringRef;
    pub static kCMFormatDescriptionExtension_CleanAperture: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureWidth: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureHeight: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureHorizontalOffset: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureVerticalOffset: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureWidthRational: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureHeightRational: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureHorizontalOffsetRational: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureVerticalOffsetRational: CFStringRef;
    pub static kCMFormatDescriptionExtension_FieldCount: CFStringRef;
    pub static kCMFormatDescriptionFieldDetail_TemporalTopFirst: CFStringRef;
    pub static kCMFormatDescriptionFieldDetail_TemporalBottomFirst: CFStringRef;
    pub static kCMFormatDescriptionFieldDetail_SpatialFirstLineEarly: CFStringRef;
    pub static kCMFormatDescriptionFieldDetail_SpatialFirstLineLate: CFStringRef;
    pub static kCMFormatDescriptionExtension_PixelAspectRatio: CFStringRef;
    pub static kCMFormatDescriptionKey_PixelAspectRatioHorizontalSpacing: CFStringRef;
    pub static kCMFormatDescriptionKey_PixelAspectRatioVerticalSpacing: CFStringRef;
    pub static kCMFormatDescriptionExtension_ColorPrimaries: CFStringRef;
    pub static kCMFormatDescriptionColorPrimaries_ITU_R_709_2: CFStringRef;
    pub static kCMFormatDescriptionColorPrimaries_EBU_3213: CFStringRef;
    pub static kCMFormatDescriptionColorPrimaries_SMPTE_C: CFStringRef;
    pub static kCMFormatDescriptionColorPrimaries_DCI_P3: CFStringRef;
    pub static kCMFormatDescriptionColorPrimaries_P3_D65: CFStringRef;
    pub static kCMFormatDescriptionColorPrimaries_ITU_R_2020: CFStringRef;
    pub static kCMFormatDescriptionColorPrimaries_P22: CFStringRef;
    pub static kCMFormatDescriptionExtension_TransferFunction: CFStringRef;
    pub static kCMFormatDescriptionTransferFunction_ITU_R_709_2: CFStringRef;
    pub static kCMFormatDescriptionTransferFunction_SMPTE_240M_1995: CFStringRef;
    pub static kCMFormatDescriptionTransferFunction_UseGamma: CFStringRef;
    pub static kCMFormatDescriptionTransferFunction_ITU_R_2020: CFStringRef;
    pub static kCMFormatDescriptionTransferFunction_SMPTE_ST_428_1: CFStringRef;
    pub static kCMFormatDescriptionTransferFunction_SMPTE_ST_2084_PQ: CFStringRef;
    pub static kCMFormatDescriptionTransferFunction_ITU_R_2100_HLG: CFStringRef;
    pub static kCMFormatDescriptionTransferFunction_Linear: CFStringRef;
    pub static kCMFormatDescriptionTransferFunction_sRGB: CFStringRef;
    pub static kCMFormatDescriptionExtension_GammaLevel: CFStringRef;
    pub static kCMFormatDescriptionExtension_YCbCrMatrix: CFStringRef;
    pub static kCMFormatDescriptionYCbCrMatrix_ITU_R_709_2: CFStringRef;
    pub static kCMFormatDescriptionYCbCrMatrix_ITU_R_601_4: CFStringRef;
    pub static kCMFormatDescriptionYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
    pub static kCMFormatDescriptionYCbCrMatrix_ITU_R_2020: CFStringRef;
    pub static kCMFormatDescriptionExtension_FullRangeVideo: CFStringRef;
    pub static kCMFormatDescriptionExtension_ICCProfile: CFStringRef;
    pub static kCMFormatDescriptionExtension_BytesPerRow: CFStringRef;
    pub static kCMFormatDescriptionExtension_ChromaLocationTopField: CFStringRef;
    pub static kCMFormatDescriptionExtension_ChromaLocationBottomField: CFStringRef;
    pub static kCMFormatDescriptionChromaLocation_Left: CFStringRef;
    pub static kCMFormatDescriptionChromaLocation_Center: CFStringRef;
    pub static kCMFormatDescriptionChromaLocation_TopLeft: CFStringRef;
    pub static kCMFormatDescriptionChromaLocation_Top: CFStringRef;
    pub static kCMFormatDescriptionChromaLocation_BottomLeft: CFStringRef;
    pub static kCMFormatDescriptionChromaLocation_Bottom: CFStringRef;
    pub static kCMFormatDescriptionChromaLocation_DV420: CFStringRef;
    pub static kCMFormatDescriptionConformsToMPEG2VideoProfile: CFStringRef;
    pub static kCMFormatDescriptionExtension_ProtectedContentOriginalFormat: CFStringRef;
}

pub const kCMMPEG2VideoProfile_HDV_720p30: i32 = fourcc(b"hdv1") as i32;
pub const kCMMPEG2VideoProfile_HDV_1080i60: i32 = fourcc(b"hdv2") as i32;
pub const kCMMPEG2VideoProfile_HDV_1080i50: i32 = fourcc(b"hdv3") as i32;
pub const kCMMPEG2VideoProfile_HDV_720p24: i32 = fourcc(b"hdv4") as i32;
pub const kCMMPEG2VideoProfile_HDV_720p25: i32 = fourcc(b"hdv5") as i32;
pub const kCMMPEG2VideoProfile_HDV_1080p24: i32 = fourcc(b"hdv6") as i32;
pub const kCMMPEG2VideoProfile_HDV_1080p25: i32 = fourcc(b"hdv7") as i32;
pub const kCMMPEG2VideoProfile_HDV_1080p30: i32 = fourcc(b"hdv8") as i32;
pub const kCMMPEG2VideoProfile_HDV_720p60: i32 = fourcc(b"hdv9") as i32;
pub const kCMMPEG2VideoProfile_HDV_720p50: i32 = fourcc(b"hdva") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD_1080i60_VBR35: i32 = fourcc(b"xdv2") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD_1080i50_VBR35: i32 = fourcc(b"xdv3") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD_1080p24_VBR35: i32 = fourcc(b"xdv6") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD_1080p25_VBR35: i32 = fourcc(b"xdv7") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD_1080p30_VBR35: i32 = fourcc(b"xdv8") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_720p24_VBR35: i32 = fourcc(b"xdv4") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_720p25_VBR35: i32 = fourcc(b"xdv5") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_720p30_VBR35: i32 = fourcc(b"xdv1") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_720p50_VBR35: i32 = fourcc(b"xdva") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_720p60_VBR35: i32 = fourcc(b"xdv9") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_1080i60_VBR35: i32 = fourcc(b"xdvb") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_1080i50_VBR35: i32 = fourcc(b"xdvc") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_1080p24_VBR35: i32 = fourcc(b"xdvd") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_1080p25_VBR35: i32 = fourcc(b"xdve") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_EX_1080p30_VBR35: i32 = fourcc(b"xdvf") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_720p50_CBR50: i32 = fourcc(b"xd5a") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_720p60_CBR50: i32 = fourcc(b"xd59") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_1080i60_CBR50: i32 = fourcc(b"xd5b") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_1080i50_CBR50: i32 = fourcc(b"xd5c") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_1080p24_CBR50: i32 = fourcc(b"xd5d") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_1080p25_CBR50: i32 = fourcc(b"xd5e") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_1080p30_CBR50: i32 = fourcc(b"xd5f") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD_540p: i32 = fourcc(b"xdhd") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_540p: i32 = fourcc(b"xdh2") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_720p24_CBR50: i32 = fourcc(b"xd54") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_720p25_CBR50: i32 = fourcc(b"xd55") as i32;
pub const kCMMPEG2VideoProfile_XDCAM_HD422_720p30_CBR50: i32 = fourcc(b"xd51") as i32;
pub const kCMMPEG2VideoProfile_XF: i32 = fourcc(b"xfz1") as i32;

extern "C" {
    pub static kCMFormatDescriptionExtension_TemporalQuality: CFStringRef;
    pub static kCMFormatDescriptionExtension_SpatialQuality: CFStringRef;
    pub static kCMFormatDescriptionExtension_VerbatimImageDescription: CFStringRef;
    pub static kCMFormatDescriptionExtension_Version: CFStringRef;
    pub static kCMFormatDescriptionExtension_RevisionLevel: CFStringRef;
    pub static kCMFormatDescriptionExtension_Vendor: CFStringRef;
    pub static kCMFormatDescriptionVendor_Apple: CFStringRef;
    pub static kCMFormatDescriptionExtension_MasteringDisplayColorVolume: CFStringRef;
    pub static kCMFormatDescriptionExtension_ContentLightLevelInfo: CFStringRef;
    pub static kCMFormatDescriptionExtension_ContentColorVolume: CFStringRef;
    pub static kCMFormatDescriptionExtension_AlternativeTransferCharacteristics: CFStringRef;
    pub static kCMFormatDescriptionExtension_AuxiliaryTypeInfo: CFStringRef;
    pub static kCMFormatDescriptionExtension_AlphaChannelMode: CFStringRef;
    pub static kCMFormatDescriptionAlphaChannelMode_StraightAlpha: CFStringRef;
    pub static kCMFormatDescriptionAlphaChannelMode_PremultipliedAlpha: CFStringRef;
    pub static kCMFormatDescriptionExtension_ContainsAlphaChannel: CFStringRef;
    pub static kCMFormatDescriptionExtension_BitsPerComponent: CFStringRef;
    pub static kCMFormatDescriptionExtension_HorizontalFieldOfView: CFStringRef;
    pub static kCMFormatDescriptionExtension_HeroEye: CFStringRef;
    pub static kCMFormatDescriptionHeroEye_Left: CFStringRef;
    pub static kCMFormatDescriptionHeroEye_Right: CFStringRef;
    pub static kCMFormatDescriptionExtension_StereoCameraBaseline: CFStringRef;
    pub static kCMFormatDescriptionExtension_HorizontalDisparityAdjustment: CFStringRef;

    pub fn CMVideoFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        codecType: CMVideoCodecType,
        width: i32,
        height: i32,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionCreateForImageBuffer(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        formatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionCreateFromH264ParameterSets(
        allocator: CFAllocatorRef,
        parameterSetCount: size_t,
        parameterSetPointers: *const *const u8,
        parameterSetSizes: *const size_t,
        NALUnitHeaderLength: c_int,
        formatDescriptionOut: *mut CMFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionCreateFromHEVCParameterSets(
        allocator: CFAllocatorRef,
        parameterSetCount: size_t,
        parameterSetPointers: *const *const u8,
        parameterSetSizes: *const size_t,
        NALUnitHeaderLength: c_int,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
        videoDesc: CMFormatDescriptionRef,
        parameterSetIndex: size_t,
        parameterSetPointerOut: *mut *const u8,
        parameterSetSizeOut: *mut size_t,
        parameterSetCountOut: *mut size_t,
        NALUnitHeaderLengthOut: *mut c_int,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
        videoDesc: CMFormatDescriptionRef,
        parameterSetIndex: size_t,
        parameterSetPointerOut: *mut *const u8,
        parameterSetSizeOut: *mut size_t,
        parameterSetCountOut: *mut size_t,
        NALUnitHeaderLengthOut: *mut c_int,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionGetDimensions(videoDesc: CMVideoFormatDescriptionRef) -> CMVideoDimensions;
    pub fn CMVideoFormatDescriptionGetPresentationDimensions(
        videoDesc: CMVideoFormatDescriptionRef,
        usePixelAspectRatio: Boolean,
        useCleanAperture: Boolean,
    ) -> CGSize;
    pub fn CMVideoFormatDescriptionGetCleanAperture(videoDesc: CMVideoFormatDescriptionRef, originIsAtTopLeft: Boolean) -> CGRect;
    pub fn CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers() -> CFArrayRef;
    pub fn CMVideoFormatDescriptionMatchesImageBuffer(videoDesc: CMVideoFormatDescriptionRef, imageBuffer: CVImageBufferRef) -> Boolean;
    pub fn CMVideoFormatDescriptionCopyTagCollectionArray(
        formatDescription: CMVideoFormatDescriptionRef,
        tagCollectionsOut: *mut CFArrayRef,
    ) -> OSStatus;
}

pub type CMTaggedBufferGroupFormatDescriptionRef = CMFormatDescriptionRef;

pub type CMTaggedBufferGroupFormatType = FourCharCode;

pub const kCMTaggedBufferGroupFormatType_TaggedBufferGroup: CMTaggedBufferGroupFormatType = fourcc(b"tbgr");

pub type CMMuxedFormatDescriptionRef = CMFormatDescriptionRef;

pub type CMMuxedStreamType = FourCharCode;

pub const kCMMuxedStreamType_MPEG1System: CMMuxedStreamType = fourcc(b"mp1s");
pub const kCMMuxedStreamType_MPEG2Transport: CMMuxedStreamType = fourcc(b"mp2t");
pub const kCMMuxedStreamType_MPEG2Program: CMMuxedStreamType = fourcc(b"mp2p");
pub const kCMMuxedStreamType_DV: CMMuxedStreamType = fourcc(b"dv  ");
pub const kCMMuxedStreamType_EmbeddedDeviceScreenRecording: CMMuxedStreamType = fourcc(b"isr ");

extern "C" {
    pub fn CMMuxedFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        muxType: CMMuxedStreamType,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMMuxedFormatDescriptionRef,
    ) -> OSStatus;
}

pub type CMClosedCaptionFormatDescriptionRef = CMFormatDescriptionRef;

pub type CMClosedCaptionFormatType = FourCharCode;

pub const kCMClosedCaptionFormatType_CEA608: CMClosedCaptionFormatType = fourcc(b"c608");
pub const kCMClosedCaptionFormatType_CEA708: CMClosedCaptionFormatType = fourcc(b"c708");
pub const kCMClosedCaptionFormatType_ATSC: CMClosedCaptionFormatType = fourcc(b"atcc");

pub type CMTextFormatDescriptionRef = CMFormatDescriptionRef;

pub type CMTextFormatType = FourCharCode;

pub const kCMTextFormatType_QTText: CMTextFormatType = fourcc(b"text");
pub const kCMTextFormatType_3GText: CMTextFormatType = fourcc(b"tx3g");

pub type CMTextDisplayFlags = u32;

pub const kCMTextDisplayFlag_scrollIn: CMTextDisplayFlags = 0x00000020;
pub const kCMTextDisplayFlag_scrollOut: CMTextDisplayFlags = 0x00000040;
pub const kCMTextDisplayFlag_scrollDirectionMask: CMTextDisplayFlags = 0x00000180;
pub const kCMTextDisplayFlag_scrollDirection_bottomToTop: CMTextDisplayFlags = 0x00000000;
pub const kCMTextDisplayFlag_scrollDirection_rightToLeft: CMTextDisplayFlags = 0x00000080;
pub const kCMTextDisplayFlag_scrollDirection_topToBottom: CMTextDisplayFlags = 0x00000100;
pub const kCMTextDisplayFlag_scrollDirection_leftToRight: CMTextDisplayFlags = 0x00000180;
pub const kCMTextDisplayFlag_continuousKaraoke: CMTextDisplayFlags = 0x00000800;
pub const kCMTextDisplayFlag_writeTextVertically: CMTextDisplayFlags = 0x00020000;
pub const kCMTextDisplayFlag_fillTextRegion: CMTextDisplayFlags = 0x00040000;
pub const kCMTextDisplayFlag_obeySubtitleFormatting: CMTextDisplayFlags = 0x20000000;
pub const kCMTextDisplayFlag_forcedSubtitlesPresent: CMTextDisplayFlags = 0x40000000;
pub const kCMTextDisplayFlag_allSubtitlesForced: CMTextDisplayFlags = 0x80000000;

pub type CMTextJustificationValue = i8;

pub const kCMTextJustification_left_top: CMTextJustificationValue = 0;
pub const kCMTextJustification_centered: CMTextJustificationValue = 1;
pub const kCMTextJustification_bottom_right: CMTextJustificationValue = -1;

extern "C" {
    pub static kCMTextFormatDescriptionExtension_DisplayFlags: CFStringRef;
    pub static kCMTextFormatDescriptionExtension_BackgroundColor: CFStringRef;
    pub static kCMTextFormatDescriptionColor_Red: CFStringRef;
    pub static kCMTextFormatDescriptionColor_Green: CFStringRef;
    pub static kCMTextFormatDescriptionColor_Blue: CFStringRef;
    pub static kCMTextFormatDescriptionColor_Alpha: CFStringRef;
    pub static kCMTextFormatDescriptionExtension_DefaultTextBox: CFStringRef;
    pub static kCMTextFormatDescriptionRect_Top: CFStringRef;
    pub static kCMTextFormatDescriptionRect_Left: CFStringRef;
    pub static kCMTextFormatDescriptionRect_Bottom: CFStringRef;
    pub static kCMTextFormatDescriptionRect_Right: CFStringRef;
    pub static kCMTextFormatDescriptionExtension_DefaultStyle: CFStringRef;
    pub static kCMTextFormatDescriptionStyle_StartChar: CFStringRef;
    pub static kCMTextFormatDescriptionStyle_Font: CFStringRef;
    pub static kCMTextFormatDescriptionStyle_FontFace: CFStringRef;
    pub static kCMTextFormatDescriptionStyle_ForegroundColor: CFStringRef;
    pub static kCMTextFormatDescriptionStyle_FontSize: CFStringRef;
    pub static kCMTextFormatDescriptionExtension_HorizontalJustification: CFStringRef;
    pub static kCMTextFormatDescriptionExtension_VerticalJustification: CFStringRef;
    pub static kCMTextFormatDescriptionStyle_EndChar: CFStringRef;
    pub static kCMTextFormatDescriptionExtension_FontTable: CFStringRef;
    pub static kCMTextFormatDescriptionExtension_TextJustification: CFStringRef;
    pub static kCMTextFormatDescriptionStyle_Height: CFStringRef;
    pub static kCMTextFormatDescriptionStyle_Ascent: CFStringRef;
    pub static kCMTextFormatDescriptionExtension_DefaultFontName: CFStringRef;
    pub static kCMFormatDescriptionExtension_AmbientViewingEnvironment: CFStringRef;

    pub fn CMTextFormatDescriptionGetDisplayFlags(desc: CMFormatDescriptionRef, displayFlagsOut: *mut CMTextDisplayFlags) -> OSStatus;
    pub fn CMTextFormatDescriptionGetJustification(
        desc: CMFormatDescriptionRef,
        horizontaJustificationlOut: *mut CMTextJustificationValue,
        verticalJustificationOut: *mut CMTextJustificationValue,
    ) -> OSStatus;
    pub fn CMTextFormatDescriptionGetDefaultTextBox(
        desc: CMFormatDescriptionRef,
        originIsAtTopLeft: Boolean,
        heightOfTextTrack: CGFloat,
        defaultTextBoxOut: *mut CGRect,
    ) -> OSStatus;
    pub fn CMTextFormatDescriptionGetDefaultStyle(
        desc: CMFormatDescriptionRef,
        localFontIDOut: *mut u16,
        boldOut: *mut Boolean,
        italicOut: *mut Boolean,
        underlineOut: *mut Boolean,
        fontSizeOut: *mut CGFloat,
        colorComponentsOut: *mut [CGFloat; 4usize],
    ) -> OSStatus;
    pub fn CMTextFormatDescriptionGetFontName(desc: CMFormatDescriptionRef, localFontID: u16, fontNameOut: *mut CFStringRef) -> OSStatus;
}

pub type CMSubtitleFormatType = FourCharCode;

pub const kCMSubtitleFormatType_3GText: CMSubtitleFormatType = fourcc(b"tx3g");
pub const kCMSubtitleFormatType_WebVTT: CMSubtitleFormatType = fourcc(b"wvtt");

pub type CMTimeCodeFormatDescriptionRef = CMFormatDescriptionRef;

pub type CMTimeCodeFormatType = FourCharCode;

pub const kCMTimeCodeFormatType_TimeCode32: CMTimeCodeFormatType = fourcc(b"tmcd");
pub const kCMTimeCodeFormatType_TimeCode64: CMTimeCodeFormatType = fourcc(b"tc64");
pub const kCMTimeCodeFormatType_Counter32: CMTimeCodeFormatType = fourcc(b"cn32");
pub const kCMTimeCodeFormatType_Counter64: CMTimeCodeFormatType = fourcc(b"cn64");

pub const kCMTimeCodeFlag_DropFrame: u32 = 1 << 0;
pub const kCMTimeCodeFlag_24HourMax: u32 = 1 << 1;
pub const kCMTimeCodeFlag_NegTimesOK: u32 = 1 << 2;

extern "C" {
    pub fn CMTimeCodeFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        timeCodeFormatType: CMTimeCodeFormatType,
        frameDuration: CMTime,
        frameQuanta: u32,
        flags: u32,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMTimeCodeFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMTimeCodeFormatDescriptionGetFrameDuration(desc: CMTimeCodeFormatDescriptionRef) -> CMTime;
    pub fn CMTimeCodeFormatDescriptionGetFrameQuanta(desc: CMTimeCodeFormatDescriptionRef) -> u32;
    pub fn CMTimeCodeFormatDescriptionGetTimeCodeFlags(desc: CMTimeCodeFormatDescriptionRef) -> u32;

    pub static kCMTimeCodeFormatDescriptionExtension_SourceReferenceName: CFStringRef;
    pub static kCMTimeCodeFormatDescriptionKey_Value: CFStringRef;
    pub static kCMTimeCodeFormatDescriptionKey_LangCode: CFStringRef;
}

pub type CMMetadataFormatDescriptionRef = CMFormatDescriptionRef;

pub type CMMetadataFormatType = FourCharCode;

pub const kCMMetadataFormatType_ICY: CMMetadataFormatType = fourcc(b"icy ");
pub const kCMMetadataFormatType_ID3: CMMetadataFormatType = fourcc(b"id3 ");
pub const kCMMetadataFormatType_Boxed: CMMetadataFormatType = fourcc(b"mebx");
pub const kCMMetadataFormatType_EMSG: CMMetadataFormatType = fourcc(b"emsg");

extern "C" {
    pub static kCMFormatDescriptionExtensionKey_MetadataKeyTable: CFStringRef;
    pub static kCMMetadataFormatDescriptionKey_Namespace: CFStringRef;
    pub static kCMMetadataFormatDescriptionKey_Value: CFStringRef;
    pub static kCMMetadataFormatDescriptionKey_LocalID: CFStringRef;
    pub static kCMMetadataFormatDescriptionKey_DataType: CFStringRef;
    pub static kCMMetadataFormatDescriptionKey_DataTypeNamespace: CFStringRef;
    pub static kCMMetadataFormatDescriptionKey_ConformingDataTypes: CFStringRef;
    pub static kCMMetadataFormatDescriptionKey_LanguageTag: CFStringRef;
    pub static kCMMetadataFormatDescriptionKey_StructuralDependency: CFStringRef;
    pub static kCMMetadataFormatDescriptionKey_SetupData: CFStringRef;
    pub static kCMMetadataFormatDescription_StructuralDependencyKey_DependencyIsInvalidFlag: CFStringRef;
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_Identifier: CFStringRef;
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_DataType: CFStringRef;
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_ExtendedLanguageTag: CFStringRef;
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_StructuralDependency: CFStringRef;
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_SetupData: CFStringRef;

    pub fn CMMetadataFormatDescriptionCreateWithKeys(
        allocator: CFAllocatorRef,
        metadataType: CMMetadataFormatType,
        keys: CFArrayRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMMetadataFormatDescriptionCreateWithMetadataSpecifications(
        allocator: CFAllocatorRef,
        metadataType: CMMetadataFormatType,
        metadataSpecifications: CFArrayRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMMetadataFormatDescriptionCreateWithMetadataFormatDescriptionAndMetadataSpecifications(
        allocator: CFAllocatorRef,
        sourceDescription: CMMetadataFormatDescriptionRef,
        metadataSpecifications: CFArrayRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMMetadataFormatDescriptionCreateByMergingMetadataFormatDescriptions(
        allocator: CFAllocatorRef,
        sourceDescription: CMMetadataFormatDescriptionRef,
        otherSourceDescription: CMMetadataFormatDescriptionRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMMetadataFormatDescriptionGetKeyWithLocalID(desc: CMMetadataFormatDescriptionRef, localKeyID: OSType) -> CFDictionaryRef;
    pub fn CMMetadataFormatDescriptionGetIdentifiers(desc: CMMetadataFormatDescriptionRef) -> CFArrayRef;
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for opaqueCMFormatDescription {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("opaqueCMFormatDescription", &[]));
}

pub trait TCMFormatDescription: TCFType {
    #[inline]
    fn as_buffer(&self) -> CMFormatDescription {
        unsafe { CMFormatDescription::wrap_under_get_rule(self.as_concrete_TypeRef().as_void_ptr() as CMFormatDescriptionRef) }
    }

    #[inline]
    fn into_buffer(self) -> CMFormatDescription
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef().as_void_ptr() as CMFormatDescriptionRef;
        forget(self);
        unsafe { CMFormatDescription::wrap_under_create_rule(reference) }
    }
}

impl CMFormatDescription {
    #[inline]
    pub fn downcast<T: TCMFormatDescription>(&self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe { Some(T::wrap_under_get_rule(T::Ref::from_void_ptr(self.as_concrete_TypeRef() as *const c_void))) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_into<T: TCMFormatDescription>(self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let reference = T::Ref::from_void_ptr(self.as_concrete_TypeRef() as *const c_void);
                forget(self);
                Some(T::wrap_under_create_rule(reference))
            }
        } else {
            None
        }
    }
}

declare_TCFType!(CMFormatDescription, CMFormatDescriptionRef);
impl_TCFType!(CMFormatDescription, CMFormatDescriptionRef, CMFormatDescriptionGetTypeID);
impl_CFTypeDescription!(CMFormatDescription);

impl CMFormatDescription {
    #[inline]
    pub fn new(media_type: CMMediaType, media_subtype: FourCharCode, extensions: Option<&CFDictionary<CFString, CFType>>) -> Result<Self, OSStatus> {
        let mut format_description: CMFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMFormatDescriptionCreate(
                kCFAllocatorDefault,
                media_type,
                media_subtype,
                extensions.map_or(null(), |exts| exts.as_concrete_TypeRef()),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn equal(&self, other: &Self) -> bool {
        unsafe { CMFormatDescriptionEqual(self.as_concrete_TypeRef(), other.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn equal_ignoring_extension_keys(
        &self,
        other: &Self,
        extension_keys_to_ignore: &CFType,
        sample_description_extension_atom_keys_to_ignore: &CFType,
    ) -> bool {
        unsafe {
            CMFormatDescriptionEqualIgnoringExtensionKeys(
                self.as_concrete_TypeRef(),
                other.as_concrete_TypeRef(),
                extension_keys_to_ignore.as_concrete_TypeRef(),
                sample_description_extension_atom_keys_to_ignore.as_concrete_TypeRef(),
            ) != 0
        }
    }

    #[inline]
    pub fn get_media_type(&self) -> CMMediaType {
        unsafe { CMFormatDescriptionGetMediaType(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_media_subtype(&self) -> FourCharCode {
        unsafe { CMFormatDescriptionGetMediaSubType(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_extensions(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let extensions = CMFormatDescriptionGetExtensions(self.as_concrete_TypeRef());
            if extensions.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(extensions))
            }
        }
    }

    #[inline]
    pub fn get_extension(&self, extension_key: &CFString) -> Option<CFPropertyList> {
        unsafe {
            let extension = CMFormatDescriptionGetExtension(self.as_concrete_TypeRef(), extension_key.as_concrete_TypeRef());
            if extension.is_null() {
                None
            } else {
                Some(CFPropertyList::wrap_under_get_rule(extension))
            }
        }
    }
}

impl TCMFormatDescription for CMAudioFormatDescription {}

declare_TCFType!(CMAudioFormatDescription, CMAudioFormatDescriptionRef);
impl_TCFType!(CMAudioFormatDescription, CMAudioFormatDescriptionRef, CMFormatDescriptionGetTypeID);
impl_CFTypeDescription!(CMAudioFormatDescription);

impl CMAudioFormatDescription {
    #[inline]
    pub fn new(
        asbd: &AudioStreamBasicDescription,
        layout: &AudioChannelLayout,
        magic_cookie: &[u8],
        extensions: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<Self, OSStatus> {
        let mut format_description: CMAudioFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMAudioFormatDescriptionCreate(
                kCFAllocatorDefault,
                asbd,
                size_of_val(layout),
                layout,
                magic_cookie.len(),
                magic_cookie.as_ptr() as *const _,
                extensions.map_or(null(), |exts| exts.as_concrete_TypeRef()),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_codec_type(&self) -> CMAudioCodecType {
        unsafe { CMFormatDescriptionGetMediaSubType(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_stream_basic_description(&self) -> Option<&AudioStreamBasicDescription> {
        unsafe {
            let asbd = CMAudioFormatDescriptionGetStreamBasicDescription(self.as_concrete_TypeRef());
            if asbd.is_null() {
                None
            } else {
                Some(&*asbd)
            }
        }
    }

    #[inline]
    pub fn get_magic_cookie(&self) -> Option<&[u8]> {
        unsafe {
            let mut size = 0;
            let cookie = CMAudioFormatDescriptionGetMagicCookie(self.as_concrete_TypeRef(), &mut size);
            if cookie.is_null() {
                None
            } else {
                Some(from_raw_parts(cookie as *const u8, size))
            }
        }
    }

    #[inline]
    pub fn get_channel_layout(&self) -> Option<(&AudioChannelLayout, usize)> {
        unsafe {
            let mut size = 0;
            let layout = CMAudioFormatDescriptionGetChannelLayout(self.as_concrete_TypeRef(), &mut size);
            if layout.is_null() {
                None
            } else {
                Some((&*layout, size))
            }
        }
    }

    #[inline]
    pub fn get_format_list(&self) -> Option<&[AudioFormatListItem]> {
        unsafe {
            let mut size = 0;
            let list = CMAudioFormatDescriptionGetFormatList(self.as_concrete_TypeRef(), &mut size);
            if list.is_null() {
                None
            } else {
                Some(from_raw_parts(list, size))
            }
        }
    }

    #[inline]
    pub fn get_richest_decodable_format(&self) -> Option<&AudioFormatListItem> {
        unsafe {
            let format = CMAudioFormatDescriptionGetRichestDecodableFormat(self.as_concrete_TypeRef());
            if format.is_null() {
                None
            } else {
                Some(&*format)
            }
        }
    }

    #[inline]
    pub fn get_most_compatible_format(&self) -> Option<&AudioFormatListItem> {
        unsafe {
            let format = CMAudioFormatDescriptionGetMostCompatibleFormat(self.as_concrete_TypeRef());
            if format.is_null() {
                None
            } else {
                Some(&*format)
            }
        }
    }

    #[inline]
    pub fn new_summary(format_description_array: &CFArray<CMAudioFormatDescription>, flags: u32) -> Result<Self, OSStatus> {
        let mut format_description: CMAudioFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMAudioFormatDescriptionCreateSummary(kCFAllocatorDefault, format_description_array.as_concrete_TypeRef(), flags, &mut format_description)
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn equal(&self, other: &Self, eequality_mask: CMAudioFormatDescriptionMask) -> (bool, CMAudioFormatDescriptionMask) {
        let mut mask = 0;
        let equal = unsafe { CMAudioFormatDescriptionEqual(self.as_concrete_TypeRef(), other.as_concrete_TypeRef(), eequality_mask, &mut mask) != 0 };
        (equal, mask)
    }
}

impl TCMFormatDescription for CMVideoFormatDescription {}

declare_TCFType!(CMVideoFormatDescription, CMVideoFormatDescriptionRef);
impl_TCFType!(CMVideoFormatDescription, CMVideoFormatDescriptionRef, CMFormatDescriptionGetTypeID);
impl_CFTypeDescription!(CMVideoFormatDescription);

impl CMVideoFormatDescription {
    #[inline]
    pub fn new(codec_type: CMVideoCodecType, width: i32, height: i32, extensions: Option<&CFDictionary<CFString, CFType>>) -> Result<Self, OSStatus> {
        let mut format_description: CMVideoFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMVideoFormatDescriptionCreate(
                kCFAllocatorDefault,
                codec_type,
                width,
                height,
                extensions.as_ref().map_or(null(), |exts| exts.as_concrete_TypeRef()),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn from_image_buffer(image_buffer: &CVImageBuffer) -> Result<Self, OSStatus> {
        let mut format_description: CMVideoFormatDescriptionRef = null_mut();
        let status =
            unsafe { CMVideoFormatDescriptionCreateForImageBuffer(kCFAllocatorDefault, image_buffer.as_concrete_TypeRef(), &mut format_description) };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn from_h264_parameter_sets(parameter_sets: &[&[u8]], nal_unit_header_length: i32) -> Result<Self, OSStatus> {
        let mut format_description: CMVideoFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMVideoFormatDescriptionCreateFromH264ParameterSets(
                kCFAllocatorDefault,
                parameter_sets.len(),
                parameter_sets.iter().map(|data| data.as_ptr()).collect::<Vec<_>>().as_ptr(),
                parameter_sets.iter().map(|data| data.len()).collect::<Vec<_>>().as_ptr(),
                nal_unit_header_length,
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn from_hevc_parameter_sets(
        parameter_sets: &[&[u8]],
        nal_unit_header_length: i32,
        extensions: &CFDictionary<CFString, CFType>,
    ) -> Result<Self, OSStatus> {
        let mut format_description: CMVideoFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMVideoFormatDescriptionCreateFromHEVCParameterSets(
                kCFAllocatorDefault,
                parameter_sets.len(),
                parameter_sets.iter().map(|data| data.as_ptr()).collect::<Vec<_>>().as_ptr(),
                parameter_sets.iter().map(|data| data.len()).collect::<Vec<_>>().as_ptr(),
                nal_unit_header_length,
                extensions.as_concrete_TypeRef(),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_h264_parameter_set_at_index(&self, parameter_set_index: usize) -> Result<(&[u8], usize, i32), OSStatus> {
        let mut parameter_set_pointer = null();
        let mut parameter_set_size = 0;
        let mut parameter_set_count = 0;
        let mut nal_unit_header_length = 0;
        let status = unsafe {
            CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
                self.as_concrete_TypeRef(),
                parameter_set_index,
                &mut parameter_set_pointer,
                &mut parameter_set_size,
                &mut parameter_set_count,
                &mut nal_unit_header_length,
            )
        };
        if status == 0 {
            Ok((unsafe { from_raw_parts(parameter_set_pointer, parameter_set_size) }, parameter_set_count, nal_unit_header_length))
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_hevc_parameter_set_at_index(&self, parameter_set_index: usize) -> Result<(&[u8], usize, i32), OSStatus> {
        let mut parameter_set_pointer = null();
        let mut parameter_set_size = 0;
        let mut parameter_set_count = 0;
        let mut nal_unit_header_length = 0;
        let status = unsafe {
            CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
                self.as_concrete_TypeRef(),
                parameter_set_index,
                &mut parameter_set_pointer,
                &mut parameter_set_size,
                &mut parameter_set_count,
                &mut nal_unit_header_length,
            )
        };
        if status == 0 {
            Ok((unsafe { from_raw_parts(parameter_set_pointer, parameter_set_size) }, parameter_set_count, nal_unit_header_length))
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_codec_type(&self) -> CMVideoCodecType {
        unsafe { CMFormatDescriptionGetMediaSubType(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_dimensions(&self) -> CMVideoDimensions {
        unsafe { CMVideoFormatDescriptionGetDimensions(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_presentation_dimensions(&self, use_pixel_aspect_ratio: bool, use_clean_aperture: bool) -> CGSize {
        unsafe { CMVideoFormatDescriptionGetPresentationDimensions(self.as_concrete_TypeRef(), use_pixel_aspect_ratio as _, use_clean_aperture as _) }
    }

    #[inline]
    pub fn get_clean_aperture(&self, origin_is_at_top_left: bool) -> CGRect {
        unsafe { CMVideoFormatDescriptionGetCleanAperture(self.as_concrete_TypeRef(), origin_is_at_top_left as _) }
    }

    #[inline]
    pub fn get_extension_keys_common_with_image_buffers() -> CFArray<CFString> {
        unsafe { TCFType::wrap_under_create_rule(CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers()) }
    }

    #[inline]
    pub fn matches_image_buffer(&self, image_buffer: &CVImageBuffer) -> bool {
        unsafe { CMVideoFormatDescriptionMatchesImageBuffer(self.as_concrete_TypeRef(), image_buffer.as_concrete_TypeRef()) != 0 }
    }
}

impl TCMFormatDescription for CMMuxedFormatDescription {}

declare_TCFType!(CMMuxedFormatDescription, CMMuxedFormatDescriptionRef);
impl_TCFType!(CMMuxedFormatDescription, CMMuxedFormatDescriptionRef, CMFormatDescriptionGetTypeID);
impl_CFTypeDescription!(CMMuxedFormatDescription);

impl CMMuxedFormatDescription {
    #[inline]
    pub fn new(mux_type: CMMuxedStreamType, extensions: &CFDictionary<CFString, CFType>) -> Result<Self, OSStatus> {
        let mut format_description: CMMuxedFormatDescriptionRef = null_mut();
        let status =
            unsafe { CMMuxedFormatDescriptionCreate(kCFAllocatorDefault, mux_type, extensions.as_concrete_TypeRef(), &mut format_description) };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_stream_type(&self) -> CMMuxedStreamType {
        unsafe { CMFormatDescriptionGetMediaSubType(self.as_concrete_TypeRef()) }
    }
}

impl TCMFormatDescription for CMClosedCaptionFormatDescription {}

declare_TCFType!(CMClosedCaptionFormatDescription, CMClosedCaptionFormatDescriptionRef);
impl_TCFType!(CMClosedCaptionFormatDescription, CMClosedCaptionFormatDescriptionRef, CMFormatDescriptionGetTypeID);
impl_CFTypeDescription!(CMClosedCaptionFormatDescription);

impl CMClosedCaptionFormatDescription {
    #[inline]
    pub fn new(format_type: CMClosedCaptionFormatType, extensions: Option<&CFDictionary<CFString, CFType>>) -> Result<Self, OSStatus> {
        let mut format_description: CMClosedCaptionFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMFormatDescriptionCreate(
                kCFAllocatorDefault,
                kCMMediaType_ClosedCaption,
                format_type,
                extensions.map_or(null(), |exts| exts.as_concrete_TypeRef()),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_format_type(&self) -> CMClosedCaptionFormatType {
        unsafe { CMFormatDescriptionGetMediaSubType(self.as_concrete_TypeRef()) }
    }
}

impl TCMFormatDescription for CMTextFormatDescription {}

declare_TCFType!(CMTextFormatDescription, CMTextFormatDescriptionRef);
impl_TCFType!(CMTextFormatDescription, CMTextFormatDescriptionRef, CMFormatDescriptionGetTypeID);
impl_CFTypeDescription!(CMTextFormatDescription);

impl CMTextFormatDescription {
    #[inline]
    pub fn new(format_type: CMTextFormatType, extensions: Option<&CFDictionary<CFString, CFType>>) -> Result<Self, OSStatus> {
        let mut format_description: CMTextFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMFormatDescriptionCreate(
                kCFAllocatorDefault,
                kCMMediaType_Text,
                format_type,
                extensions.map_or(null(), |exts| exts.as_concrete_TypeRef()),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_format_type(&self) -> CMTextFormatType {
        unsafe { CMFormatDescriptionGetMediaSubType(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_display_flags(&self) -> Result<CMTextDisplayFlags, OSStatus> {
        let mut display_flags = 0;
        let status = unsafe { CMTextFormatDescriptionGetDisplayFlags(self.as_concrete_TypeRef(), &mut display_flags) };
        if status == 0 {
            Ok(display_flags)
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_justification(&self) -> Result<(CMTextJustificationValue, CMTextJustificationValue), OSStatus> {
        let mut horizontal_justification = 0;
        let mut vertical_justification = 0;
        let status = unsafe {
            CMTextFormatDescriptionGetJustification(self.as_concrete_TypeRef(), &mut horizontal_justification, &mut vertical_justification)
        };
        if status == 0 {
            Ok((horizontal_justification, vertical_justification))
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_default_text_box(&self, origin_is_at_top_left: bool, height_of_text_track: CGFloat) -> Result<CGRect, OSStatus> {
        let mut default_text_box = CGRect::default();
        let status = unsafe {
            CMTextFormatDescriptionGetDefaultTextBox(
                self.as_concrete_TypeRef(),
                origin_is_at_top_left as _,
                height_of_text_track,
                &mut default_text_box,
            )
        };
        if status == 0 {
            Ok(default_text_box)
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_default_style(&self) -> Result<(u16, bool, bool, bool, CGFloat, [CGFloat; 4]), OSStatus> {
        let mut local_font_id = 0;
        let mut bold: Boolean = 0;
        let mut italic: Boolean = 0;
        let mut underline: Boolean = 0;
        let mut font_size = 0.0;
        let mut color_components = [0.0; 4];
        let status = unsafe {
            CMTextFormatDescriptionGetDefaultStyle(
                self.as_concrete_TypeRef(),
                &mut local_font_id,
                &mut bold,
                &mut italic,
                &mut underline,
                &mut font_size,
                &mut color_components,
            )
        };
        if status == 0 {
            Ok((local_font_id, bold != 0, italic != 0, underline != 0, font_size, color_components))
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_font_name(&self, local_font_id: u16) -> Result<CFString, OSStatus> {
        let mut font_name = null();
        let status = unsafe { CMTextFormatDescriptionGetFontName(self.as_concrete_TypeRef(), local_font_id, &mut font_name) };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(font_name) })
        } else {
            Err(status)
        }
    }
}

impl TCMFormatDescription for CMTimeCodeFormatDescription {}

declare_TCFType!(CMTimeCodeFormatDescription, CMTimeCodeFormatDescriptionRef);
impl_TCFType!(CMTimeCodeFormatDescription, CMTimeCodeFormatDescriptionRef, CMFormatDescriptionGetTypeID);
impl_CFTypeDescription!(CMTimeCodeFormatDescription);

impl CMTimeCodeFormatDescription {
    #[inline]
    pub fn new(
        time_code_format_type: CMTimeCodeFormatType,
        frame_duration: CMTime,
        frame_quanta: u32,
        flags: u32,
        extensions: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<Self, OSStatus> {
        let mut format_description: CMTimeCodeFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMTimeCodeFormatDescriptionCreate(
                kCFAllocatorDefault,
                time_code_format_type,
                frame_duration,
                frame_quanta,
                flags,
                extensions.map_or(null(), |exts| exts.as_concrete_TypeRef()),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_format_type(&self) -> CMTimeCodeFormatType {
        unsafe { CMFormatDescriptionGetMediaSubType(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_frame_duration(&self) -> CMTime {
        unsafe { CMTimeCodeFormatDescriptionGetFrameDuration(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_frame_quanta(&self) -> u32 {
        unsafe { CMTimeCodeFormatDescriptionGetFrameQuanta(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_time_code_flags(&self) -> u32 {
        unsafe { CMTimeCodeFormatDescriptionGetTimeCodeFlags(self.as_concrete_TypeRef()) }
    }
}

impl TCMFormatDescription for CMMetadataFormatDescription {}

declare_TCFType!(CMMetadataFormatDescription, CMMetadataFormatDescriptionRef);
impl_TCFType!(CMMetadataFormatDescription, CMMetadataFormatDescriptionRef, CMFormatDescriptionGetTypeID);
impl_CFTypeDescription!(CMMetadataFormatDescription);

impl CMMetadataFormatDescription {
    #[inline]
    pub fn new_with_keys(metadata_type: CMMetadataFormatType, keys: Option<&CFArray<CFString>>) -> Result<Self, OSStatus> {
        let mut format_description: CMMetadataFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMMetadataFormatDescriptionCreateWithKeys(
                kCFAllocatorDefault,
                metadata_type,
                keys.map_or(null(), |keys| keys.as_concrete_TypeRef()),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn new_with_metadata_specifications(
        metadata_type: CMMetadataFormatType,
        metadata_specifications: &CFArray<CFDictionary<CFString, CFType>>,
    ) -> Result<Self, OSStatus> {
        let mut format_description: CMMetadataFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMMetadataFormatDescriptionCreateWithMetadataSpecifications(
                kCFAllocatorDefault,
                metadata_type,
                metadata_specifications.as_concrete_TypeRef(),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn new_with_metadata_format_description_and_metadata_specifications(
        source_description: &CMMetadataFormatDescription,
        metadata_specifications: &CFArray<CFDictionary<CFString, CFType>>,
    ) -> Result<Self, OSStatus> {
        let mut format_description: CMMetadataFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMMetadataFormatDescriptionCreateWithMetadataFormatDescriptionAndMetadataSpecifications(
                kCFAllocatorDefault,
                source_description.as_concrete_TypeRef(),
                metadata_specifications.as_concrete_TypeRef(),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn new_by_merging_metadata_format_descriptions(
        source_description: &CMMetadataFormatDescription,
        other_source_description: &CMMetadataFormatDescription,
    ) -> Result<Self, OSStatus> {
        let mut format_description: CMMetadataFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMMetadataFormatDescriptionCreateByMergingMetadataFormatDescriptions(
                kCFAllocatorDefault,
                source_description.as_concrete_TypeRef(),
                other_source_description.as_concrete_TypeRef(),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { TCFType::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_key_with_local_id(&self, local_key_id: OSType) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let key = CMMetadataFormatDescriptionGetKeyWithLocalID(self.as_concrete_TypeRef(), local_key_id);
            if key.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(key))
            }
        }
    }

    #[inline]
    pub fn get_identifiers(&self) -> Option<CFArray<CFString>> {
        unsafe {
            let identifiers = CMMetadataFormatDescriptionGetIdentifiers(self.as_concrete_TypeRef());
            if identifiers.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(identifiers))
            }
        }
    }
}
