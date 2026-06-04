use std::ptr::{null_mut, NonNull};

use core_foundation::base::OSStatus;
use objc2_audio_toolbox as sys;
pub use sys::{AudioComponentDescription, AudioComponentFlags, AudioComponentInstantiationOptions};

use crate::base::status_to_result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Component {
    raw: NonNull<sys::OpaqueAudioComponent>,
}

impl Component {
    #[inline]
    pub unsafe fn from_raw(raw: sys::AudioComponent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self {
            raw,
        })
    }

    #[inline]
    pub fn as_raw(self) -> sys::AudioComponent {
        self.raw.as_ptr()
    }

    #[inline]
    pub fn find_next(previous: Option<Component>, description: &AudioComponentDescription) -> Option<Component> {
        let raw = unsafe { sys::AudioComponentFindNext(previous.map_or(null_mut(), Component::as_raw), NonNull::from(description)) };
        unsafe { Component::from_raw(raw) }
    }

    #[inline]
    pub fn count(description: &AudioComponentDescription) -> u32 {
        unsafe { sys::AudioComponentCount(NonNull::from(description)) }
    }

    pub fn description(self) -> Result<AudioComponentDescription, OSStatus> {
        let mut description = AudioComponentDescription {
            componentType: 0,
            componentSubType: 0,
            componentManufacturer: 0,
            componentFlags: 0,
            componentFlagsMask: 0,
        };
        let status = unsafe { sys::AudioComponentGetDescription(self.as_raw(), NonNull::from(&mut description)) };
        status_to_result(status).map(|_| description)
    }

    pub fn version(self) -> Result<u32, OSStatus> {
        let mut version = 0;
        let status = unsafe { sys::AudioComponentGetVersion(self.as_raw(), NonNull::from(&mut version)) };
        status_to_result(status).map(|_| version)
    }

    pub fn instantiate(self) -> Result<ComponentInstance, OSStatus> {
        let mut instance = null_mut();
        let status = unsafe { sys::AudioComponentInstanceNew(self.as_raw(), NonNull::from(&mut instance)) };
        status_to_result(status)?;
        unsafe { ComponentInstance::from_raw(instance).ok_or(sys::kAudioComponentErr_InstanceInvalidated) }
    }
}

#[derive(Debug)]
pub struct ComponentInstance {
    raw: NonNull<sys::OpaqueAudioComponentInstance>,
}

impl ComponentInstance {
    #[inline]
    pub unsafe fn from_raw(raw: sys::AudioComponentInstance) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self {
            raw,
        })
    }

    #[inline]
    pub fn as_raw(&self) -> sys::AudioComponentInstance {
        self.raw.as_ptr()
    }

    #[inline]
    pub fn into_raw(self) -> sys::AudioComponentInstance {
        let raw = self.as_raw();
        std::mem::forget(self);
        raw
    }

    #[inline]
    pub fn component(&self) -> Option<Component> {
        let raw = unsafe { sys::AudioComponentInstanceGetComponent(self.as_raw()) };
        unsafe { Component::from_raw(raw) }
    }

    #[inline]
    pub fn can_do(&self, selector_id: i16) -> bool {
        unsafe { sys::AudioComponentInstanceCanDo(self.as_raw(), selector_id) }
    }
}

impl Drop for ComponentInstance {
    fn drop(&mut self) {
        unsafe {
            sys::AudioComponentInstanceDispose(self.as_raw());
        }
    }
}
