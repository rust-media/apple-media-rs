use std::{mem, ptr::NonNull};

use core_audio_types::{AudioBufferList, AudioTimeStamp};
use core_foundation::base::OSStatus;
use libc::c_void;
use objc2_audio_toolbox as sys;
pub use sys::{AudioUnitElement, AudioUnitParameterID, AudioUnitParameterValue, AudioUnitPropertyID, AudioUnitRenderActionFlags, AudioUnitScope};

use crate::{audio_component::ComponentInstance, base::status_to_result};

pub type Unit = ComponentInstance;

impl Unit {
    #[inline]
    pub fn initialize(&self) -> Result<(), OSStatus> {
        let status = unsafe { sys::AudioUnitInitialize(self.as_raw()) };
        status_to_result(status)
    }

    #[inline]
    pub fn uninitialize(&self) -> Result<(), OSStatus> {
        let status = unsafe { sys::AudioUnitUninitialize(self.as_raw()) };
        status_to_result(status)
    }

    #[inline]
    pub fn start_output_unit(&self) -> Result<(), OSStatus> {
        let status = unsafe { sys::AudioOutputUnitStart(self.as_raw()) };
        status_to_result(status)
    }

    #[inline]
    pub fn stop_output_unit(&self) -> Result<(), OSStatus> {
        let status = unsafe { sys::AudioOutputUnitStop(self.as_raw()) };
        status_to_result(status)
    }

    pub fn property_info(&self, property_id: AudioUnitPropertyID, scope: AudioUnitScope, element: AudioUnitElement) -> Result<(u32, bool), OSStatus> {
        let mut size: u32 = 0;
        let mut writable: u8 = 0;
        let status = unsafe { sys::AudioUnitGetPropertyInfo(self.as_raw(), property_id, scope, element, &mut size, &mut writable) };
        status_to_result(status).map(|_| (size, writable != 0))
    }

    #[inline]
    pub fn get_property<T: Copy>(&self, property_id: AudioUnitPropertyID, scope: AudioUnitScope, element: AudioUnitElement) -> Result<T, OSStatus> {
        let mut value = mem::MaybeUninit::<T>::uninit();
        let mut size = mem::size_of::<T>() as u32;
        let status = unsafe {
            sys::AudioUnitGetProperty(
                self.as_raw(),
                property_id,
                scope,
                element,
                NonNull::new_unchecked(value.as_mut_ptr().cast()),
                NonNull::from(&mut size),
            )
        };
        status_to_result(status)?;
        Ok(unsafe { value.assume_init() })
    }

    pub fn get_property_bytes(
        &self,
        property_id: AudioUnitPropertyID,
        scope: AudioUnitScope,
        element: AudioUnitElement,
    ) -> Result<Vec<u8>, OSStatus> {
        let (size, _) = self.property_info(property_id, scope, element)?;
        let mut bytes = vec![0u8; size as usize];
        let mut io_size = size;
        let out_data = NonNull::new(bytes.as_mut_ptr().cast::<c_void>()).unwrap_or_else(NonNull::dangling);
        let status = unsafe { sys::AudioUnitGetProperty(self.as_raw(), property_id, scope, element, out_data, NonNull::from(&mut io_size)) };
        status_to_result(status)?;
        bytes.truncate(io_size as usize);
        Ok(bytes)
    }

    #[inline]
    pub fn set_property<T: Copy>(
        &self,
        property_id: AudioUnitPropertyID,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        value: &T,
    ) -> Result<(), OSStatus> {
        let status = unsafe {
            sys::AudioUnitSetProperty(self.as_raw(), property_id, scope, element, (value as *const T).cast::<c_void>(), mem::size_of::<T>() as u32)
        };
        status_to_result(status)
    }

    pub fn set_property_bytes(
        &self,
        property_id: AudioUnitPropertyID,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        bytes: &[u8],
    ) -> Result<(), OSStatus> {
        let status =
            unsafe { sys::AudioUnitSetProperty(self.as_raw(), property_id, scope, element, bytes.as_ptr().cast::<c_void>(), bytes.len() as u32) };
        status_to_result(status)
    }

    #[inline]
    pub fn clear_property(&self, property_id: AudioUnitPropertyID, scope: AudioUnitScope, element: AudioUnitElement) -> Result<(), OSStatus> {
        let status = unsafe { sys::AudioUnitSetProperty(self.as_raw(), property_id, scope, element, std::ptr::null(), 0) };
        status_to_result(status)
    }

    pub fn get_parameter(
        &self,
        parameter_id: AudioUnitParameterID,
        scope: AudioUnitScope,
        element: AudioUnitElement,
    ) -> Result<AudioUnitParameterValue, OSStatus> {
        let mut value: AudioUnitParameterValue = 0.0;
        let status = unsafe { sys::AudioUnitGetParameter(self.as_raw(), parameter_id, scope, element, NonNull::from(&mut value)) };
        status_to_result(status).map(|_| value)
    }

    #[inline]
    pub fn set_parameter(
        &self,
        parameter_id: AudioUnitParameterID,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        value: AudioUnitParameterValue,
        buffer_offset_in_frames: u32,
    ) -> Result<(), OSStatus> {
        let status = unsafe { sys::AudioUnitSetParameter(self.as_raw(), parameter_id, scope, element, value, buffer_offset_in_frames) };
        status_to_result(status)
    }

    #[inline]
    pub fn reset(&self, scope: AudioUnitScope, element: AudioUnitElement) -> Result<(), OSStatus> {
        let status = unsafe { sys::AudioUnitReset(self.as_raw(), scope, element) };
        status_to_result(status)
    }

    pub unsafe fn render(
        &self,
        action_flags: Option<&mut AudioUnitRenderActionFlags>,
        time_stamp: &AudioTimeStamp,
        output_bus_number: u32,
        number_frames: u32,
        io_data: &mut AudioBufferList,
    ) -> Result<(), OSStatus> {
        let action_flags = action_flags.map_or(std::ptr::null_mut(), |flags| flags as *mut AudioUnitRenderActionFlags);
        let status =
            sys::AudioUnitRender(self.as_raw(), action_flags, NonNull::from(time_stamp), output_bus_number, number_frames, NonNull::from(io_data));
        status_to_result(status)
    }
}
