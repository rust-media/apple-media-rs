use std::{
    mem,
    ptr::{addr_of_mut, null_mut},
};

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFGetTypeID, CFRetain, CFType, CFTypeID, CFTypeRef, OSStatus, TCFType, TCFTypeRef},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription,
    string::{CFString, CFStringRef},
};
use libc::c_void;

pub type VTSessionRef = CFTypeRef;

pub type FrameDelay = i32;

pub const kVTUnlimitedFrameDelayCount: FrameDelay = -1;

extern "C" {
    pub fn VTSessionCopySupportedPropertyDictionary(session: VTSessionRef, supportedPropertyDictionaryOut: *mut CFDictionaryRef) -> OSStatus;

    pub static kVTPropertyTypeKey: CFStringRef;
    pub static kVTPropertyType_Enumeration: CFStringRef;
    pub static kVTPropertyType_Boolean: CFStringRef;
    pub static kVTPropertyType_Number: CFStringRef;
    pub static kVTPropertyReadWriteStatusKey: CFStringRef;
    pub static kVTPropertyReadWriteStatus_ReadOnly: CFStringRef;
    pub static kVTPropertyReadWriteStatus_ReadWrite: CFStringRef;
    pub static kVTPropertyShouldBeSerializedKey: CFStringRef;
    pub static kVTPropertySupportedValueMinimumKey: CFStringRef;
    pub static kVTPropertySupportedValueMaximumKey: CFStringRef;
    pub static kVTPropertySupportedValueListKey: CFStringRef;
    pub static kVTPropertyDocumentationKey: CFStringRef;

    pub fn VTSessionSetProperty(session: VTSessionRef, propertyKey: CFStringRef, propertyValue: CFTypeRef) -> OSStatus;
    pub fn VTSessionSetProperties(session: VTSessionRef, propertyDictionary: CFDictionaryRef) -> OSStatus;
    pub fn VTSessionCopyProperty(
        session: VTSessionRef,
        propertyKey: CFStringRef,
        allocator: CFAllocatorRef,
        propertyValueOut: *mut c_void,
    ) -> OSStatus;
    pub fn VTSessionCopySerializableProperties(session: VTSessionRef, allocator: CFAllocatorRef, dictionaryOut: *mut CFDictionaryRef) -> OSStatus;
}

declare_TCFType!(VTSession, VTSessionRef);
impl_CFTypeDescription!(VTSession);

impl VTSession {
    #[inline]
    pub fn as_concrete_TypeRef(&self) -> VTSessionRef {
        self.0
    }

    #[inline]
    pub fn as_CFType(&self) -> CFType {
        unsafe { CFType::wrap_under_get_rule(self.as_CFTypeRef()) }
    }

    #[inline]
    pub fn as_CFTypeRef(&self) -> CFTypeRef {
        self.as_concrete_TypeRef() as CFTypeRef
    }

    #[inline]
    pub fn into_CFType(self) -> CFType {
        let reference = self.as_CFTypeRef();
        mem::forget(self);
        unsafe { CFType::wrap_under_create_rule(reference) }
    }

    #[inline]
    pub unsafe fn wrap_under_create_rule(reference: VTSessionRef) -> VTSession {
        VTSession(reference)
    }

    #[inline]
    pub unsafe fn wrap_under_get_rule(reference: VTSessionRef) -> VTSession {
        VTSession(CFRetain(reference as CFTypeRef) as VTSessionRef)
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

impl Clone for VTSession {
    #[inline]
    fn clone(&self) -> VTSession {
        unsafe { VTSession::wrap_under_get_rule(self.0) }
    }
}

impl PartialEq for VTSession {
    #[inline]
    fn eq(&self, other: &VTSession) -> bool {
        self.as_CFType().eq(&other.as_CFType())
    }
}

pub trait TVTSession: TCFType {
    #[inline]
    fn as_session(&self) -> VTSession {
        unsafe { VTSession::wrap_under_get_rule(self.as_concrete_TypeRef().as_void_ptr() as VTSessionRef) }
    }

    #[inline]
    fn into_session(self) -> VTSession
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef().as_void_ptr() as VTSessionRef;
        mem::forget(self);
        unsafe { VTSession::wrap_under_create_rule(reference) }
    }
}

impl VTSession {
    #[inline]
    pub fn downcast<T: TVTSession>(&self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe { Some(T::wrap_under_get_rule(T::Ref::from_void_ptr(self.as_concrete_TypeRef().as_void_ptr()))) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_into<T: TVTSession>(self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let reference = T::Ref::from_void_ptr(self.as_concrete_TypeRef().as_void_ptr());
                mem::forget(self);
                Some(T::wrap_under_create_rule(reference))
            }
        } else {
            None
        }
    }
}

impl VTSession {
    pub fn copy_supported_property_dictionary(&self) -> Result<CFDictionary<CFString, CFType>, OSStatus> {
        unsafe {
            let mut dict: CFDictionaryRef = null_mut();
            let status = VTSessionCopySupportedPropertyDictionary(self.as_concrete_TypeRef(), &mut dict);
            if status == 0 {
                Ok(CFDictionary::wrap_under_create_rule(dict))
            } else {
                Err(status)
            }
        }
    }

    pub fn copy_property(&self, property_key: CFString) -> Result<CFType, OSStatus> {
        unsafe {
            let mut value: CFTypeRef = null_mut();
            let status = VTSessionCopyProperty(
                self.as_concrete_TypeRef(),
                property_key.as_concrete_TypeRef(),
                kCFAllocatorDefault,
                addr_of_mut!(value).cast(),
            );
            if status == 0 {
                Ok(CFType::wrap_under_create_rule(value))
            } else {
                Err(status)
            }
        }
    }

    pub fn copy_serializable_properties(&self) -> Result<CFDictionary<CFString, CFType>, OSStatus> {
        unsafe {
            let mut dict: CFDictionaryRef = null_mut();
            let status = VTSessionCopySerializableProperties(self.as_concrete_TypeRef(), kCFAllocatorDefault, &mut dict);
            if status == 0 {
                Ok(CFDictionary::wrap_under_create_rule(dict))
            } else {
                Err(status)
            }
        }
    }

    pub fn set_property(&self, property_key: CFString, property_value: CFType) -> Result<(), OSStatus> {
        unsafe {
            let status = VTSessionSetProperty(self.as_concrete_TypeRef(), property_key.as_concrete_TypeRef(), property_value.as_concrete_TypeRef());
            if status == 0 {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    pub fn set_properties(&self, property_dictionary: &CFDictionary<CFString, CFType>) -> Result<(), OSStatus> {
        unsafe {
            let status = VTSessionSetProperties(self.as_concrete_TypeRef(), property_dictionary.as_concrete_TypeRef());
            if status == 0 {
                Ok(())
            } else {
                Err(status)
            }
        }
    }
}
