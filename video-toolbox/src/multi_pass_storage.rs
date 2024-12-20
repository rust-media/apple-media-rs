use core_foundation::base::{CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::CFStringRef;
use core_foundation::url::CFURLRef;
use core_media::time_range::CMTimeRange;

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
