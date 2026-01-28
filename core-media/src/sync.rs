use std::{mem, ptr::null_mut};

use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, CFRetain, CFType, CFTypeID, CFTypeRef, OSStatus, TCFType},
    date::{CFAbsoluteTime, CFTimeInterval},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
    runloop::{CFRunLoop, CFRunLoopRef, CFRunLoopTimer, CFRunLoopTimerRef},
    string::CFStringRef,
};
use libc::c_void;

use crate::time::{CMTime, CMTimeRoundingMethod, CMTimeScale};

#[repr(C)]
pub struct OpaqueCMClock(c_void);

pub type CMClockRef = *mut OpaqueCMClock;

#[repr(C)]
pub struct OpaqueCMTimebase(c_void);

pub type CMTimebaseRef = *mut OpaqueCMTimebase;

pub type CMClockOrTimebaseRef = CFTypeRef;

pub const kCMClockError_MissingRequiredParameter: OSStatus = -12745;
pub const kCMClockError_InvalidParameter: OSStatus = -12746;
pub const kCMClockError_AllocationFailed: OSStatus = -12747;
pub const kCMClockError_UnsupportedOperation: OSStatus = -12756;

pub const kCMTimebaseError_MissingRequiredParameter: OSStatus = -12748;
pub const kCMTimebaseError_InvalidParameter: OSStatus = -12749;
pub const kCMTimebaseError_AllocationFailed: OSStatus = -12750;
pub const kCMTimebaseError_TimerIntervalTooShort: OSStatus = -12751;
pub const kCMTimebaseError_ReadOnly: OSStatus = -12757;

pub const kCMSyncError_MissingRequiredParameter: OSStatus = -12752;
pub const kCMSyncError_InvalidParameter: OSStatus = -12753;
pub const kCMSyncError_AllocationFailed: OSStatus = -12754;
pub const kCMSyncError_RateMustBeNonZero: OSStatus = -12755;

extern "C" {
    pub fn CMClockGetTypeID() -> CFTypeID;
    pub fn CMClockGetHostTimeClock() -> CMClockRef;
    pub fn CMClockConvertHostTimeToSystemUnits(hostTime: CMTime) -> u64;
    pub fn CMClockMakeHostTimeFromSystemUnits(hostTime: u64) -> CMTime;
    pub fn CMClockGetTime(clock: CMClockRef) -> CMTime;
    pub fn CMClockGetAnchorTime(clock: CMClockRef, clockTimeOut: *mut CMTime, referenceClockTimeOut: *mut CMTime) -> OSStatus;
    pub fn CMClockMightDrift(clock: CMClockRef, otherClock: CMClockRef) -> Boolean;
    pub fn CMClockInvalidate(clock: CMClockRef);

    pub fn CMTimebaseGetTypeID() -> CFTypeID;
    pub fn CMTimebaseCreateWithSourceClock(allocator: CFAllocatorRef, sourceClock: CMClockRef, timebaseOut: *mut CMTimebaseRef) -> OSStatus;
    pub fn CMTimebaseCreateWithSourceTimebase(allocator: CFAllocatorRef, sourceTimebase: CMTimebaseRef, timebaseOut: *mut CMTimebaseRef) -> OSStatus;
    pub fn CMTimebaseCopySourceTimebase(timebase: CMTimebaseRef) -> CMTimebaseRef;
    pub fn CMTimebaseCopySourceClock(timebase: CMTimebaseRef) -> CMClockRef;
    pub fn CMTimebaseCopySource(timebase: CMTimebaseRef) -> CMClockOrTimebaseRef;
    pub fn CMTimebaseCopyUltimateSourceClock(timebase: CMTimebaseRef) -> CMClockRef;
    pub fn CMTimebaseSetSourceClock(timebase: CMTimebaseRef, newSourceClock: CMClockRef) -> OSStatus;
    pub fn CMTimebaseSetSourceTimebase(timebase: CMTimebaseRef, newSourceTimebase: CMTimebaseRef) -> OSStatus;
    pub fn CMTimebaseGetTime(timebase: CMTimebaseRef) -> CMTime;
    pub fn CMTimebaseGetTimeWithTimeScale(timebase: CMTimebaseRef, timeScale: CMTimeScale, roundingMethod: CMTimeRoundingMethod) -> CMTime;
    pub fn CMTimebaseSetTime(timebase: CMTimebaseRef, time: CMTime) -> OSStatus;
    pub fn CMTimebaseSetAnchorTime(timebase: CMTimebaseRef, timebaseTime: CMTime, immediateSourceTime: CMTime) -> OSStatus;
    pub fn CMTimebaseGetRate(timebase: CMTimebaseRef) -> f64;
    pub fn CMTimebaseGetTimeAndRate(timebase: CMTimebaseRef, timeOut: *mut CMTime, rateOut: *mut f64) -> OSStatus;
    pub fn CMTimebaseSetRate(timebase: CMTimebaseRef, rate: f64) -> OSStatus;
    pub fn CMTimebaseSetRateAndAnchorTime(timebase: CMTimebaseRef, rate: f64, timebaseTime: CMTime, immediateSourceTime: CMTime) -> OSStatus;
    pub fn CMTimebaseGetEffectiveRate(timebase: CMTimebaseRef) -> f64;
    pub fn CMTimebaseAddTimer(timebase: CMTimebaseRef, timer: CFRunLoopTimerRef, runLoop: CFRunLoopRef) -> OSStatus;
}

pub const kCMTimebaseVeryLongCFTimeInterval: CFTimeInterval = 256.0 * 365.0 * 24.0 * 60.0 * 60.0;
pub const kCMTimebaseFarFutureCFAbsoluteTime: CFAbsoluteTime = kCMTimebaseVeryLongCFTimeInterval as CFAbsoluteTime;

extern "C" {
    pub fn CMTimebaseRemoveTimer(timebase: CMTimebaseRef, timer: CFRunLoopTimerRef) -> OSStatus;
    pub fn CMTimebaseSetTimerNextFireTime(timebase: CMTimebaseRef, timer: CFRunLoopTimerRef, fireTime: CMTime, flags: u32) -> OSStatus;
    pub fn CMTimebaseSetTimerToFireImmediately(timebase: CMTimebaseRef, timer: CFRunLoopTimerRef) -> OSStatus;
    pub fn CMSyncGetRelativeRate(ofClockOrTimebase: CMClockOrTimebaseRef, relativeToClockOrTimebase: CMClockOrTimebaseRef) -> f64;
    pub fn CMSyncGetRelativeRateAndAnchorTime(
        ofClockOrTimebase: CMClockOrTimebaseRef,
        relativeToClockOrTimebase: CMClockOrTimebaseRef,
        outRelativeRate: *mut f64,
        outOfClockOrTimebaseAnchorTime: *mut CMTime,
        outRelativeToClockOrTimebaseAnchorTime: *mut CMTime,
    ) -> OSStatus;
    pub fn CMSyncConvertTime(time: CMTime, fromClockOrTimebase: CMClockOrTimebaseRef, toClockOrTimebase: CMClockOrTimebaseRef) -> CMTime;
    pub fn CMSyncMightDrift(clockOrTimebase1: CMClockOrTimebaseRef, clockOrTimebase2: CMClockOrTimebaseRef) -> Boolean;
    pub fn CMSyncGetTime(clockOrTimebase: CMClockOrTimebaseRef) -> CMTime;
    pub fn CMTimebaseNotificationBarrier(timebase: CMTimebaseRef) -> OSStatus;

    pub static kCMTimebaseNotification_EffectiveRateChanged: CFStringRef;
    pub static kCMTimebaseNotification_TimeJumped: CFStringRef;
    pub static kCMTimebaseNotificationKey_EventTime: CFStringRef;
}

declare_TCFType!(CMClock, CMClockRef);
impl_TCFType!(CMClock, CMClockRef, CMClockGetTypeID);
impl_CFTypeDescription!(CMClock);

impl CMClock {
    #[inline]
    pub fn get_host_time_clock() -> Self {
        unsafe { TCFType::wrap_under_create_rule(CMClockGetHostTimeClock()) }
    }

    #[inline]
    pub fn convert_host_time_to_system_units(host_time: CMTime) -> u64 {
        unsafe { CMClockConvertHostTimeToSystemUnits(host_time) }
    }

    #[inline]
    pub fn make_host_time_from_system_units(host_time: u64) -> CMTime {
        unsafe { CMClockMakeHostTimeFromSystemUnits(host_time) }
    }

    #[inline]
    pub fn get_time(&self) -> CMTime {
        unsafe { CMClockGetTime(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_anchor_time(&self) -> Result<(CMTime, CMTime), OSStatus> {
        let mut clock_time = CMTime::default();
        let mut reference_clock_time = CMTime::default();
        let status = unsafe { CMClockGetAnchorTime(self.as_concrete_TypeRef(), &mut clock_time, &mut reference_clock_time) };
        if status == 0 {
            Ok((clock_time, reference_clock_time))
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn might_drift(&self, other_clock: &CMClock) -> bool {
        unsafe { CMClockMightDrift(self.as_concrete_TypeRef(), other_clock.0) != 0 }
    }

    #[inline]
    pub fn invalidate(&self) {
        unsafe { CMClockInvalidate(self.as_concrete_TypeRef()) }
    }
}

declare_TCFType!(CMTimebase, CMTimebaseRef);
impl_TCFType!(CMTimebase, CMTimebaseRef, CMTimebaseGetTypeID);
impl_CFTypeDescription!(CMTimebase);

impl CMTimebase {
    #[inline]
    pub fn new_with_source_clock(source_clock: &CMClock) -> Result<Self, OSStatus> {
        unsafe {
            let mut timebase: CMTimebaseRef = null_mut();
            let status = CMTimebaseCreateWithSourceClock(kCFAllocatorDefault, source_clock.0, &mut timebase);
            if status == 0 {
                Ok(TCFType::wrap_under_create_rule(timebase))
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn new_with_source_timebase(source_timebase: &CMTimebase) -> Result<Self, OSStatus> {
        unsafe {
            let mut timebase: CMTimebaseRef = null_mut();
            let status = CMTimebaseCreateWithSourceTimebase(kCFAllocatorDefault, source_timebase.0, &mut timebase);
            if status == 0 {
                Ok(TCFType::wrap_under_create_rule(timebase))
            } else {
                Err(status)
            }
        }
    }

    #[inline]
    pub fn copy_source_timebase(&self) -> Option<Self> {
        unsafe {
            let timebase = CMTimebaseCopySourceTimebase(self.as_concrete_TypeRef());
            if timebase.is_null() {
                Some(TCFType::wrap_under_create_rule(timebase))
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn copy_source_clock(&self) -> Option<CMClock> {
        unsafe {
            let clock = CMTimebaseCopySourceClock(self.as_concrete_TypeRef());
            if clock.is_null() {
                Some(TCFType::wrap_under_create_rule(clock))
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn copy_source(&self) -> Option<CMClockOrTimebase> {
        unsafe {
            let source = CMTimebaseCopySource(self.as_concrete_TypeRef());
            if source.is_null() {
                Some(CMClockOrTimebase::wrap_under_create_rule(source))
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn copy_ultimate_source_clock(&self) -> Option<CMClock> {
        unsafe {
            let clock = CMTimebaseCopyUltimateSourceClock(self.as_concrete_TypeRef());
            if clock.is_null() {
                Some(TCFType::wrap_under_create_rule(clock))
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn set_source_clock(&self, new_source_clock: &CMClock) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseSetSourceClock(self.as_concrete_TypeRef(), new_source_clock.0) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn set_source_timebase(&self, new_source_timebase: &CMTimebase) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseSetSourceTimebase(self.as_concrete_TypeRef(), new_source_timebase.0) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_time(&self) -> CMTime {
        unsafe { CMTimebaseGetTime(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_time_with_time_scale(&self, time_scale: CMTimeScale, rounding_method: CMTimeRoundingMethod) -> CMTime {
        unsafe { CMTimebaseGetTimeWithTimeScale(self.as_concrete_TypeRef(), time_scale, rounding_method) }
    }

    #[inline]
    pub fn set_time(&self, time: CMTime) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseSetTime(self.as_concrete_TypeRef(), time) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn set_anchor_time(&self, timebase_time: CMTime, immediate_source_time: CMTime) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseSetAnchorTime(self.as_concrete_TypeRef(), timebase_time, immediate_source_time) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_rate(&self) -> f64 {
        unsafe { CMTimebaseGetRate(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_time_and_rate(&self) -> Result<(CMTime, f64), OSStatus> {
        let mut time = CMTime::default();
        let mut rate = 0.0;
        let status = unsafe { CMTimebaseGetTimeAndRate(self.as_concrete_TypeRef(), &mut time, &mut rate) };
        if status == 0 {
            Ok((time, rate))
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn set_rate(&self, rate: f64) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseSetRate(self.as_concrete_TypeRef(), rate) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn set_rate_and_anchor_time(&self, rate: f64, timebase_time: CMTime, immediate_source_time: CMTime) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseSetRateAndAnchorTime(self.as_concrete_TypeRef(), rate, timebase_time, immediate_source_time) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_effective_rate(&self) -> f64 {
        unsafe { CMTimebaseGetEffectiveRate(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn add_timer(&self, timer: &CFRunLoopTimer, run_loop: &CFRunLoop) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseAddTimer(self.as_concrete_TypeRef(), timer.as_concrete_TypeRef(), run_loop.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn remove_timer(&self, timer: &CFRunLoopTimer) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseRemoveTimer(self.as_concrete_TypeRef(), timer.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn set_timer_next_fire_time(&self, timer: &CFRunLoopTimer, fire_time: CMTime, flags: u32) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseSetTimerNextFireTime(self.as_concrete_TypeRef(), timer.as_concrete_TypeRef(), fire_time, flags) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn set_timer_to_fire_immediately(&self, timer: &CFRunLoopTimer) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseSetTimerToFireImmediately(self.as_concrete_TypeRef(), timer.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn notification_barrier(&self) -> Result<(), OSStatus> {
        let status = unsafe { CMTimebaseNotificationBarrier(self.as_concrete_TypeRef()) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }
}

declare_TCFType!(CMClockOrTimebase, CMClockOrTimebaseRef);
impl_CFTypeDescription!(CMClockOrTimebase);

impl CMClockOrTimebase {
    #[inline]
    pub fn as_concrete_TypeRef(&self) -> CMClockOrTimebaseRef {
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
    pub unsafe fn wrap_under_create_rule(reference: CMClockOrTimebaseRef) -> Self {
        assert!(!reference.is_null(), "Attempted to create a NULL object.");
        CMClockOrTimebase(reference)
    }

    #[inline]
    pub unsafe fn wrap_under_get_rule(reference: CMClockOrTimebaseRef) -> Self {
        assert!(!reference.is_null(), "Attempted to create a NULL object.");
        let reference = CFRetain(reference);
        CMClockOrTimebase(reference)
    }
}

impl Clone for CMClockOrTimebase {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { CMClockOrTimebase::wrap_under_get_rule(CFRetain(self.as_CFTypeRef())) }
    }
}

impl PartialEq for CMClockOrTimebase {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_CFType().eq(&other.as_CFType())
    }
}

impl Eq for CMClockOrTimebase {}

impl CMClockOrTimebase {
    #[inline]
    pub fn get_relative_rate(&self, relative_to: &CMClockOrTimebase) -> f64 {
        unsafe { CMSyncGetRelativeRate(self.as_concrete_TypeRef(), relative_to.0) }
    }

    #[inline]
    pub fn get_relative_rate_and_anchor_time(&self, relative_to: &CMClockOrTimebase) -> Result<(f64, CMTime, CMTime), OSStatus> {
        let mut relative_rate = 0.0;
        let mut of_clock_or_timebase_anchor_time = CMTime::default();
        let mut relative_to_clock_or_timebase_anchor_time = CMTime::default();
        let status = unsafe {
            CMSyncGetRelativeRateAndAnchorTime(
                self.as_concrete_TypeRef(),
                relative_to.0,
                &mut relative_rate,
                &mut of_clock_or_timebase_anchor_time,
                &mut relative_to_clock_or_timebase_anchor_time,
            )
        };
        if status == 0 {
            Ok((relative_rate, of_clock_or_timebase_anchor_time, relative_to_clock_or_timebase_anchor_time))
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn convert_time(&self, time: CMTime, from: &CMClockOrTimebase, to: &CMClockOrTimebase) -> CMTime {
        unsafe { CMSyncConvertTime(time, from.0, to.0) }
    }

    #[inline]
    pub fn might_drift(&self, other: &CMClockOrTimebase) -> bool {
        unsafe { CMSyncMightDrift(self.as_concrete_TypeRef(), other.0) != 0 }
    }

    #[inline]
    pub fn get_time(&self) -> CMTime {
        unsafe { CMSyncGetTime(self.as_concrete_TypeRef()) }
    }
}
