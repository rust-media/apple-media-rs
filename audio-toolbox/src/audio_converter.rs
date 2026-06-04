use std::{mem, ptr::NonNull};

use core_audio_types::{AudioBufferList, AudioClassDescription, AudioStreamBasicDescription, AudioStreamPacketDescription};
use core_foundation::base::OSStatus;
use libc::c_void;
use objc2_audio_toolbox as sys;
pub use sys::{AudioConverterOptions, AudioConverterPropertyID};

use crate::base::status_to_result;

#[derive(Debug)]
pub struct AudioConverter {
    raw: NonNull<sys::OpaqueAudioConverter>,
}

impl AudioConverter {
    #[inline]
    pub unsafe fn from_raw(raw: sys::AudioConverterRef) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self {
            raw,
        })
    }

    #[inline]
    pub fn as_raw(&self) -> sys::AudioConverterRef {
        self.raw.as_ptr()
    }

    #[inline]
    pub fn into_raw(self) -> sys::AudioConverterRef {
        let raw = self.as_raw();
        mem::forget(self);
        raw
    }

    #[inline]
    pub fn new(source_format: &AudioStreamBasicDescription, destination_format: &AudioStreamBasicDescription) -> Result<Self, OSStatus> {
        let mut converter = std::ptr::null_mut();
        let status =
            unsafe { sys::AudioConverterNew(NonNull::from(source_format), NonNull::from(destination_format), NonNull::from(&mut converter)) };
        if status == 0 {
            unsafe { Self::from_raw(converter).ok_or(sys::kAudioConverterErr_UnspecifiedError) }
        } else {
            Err(status)
        }
    }

    pub fn new_specific(
        source_format: &AudioStreamBasicDescription,
        destination_format: &AudioStreamBasicDescription,
        class_descriptions: &[AudioClassDescription],
    ) -> Result<Self, OSStatus> {
        let mut converter = std::ptr::null_mut();
        let class_description_count = class_descriptions.len() as u32;
        let class_descriptions = NonNull::new(class_descriptions.as_ptr() as *mut AudioClassDescription).unwrap_or_else(NonNull::dangling);
        let status = unsafe {
            sys::AudioConverterNewSpecific(
                NonNull::from(source_format),
                NonNull::from(destination_format),
                class_description_count,
                class_descriptions,
                NonNull::from(&mut converter),
            )
        };
        if status == 0 {
            unsafe { Self::from_raw(converter).ok_or(sys::kAudioConverterErr_UnspecifiedError) }
        } else {
            Err(status)
        }
    }

    pub fn new_with_options(
        source_format: &AudioStreamBasicDescription,
        destination_format: &AudioStreamBasicDescription,
        options: AudioConverterOptions,
    ) -> Result<Self, OSStatus> {
        let mut converter = std::ptr::null_mut();
        let status = unsafe {
            sys::AudioConverterNewWithOptions(NonNull::from(source_format), NonNull::from(destination_format), options, NonNull::from(&mut converter))
        };
        if status == 0 {
            unsafe { Self::from_raw(converter).ok_or(sys::kAudioConverterErr_UnspecifiedError) }
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn reset(&self) -> Result<(), OSStatus> {
        let status = unsafe { sys::AudioConverterReset(self.as_raw()) };
        status_to_result(status)
    }

    pub fn property_info(&self, property_id: AudioConverterPropertyID) -> Result<(u32, bool), OSStatus> {
        let mut size = 0;
        let mut writable = 0;
        let status = unsafe { sys::AudioConverterGetPropertyInfo(self.as_raw(), property_id, &mut size, &mut writable) };
        if status == 0 {
            Ok((size, writable != 0))
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_property<T: Copy>(&self, property_id: AudioConverterPropertyID) -> Result<T, OSStatus> {
        let mut value = mem::MaybeUninit::<T>::uninit();
        let mut size = mem::size_of::<T>() as u32;
        let status = unsafe {
            sys::AudioConverterGetProperty(self.as_raw(), property_id, NonNull::from(&mut size), NonNull::new_unchecked(value.as_mut_ptr().cast()))
        };
        if status == 0 {
            Ok(unsafe { value.assume_init() })
        } else {
            Err(status)
        }
    }

    pub fn get_property_bytes(&self, property_id: AudioConverterPropertyID) -> Result<Vec<u8>, OSStatus> {
        let (size, _) = self.property_info(property_id)?;
        let mut bytes = vec![0; size as usize];
        let mut io_size = size;
        let out_data = NonNull::new(bytes.as_mut_ptr().cast::<c_void>()).unwrap_or_else(NonNull::dangling);
        let status = unsafe { sys::AudioConverterGetProperty(self.as_raw(), property_id, NonNull::from(&mut io_size), out_data) };
        if status == 0 {
            bytes.truncate(io_size as usize);
            Ok(bytes)
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn set_property<T: Copy>(&self, property_id: AudioConverterPropertyID, value: &T) -> Result<(), OSStatus> {
        let status = unsafe { sys::AudioConverterSetProperty(self.as_raw(), property_id, mem::size_of::<T>() as u32, NonNull::from(value).cast()) };
        status_to_result(status)
    }

    pub fn set_property_bytes(&self, property_id: AudioConverterPropertyID, bytes: &[u8]) -> Result<(), OSStatus> {
        let data = NonNull::new(bytes.as_ptr() as *mut c_void).unwrap_or_else(NonNull::dangling);
        let status = unsafe { sys::AudioConverterSetProperty(self.as_raw(), property_id, bytes.len() as u32, data) };
        status_to_result(status)
    }

    pub fn convert_buffer(&self, input: &[u8], output: &mut [u8]) -> Result<usize, OSStatus> {
        let input_data = NonNull::new(input.as_ptr() as *mut c_void).unwrap_or_else(NonNull::dangling);
        let output_data = NonNull::new(output.as_mut_ptr().cast::<c_void>()).unwrap_or_else(NonNull::dangling);
        let mut output_size = output.len() as u32;
        let status =
            unsafe { sys::AudioConverterConvertBuffer(self.as_raw(), input.len() as u32, input_data, NonNull::from(&mut output_size), output_data) };
        if status == 0 {
            Ok(output_size as usize)
        } else {
            Err(status)
        }
    }

    pub unsafe fn convert_complex_buffer(
        &self,
        number_pcm_frames: u32,
        input_data: &mut AudioBufferList,
        output_data: &mut AudioBufferList,
    ) -> Result<(), OSStatus> {
        let status = sys::AudioConverterConvertComplexBuffer(self.as_raw(), number_pcm_frames, NonNull::from(input_data), NonNull::from(output_data));
        status_to_result(status)
    }

    pub unsafe fn fill_complex_buffer(
        &self,
        input_data_proc: sys::AudioConverterComplexInputDataProc,
        input_data_proc_user_data: *mut c_void,
        output_data_packet_size: &mut u32,
        output_data: &mut AudioBufferList,
        packet_descriptions: Option<&mut [AudioStreamPacketDescription]>,
    ) -> Result<(), OSStatus> {
        let packet_descriptions = packet_descriptions.map_or(std::ptr::null_mut(), |descriptions| descriptions.as_mut_ptr());
        let status = sys::AudioConverterFillComplexBuffer(
            self.as_raw(),
            input_data_proc,
            input_data_proc_user_data,
            NonNull::from(output_data_packet_size),
            NonNull::from(output_data),
            packet_descriptions,
        );
        status_to_result(status)
    }
}

impl Drop for AudioConverter {
    fn drop(&mut self) {
        unsafe {
            sys::AudioConverterDispose(self.as_raw());
        }
    }
}
