use std::{ptr::null_mut, slice::from_raw_parts_mut};

use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, CFTypeID, OSStatus, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
};
use libc::{c_void, size_t};

pub const kCMBlockBufferNoErr: OSStatus = 0;
pub const kCMBlockBufferStructureAllocationFailedErr: OSStatus = -12700;
pub const kCMBlockBufferBlockAllocationFailedErr: OSStatus = -12701;
pub const kCMBlockBufferBadCustomBlockSourceErr: OSStatus = -12702;
pub const kCMBlockBufferBadOffsetParameterErr: OSStatus = -12703;
pub const kCMBlockBufferBadLengthParameterErr: OSStatus = -12704;
pub const kCMBlockBufferBadPointerParameterErr: OSStatus = -12705;
pub const kCMBlockBufferEmptyBBufErr: OSStatus = -12706;
pub const kCMBlockBufferUnallocatedBlock: OSStatus = -12707;
pub const kCMBlockBufferInsufficientSpaceErr: OSStatus = -12708;

pub type CMBlockBufferFlags = u32;

pub const kCMBlockBufferAssureMemoryNowFlag: CMBlockBufferFlags = 1 << 0;
pub const kCMBlockBufferAlwaysCopyDataFlag: CMBlockBufferFlags = 1 << 1;
pub const kCMBlockBufferDontOptimizeDepthFlag: CMBlockBufferFlags = 1 << 2;
pub const kCMBlockBufferPermitEmptyReferenceFlag: CMBlockBufferFlags = 1 << 3;

#[repr(C)]
pub struct OpaqueCMBlockBuffer(c_void);

pub type CMBlockBufferRef = *mut OpaqueCMBlockBuffer;

#[repr(C, align(4))]
pub struct CMBlockBufferCustomBlockSource {
    pub version: u32,
    pub AllocateBlock: extern "C" fn(*mut c_void, size_t) -> *mut c_void,
    pub FreeBlock: extern "C" fn(*mut c_void, *mut c_void, size_t),
    pub refcon: *mut c_void,
}

pub const kCMBlockBufferCustomBlockSourceVersion: u32 = 0;

extern "C" {
    pub fn CMBlockBufferCreateEmpty(
        structureAllocator: CFAllocatorRef,
        subBlockCapacity: size_t,
        flags: CMBlockBufferFlags,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
    pub fn CMBlockBufferCreateWithMemoryBlock(
        structureAllocator: CFAllocatorRef,
        memoryBlock: *const c_void,
        blockLength: size_t,
        blockAllocator: CFAllocatorRef,
        customBlockSource: *const CMBlockBufferCustomBlockSource,
        offsetToData: size_t,
        dataLength: size_t,
        flags: CMBlockBufferFlags,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
    pub fn CMBlockBufferCreateWithBufferReference(
        structureAllocator: CFAllocatorRef,
        bufferReference: CMBlockBufferRef,
        offsetToData: size_t,
        dataLength: size_t,
        flags: CMBlockBufferFlags,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
    pub fn CMBlockBufferCreateContiguous(
        structureAllocator: CFAllocatorRef,
        sourceBuffer: CMBlockBufferRef,
        blockAllocator: CFAllocatorRef,
        customBlockSource: *const CMBlockBufferCustomBlockSource,
        offsetToData: size_t,
        dataLength: size_t,
        flags: CMBlockBufferFlags,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
    pub fn CMBlockBufferGetTypeID() -> CFTypeID;
    pub fn CMBlockBufferAppendMemoryBlock(
        theBuffer: CMBlockBufferRef,
        memoryBlock: *const c_void,
        blockLength: size_t,
        blockAllocator: CFAllocatorRef,
        customBlockSource: *const CMBlockBufferCustomBlockSource,
        offsetToData: size_t,
        dataLength: size_t,
        flags: CMBlockBufferFlags,
    ) -> OSStatus;
    pub fn CMBlockBufferAppendBufferReference(
        theBuffer: CMBlockBufferRef,
        targetBBuf: CMBlockBufferRef,
        offsetToData: size_t,
        dataLength: size_t,
        flags: CMBlockBufferFlags,
    ) -> OSStatus;
    pub fn CMBlockBufferAssureBlockMemory(theBuffer: CMBlockBufferRef) -> OSStatus;
    pub fn CMBlockBufferAccessDataBytes(
        theBuffer: CMBlockBufferRef,
        offset: size_t,
        length: size_t,
        temporaryBlock: *mut c_void,
        returnedPointer: *mut *mut c_void,
    ) -> OSStatus;
    pub fn CMBlockBufferCopyDataBytes(
        theSourceBuffer: CMBlockBufferRef,
        offsetToData: size_t,
        dataLength: size_t,
        destination: *mut c_void,
    ) -> OSStatus;
    pub fn CMBlockBufferReplaceDataBytes(
        sourceBytes: *const c_void,
        destinationBuffer: CMBlockBufferRef,
        offsetIntoDestination: size_t,
        dataLength: size_t,
    ) -> OSStatus;
    pub fn CMBlockBufferFillDataBytes(
        fillByte: u8,
        destinationBuffer: CMBlockBufferRef,
        offsetIntoDestination: size_t,
        dataLength: size_t,
    ) -> OSStatus;
    pub fn CMBlockBufferGetDataPointer(
        theBuffer: CMBlockBufferRef,
        offset: size_t,
        lengthAtOffset: *mut size_t,
        totalLength: *mut size_t,
        dataPointer: *mut *mut c_void,
    ) -> OSStatus;
    pub fn CMBlockBufferGetDataLength(theBuffer: CMBlockBufferRef) -> size_t;
    pub fn CMBlockBufferIsRangeContiguous(theBuffer: CMBlockBufferRef, offset: size_t, length: size_t) -> Boolean;
    pub fn CMBlockBufferIsEmpty(theBuffer: CMBlockBufferRef) -> Boolean;
}

declare_TCFType!(CMBlockBuffer, CMBlockBufferRef);
impl_TCFType!(CMBlockBuffer, CMBlockBufferRef, CMBlockBufferGetTypeID);
impl_CFTypeDescription!(CMBlockBuffer);

impl CMBlockBuffer {
    #[inline]
    pub fn new_empty(sub_block_capacity: size_t, flags: CMBlockBufferFlags) -> Result<CMBlockBuffer, OSStatus> {
        unsafe {
            let mut block_buffer: CMBlockBufferRef = null_mut();
            let status = CMBlockBufferCreateEmpty(kCFAllocatorDefault, sub_block_capacity, flags, &mut block_buffer);
            if status == kCMBlockBufferNoErr {
                Ok(TCFType::wrap_under_create_rule(block_buffer))
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub unsafe fn new_with_memory_block(
        memory_block: &[u8],
        custom_block_source: *const CMBlockBufferCustomBlockSource,
        offset_to_data: size_t,
        data_length: size_t,
        flags: CMBlockBufferFlags,
    ) -> Result<CMBlockBuffer, OSStatus> {
        unsafe {
            let mut block_buffer: CMBlockBufferRef = null_mut();
            let status = CMBlockBufferCreateWithMemoryBlock(
                kCFAllocatorDefault,
                memory_block.as_ptr() as *const c_void,
                memory_block.len() as size_t,
                kCFAllocatorDefault,
                custom_block_source,
                offset_to_data,
                data_length,
                flags,
                &mut block_buffer,
            );
            if status == kCMBlockBufferNoErr {
                Ok(TCFType::wrap_under_create_rule(block_buffer))
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn new_with_buffer_reference(
        &self,
        offset_to_data: size_t,
        data_length: size_t,
        flags: CMBlockBufferFlags,
    ) -> Result<CMBlockBuffer, OSStatus> {
        unsafe {
            let mut block_buffer: CMBlockBufferRef = null_mut();
            let status = CMBlockBufferCreateWithBufferReference(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                offset_to_data,
                data_length,
                flags,
                &mut block_buffer,
            );
            if status == kCMBlockBufferNoErr {
                Ok(TCFType::wrap_under_create_rule(block_buffer))
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub unsafe fn new_contiguous(
        &self,
        custom_block_source: *const CMBlockBufferCustomBlockSource,
        offset_to_data: size_t,
        data_length: size_t,
        flags: CMBlockBufferFlags,
    ) -> Result<CMBlockBuffer, OSStatus> {
        unsafe {
            let mut block_buffer: CMBlockBufferRef = null_mut();
            let status = CMBlockBufferCreateContiguous(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                kCFAllocatorDefault,
                custom_block_source,
                offset_to_data,
                data_length,
                flags,
                &mut block_buffer,
            );
            if status == kCMBlockBufferNoErr {
                Ok(TCFType::wrap_under_create_rule(block_buffer))
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub unsafe fn append_memory_block(
        &self,
        memory_block: &[u8],
        custom_block_source: *const CMBlockBufferCustomBlockSource,
        offset_to_data: size_t,
        data_length: size_t,
        flags: CMBlockBufferFlags,
    ) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBlockBufferAppendMemoryBlock(
                self.as_concrete_TypeRef(),
                memory_block.as_ptr() as *const c_void,
                memory_block.len() as size_t,
                kCFAllocatorDefault,
                custom_block_source,
                offset_to_data,
                data_length,
                flags,
            );
            if status == kCMBlockBufferNoErr {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn append_buffer_reference(
        &self,
        target_block_buf: &CMBlockBuffer,
        offset_to_data: size_t,
        data_length: size_t,
        flags: CMBlockBufferFlags,
    ) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBlockBufferAppendBufferReference(self.as_concrete_TypeRef(), target_block_buf.0, offset_to_data, data_length, flags);
            if status == kCMBlockBufferNoErr {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn assure_block_memory(&self) -> OSStatus {
        unsafe { CMBlockBufferAssureBlockMemory(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn access_data_bytes(&self, offset: size_t, temporary_block: &mut [u8]) -> Result<&mut [u8], OSStatus> {
        unsafe {
            let mut returned_pointer: *mut c_void = null_mut();
            let status = CMBlockBufferAccessDataBytes(
                self.as_concrete_TypeRef(),
                offset,
                temporary_block.len() as size_t,
                temporary_block.as_mut_ptr() as *mut c_void,
                &mut returned_pointer,
            );
            if status == kCMBlockBufferNoErr {
                Ok(from_raw_parts_mut(returned_pointer as *mut u8, temporary_block.len()))
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn copy_data_bytes(&self, offset_to_data: size_t, destination: &mut [u8]) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBlockBufferCopyDataBytes(
                self.as_concrete_TypeRef(),
                offset_to_data,
                destination.len() as size_t,
                destination.as_mut_ptr() as *mut c_void,
            );
            if status == kCMBlockBufferNoErr {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn replace_data_bytes(&self, source_bytes: &[u8], offset_into_destination: size_t) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBlockBufferReplaceDataBytes(
                source_bytes.as_ptr() as *const c_void,
                self.as_concrete_TypeRef(),
                offset_into_destination,
                source_bytes.len() as size_t,
            );
            if status == kCMBlockBufferNoErr {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn fill_data_bytes(&self, fill_byte: u8, offset_into_destination: size_t, data_length: size_t) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBlockBufferFillDataBytes(fill_byte, self.as_concrete_TypeRef(), offset_into_destination, data_length);
            if status == kCMBlockBufferNoErr {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn get_data(&self, offset: size_t) -> Result<&mut [u8], OSStatus> {
        unsafe {
            let mut length_at_offset: size_t = 0;
            let mut total_length: size_t = 0;
            let mut data_pointer: *mut c_void = null_mut();
            let status = CMBlockBufferGetDataPointer(self.as_concrete_TypeRef(), offset, &mut length_at_offset, &mut total_length, &mut data_pointer);
            if status == kCMBlockBufferNoErr {
                Ok(from_raw_parts_mut(data_pointer as *mut u8, length_at_offset))
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn get_data_length(&self) -> size_t {
        unsafe { CMBlockBufferGetDataLength(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn is_range_contiguous(&self, offset: size_t, length: size_t) -> bool {
        unsafe { CMBlockBufferIsRangeContiguous(self.as_concrete_TypeRef(), offset, length) != 0 }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { CMBlockBufferIsEmpty(self.as_concrete_TypeRef()) != 0 }
    }
}
