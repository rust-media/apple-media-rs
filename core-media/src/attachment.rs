use std::mem;

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFGetTypeID, CFRetain, CFType, CFTypeID, CFTypeRef, TCFType, TCFTypeRef},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription,
    string::{CFString, CFStringRef},
};

pub type CMAttachmentBearerRef = CFTypeRef;

pub type CMAttachmentMode = u32;

pub const kCMAttachmentMode_ShouldNotPropagate: CMAttachmentMode = 0;
pub const kCMAttachmentMode_ShouldPropagate: CMAttachmentMode = 1;

extern "C" {
    pub fn CMSetAttachment(target: CMAttachmentBearerRef, key: CFStringRef, value: CFTypeRef, attachmentMode: CMAttachmentMode);
    pub fn CMGetAttachment(target: CMAttachmentBearerRef, key: CFStringRef, attachmentModeOut: *mut CMAttachmentMode) -> CFTypeRef;
    pub fn CMRemoveAttachment(target: CMAttachmentBearerRef, key: CFStringRef);
    pub fn CMRemoveAllAttachments(target: CMAttachmentBearerRef);
    pub fn CMCopyDictionaryOfAttachments(
        allocator: CFAllocatorRef,
        target: CMAttachmentBearerRef,
        attachmentMode: CMAttachmentMode,
    ) -> CFDictionaryRef;
    pub fn CMSetAttachments(target: CMAttachmentBearerRef, theAttachments: CFDictionaryRef, attachmentMode: CMAttachmentMode);
    pub fn CMPropagateAttachments(source: CMAttachmentBearerRef, destination: CMAttachmentBearerRef);
}

pub trait CMAttachmentBearerSubClass: TCFType {
    #[inline]
    fn as_CMAttachmentBearer(&self) -> CMAttachmentBearer {
        unsafe { CMAttachmentBearer::wrap_under_get_rule(self.as_concrete_TypeRef().as_void_ptr()) }
    }

    #[inline]
    fn into_CMAttachmentBearer(self) -> CMAttachmentBearer
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef().as_void_ptr();
        mem::forget(self);
        unsafe { CMAttachmentBearer::wrap_under_create_rule(reference) }
    }
}

declare_TCFType!(CMAttachmentBearer, CMAttachmentBearerRef);
impl_CFTypeDescription!(CMAttachmentBearer);

impl CMAttachmentBearer {
    #[inline]
    pub fn as_concrete_TypeRef(&self) -> CMAttachmentBearerRef {
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
    pub unsafe fn wrap_under_create_rule(reference: CMAttachmentBearerRef) -> Self {
        CMAttachmentBearer(reference)
    }

    #[inline]
    pub unsafe fn wrap_under_get_rule(reference: CMAttachmentBearerRef) -> Self {
        let reference = CFRetain(reference);
        CMAttachmentBearer(reference)
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

impl Clone for CMAttachmentBearer {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { CMAttachmentBearer::wrap_under_get_rule(self.as_concrete_TypeRef()) }
    }
}

impl PartialEq for CMAttachmentBearer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_CFType().eq(&other.as_CFType())
    }
}

impl Eq for CMAttachmentBearer {}

impl CMAttachmentBearer {
    #[inline]
    pub fn downcast<T: CMAttachmentBearerSubClass>(&self) -> Option<T> {
        if self.instance_of::<T>() {
            Some(unsafe { T::wrap_under_get_rule(T::Ref::from_void_ptr(self.as_concrete_TypeRef())) })
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_into<T: CMAttachmentBearerSubClass>(self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let reference = T::Ref::from_void_ptr(self.as_concrete_TypeRef());
                mem::forget(self);
                Some(T::wrap_under_create_rule(reference))
            }
        } else {
            None
        }
    }
}

impl CMAttachmentBearer {
    #[inline]
    pub fn set_attachment(&self, key: &CFString, value: &CFType, attachment_mode: CMAttachmentMode) {
        unsafe { CMSetAttachment(self.as_concrete_TypeRef(), key.as_concrete_TypeRef(), value.as_CFTypeRef(), attachment_mode) }
    }

    #[inline]
    pub fn get_attachment(&self, key: &CFString) -> Option<(CFType, CMAttachmentMode)> {
        unsafe {
            let mut attachment_mode = 0;
            let value = CMGetAttachment(self.as_concrete_TypeRef(), key.as_concrete_TypeRef(), &mut attachment_mode);
            if value.is_null() {
                None
            } else {
                Some((TCFType::wrap_under_create_rule(value), attachment_mode))
            }
        }
    }

    #[inline]
    pub fn remove_attachment(&self, key: &CFString) {
        unsafe { CMRemoveAttachment(self.as_concrete_TypeRef(), key.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn remove_all_attachments(&self) {
        unsafe { CMRemoveAllAttachments(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn copy_dictionary_of_attachments(&self, attachment_mode: CMAttachmentMode) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let dict = CMCopyDictionaryOfAttachments(kCFAllocatorDefault, self.as_concrete_TypeRef(), attachment_mode);
            if dict.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(dict))
            }
        }
    }

    #[inline]
    pub fn set_attachments(&self, the_attachments: &CFDictionary<CFString, CFType>, attachment_mode: CMAttachmentMode) {
        unsafe { CMSetAttachments(self.as_concrete_TypeRef(), the_attachments.as_concrete_TypeRef(), attachment_mode) }
    }

    #[inline]
    pub fn propagate_attachments(&self, destination: &mut CMAttachmentBearer) {
        unsafe { CMPropagateAttachments(self.as_concrete_TypeRef(), destination.0) }
    }
}
