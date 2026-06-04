use core_foundation::base::TCFType;
use core_media::format_description::{CMFormatDescription, CMFormatDescriptionRef};
use objc2::{
    extern_class, msg_send,
    rc::{Allocated, Retained},
    ClassType,
};
use objc2_foundation::{NSArray, NSError, NSObject, NSObjectProtocol, NSString};

use crate::{capture_device::AVCaptureDevice, media_format::AVMediaType};

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureInput;
);

unsafe impl NSObjectProtocol for AVCaptureInput {}

impl AVCaptureInput {
    pub fn ports(&self) -> Retained<NSArray<AVCaptureInputPort>> {
        unsafe { msg_send![self, ports] }
    }
}

extern "C" {
    pub static AVCaptureInputPortFormatDescriptionDidChangeNotification: &'static NSString;
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureInputPort;
);

unsafe impl NSObjectProtocol for AVCaptureInputPort {}

impl AVCaptureInputPort {
    pub fn media_type(&self) -> Retained<AVMediaType> {
        unsafe { msg_send![self, mediaType] }
    }

    pub fn format_description(&self) -> Option<CMFormatDescription> {
        unsafe {
            let format_description: CMFormatDescriptionRef = msg_send![self, formatDescription];
            if format_description.is_null() {
                None
            } else {
                Some(CMFormatDescription::wrap_under_get_rule(format_description))
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { msg_send![self, isEnabled] }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setEnabled: enabled] }
    }
}

extern_class!(
    #[unsafe(super(AVCaptureInput))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeviceInput;
);

unsafe impl NSObjectProtocol for AVCaptureDeviceInput {}

impl AVCaptureDeviceInput {
    pub fn from_device(device: &AVCaptureDevice) -> Result<Retained<Self>, Retained<NSError>> {
        unsafe { msg_send![Self::class(), deviceInputWithDevice: device, error: _] }
    }

    pub fn init_with_device(this: Allocated<Self>, device: &AVCaptureDevice) -> Result<Retained<Self>, Retained<NSError>> {
        unsafe { msg_send![this, initWithDevice: device, error: _] }
    }
}
