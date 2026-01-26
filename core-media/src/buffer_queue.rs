use std::{
    mem,
    ptr::{null, null_mut},
};

use block::{Block, ConcreteBlock};
use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, CFComparisonResult, CFGetTypeID, CFRetain, CFType, CFTypeID, CFTypeRef, OSStatus, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
    string::CFStringRef,
};
use libc::{c_void, size_t};

use crate::{base::CMItemCount, time::CMTime};

pub const kCMBufferQueueError_AllocationFailed: OSStatus = -12760;
pub const kCMBufferQueueError_RequiredParameterMissing: OSStatus = -12761;
pub const kCMBufferQueueError_InvalidCMBufferCallbacksStruct: OSStatus = -12762;
pub const kCMBufferQueueError_EnqueueAfterEndOfData: OSStatus = -12763;
pub const kCMBufferQueueError_QueueIsFull: OSStatus = -12764;
pub const kCMBufferQueueError_BadTriggerDuration: OSStatus = -12765;
pub const kCMBufferQueueError_CannotModifyQueueFromTriggerCallback: OSStatus = -12766;
pub const kCMBufferQueueError_InvalidTriggerCondition: OSStatus = -12767;
pub const kCMBufferQueueError_InvalidTriggerToken: OSStatus = -12768;
pub const kCMBufferQueueError_InvalidBuffer: OSStatus = -12769;

#[repr(C)]
pub struct OpaqueCMBufferQueue(c_void);

pub type CMBufferQueueRef = *mut OpaqueCMBufferQueue;

pub type CMBufferRef = CFTypeRef;

pub type CMBufferGetTimeCallback = extern "C" fn(buf: CMBufferRef, refcon: *mut c_void) -> CMTime;
pub type CMBufferGetTimeHandler = *const Block<CMBufferRef, CMTime>;
pub type CMBufferGetBooleanCallback = extern "C" fn(buf: CMBufferRef, refcon: *mut c_void) -> Boolean;
pub type CMBufferGetBooleanHandler = *const Block<CMBufferRef, Boolean>;
pub type CMBufferCompareCallback = extern "C" fn(buf1: CMBufferRef, buf2: CMBufferRef, refcon: *mut c_void) -> CFComparisonResult;
pub type CMBufferCompareHandler = *const Block<(CMBufferRef, CMBufferRef), CFComparisonResult>;
pub type CMBufferGetSizeCallback = extern "C" fn(buf: CMBufferRef, refcon: *mut c_void) -> size_t;
pub type CMBufferGetSizeHandler = *const Block<CMBufferRef, size_t>;

#[repr(C, align(4))]
#[derive(Clone, Copy, Debug)]
pub struct CMBufferCallbacks {
    pub version: u32,
    pub refcon: *mut c_void,
    pub getDecodeTimeStamp: CMBufferGetTimeCallback,
    pub getPresentationTimeStamp: CMBufferGetTimeCallback,
    pub getDuration: CMBufferGetTimeCallback,
    pub isDataReady: CMBufferGetBooleanCallback,
    pub compare: CMBufferCompareCallback,
    pub dataBecameReadyNotification: CFStringRef,
    pub getSize: CMBufferGetSizeCallback,
}

extern "C" {
    pub fn CMBufferQueueGetCallbacksForUnsortedSampleBuffers() -> *const CMBufferCallbacks;
    pub fn CMBufferQueueGetCallbacksForSampleBuffersSortedByOutputPTS() -> *const CMBufferCallbacks;
    pub fn CMBufferQueueCreate(
        allocator: CFAllocatorRef,
        capacity: CMItemCount,
        callbacks: *const CMBufferCallbacks,
        queueOut: *mut CMBufferQueueRef,
    ) -> OSStatus;
    pub fn CMBufferQueueGetTypeID() -> CFTypeID;
    pub fn CMBufferQueueEnqueue(queue: CMBufferQueueRef, buf: CMBufferRef) -> OSStatus;
    pub fn CMBufferQueueDequeueAndRetain(queue: CMBufferQueueRef) -> CMBufferRef;
    pub fn CMBufferQueueDequeueIfDataReadyAndRetain(queue: CMBufferQueueRef) -> CMBufferRef;
    pub fn CMBufferQueueGetHead(queue: CMBufferQueueRef) -> CMBufferRef;
    pub fn CMBufferQueueCopyHead(queue: CMBufferQueueRef) -> CMBufferRef;
    pub fn CMBufferQueueIsEmpty(queue: CMBufferQueueRef) -> Boolean;
    pub fn CMBufferQueueMarkEndOfData(queue: CMBufferQueueRef) -> OSStatus;
    pub fn CMBufferQueueContainsEndOfData(queue: CMBufferQueueRef) -> Boolean;
    pub fn CMBufferQueueIsAtEndOfData(queue: CMBufferQueueRef) -> Boolean;
    pub fn CMBufferQueueReset(queue: CMBufferQueueRef) -> OSStatus;
    pub fn CMBufferQueueResetWithCallback(
        queue: CMBufferQueueRef,
        callback: extern "C" fn(buffer: CMBufferRef, refcon: *mut c_void),
        refcon: *mut c_void,
    ) -> OSStatus;
    pub fn CMBufferQueueGetBufferCount(queue: CMBufferQueueRef) -> CMItemCount;
    pub fn CMBufferQueueGetDuration(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetMinDecodeTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetFirstDecodeTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetMinPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetFirstPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetMaxPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetEndPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetTotalSize(queue: CMBufferQueueRef) -> size_t;
}

#[repr(C)]
pub struct opaqueCMBufferQueueTriggerToken(c_void);

pub type CMBufferQueueTriggerToken = *const opaqueCMBufferQueueTriggerToken;

pub type CMBufferQueueTriggerCallback = extern "C" fn(triggerRefcon: *mut c_void, triggerToken: CMBufferQueueTriggerToken);
pub type CMBufferQueueTriggerHandler = *const Block<(CMBufferQueueTriggerToken,), ()>;

pub type CMBufferQueueTriggerCondition = i32;

pub const kCMBufferQueueTrigger_WhenDurationBecomesLessThan: CMBufferQueueTriggerCondition = 1;
pub const kCMBufferQueueTrigger_WhenDurationBecomesLessThanOrEqualTo: CMBufferQueueTriggerCondition = 2;
pub const kCMBufferQueueTrigger_WhenDurationBecomesGreaterThan: CMBufferQueueTriggerCondition = 3;
pub const kCMBufferQueueTrigger_WhenDurationBecomesGreaterThanOrEqualTo: CMBufferQueueTriggerCondition = 4;
pub const kCMBufferQueueTrigger_WhenMinPresentationTimeStampChanges: CMBufferQueueTriggerCondition = 5;
pub const kCMBufferQueueTrigger_WhenMaxPresentationTimeStampChanges: CMBufferQueueTriggerCondition = 6;
pub const kCMBufferQueueTrigger_WhenDataBecomesReady: CMBufferQueueTriggerCondition = 7;
pub const kCMBufferQueueTrigger_WhenEndOfDataReached: CMBufferQueueTriggerCondition = 8;
pub const kCMBufferQueueTrigger_WhenReset: CMBufferQueueTriggerCondition = 9;
pub const kCMBufferQueueTrigger_WhenBufferCountBecomesLessThan: CMBufferQueueTriggerCondition = 10;
pub const kCMBufferQueueTrigger_WhenBufferCountBecomesGreaterThan: CMBufferQueueTriggerCondition = 11;
pub const kCMBufferQueueTrigger_WhenDurationBecomesGreaterThanOrEqualToAndBufferCountBecomesGreaterThan: CMBufferQueueTriggerCondition = 12;

extern "C" {
    pub fn CMBufferQueueInstallTrigger(
        queue: CMBufferQueueRef,
        callback: CMBufferQueueTriggerCallback,
        refcon: *mut c_void,
        condition: CMBufferQueueTriggerCondition,
        time: CMTime,
        triggerTokenOut: *mut CMBufferQueueTriggerToken,
    ) -> OSStatus;
    pub fn CMBufferQueueInstallTriggerWithIntegerThreshold(
        queue: CMBufferQueueRef,
        callback: CMBufferQueueTriggerCallback,
        refcon: *mut c_void,
        condition: CMBufferQueueTriggerCondition,
        threshold: CMItemCount,
        triggerTokenOut: *mut CMBufferQueueTriggerToken,
    ) -> OSStatus;
    pub fn CMBufferQueueInstallTriggerHandler(
        queue: CMBufferQueueRef,
        condition: CMBufferQueueTriggerCondition,
        time: CMTime,
        triggerTokenOut: *mut CMBufferQueueTriggerToken,
        handler: CMBufferQueueTriggerHandler,
    ) -> OSStatus;
    pub fn CMBufferQueueInstallTriggerHandlerWithIntegerThreshold(
        queue: CMBufferQueueRef,
        condition: CMBufferQueueTriggerCondition,
        threshold: CMItemCount,
        triggerTokenOut: *mut CMBufferQueueTriggerToken,
        handler: CMBufferQueueTriggerHandler,
    ) -> OSStatus;
    pub fn CMBufferQueueRemoveTrigger(queue: CMBufferQueueRef, triggerToken: CMBufferQueueTriggerToken) -> OSStatus;
    pub fn CMBufferQueueTestTrigger(queue: CMBufferQueueRef, triggerToken: CMBufferQueueTriggerToken) -> Boolean;
    pub fn CMBufferQueueCallForEachBuffer(
        queue: CMBufferQueueRef,
        callback: extern "C" fn(buffer: CMBufferRef, refcon: *mut c_void) -> OSStatus,
        refcon: *mut c_void,
    ) -> OSStatus;
}

pub type CMBufferValidationCallback = extern "C" fn(queue: CMBufferQueueRef, buf: CMBufferRef, validationRefCon: *mut c_void) -> OSStatus;
pub type CMBufferValidationHandler = *const Block<(CMBufferQueueRef, CMBufferRef), OSStatus>;

extern "C" {
    pub fn CMBufferQueueSetValidationCallback(queue: CMBufferQueueRef, callback: CMBufferValidationCallback, refCon: *mut c_void) -> OSStatus;
    pub fn CMBufferQueueSetValidationHandler(queue: CMBufferQueueRef, handler: CMBufferValidationHandler) -> OSStatus;
}

declare_TCFType! {
    CMBuffer, CMBufferRef
}
impl_CFTypeDescription!(CMBuffer);

impl CMBuffer {
    #[inline]
    pub fn as_concrete_TypeRef(&self) -> CMBufferRef {
        self.0
    }

    #[inline]
    pub fn as_CFType(&self) -> CFType {
        unsafe { CFType::wrap_under_get_rule(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn as_CFTypeRef(&self) -> CFTypeRef {
        self.as_concrete_TypeRef() as CFTypeRef
    }

    #[inline]
    pub fn into_CFType(self) -> CFType
    where
        Self: Sized,
    {
        let reference = self.as_CFTypeRef();
        mem::forget(self);
        unsafe { CFType::wrap_under_create_rule(reference) }
    }

    #[inline]
    pub unsafe fn wrap_under_create_rule(reference: CMBufferRef) -> Self {
        CMBuffer(reference)
    }

    #[inline]
    pub unsafe fn wrap_under_get_rule(reference: CMBufferRef) -> Self {
        let reference = CFRetain(reference);
        CMBuffer(reference)
    }

    #[inline]
    pub fn type_of(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_CFTypeRef()) }
    }

    #[inline]
    pub fn instance_of<T: TCFType>(&self) -> bool {
        self.type_of() == T::type_id()
    }
}

declare_TCFType! {
    CMBufferQueue, CMBufferQueueRef
}
impl_TCFType!(CMBufferQueue, CMBufferQueueRef, CMBufferQueueGetTypeID);
impl_CFTypeDescription!(CMBufferQueue);

impl CMBufferQueue {
    #[inline]
    pub fn new(callbacks: &[CMBufferCallbacks]) -> Result<CMBufferQueue, OSStatus> {
        unsafe {
            let mut queue: CMBufferQueueRef = null_mut();
            let status = CMBufferQueueCreate(kCFAllocatorDefault, callbacks.len() as CMItemCount, callbacks.as_ptr(), &mut queue);
            if status == 0 {
                Ok(TCFType::wrap_under_create_rule(queue))
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn enqueue(&self, buf: &CMBuffer) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBufferQueueEnqueue(self.as_concrete_TypeRef(), buf.as_concrete_TypeRef());
            if status == 0 {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn dequeue_and_retain(&self) -> Option<CMBuffer> {
        unsafe {
            let buf = CMBufferQueueDequeueAndRetain(self.as_concrete_TypeRef());
            if buf.is_null() {
                None
            } else {
                Some(CMBuffer::wrap_under_create_rule(buf))
            }
        }
    }

    #[inline]
    pub fn dequeue_if_data_ready_and_retain(&self) -> Option<CMBuffer> {
        unsafe {
            let buf = CMBufferQueueDequeueIfDataReadyAndRetain(self.as_concrete_TypeRef());
            if buf.is_null() {
                None
            } else {
                Some(CMBuffer::wrap_under_create_rule(buf))
            }
        }
    }

    #[inline]
    pub fn get_head(&self) -> Option<CMBuffer> {
        unsafe {
            let buf = CMBufferQueueGetHead(self.as_concrete_TypeRef());
            if buf.is_null() {
                None
            } else {
                Some(CMBuffer::wrap_under_get_rule(buf))
            }
        }
    }

    #[inline]
    pub fn copy_head(&self) -> Option<CMBuffer> {
        unsafe {
            let buf = CMBufferQueueCopyHead(self.as_concrete_TypeRef());
            if buf.is_null() {
                None
            } else {
                Some(CMBuffer::wrap_under_create_rule(buf))
            }
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { CMBufferQueueIsEmpty(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn mark_end_of_data(&self) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBufferQueueMarkEndOfData(self.as_concrete_TypeRef());
            if status == 0 {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn contains_end_of_data(&self) -> bool {
        unsafe { CMBufferQueueContainsEndOfData(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn is_at_end_of_data(&self) -> bool {
        unsafe { CMBufferQueueIsAtEndOfData(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn reset(&self) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBufferQueueReset(self.as_concrete_TypeRef());
            if status == 0 {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub unsafe fn reset_with_callback(
        &self,
        callback: extern "C" fn(buffer: CMBufferRef, refcon: *mut c_void),
        refcon: *mut c_void,
    ) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBufferQueueResetWithCallback(self.as_concrete_TypeRef(), callback, refcon);
            if status == 0 {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn get_buffer_count(&self) -> CMItemCount {
        unsafe { CMBufferQueueGetBufferCount(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_duration(&self) -> CMTime {
        unsafe { CMBufferQueueGetDuration(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_min_decode_time_stamp(&self) -> CMTime {
        unsafe { CMBufferQueueGetMinDecodeTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_first_decode_time_stamp(&self) -> CMTime {
        unsafe { CMBufferQueueGetFirstDecodeTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_min_presentation_time_stamp(&self) -> CMTime {
        unsafe { CMBufferQueueGetMinPresentationTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_first_presentation_time_stamp(&self) -> CMTime {
        unsafe { CMBufferQueueGetFirstPresentationTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_max_presentation_time_stamp(&self) -> CMTime {
        unsafe { CMBufferQueueGetMaxPresentationTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_end_presentation_time_stamp(&self) -> CMTime {
        unsafe { CMBufferQueueGetEndPresentationTimeStamp(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_total_size(&self) -> size_t {
        unsafe { CMBufferQueueGetTotalSize(self.as_concrete_TypeRef()) }
    }

    pub unsafe fn install_trigger(
        &self,
        callback: CMBufferQueueTriggerCallback,
        refcon: *mut c_void,
        condition: CMBufferQueueTriggerCondition,
        time: CMTime,
    ) -> Result<CMBufferQueueTriggerToken, OSStatus> {
        unsafe {
            let mut token = null();
            let status = CMBufferQueueInstallTrigger(self.as_concrete_TypeRef(), callback, refcon, condition, time, &mut token);
            if status == 0 {
                Ok(token)
            } else {
                Err(status)
            }
        }
    }

    pub unsafe fn install_trigger_with_integer_threshold(
        &self,
        callback: CMBufferQueueTriggerCallback,
        refcon: *mut c_void,
        condition: CMBufferQueueTriggerCondition,
        threshold: CMItemCount,
    ) -> Result<CMBufferQueueTriggerToken, OSStatus> {
        unsafe {
            let mut token = null();
            let status =
                CMBufferQueueInstallTriggerWithIntegerThreshold(self.as_concrete_TypeRef(), callback, refcon, condition, threshold, &mut token);
            if status == 0 {
                Ok(token)
            } else {
                Err(status)
            }
        }
    }

    pub fn install_trigger_closure<F>(
        &self,
        condition: CMBufferQueueTriggerCondition,
        time: CMTime,
        closure: Option<F>,
    ) -> Result<CMBufferQueueTriggerToken, OSStatus>
    where
        F: Fn(CMBufferQueueTriggerToken) + 'static,
    {
        let mut token = null();
        let handler = closure.map(|closure| {
            ConcreteBlock::new(move |trigger_token: CMBufferQueueTriggerToken| {
                closure(trigger_token);
            })
            .copy()
        });
        let status = unsafe {
            CMBufferQueueInstallTriggerHandler(self.as_concrete_TypeRef(), condition, time, &mut token, handler.as_ref().map_or(null(), |h| &**h))
        };
        if status == 0 {
            Ok(token)
        } else {
            Err(status)
        }
    }

    pub fn install_trigger_with_integer_threshold_closure<F>(
        &self,
        condition: CMBufferQueueTriggerCondition,
        threshold: CMItemCount,
        closure: Option<F>,
    ) -> Result<CMBufferQueueTriggerToken, OSStatus>
    where
        F: Fn(CMBufferQueueTriggerToken) + 'static,
    {
        let mut token = null();
        let handler = closure.map(|closure| {
            ConcreteBlock::new(move |trigger_token: CMBufferQueueTriggerToken| {
                closure(trigger_token);
            })
            .copy()
        });
        let status = unsafe {
            CMBufferQueueInstallTriggerHandlerWithIntegerThreshold(
                self.as_concrete_TypeRef(),
                condition,
                threshold,
                &mut token,
                handler.as_ref().map_or(null(), |h| &**h),
            )
        };
        if status == 0 {
            Ok(token)
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn remove_trigger(&self, token: CMBufferQueueTriggerToken) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBufferQueueRemoveTrigger(self.as_concrete_TypeRef(), token);
            if status == 0 {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn test_trigger(&self, token: CMBufferQueueTriggerToken) -> bool {
        unsafe { CMBufferQueueTestTrigger(self.as_concrete_TypeRef(), token) != 0 }
    }

    #[inline]
    pub unsafe fn call_for_each_buffer(
        &self,
        callback: extern "C" fn(buffer: CMBufferRef, refcon: *mut c_void) -> OSStatus,
        refcon: *mut c_void,
    ) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBufferQueueCallForEachBuffer(self.as_concrete_TypeRef(), callback, refcon);
            if status == 0 {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    pub unsafe fn set_validation_callback(&self, callback: CMBufferValidationCallback, refcon: *mut c_void) -> Result<(), OSStatus> {
        unsafe {
            let status = CMBufferQueueSetValidationCallback(self.as_concrete_TypeRef(), callback, refcon);
            if status == 0 {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    pub fn set_validation_closure<F>(&self, closure: Option<F>) -> Result<(), OSStatus>
    where
        F: Fn(CMBufferQueue, CMBuffer) -> OSStatus + 'static,
    {
        let handler = closure.map(|closure| {
            ConcreteBlock::new(move |queue: CMBufferQueueRef, buf: CMBufferRef| {
                let queue = unsafe { CMBufferQueue::wrap_under_get_rule(queue) };
                let buf = unsafe { CMBuffer::wrap_under_get_rule(buf) };
                closure(queue, buf)
            })
            .copy()
        });

        let status = unsafe { CMBufferQueueSetValidationHandler(self.as_concrete_TypeRef(), handler.as_ref().map_or(null(), |h| &**h)) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }
}
