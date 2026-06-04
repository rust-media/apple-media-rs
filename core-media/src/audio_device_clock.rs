use std::ptr::{null, null_mut};

use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, OSStatus, TCFType},
    string::{CFString, CFStringRef},
};

use crate::{
    base::status_to_result,
    sync::{CMClock, CMClockRef},
};

pub type AudioObjectID = u32;

pub type AudioDeviceID = AudioObjectID;

pub const kAudioObjectUnknown: AudioObjectID = 0;
pub const kAudioStreamUnknown: AudioObjectID = kAudioObjectUnknown;

extern "C" {
    pub fn CMAudioDeviceClockCreate(allocator: CFAllocatorRef, deviceUID: CFStringRef, clockOut: *mut CMClockRef) -> OSStatus;
    pub fn CMAudioDeviceClockCreateFromAudioDeviceID(allocator: CFAllocatorRef, deviceID: AudioDeviceID, clockOut: *mut CMClockRef) -> OSStatus;
    pub fn CMAudioDeviceClockSetAudioDeviceUID(clock: CMClockRef, deviceUID: CFStringRef) -> OSStatus;
    pub fn CMAudioDeviceClockSetAudioDeviceID(clock: CMClockRef, deviceID: AudioDeviceID) -> OSStatus;
    pub fn CMAudioDeviceClockGetAudioDevice(
        clock: CMClockRef,
        deviceUIDOut: *mut CFStringRef,
        deviceIDOut: *mut AudioDeviceID,
        trackingDefaultDeviceOut: *mut Boolean,
    ) -> OSStatus;
}

impl CMClock {
    #[inline]
    pub fn new_audio_device_clock(device_uid: &CFString) -> Result<CMClock, OSStatus> {
        unsafe {
            let mut clock: CMClockRef = null_mut();
            let status = CMAudioDeviceClockCreate(kCFAllocatorDefault, device_uid.as_concrete_TypeRef(), &mut clock);
            status_to_result(status).map(|_| TCFType::wrap_under_create_rule(clock))
        }
    }

    #[inline]
    pub fn new_audio_device_clock_from_device_id(device_id: AudioDeviceID) -> Result<CMClock, OSStatus> {
        unsafe {
            let mut clock: CMClockRef = null_mut();
            let status = CMAudioDeviceClockCreateFromAudioDeviceID(kCFAllocatorDefault, device_id, &mut clock);
            status_to_result(status).map(|_| TCFType::wrap_under_create_rule(clock))
        }
    }

    #[inline]
    pub fn set_audio_device_uid(&self, device_uid: &CFString) -> Result<(), OSStatus> {
        unsafe {
            let status = CMAudioDeviceClockSetAudioDeviceUID(self.as_concrete_TypeRef(), device_uid.as_concrete_TypeRef());
            status_to_result(status)
        }
    }

    #[inline]
    pub fn set_audio_device_id(&self, device_id: AudioDeviceID) -> Result<(), OSStatus> {
        unsafe {
            let status = CMAudioDeviceClockSetAudioDeviceID(self.as_concrete_TypeRef(), device_id);
            status_to_result(status)
        }
    }

    #[inline]
    pub fn get_audio_device(&self) -> Result<(CFString, AudioDeviceID, Boolean), OSStatus> {
        unsafe {
            let mut device_uid = null();
            let mut device_id = 0;
            let mut tracking_default_device = 0;
            let status = CMAudioDeviceClockGetAudioDevice(self.as_concrete_TypeRef(), &mut device_uid, &mut device_id, &mut tracking_default_device);
            status_to_result(status).map(|_| (TCFType::wrap_under_create_rule(device_uid), device_id, tracking_default_device))
        }
    }
}
