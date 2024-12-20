use core_foundation::base::{CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::url::CFURLRef;
use core_media::base::CMItemCount;
use core_media::sample_buffer::CMSampleBufferRef;
use core_media::time_range::CMTimeRange;
use libc::{c_float, c_void};

pub type VTFrameSiloRef = CFTypeRef;
pub type Float32 = c_float;

#[link(name = "VideoToolBox", kind = "framework")]
extern "C" {

    pub fn VTFrameSiloCreate(
        allocator: CFAllocatorRef,
        fileURL: CFURLRef,
        timeRange: CMTimeRange,
        options: CFDictionaryRef,
        multiPassStorageOut: *mut VTFrameSiloRef,
    ) -> OSStatus;
    pub fn VTFrameSiloAddSampleBuffer(silo: VTFrameSiloRef, sampleBuffer: CMSampleBufferRef) -> OSStatus;
    pub fn VTFrameSiloSetTimeRangesForNextPass(silo: VTFrameSiloRef, timeRangeCount: CMItemCount, timeRangeArray: *const CMTimeRange) -> OSStatus;

    pub fn VTFrameSiloCallBlockForEachSampleBuffer(
        silo: VTFrameSiloRef,
        timeRange: CMTimeRange,
        handler: extern "C" fn(sampleBuffer: CMSampleBufferRef) -> OSStatus,
    ) -> OSStatus;
    pub fn VTFrameSiloCallFunctionForEachSampleBuffer(
        silo: VTFrameSiloRef,
        timeRange: CMTimeRange,
        refcon: *mut c_void,
        callback: extern "C" fn(refcon: *mut c_void, sampleBuffer: CMSampleBufferRef) -> OSStatus,
    ) -> OSStatus;

    pub fn VTFrameSiloGetProgressOfCurrentPass(silo: VTFrameSiloRef, progressOut: *mut Float32) -> OSStatus;
    pub fn VTFrameSiloGetTypeID() -> CFTypeID;

}
