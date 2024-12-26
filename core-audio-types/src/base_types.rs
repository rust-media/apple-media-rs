use core_foundation_sys::base::OSStatus;
use libc::{c_long, c_void};

use crate::OSType;

#[inline]
const fn fourcc(code: &[u8; 4]) -> u32 {
    ((code[0] as u32) << 24) | ((code[1] as u32) << 16) | ((code[2] as u32) << 8) | (code[3] as u32)
}

pub const kAudio_NoError: OSStatus = 0;
pub const kAudio_UnimplementedError: OSStatus = -4;
pub const kAudio_FileNotFoundError: OSStatus = -43;
pub const kAudio_FilePermissionError: OSStatus = -54;
pub const kAudio_TooManyFilesOpenError: OSStatus = -42;
pub const kAudio_BadFilePathError: OSStatus = fourcc(b"!pth") as OSStatus;
pub const kAudio_ParamError: OSStatus = -50;
pub const kAudio_MemFullError: OSStatus = -108;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioValueRange {
    pub mMinimum: f64,
    pub mMaximum: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioValueTranslation {
    pub mInputData: *const c_void,
    pub mInputDataSize: u32,
    pub mOutputData: *mut c_void,
    pub mOutputDataSize: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioBuffer {
    pub mNumberChannels: u32,
    pub mDataByteSize: u32,
    pub mData: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioBufferList {
    pub mNumberBuffers: u32,
    pub mBuffers: [AudioBuffer; 1usize],
}

cfg_if! {
    if #[cfg(not(feature = "prefer-fixed-point"))] {
        pub type AudioSampleType = f32;
        pub type AudioUnitSampleType = f32;
    } else {
        pub type AudioSampleType = i16;
        pub type AudioUnitSampleType = i32;
        pub const kAudioUnitSampleFractionBits: u32 = 24;
    }
}

pub type AudioFormatID = u32;

pub type AudioFormatFlags = u32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioStreamBasicDescription {
    pub mSampleRate: f64,
    pub mFormatID: AudioFormatID,
    pub mFormatFlags: AudioFormatFlags,
    pub mBytesPerPacket: u32,
    pub mFramesPerPacket: u32,
    pub mBytesPerFrame: u32,
    pub mChannelsPerFrame: u32,
    pub mBitsPerChannel: u32,
    pub mReserved: u32,
}

pub static kAudioStreamAnyRate: f64 = 0.0;

pub const kAudioFormatLinearPCM: AudioFormatID = fourcc(b"lpcm");
pub const kAudioFormatAC3: AudioFormatID = fourcc(b"ac-3");
pub const kAudioFormat60958AC3: AudioFormatID = fourcc(b"cac3");
pub const kAudioFormatAppleIMA4: AudioFormatID = fourcc(b"ima4");
pub const kAudioFormatMPEG4AAC: AudioFormatID = fourcc(b"aac ");
pub const kAudioFormatMPEG4CELP: AudioFormatID = fourcc(b"celp");
pub const kAudioFormatMPEG4HVXC: AudioFormatID = fourcc(b"hvxc");
pub const kAudioFormatMPEG4TwinVQ: AudioFormatID = fourcc(b"twvq");
pub const kAudioFormatMACE3: AudioFormatID = fourcc(b"MAC3");
pub const kAudioFormatMACE6: AudioFormatID = fourcc(b"MAC6");
pub const kAudioFormatULaw: AudioFormatID = fourcc(b"ulaw");
pub const kAudioFormatALaw: AudioFormatID = fourcc(b"alaw");
pub const kAudioFormatQDesign: AudioFormatID = fourcc(b"QDMC");
pub const kAudioFormatQDesign2: AudioFormatID = fourcc(b"QDM2");
pub const kAudioFormatQUALCOMM: AudioFormatID = fourcc(b"Qclp");
pub const kAudioFormatMPEGLayer1: AudioFormatID = fourcc(b".mp1");
pub const kAudioFormatMPEGLayer2: AudioFormatID = fourcc(b".mp2");
pub const kAudioFormatMPEGLayer3: AudioFormatID = fourcc(b".mp3");
pub const kAudioFormatTimeCode: AudioFormatID = fourcc(b"time");
pub const kAudioFormatMIDIStream: AudioFormatID = fourcc(b"midi");
pub const kAudioFormatParameterValueStream: AudioFormatID = fourcc(b"apvs");
pub const kAudioFormatAppleLossless: AudioFormatID = fourcc(b"alac");
pub const kAudioFormatMPEG4AAC_HE: AudioFormatID = fourcc(b"aach");
pub const kAudioFormatMPEG4AAC_LD: AudioFormatID = fourcc(b"aacl");
pub const kAudioFormatMPEG4AAC_ELD: AudioFormatID = fourcc(b"aace");
pub const kAudioFormatMPEG4AAC_ELD_SBR: AudioFormatID = fourcc(b"aacf");
pub const kAudioFormatMPEG4AAC_ELD_V2: AudioFormatID = fourcc(b"aacg");
pub const kAudioFormatMPEG4AAC_HE_V2: AudioFormatID = fourcc(b"aacp");
pub const kAudioFormatMPEG4AAC_Spatial: AudioFormatID = fourcc(b"aacs");
pub const kAudioFormatMPEGD_USAC: AudioFormatID = fourcc(b"usac");
pub const kAudioFormatAMR: AudioFormatID = fourcc(b"samr");
pub const kAudioFormatAMR_WB: AudioFormatID = fourcc(b"sawb");
pub const kAudioFormatAudible: AudioFormatID = fourcc(b"AUDB");
pub const kAudioFormatiLBC: AudioFormatID = fourcc(b"ilbc");
pub const kAudioFormatDVIIntelIMA: AudioFormatID = 0x6D730011;
pub const kAudioFormatMicrosoftGSM: AudioFormatID = 0x6D730031;
pub const kAudioFormatAES3: AudioFormatID = fourcc(b"aes3");
pub const kAudioFormatEnhancedAC3: AudioFormatID = fourcc(b"ec-3");
pub const kAudioFormatFLAC: AudioFormatID = fourcc(b"flac");
pub const kAudioFormatOpus: AudioFormatID = fourcc(b"opus");

pub const kAudioFormatFlagIsFloat: AudioFormatFlags = 1 << 0;
pub const kAudioFormatFlagIsBigEndian: AudioFormatFlags = 1 << 1;
pub const kAudioFormatFlagIsSignedInteger: AudioFormatFlags = 1 << 2;
pub const kAudioFormatFlagIsPacked: AudioFormatFlags = 1 << 3;
pub const kAudioFormatFlagIsAlignedHigh: AudioFormatFlags = 1 << 4;
pub const kAudioFormatFlagIsNonInterleaved: AudioFormatFlags = 1 << 5;
pub const kAudioFormatFlagIsNonMixable: AudioFormatFlags = 1 << 6;
pub const kAudioFormatFlagsAreAllClear: AudioFormatFlags = 0x80000000;

pub const kLinearPCMFormatFlagIsFloat: AudioFormatFlags = kAudioFormatFlagIsFloat;
pub const kLinearPCMFormatFlagIsBigEndian: AudioFormatFlags = kAudioFormatFlagIsBigEndian;
pub const kLinearPCMFormatFlagIsSignedInteger: AudioFormatFlags = kAudioFormatFlagIsSignedInteger;
pub const kLinearPCMFormatFlagIsPacked: AudioFormatFlags = kAudioFormatFlagIsPacked;
pub const kLinearPCMFormatFlagIsAlignedHigh: AudioFormatFlags = kAudioFormatFlagIsAlignedHigh;
pub const kLinearPCMFormatFlagIsNonInterleaved: AudioFormatFlags = kAudioFormatFlagIsNonInterleaved;
pub const kLinearPCMFormatFlagIsNonMixable: AudioFormatFlags = kAudioFormatFlagIsNonMixable;
pub const kLinearPCMFormatFlagsSampleFractionShift: AudioFormatFlags = 7;
pub const kLinearPCMFormatFlagsSampleFractionMask: AudioFormatFlags = 0x3F << kLinearPCMFormatFlagsSampleFractionShift;
pub const kLinearPCMFormatFlagsAreAllClear: AudioFormatFlags = kAudioFormatFlagsAreAllClear;

pub const kAppleLosslessFormatFlag_16BitSourceData: AudioFormatFlags = 1;
pub const kAppleLosslessFormatFlag_20BitSourceData: AudioFormatFlags = 2;
pub const kAppleLosslessFormatFlag_24BitSourceData: AudioFormatFlags = 3;
pub const kAppleLosslessFormatFlag_32BitSourceData: AudioFormatFlags = 4;

cfg_if! {
    if #[cfg(target_endian = "big")] {
        pub const kAudioFormatFlagsNativeEndian: AudioFormatFlags = kAudioFormatFlagIsBigEndian;
    } else {
        pub const kAudioFormatFlagsNativeEndian: AudioFormatFlags = 0;
    }
}

cfg_if! {
    if #[cfg(not(feature = "prefer-fixed-point"))] {
        pub const kAudioFormatFlagsCanonical: AudioFormatFlags =
            kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;
        pub const kAudioFormatFlagsAudioUnitCanonical: AudioFormatFlags = kAudioFormatFlagIsFloat
            | kAudioFormatFlagsNativeEndian
            | kAudioFormatFlagIsPacked
            | kAudioFormatFlagIsNonInterleaved;
    } else {
        pub const kAudioFormatFlagsCanonical: AudioFormatFlags =
            kAudioFormatFlagIsSignedInteger | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;
        pub const kAudioFormatFlagsAudioUnitCanonical: AudioFormatFlags = kAudioFormatFlagIsSignedInteger
            | kAudioFormatFlagsNativeEndian
            | kAudioFormatFlagIsPacked
            | kAudioFormatFlagIsNonInterleaved
            | (kAudioUnitSampleFractionBits << kLinearPCMFormatFlagsSampleFractionShift);
    }
}

pub const kAudioFormatFlagsNativeFloatPacked: AudioFormatFlags = kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;

#[inline]
pub fn IsAudioFormatNativeEndian(f: &AudioStreamBasicDescription) -> bool {
    (f.mFormatID == kAudioFormatLinearPCM) && ((f.mFormatFlags & kAudioFormatFlagIsBigEndian) == kAudioFormatFlagsNativeEndian)
}

#[inline]
pub fn CalculateLPCMFlags(
    inValidBitsPerChannel: u32,
    inTotalBitsPerChannel: u32,
    inIsFloat: bool,
    inIsBigEndian: bool,
    inIsNonInterleaved: bool,
) -> AudioFormatFlags {
    (if inIsFloat {
        kAudioFormatFlagIsFloat
    } else {
        kAudioFormatFlagIsSignedInteger
    }) | (if inIsBigEndian {
        kAudioFormatFlagIsBigEndian
    } else {
        0
    }) | (if inValidBitsPerChannel == inTotalBitsPerChannel {
        kAudioFormatFlagIsPacked
    } else {
        kAudioFormatFlagIsAlignedHigh
    }) | (if inIsNonInterleaved {
        kAudioFormatFlagIsNonInterleaved
    } else {
        0
    })
}

#[allow(clippy::too_many_arguments)]
#[inline]
pub fn FillOutASBDForLPCM(
    outASBD: &mut AudioStreamBasicDescription,
    inSampleRate: f64,
    inChannelsPerFrame: u32,
    inValidBitsPerChannel: u32,
    inTotalBitsPerChannel: u32,
    inIsFloat: bool,
    inIsBigEndian: bool,
    inIsNonInterleaved: bool,
) {
    outASBD.mSampleRate = inSampleRate;
    outASBD.mFormatID = kAudioFormatLinearPCM;
    outASBD.mFormatFlags = CalculateLPCMFlags(inValidBitsPerChannel, inTotalBitsPerChannel, inIsFloat, inIsBigEndian, inIsNonInterleaved);
    outASBD.mBytesPerPacket = (if inIsNonInterleaved {
        1
    } else {
        inChannelsPerFrame
    }) * (inTotalBitsPerChannel / 8);
    outASBD.mFramesPerPacket = 1;
    outASBD.mBytesPerFrame = (if inIsNonInterleaved {
        1
    } else {
        inChannelsPerFrame
    }) * (inTotalBitsPerChannel / 8);
    outASBD.mChannelsPerFrame = inChannelsPerFrame;
    outASBD.mBitsPerChannel = inValidBitsPerChannel;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioStreamPacketDescription {
    pub mStartOffset: i64,
    pub mVariableFramesInPacket: u32,
    pub mDataByteSize: u32,
}

pub type SMPTETimeType = u32;

pub const kSMPTETimeType24: SMPTETimeType = 0;
pub const kSMPTETimeType25: SMPTETimeType = 1;
pub const kSMPTETimeType30Drop: SMPTETimeType = 2;
pub const kSMPTETimeType30: SMPTETimeType = 3;
pub const kSMPTETimeType2997: SMPTETimeType = 4;
pub const kSMPTETimeType2997Drop: SMPTETimeType = 5;
pub const kSMPTETimeType60: SMPTETimeType = 6;
pub const kSMPTETimeType5994: SMPTETimeType = 7;
pub const kSMPTETimeType60Drop: SMPTETimeType = 8;
pub const kSMPTETimeType5994Drop: SMPTETimeType = 9;
pub const kSMPTETimeType50: SMPTETimeType = 10;
pub const kSMPTETimeType2398: SMPTETimeType = 11;

pub type SMPTETimeFlags = u32;

pub const kSMPTETimeUnknown: SMPTETimeFlags = 0;
pub const kSMPTETimeValid: SMPTETimeFlags = 1 << 0;
pub const kSMPTETimeRunning: SMPTETimeFlags = 1 << 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SMPTETime {
    pub mSubframes: i16,
    pub mSubframeDivisor: i16,
    pub mCounter: u32,
    pub mType: SMPTETimeType,
    pub mFlags: SMPTETimeFlags,
    pub mHours: i16,
    pub mMinutes: i16,
    pub mSeconds: i16,
    pub mFrames: i16,
}

pub type AudioTimeStampFlags = u32;

pub const kAudioTimeStampNothingValid: AudioTimeStampFlags = 0;
pub const kAudioTimeStampSampleTimeValid: AudioTimeStampFlags = 1 << 0;
pub const kAudioTimeStampHostTimeValid: AudioTimeStampFlags = 1 << 1;
pub const kAudioTimeStampRateScalarValid: AudioTimeStampFlags = 1 << 2;
pub const kAudioTimeStampWordClockTimeValid: AudioTimeStampFlags = 1 << 3;
pub const kAudioTimeStampSMPTETimeValid: AudioTimeStampFlags = 1 << 4;
pub const kAudioTimeStampSampleHostTimeValid: AudioTimeStampFlags = kAudioTimeStampSampleTimeValid | kAudioTimeStampHostTimeValid;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioTimeStamp {
    pub mSampleTime: f64,
    pub mHostTime: u64,
    pub mRateScalar: f64,
    pub mWordClockTime: u64,
    pub mSMPTETime: SMPTETime,
    pub mFlags: AudioTimeStampFlags,
    pub mReserved: u32,
}

#[inline]
pub fn FillOutAudioTimeStampWithSampleTime(outATS: &mut AudioTimeStamp, inSampleTime: f64) {
    outATS.mSampleTime = inSampleTime;
    outATS.mHostTime = 0;
    outATS.mRateScalar = 0.0;
    outATS.mWordClockTime = 0;
    outATS.mSMPTETime = SMPTETime {
        mSubframes: 0,
        mSubframeDivisor: 0,
        mCounter: 0,
        mType: kSMPTETimeType24,
        mFlags: kSMPTETimeUnknown,
        mHours: 0,
        mMinutes: 0,
        mSeconds: 0,
        mFrames: 0,
    };
    outATS.mFlags = kAudioTimeStampSampleTimeValid;
}

#[inline]
pub fn FillOutAudioTimeStampWithHostTime(outATS: &mut AudioTimeStamp, inHostTime: u64) {
    outATS.mSampleTime = 0.0;
    outATS.mHostTime = inHostTime;
    outATS.mRateScalar = 0.0;
    outATS.mWordClockTime = 0;
    outATS.mSMPTETime = SMPTETime {
        mSubframes: 0,
        mSubframeDivisor: 0,
        mCounter: 0,
        mType: kSMPTETimeType24,
        mFlags: kSMPTETimeUnknown,
        mHours: 0,
        mMinutes: 0,
        mSeconds: 0,
        mFrames: 0,
    };
    outATS.mFlags = kAudioTimeStampHostTimeValid;
}

#[inline]
pub fn FillOutAudioTimeStampWithSampleAndHostTime(outATS: &mut AudioTimeStamp, inSampleTime: f64, inHostTime: u64) {
    outATS.mSampleTime = inSampleTime;
    outATS.mHostTime = inHostTime;
    outATS.mRateScalar = 0.0;
    outATS.mWordClockTime = 0;
    outATS.mSMPTETime = SMPTETime {
        mSubframes: 0,
        mSubframeDivisor: 0,
        mCounter: 0,
        mType: kSMPTETimeType24,
        mFlags: kSMPTETimeUnknown,
        mHours: 0,
        mMinutes: 0,
        mSeconds: 0,
        mFrames: 0,
    };
    outATS.mFlags = kAudioTimeStampSampleTimeValid | kAudioTimeStampHostTimeValid;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioClassDescription {
    pub mType: OSType,
    pub mSubType: OSType,
    pub mManufacturer: OSType,
}

pub type AudioChannelLabel = u32;

pub const kAudioChannelLabel_Unknown: AudioChannelLabel = 0xFFFFFFFF;
pub const kAudioChannelLabel_Unused: AudioChannelLabel = 0;
pub const kAudioChannelLabel_UseCoordinates: AudioChannelLabel = 100;
pub const kAudioChannelLabel_Left: AudioChannelLabel = 1;
pub const kAudioChannelLabel_Right: AudioChannelLabel = 2;
pub const kAudioChannelLabel_Center: AudioChannelLabel = 3;
pub const kAudioChannelLabel_LFEScreen: AudioChannelLabel = 4;
pub const kAudioChannelLabel_LeftSurround: AudioChannelLabel = 5;
pub const kAudioChannelLabel_RightSurround: AudioChannelLabel = 6;
pub const kAudioChannelLabel_LeftCenter: AudioChannelLabel = 7;
pub const kAudioChannelLabel_RightCenter: AudioChannelLabel = 8;
pub const kAudioChannelLabel_CenterSurround: AudioChannelLabel = 9;
pub const kAudioChannelLabel_LeftSurroundDirect: AudioChannelLabel = 10;
pub const kAudioChannelLabel_RightSurroundDirect: AudioChannelLabel = 11;
pub const kAudioChannelLabel_TopCenterSurround: AudioChannelLabel = 12;
pub const kAudioChannelLabel_VerticalHeightLeft: AudioChannelLabel = 13;
pub const kAudioChannelLabel_VerticalHeightCenter: AudioChannelLabel = 14;
pub const kAudioChannelLabel_VerticalHeightRight: AudioChannelLabel = 15;
pub const kAudioChannelLabel_TopBackLeft: AudioChannelLabel = 16;
pub const kAudioChannelLabel_TopBackCenter: AudioChannelLabel = 17;
pub const kAudioChannelLabel_TopBackRight: AudioChannelLabel = 18;
pub const kAudioChannelLabel_RearSurroundLeft: AudioChannelLabel = 33;
pub const kAudioChannelLabel_RearSurroundRight: AudioChannelLabel = 34;
pub const kAudioChannelLabel_LeftWide: AudioChannelLabel = 35;
pub const kAudioChannelLabel_RightWide: AudioChannelLabel = 36;
pub const kAudioChannelLabel_LFE2: AudioChannelLabel = 37;
pub const kAudioChannelLabel_LeftTotal: AudioChannelLabel = 38;
pub const kAudioChannelLabel_RightTotal: AudioChannelLabel = 39;
pub const kAudioChannelLabel_HearingImpaired: AudioChannelLabel = 40;
pub const kAudioChannelLabel_Narration: AudioChannelLabel = 41;
pub const kAudioChannelLabel_Mono: AudioChannelLabel = 42;
pub const kAudioChannelLabel_DialogCentricMix: AudioChannelLabel = 43;
pub const kAudioChannelLabel_CenterSurroundDirect: AudioChannelLabel = 44;
pub const kAudioChannelLabel_Haptic: AudioChannelLabel = 45;
pub const kAudioChannelLabel_LeftTopFront: AudioChannelLabel = kAudioChannelLabel_VerticalHeightLeft;
pub const kAudioChannelLabel_CenterTopFront: AudioChannelLabel = kAudioChannelLabel_VerticalHeightCenter;
pub const kAudioChannelLabel_RightTopFront: AudioChannelLabel = kAudioChannelLabel_VerticalHeightRight;
pub const kAudioChannelLabel_LeftTopMiddle: AudioChannelLabel = 49;
pub const kAudioChannelLabel_CenterTopMiddle: AudioChannelLabel = kAudioChannelLabel_TopCenterSurround;
pub const kAudioChannelLabel_RightTopMiddle: AudioChannelLabel = 51;
pub const kAudioChannelLabel_LeftTopRear: AudioChannelLabel = 52;
pub const kAudioChannelLabel_CenterTopRear: AudioChannelLabel = 53;
pub const kAudioChannelLabel_RightTopRear: AudioChannelLabel = 54;
pub const kAudioChannelLabel_LeftSideSurround: AudioChannelLabel = 55;
pub const kAudioChannelLabel_RightSideSurround: AudioChannelLabel = 56;
pub const kAudioChannelLabel_LeftBottom: AudioChannelLabel = 57;
pub const kAudioChannelLabel_RightBottomL: AudioChannelLabel = 58;
pub const kAudioChannelLabel_CenterBottom: AudioChannelLabel = 59;
pub const kAudioChannelLabel_LeftTopSurround: AudioChannelLabel = 60;
pub const kAudioChannelLabel_RightTopSurround: AudioChannelLabel = 61;
pub const kAudioChannelLabel_LFE3: AudioChannelLabel = 62;
pub const kAudioChannelLabel_LeftBackSurround: AudioChannelLabel = 63;
pub const kAudioChannelLabel_RightBackSurround: AudioChannelLabel = 64;
pub const kAudioChannelLabel_LeftEdgeOfScreen: AudioChannelLabel = 65;
pub const kAudioChannelLabel_RightEdgeOfScreen: AudioChannelLabel = 66;
pub const kAudioChannelLabel_Ambisonic_W: AudioChannelLabel = 200;
pub const kAudioChannelLabel_Ambisonic_X: AudioChannelLabel = 201;
pub const kAudioChannelLabel_Ambisonic_Y: AudioChannelLabel = 202;
pub const kAudioChannelLabel_Ambisonic_Z: AudioChannelLabel = 203;
pub const kAudioChannelLabel_MS_Mid: AudioChannelLabel = 204;
pub const kAudioChannelLabel_MS_Side: AudioChannelLabel = 205;
pub const kAudioChannelLabel_XY_X: AudioChannelLabel = 206;
pub const kAudioChannelLabel_XY_Y: AudioChannelLabel = 207;
pub const kAudioChannelLabel_BinauralLeft: AudioChannelLabel = 208;
pub const kAudioChannelLabel_BinauralRight: AudioChannelLabel = 209;
pub const kAudioChannelLabel_HeadphonesLeft: AudioChannelLabel = 301;
pub const kAudioChannelLabel_HeadphonesRight: AudioChannelLabel = 302;
pub const kAudioChannelLabel_ClickTrack: AudioChannelLabel = 304;
pub const kAudioChannelLabel_ForeignLanguage: AudioChannelLabel = 305;
pub const kAudioChannelLabel_Discrete: AudioChannelLabel = 400;
pub const kAudioChannelLabel_Discrete_0: AudioChannelLabel = 1 << 16;
pub const kAudioChannelLabel_Discrete_1: AudioChannelLabel = (1 << 16) | 1;
pub const kAudioChannelLabel_Discrete_2: AudioChannelLabel = (1 << 16) | 2;
pub const kAudioChannelLabel_Discrete_3: AudioChannelLabel = (1 << 16) | 3;
pub const kAudioChannelLabel_Discrete_4: AudioChannelLabel = (1 << 16) | 4;
pub const kAudioChannelLabel_Discrete_5: AudioChannelLabel = (1 << 16) | 5;
pub const kAudioChannelLabel_Discrete_6: AudioChannelLabel = (1 << 16) | 6;
pub const kAudioChannelLabel_Discrete_7: AudioChannelLabel = (1 << 16) | 7;
pub const kAudioChannelLabel_Discrete_8: AudioChannelLabel = (1 << 16) | 8;
pub const kAudioChannelLabel_Discrete_9: AudioChannelLabel = (1 << 16) | 9;
pub const kAudioChannelLabel_Discrete_10: AudioChannelLabel = (1 << 16) | 10;
pub const kAudioChannelLabel_Discrete_11: AudioChannelLabel = (1 << 16) | 11;
pub const kAudioChannelLabel_Discrete_12: AudioChannelLabel = (1 << 16) | 12;
pub const kAudioChannelLabel_Discrete_13: AudioChannelLabel = (1 << 16) | 13;
pub const kAudioChannelLabel_Discrete_14: AudioChannelLabel = (1 << 16) | 14;
pub const kAudioChannelLabel_Discrete_15: AudioChannelLabel = (1 << 16) | 15;
pub const kAudioChannelLabel_Discrete_65535: AudioChannelLabel = (1 << 16) | 65535;
pub const kAudioChannelLabel_HOA_ACN: AudioChannelLabel = 500;
pub const kAudioChannelLabel_HOA_ACN_0: AudioChannelLabel = 2 << 16;
pub const kAudioChannelLabel_HOA_ACN_1: AudioChannelLabel = (2 << 16) | 1;
pub const kAudioChannelLabel_HOA_ACN_2: AudioChannelLabel = (2 << 16) | 2;
pub const kAudioChannelLabel_HOA_ACN_3: AudioChannelLabel = (2 << 16) | 3;
pub const kAudioChannelLabel_HOA_ACN_4: AudioChannelLabel = (2 << 16) | 4;
pub const kAudioChannelLabel_HOA_ACN_5: AudioChannelLabel = (2 << 16) | 5;
pub const kAudioChannelLabel_HOA_ACN_6: AudioChannelLabel = (2 << 16) | 6;
pub const kAudioChannelLabel_HOA_ACN_7: AudioChannelLabel = (2 << 16) | 7;
pub const kAudioChannelLabel_HOA_ACN_8: AudioChannelLabel = (2 << 16) | 8;
pub const kAudioChannelLabel_HOA_ACN_9: AudioChannelLabel = (2 << 16) | 9;
pub const kAudioChannelLabel_HOA_ACN_10: AudioChannelLabel = (2 << 16) | 10;
pub const kAudioChannelLabel_HOA_ACN_11: AudioChannelLabel = (2 << 16) | 11;
pub const kAudioChannelLabel_HOA_ACN_12: AudioChannelLabel = (2 << 16) | 12;
pub const kAudioChannelLabel_HOA_ACN_13: AudioChannelLabel = (2 << 16) | 13;
pub const kAudioChannelLabel_HOA_ACN_14: AudioChannelLabel = (2 << 16) | 14;
pub const kAudioChannelLabel_HOA_ACN_15: AudioChannelLabel = (2 << 16) | 15;
pub const kAudioChannelLabel_HOA_ACN_65024: AudioChannelLabel = (2 << 16) | 65024;
pub const kAudioChannelLabel_HOA_SN3D: AudioChannelLabel = kAudioChannelLabel_HOA_ACN_0;
pub const kAudioChannelLabel_HOA_N3D: AudioChannelLabel = 3 << 16;
pub const kAudioChannelLabel_Object: AudioChannelLabel = 4 << 16;
pub const kAudioChannelLabel_BeginReserved: AudioChannelLabel = 0xF0000000;
pub const kAudioChannelLabel_EndReserved: AudioChannelLabel = 0xFFFFFFFE;

pub type AudioChannelBitmap = u32;

pub const kAudioChannelBit_Left: AudioChannelBitmap = 1 << 0;
pub const kAudioChannelBit_Right: AudioChannelBitmap = 1 << 1;
pub const kAudioChannelBit_Center: AudioChannelBitmap = 1 << 2;
pub const kAudioChannelBit_LFEScreen: AudioChannelBitmap = 1 << 3;
pub const kAudioChannelBit_LeftSurround: AudioChannelBitmap = 1 << 4;
pub const kAudioChannelBit_RightSurround: AudioChannelBitmap = 1 << 5;
pub const kAudioChannelBit_LeftCenter: AudioChannelBitmap = 1 << 6;
pub const kAudioChannelBit_RightCenter: AudioChannelBitmap = 1 << 7;
pub const kAudioChannelBit_CenterSurround: AudioChannelBitmap = 1 << 8;
pub const kAudioChannelBit_LeftSurroundDirect: AudioChannelBitmap = 1 << 9;
pub const kAudioChannelBit_RightSurroundDirect: AudioChannelBitmap = 1 << 10;
pub const kAudioChannelBit_TopCenterSurround: AudioChannelBitmap = 1 << 11;
pub const kAudioChannelBit_VerticalHeightLeft: AudioChannelBitmap = 1 << 12;
pub const kAudioChannelBit_VerticalHeightCenter: AudioChannelBitmap = 1 << 13;
pub const kAudioChannelBit_VerticalHeightRight: AudioChannelBitmap = 1 << 14;
pub const kAudioChannelBit_TopBackLeft: AudioChannelBitmap = 1 << 15;
pub const kAudioChannelBit_TopBackCenter: AudioChannelBitmap = 1 << 16;
pub const kAudioChannelBit_TopBackRight: AudioChannelBitmap = 1 << 17;
pub const kAudioChannelBit_LeftTopFront: AudioChannelBitmap = kAudioChannelBit_VerticalHeightLeft;
pub const kAudioChannelBit_CenterTopFront: AudioChannelBitmap = kAudioChannelBit_VerticalHeightCenter;
pub const kAudioChannelBit_RightTopFront: AudioChannelBitmap = kAudioChannelBit_VerticalHeightRight;
pub const kAudioChannelBit_LeftTopMiddle: AudioChannelBitmap = 1 << 21;
pub const kAudioChannelBit_CenterTopMiddle: AudioChannelBitmap = kAudioChannelBit_TopCenterSurround;
pub const kAudioChannelBit_RightTopMiddle: AudioChannelBitmap = 1 << 23;
pub const kAudioChannelBit_LeftTopRear: AudioChannelBitmap = 1 << 24;
pub const kAudioChannelBit_CenterTopRear: AudioChannelBitmap = 1 << 25;
pub const kAudioChannelBit_RightTopRear: AudioChannelBitmap = 1 << 26;

pub type AudioChannelFlags = u32;

pub const kAudioChannelFlags_AllOff: AudioChannelFlags = 0;
pub const kAudioChannelFlags_RectangularCoordinates: AudioChannelFlags = 1 << 0;
pub const kAudioChannelFlags_SphericalCoordinates: AudioChannelFlags = 1 << 1;
pub const kAudioChannelFlags_Meters: AudioChannelFlags = 1 << 2;

pub type AudioChannelCoordinateIndex = u32;

pub const kAudioChannelCoordinates_LeftRight: AudioChannelCoordinateIndex = 0;
pub const kAudioChannelCoordinates_BackFront: AudioChannelCoordinateIndex = 1;
pub const kAudioChannelCoordinates_DownUp: AudioChannelCoordinateIndex = 2;
pub const kAudioChannelCoordinates_Azimuth: AudioChannelCoordinateIndex = 0;
pub const kAudioChannelCoordinates_Elevation: AudioChannelCoordinateIndex = 1;
pub const kAudioChannelCoordinates_Distance: AudioChannelCoordinateIndex = 2;

pub type AudioChannelLayoutTag = u32;

pub const kAudioChannelLayoutTag_UseChannelDescriptions: AudioChannelLayoutTag = 0;
pub const kAudioChannelLayoutTag_UseChannelBitmap: AudioChannelLayoutTag = 1 << 16;
pub const kAudioChannelLayoutTag_Mono: AudioChannelLayoutTag = (100 << 16) | 1;
pub const kAudioChannelLayoutTag_Stereo: AudioChannelLayoutTag = (101 << 16) | 2;
pub const kAudioChannelLayoutTag_StereoHeadphones: AudioChannelLayoutTag = (102 << 16) | 2;
pub const kAudioChannelLayoutTag_MatrixStereo: AudioChannelLayoutTag = (103 << 16) | 2;
pub const kAudioChannelLayoutTag_MidSide: AudioChannelLayoutTag = (104 << 16) | 2;
pub const kAudioChannelLayoutTag_XY: AudioChannelLayoutTag = (105 << 16) | 2;
pub const kAudioChannelLayoutTag_Binaural: AudioChannelLayoutTag = (106 << 16) | 2;
pub const kAudioChannelLayoutTag_Ambisonic_B_Format: AudioChannelLayoutTag = (107 << 16) | 4;
pub const kAudioChannelLayoutTag_Quadraphonic: AudioChannelLayoutTag = (108 << 16) | 4;
pub const kAudioChannelLayoutTag_Pentagonal: AudioChannelLayoutTag = (109 << 16) | 5;
pub const kAudioChannelLayoutTag_Hexagonal: AudioChannelLayoutTag = (110 << 16) | 6;
pub const kAudioChannelLayoutTag_Octagonal: AudioChannelLayoutTag = (111 << 16) | 8;
pub const kAudioChannelLayoutTag_Cube: AudioChannelLayoutTag = (112 << 16) | 8;
pub const kAudioChannelLayoutTag_MPEG_1_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_MPEG_2_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_MPEG_3_0_A: AudioChannelLayoutTag = (113 << 16) | 3;
pub const kAudioChannelLayoutTag_MPEG_3_0_B: AudioChannelLayoutTag = (114 << 16) | 3;
pub const kAudioChannelLayoutTag_MPEG_4_0_A: AudioChannelLayoutTag = (115 << 16) | 4;
pub const kAudioChannelLayoutTag_MPEG_4_0_B: AudioChannelLayoutTag = (116 << 16) | 4;
pub const kAudioChannelLayoutTag_MPEG_5_0_A: AudioChannelLayoutTag = (117 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_0_B: AudioChannelLayoutTag = (118 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_0_C: AudioChannelLayoutTag = (119 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_0_D: AudioChannelLayoutTag = (120 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_1_A: AudioChannelLayoutTag = (121 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_5_1_B: AudioChannelLayoutTag = (122 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_5_1_C: AudioChannelLayoutTag = (123 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_5_1_D: AudioChannelLayoutTag = (124 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_6_1_A: AudioChannelLayoutTag = (125 << 16) | 7;
pub const kAudioChannelLayoutTag_MPEG_7_1_A: AudioChannelLayoutTag = (126 << 16) | 8;
pub const kAudioChannelLayoutTag_MPEG_7_1_B: AudioChannelLayoutTag = (127 << 16) | 8;
pub const kAudioChannelLayoutTag_MPEG_7_1_C: AudioChannelLayoutTag = (128 << 16) | 8;
pub const kAudioChannelLayoutTag_Emagic_Default_7_1: AudioChannelLayoutTag = (129 << 16) | 8;
pub const kAudioChannelLayoutTag_SMPTE_DTV: AudioChannelLayoutTag = (130 << 16) | 8;
pub const kAudioChannelLayoutTag_ITU_1_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_ITU_2_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_ITU_2_1: AudioChannelLayoutTag = (131 << 16) | 3;
pub const kAudioChannelLayoutTag_ITU_2_2: AudioChannelLayoutTag = (132 << 16) | 4;
pub const kAudioChannelLayoutTag_ITU_3_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_A;
pub const kAudioChannelLayoutTag_ITU_3_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_A;
pub const kAudioChannelLayoutTag_ITU_3_2: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_ITU_3_2_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_ITU_3_4_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_C;
pub const kAudioChannelLayoutTag_DVD_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_DVD_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_DVD_2: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_1;
pub const kAudioChannelLayoutTag_DVD_3: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_2;
pub const kAudioChannelLayoutTag_DVD_4: AudioChannelLayoutTag = (133 << 16) | 3;
pub const kAudioChannelLayoutTag_DVD_5: AudioChannelLayoutTag = (134 << 16) | 4;
pub const kAudioChannelLayoutTag_DVD_6: AudioChannelLayoutTag = (135 << 16) | 5;
pub const kAudioChannelLayoutTag_DVD_7: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_A;
pub const kAudioChannelLayoutTag_DVD_8: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_A;
pub const kAudioChannelLayoutTag_DVD_9: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_DVD_10: AudioChannelLayoutTag = (136 << 16) | 4;
pub const kAudioChannelLayoutTag_DVD_11: AudioChannelLayoutTag = (137 << 16) | 5;
pub const kAudioChannelLayoutTag_DVD_12: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_DVD_13: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_8;
pub const kAudioChannelLayoutTag_DVD_14: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_9;
pub const kAudioChannelLayoutTag_DVD_15: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_10;
pub const kAudioChannelLayoutTag_DVD_16: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_11;
pub const kAudioChannelLayoutTag_DVD_17: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_12;
pub const kAudioChannelLayoutTag_DVD_18: AudioChannelLayoutTag = (138 << 16) | 5;
pub const kAudioChannelLayoutTag_DVD_19: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_B;
pub const kAudioChannelLayoutTag_DVD_20: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_B;
pub const kAudioChannelLayoutTag_AudioUnit_4: AudioChannelLayoutTag = kAudioChannelLayoutTag_Quadraphonic;
pub const kAudioChannelLayoutTag_AudioUnit_5: AudioChannelLayoutTag = kAudioChannelLayoutTag_Pentagonal;
pub const kAudioChannelLayoutTag_AudioUnit_6: AudioChannelLayoutTag = kAudioChannelLayoutTag_Hexagonal;
pub const kAudioChannelLayoutTag_AudioUnit_8: AudioChannelLayoutTag = kAudioChannelLayoutTag_Octagonal;
pub const kAudioChannelLayoutTag_AudioUnit_5_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_B;
pub const kAudioChannelLayoutTag_AudioUnit_6_0: AudioChannelLayoutTag = (139 << 16) | 6;
pub const kAudioChannelLayoutTag_AudioUnit_7_0: AudioChannelLayoutTag = (140 << 16) | 7;
pub const kAudioChannelLayoutTag_AudioUnit_7_0_Front: AudioChannelLayoutTag = (148 << 16) | 7;
pub const kAudioChannelLayoutTag_AudioUnit_5_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_AudioUnit_6_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_6_1_A;
pub const kAudioChannelLayoutTag_AudioUnit_7_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_C;
pub const kAudioChannelLayoutTag_AudioUnit_7_1_Front: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_A;
pub const kAudioChannelLayoutTag_AAC_3_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_B;
pub const kAudioChannelLayoutTag_AAC_Quadraphonic: AudioChannelLayoutTag = kAudioChannelLayoutTag_Quadraphonic;
pub const kAudioChannelLayoutTag_AAC_4_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_B;
pub const kAudioChannelLayoutTag_AAC_5_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_D;
pub const kAudioChannelLayoutTag_AAC_5_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_D;
pub const kAudioChannelLayoutTag_AAC_6_0: AudioChannelLayoutTag = (141 << 16) | 6;
pub const kAudioChannelLayoutTag_AAC_6_1: AudioChannelLayoutTag = (142 << 16) | 7;
pub const kAudioChannelLayoutTag_AAC_7_0: AudioChannelLayoutTag = (143 << 16) | 7;
pub const kAudioChannelLayoutTag_AAC_7_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_B;
pub const kAudioChannelLayoutTag_AAC_7_1_B: AudioChannelLayoutTag = (183 << 16) | 8;
pub const kAudioChannelLayoutTag_AAC_7_1_C: AudioChannelLayoutTag = (184 << 16) | 8;
pub const kAudioChannelLayoutTag_AAC_Octagonal: AudioChannelLayoutTag = (144 << 16) | 8;
pub const kAudioChannelLayoutTag_TMH_10_2_std: AudioChannelLayoutTag = (145 << 16) | 16;
pub const kAudioChannelLayoutTag_TMH_10_2_full: AudioChannelLayoutTag = (146 << 16) | 21;
pub const kAudioChannelLayoutTag_AC3_1_0_1: AudioChannelLayoutTag = (149 << 16) | 2;
pub const kAudioChannelLayoutTag_AC3_3_0: AudioChannelLayoutTag = (150 << 16) | 3;
pub const kAudioChannelLayoutTag_AC3_3_1: AudioChannelLayoutTag = (151 << 16) | 4;
pub const kAudioChannelLayoutTag_AC3_3_0_1: AudioChannelLayoutTag = (152 << 16) | 4;
pub const kAudioChannelLayoutTag_AC3_2_1_1: AudioChannelLayoutTag = (153 << 16) | 4;
pub const kAudioChannelLayoutTag_AC3_3_1_1: AudioChannelLayoutTag = (154 << 16) | 5;
pub const kAudioChannelLayoutTag_EAC_6_0_A: AudioChannelLayoutTag = (155 << 16) | 6;
pub const kAudioChannelLayoutTag_EAC_7_0_A: AudioChannelLayoutTag = (156 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_6_1_A: AudioChannelLayoutTag = (157 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_6_1_B: AudioChannelLayoutTag = (158 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_6_1_C: AudioChannelLayoutTag = (159 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_7_1_A: AudioChannelLayoutTag = (160 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_B: AudioChannelLayoutTag = (161 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_C: AudioChannelLayoutTag = (162 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_D: AudioChannelLayoutTag = (163 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_E: AudioChannelLayoutTag = (164 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_F: AudioChannelLayoutTag = (165 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_G: AudioChannelLayoutTag = (166 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_H: AudioChannelLayoutTag = (167 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_3_1: AudioChannelLayoutTag = (168 << 16) | 4;
pub const kAudioChannelLayoutTag_DTS_4_1: AudioChannelLayoutTag = (169 << 16) | 5;
pub const kAudioChannelLayoutTag_DTS_6_0_A: AudioChannelLayoutTag = (170 << 16) | 6;
pub const kAudioChannelLayoutTag_DTS_6_0_B: AudioChannelLayoutTag = (171 << 16) | 6;
pub const kAudioChannelLayoutTag_DTS_6_0_C: AudioChannelLayoutTag = (172 << 16) | 6;
pub const kAudioChannelLayoutTag_DTS_6_1_A: AudioChannelLayoutTag = (173 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_6_1_B: AudioChannelLayoutTag = (174 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_6_1_C: AudioChannelLayoutTag = (175 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_7_0: AudioChannelLayoutTag = (176 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_7_1: AudioChannelLayoutTag = (177 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_8_0_A: AudioChannelLayoutTag = (178 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_8_0_B: AudioChannelLayoutTag = (179 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_8_1_A: AudioChannelLayoutTag = (180 << 16) | 9;
pub const kAudioChannelLayoutTag_DTS_8_1_B: AudioChannelLayoutTag = (181 << 16) | 9;
pub const kAudioChannelLayoutTag_DTS_6_1_D: AudioChannelLayoutTag = (182 << 16) | 7;
pub const kAudioChannelLayoutTag_WAVE_2_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_4;
pub const kAudioChannelLayoutTag_WAVE_3_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_A;
pub const kAudioChannelLayoutTag_WAVE_4_0_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_2;
pub const kAudioChannelLayoutTag_WAVE_4_0_B: AudioChannelLayoutTag = (185 << 16) | 4;
pub const kAudioChannelLayoutTag_WAVE_5_0_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_WAVE_5_0_B: AudioChannelLayoutTag = (186 << 16) | 5;
pub const kAudioChannelLayoutTag_WAVE_5_1_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_WAVE_5_1_B: AudioChannelLayoutTag = (187 << 16) | 6;
pub const kAudioChannelLayoutTag_WAVE_6_1: AudioChannelLayoutTag = (188 << 16) | 7;
pub const kAudioChannelLayoutTag_WAVE_7_1: AudioChannelLayoutTag = (189 << 16) | 8;
pub const kAudioChannelLayoutTag_HOA_ACN_SN3D: AudioChannelLayoutTag = 190 << 16;
pub const kAudioChannelLayoutTag_HOA_ACN_N3D: AudioChannelLayoutTag = 191 << 16;
pub const kAudioChannelLayoutTag_Atmos_5_1_2: AudioChannelLayoutTag = (194 << 16) | 8;
pub const kAudioChannelLayoutTag_Atmos_5_1_4: AudioChannelLayoutTag = (195 << 16) | 10;
pub const kAudioChannelLayoutTag_Atmos_7_1_2: AudioChannelLayoutTag = (196 << 16) | 10;
pub const kAudioChannelLayoutTag_Atmos_7_1_4: AudioChannelLayoutTag = (192 << 16) | 12;
pub const kAudioChannelLayoutTag_Atmos_9_1_6: AudioChannelLayoutTag = (193 << 16) | 16;
pub const kAudioChannelLayoutTag_Logic_Mono: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_Logic_Stereo: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_Logic_Quadraphonic: AudioChannelLayoutTag = kAudioChannelLayoutTag_Quadraphonic;
pub const kAudioChannelLayoutTag_Logic_4_0_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_A;
pub const kAudioChannelLayoutTag_Logic_4_0_B: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_B;
pub const kAudioChannelLayoutTag_Logic_4_0_C: AudioChannelLayoutTag = (197 << 16) | 4;
pub const kAudioChannelLayoutTag_Logic_5_0_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_Logic_5_0_B: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_B;
pub const kAudioChannelLayoutTag_Logic_5_0_C: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_C;
pub const kAudioChannelLayoutTag_Logic_5_0_D: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_D;
pub const kAudioChannelLayoutTag_Logic_5_1_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_Logic_5_1_B: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_B;
pub const kAudioChannelLayoutTag_Logic_5_1_C: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_C;
pub const kAudioChannelLayoutTag_Logic_5_1_D: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_D;
pub const kAudioChannelLayoutTag_Logic_6_0_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_AAC_6_0;
pub const kAudioChannelLayoutTag_Logic_6_0_B: AudioChannelLayoutTag = (198 << 16) | 6;
pub const kAudioChannelLayoutTag_Logic_6_0_C: AudioChannelLayoutTag = kAudioChannelLayoutTag_AudioUnit_6_0;
pub const kAudioChannelLayoutTag_Logic_6_1_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_AAC_6_1;
pub const kAudioChannelLayoutTag_Logic_6_1_B: AudioChannelLayoutTag = (199 << 16) | 7;
pub const kAudioChannelLayoutTag_Logic_6_1_C: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_6_1_A;
pub const kAudioChannelLayoutTag_Logic_6_1_D: AudioChannelLayoutTag = (200 << 16) | 7;
pub const kAudioChannelLayoutTag_Logic_7_1_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_AudioUnit_7_1;
pub const kAudioChannelLayoutTag_Logic_7_1_B: AudioChannelLayoutTag = (201 << 16) | 8;
pub const kAudioChannelLayoutTag_Logic_7_1_C: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_C;
pub const kAudioChannelLayoutTag_Logic_7_1_SDDS_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_A;
pub const kAudioChannelLayoutTag_Logic_7_1_SDDS_B: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_B;
pub const kAudioChannelLayoutTag_Logic_7_1_SDDS_C: AudioChannelLayoutTag = kAudioChannelLayoutTag_Emagic_Default_7_1;
pub const kAudioChannelLayoutTag_Logic_Atmos_5_1_2: AudioChannelLayoutTag = kAudioChannelLayoutTag_Atmos_5_1_2;
pub const kAudioChannelLayoutTag_Logic_Atmos_5_1_4: AudioChannelLayoutTag = kAudioChannelLayoutTag_Atmos_5_1_4;
pub const kAudioChannelLayoutTag_Logic_Atmos_7_1_2: AudioChannelLayoutTag = kAudioChannelLayoutTag_Atmos_7_1_2;
pub const kAudioChannelLayoutTag_Logic_Atmos_7_1_4_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_Atmos_7_1_4;
pub const kAudioChannelLayoutTag_Logic_Atmos_7_1_4_B: AudioChannelLayoutTag = (202 << 16) | 12;
pub const kAudioChannelLayoutTag_Logic_Atmos_7_1_6: AudioChannelLayoutTag = (203 << 16) | 14;
pub const kAudioChannelLayoutTag_DiscreteInOrder: AudioChannelLayoutTag = 147 << 16;
pub const kAudioChannelLayoutTag_CICP_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_1_0;
pub const kAudioChannelLayoutTag_CICP_2: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_2_0;
pub const kAudioChannelLayoutTag_CICP_3: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_A;
pub const kAudioChannelLayoutTag_CICP_4: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_A;
pub const kAudioChannelLayoutTag_CICP_5: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_CICP_6: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_CICP_7: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_B;
pub const kAudioChannelLayoutTag_CICP_9: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_1;
pub const kAudioChannelLayoutTag_CICP_10: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_2;
pub const kAudioChannelLayoutTag_CICP_11: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_6_1_A;
pub const kAudioChannelLayoutTag_CICP_12: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_C;
pub const kAudioChannelLayoutTag_CICP_13: AudioChannelLayoutTag = (204 << 16) | 24;
pub const kAudioChannelLayoutTag_CICP_14: AudioChannelLayoutTag = (205 << 16) | 8;
pub const kAudioChannelLayoutTag_CICP_15: AudioChannelLayoutTag = (206 << 16) | 12;
pub const kAudioChannelLayoutTag_CICP_16: AudioChannelLayoutTag = (207 << 16) | 10;
pub const kAudioChannelLayoutTag_CICP_17: AudioChannelLayoutTag = (208 << 16) | 12;
pub const kAudioChannelLayoutTag_CICP_18: AudioChannelLayoutTag = (209 << 16) | 14;
pub const kAudioChannelLayoutTag_CICP_19: AudioChannelLayoutTag = (210 << 16) | 12;
pub const kAudioChannelLayoutTag_CICP_20: AudioChannelLayoutTag = (211 << 16) | 14;
pub const kAudioChannelLayoutTag_Ogg_3_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_AC3_3_0;
pub const kAudioChannelLayoutTag_Ogg_4_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_WAVE_4_0_B;
pub const kAudioChannelLayoutTag_Ogg_5_0: AudioChannelLayoutTag = (212 << 16) | 5;
pub const kAudioChannelLayoutTag_Ogg_5_1: AudioChannelLayoutTag = (213 << 16) | 6;
pub const kAudioChannelLayoutTag_Ogg_6_1: AudioChannelLayoutTag = (214 << 16) | 7;
pub const kAudioChannelLayoutTag_Ogg_7_1: AudioChannelLayoutTag = (215 << 16) | 8;
pub const kAudioChannelLayoutTag_BeginReserved: AudioChannelLayoutTag = 0xF0000000;
pub const kAudioChannelLayoutTag_EndReserved: AudioChannelLayoutTag = 0xFFFEFFFF;
pub const kAudioChannelLayoutTag_Unknown: AudioChannelLayoutTag = 0xFFFF0000;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioChannelDescription {
    pub mChannelLabel: AudioChannelLabel,
    pub mChannelFlags: AudioChannelFlags,
    pub mCoordinates: [f32; 3usize],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioChannelLayout {
    pub mChannelLayoutTag: AudioChannelLayoutTag,
    pub mChannelBitmap: AudioChannelBitmap,
    pub mNumberChannelDescriptions: u32,
    pub mChannelDescriptions: [AudioChannelDescription; 1usize],
}

#[inline]
pub fn AudioChannelLayoutTag_GetNumberOfChannels(inLayoutTag: AudioChannelLayoutTag) -> u32 {
    inLayoutTag & 0x0000FFFF
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioFormatListItem {
    pub mASBD: AudioStreamBasicDescription,
    pub mChannelLayoutTag: AudioChannelLayoutTag,
}

pub type MPEG4ObjectID = c_long;

pub const kMPEG4Object_AAC_Main: MPEG4ObjectID = 1;
pub const kMPEG4Object_AAC_LC: MPEG4ObjectID = 2;
pub const kMPEG4Object_AAC_SSR: MPEG4ObjectID = 3;
pub const kMPEG4Object_AAC_LTP: MPEG4ObjectID = 4;
pub const kMPEG4Object_AAC_SBR: MPEG4ObjectID = 5;
pub const kMPEG4Object_AAC_Scalable: MPEG4ObjectID = 6;
pub const kMPEG4Object_TwinVQ: MPEG4ObjectID = 7;
pub const kMPEG4Object_CELP: MPEG4ObjectID = 8;
pub const kMPEG4Object_HVXC: MPEG4ObjectID = 9;

impl Default for AudioValueTranslation {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for AudioBuffer {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
