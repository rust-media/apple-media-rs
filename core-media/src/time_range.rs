use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    number::CFNumber,
    string::{CFString, CFStringRef},
};

use crate::time::CMTime;

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CMTimeRange {
    pub start: CMTime,
    pub duration: CMTime,
}

extern "C" {
    pub static kCMTimeRangeZero: CMTimeRange;
    pub static kCMTimeRangeInvalid: CMTimeRange;

    pub fn CMTimeRangeMake(start: CMTime, duration: CMTime) -> CMTimeRange;
    pub fn CMTimeRangeGetUnion(range: CMTimeRange, otherRange: CMTimeRange) -> CMTimeRange;
    pub fn CMTimeRangeGetIntersection(range: CMTimeRange, otherRange: CMTimeRange) -> CMTimeRange;
    pub fn CMTimeRangeEqual(range1: CMTimeRange, range2: CMTimeRange) -> Boolean;
    pub fn CMTimeRangeContainsTime(range: CMTimeRange, time: CMTime) -> Boolean;
    pub fn CMTimeRangeContainsTimeRange(range: CMTimeRange, otherRange: CMTimeRange) -> Boolean;
    pub fn CMTimeRangeGetEnd(range: CMTimeRange) -> CMTime;
    pub fn CMTimeRangeFromRangeToRange(t: CMTime, fromRange: CMTimeRange, toRange: CMTimeRange) -> CMTime;
    pub fn CMTimeClampToRange(time: CMTime, range: CMTimeRange) -> CMTime;
    pub fn CMTimeMapDurationFromRangeToRange(dur: CMTime, fromRange: CMTimeRange, toRange: CMTimeRange) -> CMTime;
    pub fn CMTimeFoldIntoRange(time: CMTime, foldRange: CMTimeRange) -> CMTime;
    pub fn CMTimeRangeFromTimeToTime(start: CMTime, end: CMTime) -> CMTimeRange;
    pub fn CMTimeRangeCopyAsDictionary(range: CMTimeRange, allocator: CFAllocatorRef) -> CFDictionaryRef;
    pub fn CMTimeRangeMakeFromDictionary(dict: CFDictionaryRef) -> CMTimeRange;

    pub static kCMTimeRangeStartKey: CFStringRef;
    pub static kCMTimeRangeDurationKey: CFStringRef;

    pub fn CMTimeRangeCopyDescription(allocator: CFAllocatorRef, range: CMTimeRange) -> CFStringRef;
    pub fn CMTimeRangeShow(range: CMTimeRange);
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMTimeMapping {
    pub source: CMTimeRange,
    pub target: CMTimeRange,
}

extern "C" {
    pub static kCMTimeMappingInvalid: CMTimeMapping;

    pub fn CMTimeMappingMake(source: CMTimeRange, target: CMTimeRange) -> CMTimeMapping;
    pub fn CMTimeMappingMakeEmpty(target: CMTimeRange) -> CMTimeMapping;
    pub fn CMTimeMappingCopyAsDictionary(mapping: CMTimeMapping, allocator: CFAllocatorRef) -> CFDictionaryRef;
    pub fn CMTimeMappingMakeFromDictionary(dictionaryRepresentation: CFDictionaryRef) -> CMTimeMapping;

    pub static kCMTimeMappingSourceKey: CFStringRef;
    pub static kCMTimeMappingTargetKey: CFStringRef;

    pub fn CMTimeMappingCopyDescription(allocator: CFAllocatorRef, mapping: CMTimeMapping) -> CFStringRef;
    pub fn CMTimeMappingShow(mapping: CMTimeMapping);
}

impl PartialEq for CMTimeRange {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { CMTimeRangeEqual(*self, *other) != 0 }
    }
}

impl CMTimeRange {
    #[inline]
    pub fn make(start: CMTime, duration: CMTime) -> Self {
        unsafe { CMTimeRangeMake(start, duration) }
    }

    #[inline]
    pub fn get_union(&self, other: CMTimeRange) -> CMTimeRange {
        unsafe { CMTimeRangeGetUnion(*self, other) }
    }

    #[inline]
    pub fn get_intersection(&self, other: CMTimeRange) -> CMTimeRange {
        unsafe { CMTimeRangeGetIntersection(*self, other) }
    }

    #[inline]
    pub fn equal(&self, other: CMTimeRange) -> bool {
        unsafe { CMTimeRangeEqual(*self, other) != 0 }
    }

    #[inline]
    pub fn contains_time(&self, time: CMTime) -> bool {
        unsafe { CMTimeRangeContainsTime(*self, time) != 0 }
    }

    #[inline]
    pub fn contains_time_range(&self, other: CMTimeRange) -> bool {
        unsafe { CMTimeRangeContainsTimeRange(*self, other) != 0 }
    }

    #[inline]
    pub fn get_end(&self) -> CMTime {
        unsafe { CMTimeRangeGetEnd(*self) }
    }

    #[inline]
    pub fn copy_as_dictionary(&self) -> Option<CFDictionary<CFString, CFDictionary<CFString, CFNumber>>> {
        unsafe {
            let dict = CMTimeRangeCopyAsDictionary(*self, kCFAllocatorDefault);
            if dict.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(dict))
            }
        }
    }

    #[inline]
    pub fn make_from_dictionary(dict: &CFDictionary<CFString, CFDictionary<CFString, CFNumber>>) -> Self {
        unsafe { CMTimeRangeMakeFromDictionary(dict.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn copy_description(&self) -> Option<CFString> {
        unsafe {
            let description = CMTimeRangeCopyDescription(kCFAllocatorDefault, *self);
            if description.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(description))
            }
        }
    }

    #[inline]
    pub fn show(&self) {
        unsafe { CMTimeRangeShow(*self) }
    }
}

impl CMTimeMapping {
    #[inline]
    pub fn make(source: CMTimeRange, target: CMTimeRange) -> Self {
        unsafe { CMTimeMappingMake(source, target) }
    }

    #[inline]
    pub fn make_empty(target: CMTimeRange) -> Self {
        unsafe { CMTimeMappingMakeEmpty(target) }
    }

    #[inline]
    pub fn copy_as_dictionary(&self) -> Option<CFDictionary<CFString, CFDictionary<CFString, CFDictionary<CFString, CFNumber>>>> {
        unsafe {
            let dict = CMTimeMappingCopyAsDictionary(*self, kCFAllocatorDefault);
            if dict.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(dict))
            }
        }
    }

    #[inline]
    pub fn make_from_dictionary(dict: &CFDictionary<CFString, CFDictionary<CFString, CFDictionary<CFString, CFNumber>>>) -> Self {
        unsafe { CMTimeMappingMakeFromDictionary(dict.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn copy_description(&self) -> Option<CFString> {
        unsafe {
            let description = CMTimeMappingCopyDescription(kCFAllocatorDefault, *self);
            if description.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(description))
            }
        }
    }

    #[inline]
    pub fn show(&self) {
        unsafe { CMTimeMappingShow(*self) }
    }
}
