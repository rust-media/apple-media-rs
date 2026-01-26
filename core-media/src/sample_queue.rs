use std::ptr::null_mut;

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFTypeID, OSStatus, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
};
use libc::c_void;

pub const kCMSimpleQueueError_AllocationFailed: OSStatus = -12770;
pub const kCMSimpleQueueError_RequiredParameterMissing: OSStatus = -12771;
pub const kCMSimpleQueueError_ParameterOutOfRange: OSStatus = -12772;
pub const kCMSimpleQueueError_QueueIsFull: OSStatus = -12773;

#[repr(C)]
pub struct opaqueCMSimpleQueue(c_void);

pub type CMSimpleQueueRef = *mut opaqueCMSimpleQueue;

extern "C" {
    pub fn CMSimpleQueueGetTypeID() -> CFTypeID;
    pub fn CMSimpleQueueCreate(allocator: CFAllocatorRef, capacity: i32, queueOut: *mut CMSimpleQueueRef) -> OSStatus;
    pub fn CMSimpleQueueEnqueue(queue: CMSimpleQueueRef, element: *const c_void) -> OSStatus;
    pub fn CMSimpleQueueDequeue(queue: CMSimpleQueueRef) -> *const c_void;
    pub fn CMSimpleQueueGetHead(queue: CMSimpleQueueRef) -> *const c_void;
    pub fn CMSimpleQueueReset(queue: CMSimpleQueueRef) -> OSStatus;
    pub fn CMSimpleQueueGetCapacity(queue: CMSimpleQueueRef) -> i32;
    pub fn CMSimpleQueueGetCount(queue: CMSimpleQueueRef) -> i32;
}

declare_TCFType! {
    CMSimpleQueue, CMSimpleQueueRef
}
impl_TCFType!(CMSimpleQueue, CMSimpleQueueRef, CMSimpleQueueGetTypeID);
impl_CFTypeDescription!(CMSimpleQueue);

impl CMSimpleQueue {
    #[inline]
    pub fn new(capacity: i32) -> Result<CMSimpleQueue, OSStatus> {
        let mut queue: CMSimpleQueueRef = null_mut();
        let status = unsafe { CMSimpleQueueCreate(kCFAllocatorDefault, capacity, &mut queue) };
        if status == 0 {
            Ok(unsafe { CMSimpleQueue::wrap_under_create_rule(queue) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn enqueue(&self, element: *const c_void) -> Result<(), OSStatus> {
        let status = unsafe { CMSimpleQueueEnqueue(self.as_concrete_TypeRef(), element) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn dequeue(&self) -> *const c_void {
        unsafe { CMSimpleQueueDequeue(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_head(&self) -> *const c_void {
        unsafe { CMSimpleQueueGetHead(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn reset(&self) -> Result<(), OSStatus> {
        let status = unsafe { CMSimpleQueueReset(self.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_capacity(&self) -> i32 {
        unsafe { CMSimpleQueueGetCapacity(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_count(&self) -> i32 {
        unsafe { CMSimpleQueueGetCount(self.as_concrete_TypeRef()) }
    }
}
