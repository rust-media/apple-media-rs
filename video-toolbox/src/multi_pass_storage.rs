use core_foundation_sys::base::{CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::string::CFStringRef;
use core_foundation_sys::url::CFURLRef;
use core_media_sys::CMTimeRange;

pub type VTMultiPassStorageRef = CFTypeRef;

#[link(name = "VideoToolBox", kind = "framework")]
extern "C" {
    pub static kVTMultiPassStorageCreationOption_DoNotDelete: CFStringRef;

    pub fn VTMultiPassStorageCreate(
        allocator: CFAllocatorRef,
        fileURL: CFURLRef,
        timeRange: CMTimeRange,
        options: CFDictionaryRef,
        multiPassStorageOut: *mut VTMultiPassStorageRef,
    ) -> OSStatus;
    pub fn VTMultiPassStorageClose(multiPassStorage: VTMultiPassStorageRef) -> OSStatus;
    pub fn VTMultiPassStorageGetTypeID() -> CFTypeID;

}
