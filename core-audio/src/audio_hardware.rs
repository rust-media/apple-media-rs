use std::{mem, ptr::NonNull};

use core_foundation::base::OSStatus;
use libc::c_void;
use objc2_core_audio as sys;
pub use sys::{
    AudioClassID, AudioObjectID, AudioObjectPropertyAddress, AudioObjectPropertyElement, AudioObjectPropertyScope, AudioObjectPropertySelector,
};

use crate::base::status_to_result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioObject {
    id: AudioObjectID,
}

impl AudioObject {
    #[inline]
    pub const fn new(id: AudioObjectID) -> Self {
        Self {
            id,
        }
    }

    #[inline]
    pub const fn system() -> Self {
        Self::new(sys::kAudioObjectSystemObject as AudioObjectID)
    }

    #[inline]
    pub const fn unknown() -> Self {
        Self::new(sys::kAudioObjectUnknown)
    }

    #[inline]
    pub const fn id(self) -> AudioObjectID {
        self.id
    }

    #[inline]
    pub fn show(self) {
        unsafe { sys::AudioObjectShow(self.id) }
    }

    #[inline]
    pub fn has_property(self, address: &AudioObjectPropertyAddress) -> bool {
        unsafe { sys::AudioObjectHasProperty(self.id, NonNull::from(address)) }
    }

    #[inline]
    pub fn is_property_settable(self, address: &AudioObjectPropertyAddress) -> Result<bool, OSStatus> {
        let mut is_settable = 0;
        let status = unsafe { sys::AudioObjectIsPropertySettable(self.id, NonNull::from(address), NonNull::from(&mut is_settable)) };
        status_to_result(status).map(|_| is_settable != 0)
    }

    #[inline]
    pub fn property_data_size(self, address: &AudioObjectPropertyAddress) -> Result<u32, OSStatus> {
        self.property_data_size_with_qualifier(address, &[])
    }

    pub fn property_data_size_with_qualifier(self, address: &AudioObjectPropertyAddress, qualifier: &[u8]) -> Result<u32, OSStatus> {
        let mut data_size = 0;
        let status = unsafe {
            sys::AudioObjectGetPropertyDataSize(
                self.id,
                NonNull::from(address),
                qualifier.len() as u32,
                qualifier.as_ptr().cast(),
                NonNull::from(&mut data_size),
            )
        };
        status_to_result(status).map(|_| data_size)
    }

    #[inline]
    pub fn get_property<T: Copy>(self, address: &AudioObjectPropertyAddress) -> Result<T, OSStatus> {
        self.get_property_with_qualifier(address, &[])
    }

    pub fn get_property_with_qualifier<T: Copy>(self, address: &AudioObjectPropertyAddress, qualifier: &[u8]) -> Result<T, OSStatus> {
        let mut value = mem::MaybeUninit::<T>::uninit();
        let mut data_size = mem::size_of::<T>() as u32;
        let status = unsafe {
            sys::AudioObjectGetPropertyData(
                self.id,
                NonNull::from(address),
                qualifier.len() as u32,
                qualifier.as_ptr().cast(),
                NonNull::from(&mut data_size),
                NonNull::new_unchecked(value.as_mut_ptr().cast()),
            )
        };
        status_to_result(status)?;
        Ok(unsafe { value.assume_init() })
    }

    #[inline]
    pub fn get_property_bytes(self, address: &AudioObjectPropertyAddress) -> Result<Vec<u8>, OSStatus> {
        self.get_property_bytes_with_qualifier(address, &[])
    }

    pub fn get_property_bytes_with_qualifier(self, address: &AudioObjectPropertyAddress, qualifier: &[u8]) -> Result<Vec<u8>, OSStatus> {
        let mut bytes = vec![0; self.property_data_size_with_qualifier(address, qualifier)? as usize];
        let mut data_size = bytes.len() as u32;
        let out_data = NonNull::new(bytes.as_mut_ptr().cast::<c_void>()).unwrap_or_else(NonNull::dangling);
        let status = unsafe {
            sys::AudioObjectGetPropertyData(
                self.id,
                NonNull::from(address),
                qualifier.len() as u32,
                qualifier.as_ptr().cast(),
                NonNull::from(&mut data_size),
                out_data,
            )
        };
        status_to_result(status)?;
        bytes.truncate(data_size as usize);
        Ok(bytes)
    }

    #[inline]
    pub fn get_property_array<T: Copy>(self, address: &AudioObjectPropertyAddress) -> Result<Vec<T>, OSStatus> {
        self.get_property_array_with_qualifier(address, &[])
    }

    pub fn get_property_array_with_qualifier<T: Copy>(self, address: &AudioObjectPropertyAddress, qualifier: &[u8]) -> Result<Vec<T>, OSStatus> {
        let element_size = mem::size_of::<T>();
        let data_size = self.property_data_size_with_qualifier(address, qualifier)? as usize;
        if element_size == 0 || !data_size.is_multiple_of(element_size) {
            return Err(sys::kAudioHardwareBadPropertySizeError);
        }

        let mut values = Vec::<T>::with_capacity(data_size / element_size);
        let mut io_data_size = data_size as u32;
        let out_data = NonNull::new(values.as_mut_ptr().cast::<c_void>()).unwrap_or_else(NonNull::dangling);
        let status = unsafe {
            sys::AudioObjectGetPropertyData(
                self.id,
                NonNull::from(address),
                qualifier.len() as u32,
                qualifier.as_ptr().cast(),
                NonNull::from(&mut io_data_size),
                out_data,
            )
        };
        status_to_result(status)?;
        if !(io_data_size as usize).is_multiple_of(element_size) {
            return Err(sys::kAudioHardwareBadPropertySizeError);
        }
        unsafe { values.set_len(io_data_size as usize / element_size) };
        Ok(values)
    }

    #[inline]
    pub fn set_property<T: Copy>(self, address: &AudioObjectPropertyAddress, value: &T) -> Result<(), OSStatus> {
        self.set_property_with_qualifier(address, &[], value)
    }

    pub fn set_property_with_qualifier<T: Copy>(self, address: &AudioObjectPropertyAddress, qualifier: &[u8], value: &T) -> Result<(), OSStatus> {
        let status = unsafe {
            sys::AudioObjectSetPropertyData(
                self.id,
                NonNull::from(address),
                qualifier.len() as u32,
                qualifier.as_ptr().cast(),
                mem::size_of::<T>() as u32,
                NonNull::from(value).cast(),
            )
        };
        status_to_result(status)
    }

    #[inline]
    pub fn set_property_bytes(self, address: &AudioObjectPropertyAddress, bytes: &[u8]) -> Result<(), OSStatus> {
        self.set_property_bytes_with_qualifier(address, &[], bytes)
    }

    pub fn set_property_bytes_with_qualifier(self, address: &AudioObjectPropertyAddress, qualifier: &[u8], bytes: &[u8]) -> Result<(), OSStatus> {
        let data = NonNull::new(bytes.as_ptr() as *mut c_void).unwrap_or_else(NonNull::dangling);
        let status = unsafe {
            sys::AudioObjectSetPropertyData(
                self.id,
                NonNull::from(address),
                qualifier.len() as u32,
                qualifier.as_ptr().cast(),
                bytes.len() as u32,
                data,
            )
        };
        status_to_result(status)
    }
}

impl From<AudioObjectID> for AudioObject {
    #[inline]
    fn from(id: AudioObjectID) -> Self {
        Self::new(id)
    }
}

#[inline]
pub const fn property_address(
    selector: AudioObjectPropertySelector,
    scope: AudioObjectPropertyScope,
    element: AudioObjectPropertyElement,
) -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: selector,
        mScope: scope,
        mElement: element,
    }
}

#[inline]
pub const fn global_property_address(selector: AudioObjectPropertySelector) -> AudioObjectPropertyAddress {
    property_address(selector, sys::kAudioObjectPropertyScopeGlobal, sys::kAudioObjectPropertyElementMain)
}

#[inline]
pub const fn input_property_address(selector: AudioObjectPropertySelector) -> AudioObjectPropertyAddress {
    property_address(selector, sys::kAudioObjectPropertyScopeInput, sys::kAudioObjectPropertyElementMain)
}

#[inline]
pub const fn output_property_address(selector: AudioObjectPropertySelector) -> AudioObjectPropertyAddress {
    property_address(selector, sys::kAudioObjectPropertyScopeOutput, sys::kAudioObjectPropertyElementMain)
}
