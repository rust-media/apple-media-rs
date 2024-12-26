cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        use crate::libc::{c_long, c_ulong};
        pub type AVAudioInteger = c_long;
        pub type AVAudioUInteger = c_ulong;
    } else {
        use crate::libc::{c_int, c_uint};
        pub type AVAudioInteger = c_int;
        pub type AVAudioUInteger = c_uint;
    }
}

pub type AudioSessionID = u32;

pub type AVAudioSessionErrorCode = AVAudioInteger;

#[inline]
const fn fourcc(code: &[u8; 4]) -> AVAudioInteger {
    (((code[0] as u32) << 24) | ((code[1] as u32) << 16) | ((code[2] as u32) << 8) | (code[3] as u32)) as AVAudioInteger
}

pub const AVAudioSessionErrorCodeNone: AVAudioSessionErrorCode = 0;
pub const AVAudioSessionErrorCodeMediaServicesFailed: AVAudioSessionErrorCode = fourcc(b"msrv");
pub const AVAudioSessionErrorCodeIsBusy: AVAudioSessionErrorCode = fourcc(b"!act");
pub const AVAudioSessionErrorCodeIncompatibleCategory: AVAudioSessionErrorCode = fourcc(b"!cat");
pub const AVAudioSessionErrorCodeCannotInterruptOthers: AVAudioSessionErrorCode = fourcc(b"!int");
pub const AVAudioSessionErrorCodeMissingEntitlement: AVAudioSessionErrorCode = fourcc(b"ent?");
pub const AVAudioSessionErrorCodeSiriIsRecording: AVAudioSessionErrorCode = fourcc(b"siri");
pub const AVAudioSessionErrorCodeCannotStartPlaying: AVAudioSessionErrorCode = fourcc(b"!pla");
pub const AVAudioSessionErrorCodeCannotStartRecording: AVAudioSessionErrorCode = fourcc(b"!rec");
pub const AVAudioSessionErrorCodeBadParam: AVAudioSessionErrorCode = -50;
pub const AVAudioSessionErrorCodeInsufficientPriority: AVAudioSessionErrorCode = fourcc(b"!pri");
pub const AVAudioSessionErrorCodeResourceNotAvailable: AVAudioSessionErrorCode = fourcc(b"!res");
pub const AVAudioSessionErrorCodeUnspecified: AVAudioSessionErrorCode = fourcc(b"what");
pub const AVAudioSessionErrorCodeExpiredSession: AVAudioSessionErrorCode = fourcc(b"!ses");
pub const AVAudioSessionErrorCodeSessionNotActive: AVAudioSessionErrorCode = fourcc(b"inac");
