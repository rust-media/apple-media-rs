use core_foundation::base::TCFType;
use core_graphics::{base::CGFloat, geometry::CGPoint};
use core_media::{
    format_description::{CMFormatDescription, CMFormatDescriptionRef},
    time::CMTime,
};
#[cfg(target_os = "ios")]
use objc2::{
    encode::{Encode, Encoding, RefEncode},
    Encoding::Float,
};
use objc2::{extern_class, msg_send, msg_send_id, mutability::InteriorMutable, rc::Id, runtime::Bool, ClassType};
use objc2_foundation::{NSArray, NSError, NSInteger, NSNumber, NSObject, NSObjectProtocol, NSString};

use crate::{capture_session_preset::AVCaptureSessionPreset, media_format::AVMediaType};

extern "C" {
    pub static AVCaptureDeviceWasConnectedNotification: &'static NSString;
    pub static AVCaptureDeviceWasDisconnectedNotification: &'static NSString;
    pub static AVCaptureDeviceSubjectAreaDidChangeNotification: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDevice;

    unsafe impl ClassType for AVCaptureDevice {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDevice {}

impl AVCaptureDevice {
    pub fn devices() -> Id<NSArray<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), devices] }
    }

    pub fn devices_with_media_type(media_type: &AVMediaType) -> Id<NSArray<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), devicesWithMediaType: media_type] }
    }

    pub fn default_device_with_media_type(media_type: &AVMediaType) -> Option<Id<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), defaultDeviceWithMediaType: media_type] }
    }

    pub fn device_with_unique_id(unique_id: &NSString) -> Option<Id<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), deviceWithUniqueID: unique_id] }
    }

    pub fn unique_id(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, uniqueID] }
    }

    pub fn model_id(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, modelID] }
    }

    pub fn localized_name(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, localizedName] }
    }

    pub fn manufacturer(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, manufacturer] }
    }

    #[cfg(target_os = "macos")]
    pub fn transport_type(&self) -> i32 {
        unsafe { msg_send![self, transportType] }
    }

    pub fn has_media_type(&self, media_type: &AVMediaType) -> Bool {
        unsafe { msg_send![self, hasMediaType: media_type] }
    }

    pub fn lock_for_configuration(&self) -> Result<Bool, Id<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Bool = unsafe { msg_send![self, lockForConfiguration: &mut error] };
        if result.is_true() {
            Ok(result)
        } else {
            Err(unsafe { Id::retain(error).unwrap() })
        }
    }

    pub fn unlock_for_configuration(&self) {
        unsafe { msg_send![self, unlockForConfiguration] }
    }

    pub fn supports_av_capture_session_preset(&self, preset: &AVCaptureSessionPreset) -> Bool {
        unsafe { msg_send![self, supportsAVCaptureSessionPreset: preset] }
    }

    pub fn is_connected(&self) -> Bool {
        unsafe { msg_send![self, isConnected] }
    }

    #[cfg(target_os = "macos")]
    pub fn is_in_use_by_another_application(&self) -> Bool {
        unsafe { msg_send![self, isInUseByAnotherApplication] }
    }

    pub fn is_suspended(&self) -> Bool {
        unsafe { msg_send![self, isSuspended] }
    }

    #[cfg(target_os = "macos")]
    pub fn linked_devices(&self) -> Id<NSArray<AVCaptureDevice>> {
        unsafe { msg_send_id![self, linkedDevices] }
    }

    pub fn formats(&self) -> Id<NSArray<AVCaptureDeviceFormat>> {
        unsafe { msg_send_id![self, formats] }
    }

    pub fn get_active_format(&self) -> Id<AVCaptureDeviceFormat> {
        unsafe { msg_send_id![self, activeFormat] }
    }

    pub fn set_active_format(&self, format: &AVCaptureDeviceFormat) {
        unsafe { msg_send![self, setActiveFormat: format] }
    }

    pub fn get_active_video_min_frame_duration(&self) -> CMTime {
        unsafe { msg_send![self, activeVideoMinFrameDuration] }
    }

    pub fn set_active_video_min_frame_duration(&self, duration: CMTime) {
        unsafe { msg_send![self, setActiveVideoMinFrameDuration: duration] }
    }

    pub fn get_active_video_max_frame_duration(&self) -> CMTime {
        unsafe { msg_send![self, activeVideoMaxFrameDuration] }
    }

    pub fn set_active_video_max_frame_duration(&self, duration: CMTime) {
        unsafe { msg_send![self, setActiveVideoMaxFrameDuration: duration] }
    }

    pub fn input_sources(&self) -> Id<NSArray<AVCaptureDeviceInputSource>> {
        unsafe { msg_send_id![self, inputSources] }
    }

    pub fn get_active_input_source(&self) -> Id<AVCaptureDeviceInputSource> {
        unsafe { msg_send_id![self, activeInputSource] }
    }

    pub fn set_active_input_source(&self, input_source: &AVCaptureDeviceInputSource) {
        unsafe { msg_send![self, setActiveInputSource: input_source] }
    }
}

pub type AVCaptureDevicePosition = NSInteger;

pub const AVCaptureDevicePositionUnspecified: AVCaptureDevicePosition = 0;
pub const AVCaptureDevicePositionBack: AVCaptureDevicePosition = 1;
pub const AVCaptureDevicePositionFront: AVCaptureDevicePosition = 2;

// AVCaptureDevicePosition
impl AVCaptureDevice {
    pub fn position(&self) -> AVCaptureDevicePosition {
        unsafe { msg_send![self, position] }
    }
}

pub type AVCaptureDeviceType = NSString;

extern "C" {
    pub static AVCaptureDeviceTypeExternal: &'static AVCaptureDeviceType;
    pub static AVCaptureDeviceTypeMicrophone: &'static AVCaptureDeviceType;
    pub static AVCaptureDeviceTypeBuiltInWideAngleCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInTelephotoCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInUltraWideCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInDualCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInDualWideCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInTripleCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInTrueDepthCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInLiDARDepthCamera: &'static AVCaptureDeviceType;
    pub static AVCaptureDeviceTypeContinuityCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "macos")]
    pub static AVCaptureDeviceTypeDeskViewCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "macos")]
    pub static AVCaptureDeviceTypeExternalUnknown: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInDuoCamera: &'static AVCaptureDeviceType;
    pub static AVCaptureDeviceTypeBuiltInMicrophone: &'static AVCaptureDeviceType;
}

impl AVCaptureDevice {
    pub fn device_type(&self) -> Id<AVCaptureDeviceType> {
        unsafe { msg_send_id![self, deviceType] }
    }

    pub fn default_device_with_device_type(
        device_type: &AVCaptureDeviceType,
        media_type: &AVMediaType,
        position: AVCaptureDevicePosition,
    ) -> Option<Id<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), defaultDeviceWithDeviceType: device_type mediaType: media_type position: position] }
    }
}

pub type AVCaptureFlashMode = NSInteger;

pub const AVCaptureFlashModeOff: AVCaptureFlashMode = 0;
pub const AVCaptureFlashModeOn: AVCaptureFlashMode = 1;
pub const AVCaptureFlashModeAuto: AVCaptureFlashMode = 2;

// AVCaptureDeviceFlash
impl AVCaptureDevice {
    pub fn has_flash(&self) -> Bool {
        unsafe { msg_send![self, hasFlash] }
    }

    pub fn is_flash_available(&self) -> Bool {
        unsafe { msg_send![self, isFlashAvailable] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_flash_active(&self) -> Bool {
        unsafe { msg_send![self, isFlashActive] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_flash_mode_supported(&self, flash_mode: AVCaptureFlashMode) -> Bool {
        unsafe { msg_send![self, isFlashModeSupported: flash_mode] }
    }

    #[cfg(target_os = "ios")]
    pub fn flash_mode(&self) -> AVCaptureFlashMode {
        unsafe { msg_send![self, flashMode] }
    }

    #[cfg(target_os = "ios")]
    pub fn set_flash_mode(&self, flash_mode: AVCaptureFlashMode) {
        unsafe { msg_send![self, setFlashMode: flash_mode] }
    }
}

pub type AVCaptureTorchMode = NSInteger;

pub const AVCaptureTorchModeOff: AVCaptureTorchMode = 0;
pub const AVCaptureTorchModeOn: AVCaptureTorchMode = 1;
pub const AVCaptureTorchModeAuto: AVCaptureTorchMode = 2;

extern "C" {
    pub static AVCaptureMaxAvailableTorchLevel: &'static f32;
}

// AVCaptureDeviceTorch
impl AVCaptureDevice {
    pub fn has_torch(&self) -> Bool {
        unsafe { msg_send![self, hasTorch] }
    }

    pub fn is_torch_available(&self) -> Bool {
        unsafe { msg_send![self, isTorchAvailable] }
    }

    pub fn is_torch_active(&self) -> Bool {
        unsafe { msg_send![self, isTorchActive] }
    }

    pub fn torch_level(&self) -> f32 {
        unsafe { msg_send![self, torchLevel] }
    }

    pub fn is_torch_mode_supported(&self, torch_mode: AVCaptureTorchMode) -> Bool {
        unsafe { msg_send![self, isTorchModeSupported: torch_mode] }
    }

    pub fn torch_mode(&self) -> AVCaptureTorchMode {
        unsafe { msg_send![self, torchMode] }
    }

    pub fn set_torch_mode(&self, torch_mode: AVCaptureTorchMode) {
        unsafe { msg_send![self, setTorchMode: torch_mode] }
    }

    pub fn set_torch_mode_on_with_level(&self, torch_level: f32) -> Result<Bool, Id<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Bool = unsafe { msg_send![self, setTorchModeOnWithLevel: torch_level error: &mut error] };
        if result.is_true() {
            Ok(result)
        } else {
            Err(unsafe { Id::retain(error).unwrap() })
        }
    }
}

pub type AVCaptureFocusMode = NSInteger;

pub const AVCaptureFocusModeLocked: AVCaptureFocusMode = 0;
pub const AVCaptureFocusModeAutoFocus: AVCaptureFocusMode = 1;
pub const AVCaptureFocusModeContinuousAutoFocus: AVCaptureFocusMode = 2;

#[cfg(target_os = "ios")]
pub type AVCaptureAutoFocusRangeRestriction = NSInteger;

#[cfg(target_os = "ios")]
pub const AVCaptureAutoFocusRangeRestrictionNone: AVCaptureAutoFocusRangeRestriction = 0;
#[cfg(target_os = "ios")]
pub const AVCaptureAutoFocusRangeRestrictionNear: AVCaptureAutoFocusRangeRestriction = 1;
#[cfg(target_os = "ios")]
pub const AVCaptureAutoFocusRangeRestrictionFar: AVCaptureAutoFocusRangeRestriction = 2;

#[cfg(target_os = "ios")]
extern "C" {
    pub static AVCaptureLensPositionCurrent: &'static f32;
}

// AVCaptureDeviceFocus
impl AVCaptureDevice {
    pub fn is_focus_mode_supported(&self, focus_mode: AVCaptureFocusMode) -> Bool {
        unsafe { msg_send![self, isFocusModeSupported: focus_mode] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_locking_focus_with_custom_lens_position_supported(&self) -> Bool {
        unsafe { msg_send![self, isLockingFocusWithCustomLensPositionSupported] }
    }

    pub fn focus_mode(&self) -> AVCaptureFocusMode {
        unsafe { msg_send![self, focusMode] }
    }

    pub fn set_focus_mode(&self, focus_mode: AVCaptureFocusMode) {
        unsafe { msg_send![self, setFocusMode: focus_mode] }
    }

    pub fn is_focus_point_of_interest_supported(&self) -> Bool {
        unsafe { msg_send![self, isFocusPointOfInterestSupported] }
    }

    pub fn focus_point_of_interest(&self) -> CGPoint {
        unsafe { msg_send![self, focusPointOfInterest] }
    }

    pub fn set_focus_point_of_interest(&self, point: CGPoint) {
        unsafe { msg_send![self, setFocusPointOfInterest: point] }
    }

    pub fn is_adjusting_focus(&self) -> Bool {
        unsafe { msg_send![self, isAdjustingFocus] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_auto_focus_range_restriction_supported(&self) -> Bool {
        unsafe { msg_send![self, isAutoFocusRangeRestrictionSupported] }
    }

    #[cfg(target_os = "ios")]
    pub fn auto_focus_range_restriction(&self) -> AVCaptureAutoFocusRangeRestriction {
        unsafe { msg_send![self, autoFocusRangeRestriction] }
    }

    #[cfg(target_os = "ios")]
    pub fn set_auto_focus_range_restriction(&self, auto_focus_range_restriction: AVCaptureAutoFocusRangeRestriction) {
        unsafe { msg_send![self, setAutoFocusRangeRestriction: auto_focus_range_restriction] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_smooth_auto_focus_supported(&self) -> Bool {
        unsafe { msg_send![self, isSmoothAutoFocusSupported] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_smooth_auto_focus_enabled(&self) -> Bool {
        unsafe { msg_send![self, isSmoothAutoFocusEnabled] }
    }

    #[cfg(target_os = "ios")]
    pub fn set_smooth_auto_focus_enabled(&self, enabled: Bool) {
        unsafe { msg_send![self, setSmoothAutoFocusEnabled: enabled] }
    }

    #[cfg(target_os = "ios")]
    pub fn automatically_adjusts_face_driven_auto_focus_enabled(&self) -> Bool {
        unsafe { msg_send![self, automaticallyAdjustsFaceDrivenAutoFocusEnabled] }
    }

    #[cfg(target_os = "ios")]
    pub fn set_automatically_adjusts_face_driven_auto_focus_enabled(&self, enabled: Bool) {
        unsafe { msg_send![self, setAutomaticallyAdjustsFaceDrivenAutoFocusEnabled: enabled] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_face_driven_auto_focus_enabled(&self) -> Bool {
        unsafe { msg_send![self, isFaceDrivenAutoFocusEnabled] }
    }

    #[cfg(target_os = "ios")]
    pub fn lens_position(&self) -> f32 {
        unsafe { msg_send![self, lensPosition] }
    }

    pub fn minimum_focus_distance(&self) -> NSInteger {
        unsafe { msg_send![self, minimumFocusDistance] }
    }
}

pub type AVCaptureExposureMode = NSInteger;

pub const AVCaptureExposureModeLocked: AVCaptureExposureMode = 0;
pub const AVCaptureExposureModeAutoExpose: AVCaptureExposureMode = 1;
pub const AVCaptureExposureModeContinuousAutoExposure: AVCaptureExposureMode = 2;
pub const AVCaptureExposureModeCustom: AVCaptureExposureMode = 3;

#[cfg(target_os = "ios")]
extern "C" {
    pub static AVCaptureExposureDurationCurrent: &'static CMTime;
    pub static AVCaptureExposureTargetBiasCurrent: &'static f32;
    pub static AVCaptureISOCurrent: &'static f32;
}

// AVCaptureDeviceExposure
impl AVCaptureDevice {
    pub fn is_exposure_mode_supported(&self, exposure_mode: AVCaptureExposureMode) -> Bool {
        unsafe { msg_send![self, isExposureModeSupported: exposure_mode] }
    }

    pub fn exposure_mode(&self) -> AVCaptureExposureMode {
        unsafe { msg_send![self, exposureMode] }
    }

    pub fn set_exposure_mode(&self, exposure_mode: AVCaptureExposureMode) {
        unsafe { msg_send![self, setExposureMode: exposure_mode] }
    }

    pub fn is_exposure_point_of_interest_supported(&self) -> Bool {
        unsafe { msg_send![self, isExposurePointOfInterestSupported] }
    }

    pub fn exposure_point_of_interest(&self) -> CGPoint {
        unsafe { msg_send![self, exposurePointOfInterest] }
    }

    pub fn set_exposure_point_of_interest(&self, point: CGPoint) {
        unsafe { msg_send![self, setExposurePointOfInterest: point] }
    }

    #[cfg(target_os = "ios")]
    pub fn automatically_adjusts_face_driven_auto_exposure_enabled(&self) -> Bool {
        unsafe { msg_send![self, automaticallyAdjustsFaceDrivenAutoExposureEnabled] }
    }

    #[cfg(target_os = "ios")]
    pub fn set_automatically_adjusts_face_driven_auto_exposure_enabled(&self, enabled: Bool) {
        unsafe { msg_send![self, setAutomaticallyAdjustsFaceDrivenAutoExposureEnabled: enabled] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_face_driven_auto_exposure_enabled(&self) -> Bool {
        unsafe { msg_send![self, isFaceDrivenAutoExposureEnabled] }
    }

    #[cfg(target_os = "ios")]
    pub fn active_max_exposure_duration(&self) -> CMTime {
        unsafe { msg_send![self, activeMaxExposureDuration] }
    }

    #[cfg(target_os = "ios")]
    pub fn set_active_max_exposure_duration(&self, duration: CMTime) {
        unsafe { msg_send![self, setActiveMaxExposureDuration: duration] }
    }

    pub fn is_adjusting_exposure(&self) -> Bool {
        unsafe { msg_send![self, isAdjustingExposure] }
    }

    #[cfg(target_os = "ios")]
    pub fn lens_aperture(&self) -> f32 {
        unsafe { msg_send![self, lensAperture] }
    }

    #[cfg(target_os = "ios")]
    pub fn exposure_duration(&self) -> CMTime {
        unsafe { msg_send![self, exposureDuration] }
    }

    #[cfg(target_os = "ios")]
    pub fn iso(&self) -> f32 {
        unsafe { msg_send![self, ISO] }
    }

    #[cfg(target_os = "ios")]
    pub fn exposure_target_offset(&self) -> f32 {
        unsafe { msg_send![self, exposureTargetOffset] }
    }

    #[cfg(target_os = "ios")]
    pub fn exposure_target_bias(&self) -> f32 {
        unsafe { msg_send![self, exposureTargetBias] }
    }

    #[cfg(target_os = "ios")]
    pub fn min_exposure_target_bias(&self) -> f32 {
        unsafe { msg_send![self, minExposureTargetBias] }
    }

    #[cfg(target_os = "ios")]
    pub fn max_exposure_target_bias(&self) -> f32 {
        unsafe { msg_send![self, maxExposureTargetBias] }
    }
}

// AVCaptureDeviceToneMapping
#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn is_global_tone_mapping_enabled(&self) -> Bool {
        unsafe { msg_send![self, isGlobalToneMappingEnabled] }
    }
}

pub type AVCaptureWhiteBalanceMode = NSInteger;

pub const AVCaptureWhiteBalanceModeLocked: AVCaptureWhiteBalanceMode = 0;
pub const AVCaptureWhiteBalanceModeAutoWhiteBalance: AVCaptureWhiteBalanceMode = 1;
pub const AVCaptureWhiteBalanceModeContinuousAutoWhiteBalance: AVCaptureWhiteBalanceMode = 2;

#[cfg(target_os = "ios")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AVCaptureWhiteBalanceGains {
    pub redGain: f32,
    pub greenGain: f32,
    pub blueGain: f32,
}

#[cfg(target_os = "ios")]
unsafe impl Encode for AVCaptureWhiteBalanceGains {
    const ENCODING: Encoding = Encoding::Struct("AVCaptureWhiteBalanceGains", &[Float, Float, Float]);
}

#[cfg(target_os = "ios")]
unsafe impl RefEncode for AVCaptureWhiteBalanceGains {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(target_os = "ios")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AVCaptureWhiteBalanceChromaticityValues {
    pub x: f32,
    pub y: f32,
}

#[cfg(target_os = "ios")]
unsafe impl Encode for AVCaptureWhiteBalanceChromaticityValues {
    const ENCODING: Encoding = Encoding::Struct("AVCaptureWhiteBalanceChromaticityValues", &[Float, Float]);
}

#[cfg(target_os = "ios")]
unsafe impl RefEncode for AVCaptureWhiteBalanceChromaticityValues {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(target_os = "ios")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AVCaptureWhiteBalanceTemperatureAndTintValues {
    pub temperature: f32,
    pub tint: f32,
}

#[cfg(target_os = "ios")]
extern "C" {
    pub static AVCaptureWhiteBalanceGainsCurrent: &'static AVCaptureWhiteBalanceGains;
}

#[cfg(target_os = "ios")]
unsafe impl Encode for AVCaptureWhiteBalanceTemperatureAndTintValues {
    const ENCODING: Encoding = Encoding::Struct("AVCaptureWhiteBalanceTemperatureAndTintValues", &[Float, Float]);
}

#[cfg(target_os = "ios")]
unsafe impl RefEncode for AVCaptureWhiteBalanceTemperatureAndTintValues {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// AVCaptureDeviceWhiteBalance
impl AVCaptureDevice {
    pub fn is_white_balance_mode_supported(&self, white_balance_mode: AVCaptureWhiteBalanceMode) -> Bool {
        unsafe { msg_send![self, isWhiteBalanceModeSupported: white_balance_mode] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_locking_white_balance_with_custom_device_gains_supported(&self) -> Bool {
        unsafe { msg_send![self, isLockingWhiteBalanceWithCustomDeviceGainsSupported] }
    }

    pub fn white_balance_mode(&self) -> AVCaptureWhiteBalanceMode {
        unsafe { msg_send![self, whiteBalanceMode] }
    }

    pub fn set_white_balance_mode(&self, white_balance_mode: AVCaptureWhiteBalanceMode) {
        unsafe { msg_send![self, setWhiteBalanceMode: white_balance_mode] }
    }

    pub fn is_adjusting_white_balance(&self) -> Bool {
        unsafe { msg_send![self, isAdjustingWhiteBalance] }
    }

    #[cfg(target_os = "ios")]
    pub fn device_white_balance_gains(&self) -> AVCaptureWhiteBalanceGains {
        unsafe { msg_send![self, deviceWhiteBalanceGains] }
    }

    #[cfg(target_os = "ios")]
    pub fn gray_world_device_white_balance_gains(&self) -> AVCaptureWhiteBalanceGains {
        unsafe { msg_send![self, grayWorldDeviceWhiteBalanceGains] }
    }

    #[cfg(target_os = "ios")]
    pub fn max_white_balance_gain(&self) -> f32 {
        unsafe { msg_send![self, maxWhiteBalanceGain] }
    }

    #[cfg(target_os = "ios")]
    pub fn chromaticity_values_for_device_white_balance_gains(
        &self,
        white_balance_gains: AVCaptureWhiteBalanceGains,
    ) -> AVCaptureWhiteBalanceChromaticityValues {
        unsafe { msg_send![self, chromaticityValuesForDeviceWhiteBalanceGains: white_balance_gains] }
    }

    #[cfg(target_os = "ios")]
    pub fn device_white_balance_gains_for_chromaticity_values(
        &self,
        chromaticity_values: AVCaptureWhiteBalanceChromaticityValues,
    ) -> AVCaptureWhiteBalanceGains {
        unsafe { msg_send![self, deviceWhiteBalanceGainsForChromaticityValues: chromaticity_values] }
    }

    #[cfg(target_os = "ios")]
    pub fn temperature_and_tint_values_for_device_white_balance_gains(
        &self,
        white_balance_gains: AVCaptureWhiteBalanceGains,
    ) -> AVCaptureWhiteBalanceTemperatureAndTintValues {
        unsafe { msg_send![self, temperatureAndTintValuesForDeviceWhiteBalanceGains: white_balance_gains] }
    }

    #[cfg(target_os = "ios")]
    pub fn device_white_balance_gains_for_temperature_and_tint_values(
        &self,
        temp_and_tint_values: AVCaptureWhiteBalanceTemperatureAndTintValues,
    ) -> AVCaptureWhiteBalanceGains {
        unsafe { msg_send![self, deviceWhiteBalanceGainsForTemperatureAndTintValues: temp_and_tint_values] }
    }
}

// AVCaptureDeviceLowLightBoost
#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn is_low_light_boost_supported(&self) -> Bool {
        unsafe { msg_send![self, isLowLightBoostSupported] }
    }

    pub fn is_low_light_boost_enabled(&self) -> Bool {
        unsafe { msg_send![self, isLowLightBoostEnabled] }
    }

    pub fn automatically_enables_low_light_boost_when_available(&self) -> Bool {
        unsafe { msg_send![self, automaticallyEnablesLowLightBoostWhenAvailable] }
    }

    pub fn set_automatically_enables_low_light_boost_when_available(&self, value: Bool) {
        unsafe { msg_send![self, setAutomaticallyEnablesLowLightBoostWhenAvailable: value] }
    }
}

// AVCaptureDeviceVideoZoom
impl AVCaptureDevice {
    #[cfg(target_os = "ios")]
    pub fn video_zoom_factor(&self) -> CGFloat {
        unsafe { msg_send![self, videoZoomFactor] }
    }

    #[cfg(target_os = "ios")]
    pub fn set_video_zoom_factor(&self, factor: CGFloat) {
        unsafe { msg_send![self, setVideoZoomFactor: factor] }
    }

    #[cfg(target_os = "ios")]
    pub fn ramp_to_video_zoom_factor(&self, factor: CGFloat, rate: f32) {
        unsafe { msg_send![self, rampToVideoZoomFactor: factor withRate: rate] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_ramping_video_zoom(&self) -> Bool {
        unsafe { msg_send![self, isRampingVideoZoom] }
    }

    #[cfg(target_os = "ios")]
    pub fn cancel_video_zoom_ramp(&self) {
        unsafe { msg_send![self, cancelVideoZoomRamp] }
    }

    #[cfg(target_os = "macos")]
    pub fn display_video_zoom_factor_multiplier(&self) -> CGFloat {
        unsafe { msg_send![self, displayVideoZoomFactorMultiplier] }
    }

    #[cfg(target_os = "ios")]
    pub fn min_available_video_zoom_factor(&self) -> CGFloat {
        unsafe { msg_send![self, minAvailableVideoZoomFactor] }
    }

    #[cfg(target_os = "ios")]
    pub fn max_available_video_zoom_factor(&self) -> CGFloat {
        unsafe { msg_send![self, maxAvailableVideoZoomFactor] }
    }
}

pub type AVAuthorizationStatus = NSInteger;

pub const AVAuthorizationStatusNotDetermined: AVAuthorizationStatus = 0;
pub const AVAuthorizationStatusRestricted: AVAuthorizationStatus = 1;
pub const AVAuthorizationStatusDenied: AVAuthorizationStatus = 2;
pub const AVAuthorizationStatusAuthorized: AVAuthorizationStatus = 3;

// AVCaptureDeviceAuthorization
impl AVCaptureDevice {
    pub fn authorization_status_for_media_type(media_type: &AVMediaType) -> AVAuthorizationStatus {
        unsafe { msg_send![AVCaptureDevice::class(), authorizationStatusForMediaType: media_type] }
    }
}

// AVCaptureDeviceHighDynamicRangeSupport
#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn automatically_adjusts_video_hdr_enabled(&self) -> Bool {
        unsafe { msg_send![self, automaticallyAdjustsVideoHDREnabled] }
    }

    pub fn is_video_hdr_enabled(&self) -> Bool {
        unsafe { msg_send![self, isVideoHDREnabled] }
    }
}

pub type AVCaptureColorSpace = NSInteger;

pub const AVCaptureColorSpace_sRGB: AVCaptureColorSpace = 0;
pub const AVCaptureColorSpace_P3_D65: AVCaptureColorSpace = 1;
pub const AVCaptureColorSpace_HLG_BT2020: AVCaptureColorSpace = 2;
pub const AVCaptureColorSpace_AppleLog: AVCaptureColorSpace = 3;

// AVCaptureDeviceColorSpaceSupport
impl AVCaptureDevice {
    pub fn active_color_space(&self) -> AVCaptureColorSpace {
        unsafe { msg_send![self, activeColorSpace] }
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeviceDiscoverySession;

    unsafe impl ClassType for AVCaptureDeviceDiscoverySession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDeviceDiscoverySession {}

impl AVCaptureDeviceDiscoverySession {
    pub fn discovery_session_with_device_types(
        device_types: &NSArray<AVCaptureDeviceType>,
        media_type: &AVMediaType,
        position: AVCaptureDevicePosition,
    ) -> Id<AVCaptureDeviceDiscoverySession> {
        unsafe {
            msg_send_id![AVCaptureDeviceDiscoverySession::class(), discoverySessionWithDeviceTypes: device_types mediaType: media_type position: position]
        }
    }

    pub fn devices(&self) -> Id<NSArray<AVCaptureDevice>> {
        unsafe { msg_send_id![self, devices] }
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVFrameRateRange;

    unsafe impl ClassType for AVFrameRateRange {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVFrameRateRange {}

impl AVFrameRateRange {
    pub fn min_frame_rate(&self) -> f64 {
        unsafe { msg_send![self, minFrameRate] }
    }

    pub fn max_frame_rate(&self) -> f64 {
        unsafe { msg_send![self, maxFrameRate] }
    }

    pub fn min_frame_duration(&self) -> CMTime {
        unsafe { msg_send![self, minFrameDuration] }
    }

    pub fn max_frame_duration(&self) -> CMTime {
        unsafe { msg_send![self, maxFrameDuration] }
    }
}

#[cfg(target_os = "ios")]
pub type AVCaptureVideoStabilizationMode = NSInteger;

#[cfg(target_os = "ios")]
pub const AVCaptureVideoStabilizationModeOff: AVCaptureVideoStabilizationMode = 0;
#[cfg(target_os = "ios")]
pub const AVCaptureVideoStabilizationModeStandard: AVCaptureVideoStabilizationMode = 1;
#[cfg(target_os = "ios")]
pub const AVCaptureVideoStabilizationModeCinematic: AVCaptureVideoStabilizationMode = 2;
#[cfg(target_os = "ios")]
pub const AVCaptureVideoStabilizationModeCinematicExtended: AVCaptureVideoStabilizationMode = 3;
#[cfg(target_os = "ios")]
pub const AVCaptureVideoStabilizationModePreviewOptimized: AVCaptureVideoStabilizationMode = 4;
#[cfg(target_os = "ios")]
pub const AVCaptureVideoStabilizationModeAuto: AVCaptureVideoStabilizationMode = -1;

pub type AVCaptureAutoFocusSystem = NSInteger;

pub const AVCaptureAutoFocusSystemNone: AVCaptureAutoFocusSystem = 0;
pub const AVCaptureAutoFocusSystemContrastDetection: AVCaptureAutoFocusSystem = 1;
pub const AVCaptureAutoFocusSystemPhaseDetection: AVCaptureAutoFocusSystem = 2;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeviceFormat;

    unsafe impl ClassType for AVCaptureDeviceFormat {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDeviceFormat {}

impl AVCaptureDeviceFormat {
    pub fn media_type(&self) -> Id<AVMediaType> {
        unsafe { msg_send_id![self, mediaType] }
    }

    pub fn format_description(&self) -> CMFormatDescription {
        unsafe {
            let format_description: CMFormatDescriptionRef = msg_send![self, formatDescription];
            CMFormatDescription::wrap_under_get_rule(format_description)
        }
    }

    pub fn video_supported_frame_rate_ranges(&self) -> Id<NSArray<AVFrameRateRange>> {
        unsafe { msg_send_id![self, videoSupportedFrameRateRanges] }
    }

    pub fn video_field_of_view(&self) -> f32 {
        unsafe { msg_send![self, videoFieldOfView] }
    }

    pub fn is_video_binned(&self) -> Bool {
        unsafe { msg_send![self, isVideoBinned] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_video_stabilization_mode_supported(&self, video_stabilization_mode: AVCaptureVideoStabilizationMode) -> Bool {
        unsafe { msg_send![self, isVideoStabilizationModeSupported: video_stabilization_mode] }
    }

    #[cfg(target_os = "ios")]
    pub fn video_max_zoom_factor(&self) -> CGFloat {
        unsafe { msg_send![self, videoMaxZoomFactor] }
    }

    #[cfg(target_os = "ios")]
    pub fn video_zoom_factor_upscale_threshold(&self) -> CGFloat {
        unsafe { msg_send![self, videoZoomFactorUpscaleThreshold] }
    }

    #[cfg(target_os = "ios")]
    pub fn min_exposure_duration(&self) -> CMTime {
        unsafe { msg_send![self, minExposureDuration] }
    }

    #[cfg(target_os = "ios")]
    pub fn max_exposure_duration(&self) -> CMTime {
        unsafe { msg_send![self, maxExposureDuration] }
    }

    #[cfg(target_os = "ios")]
    pub fn min_iso(&self) -> f32 {
        unsafe { msg_send![self, minISO] }
    }

    #[cfg(target_os = "ios")]
    pub fn max_iso(&self) -> f32 {
        unsafe { msg_send![self, maxISO] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_global_tone_mapping_supported(&self) -> Bool {
        unsafe { msg_send![self, isGlobalToneMappingSupported] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_video_hdr_supported(&self) -> Bool {
        unsafe { msg_send![self, isVideoHDRSupported] }
    }

    pub fn is_high_photo_quality_supported(&self) -> Bool {
        unsafe { msg_send![self, isHighPhotoQualitySupported] }
    }

    pub fn auto_focus_system(&self) -> AVCaptureAutoFocusSystem {
        unsafe { msg_send![self, autoFocusSystem] }
    }

    pub fn supported_color_spaces(&self) -> Id<NSArray<NSNumber>> {
        unsafe { msg_send_id![self, supportedColorSpaces] }
    }

    pub fn supported_depth_data_formats(&self) -> Id<NSArray<AVCaptureDeviceFormat>> {
        unsafe { msg_send_id![self, supportedDepthDataFormats] }
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeviceInputSource;

    unsafe impl ClassType for AVCaptureDeviceInputSource {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDeviceInputSource {}

impl AVCaptureDeviceInputSource {
    pub fn input_source_id(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, inputSourceID] }
    }

    pub fn localized_name(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, localizedName] }
    }
}
