use objc2::{extern_class, msg_send, rc::Retained, ClassType};
use objc2_foundation::{NSArray, NSInteger, NSObject, NSObjectProtocol, NSString};

use crate::{capture_input::AVCaptureInput, capture_output_base::AVCaptureOutput, capture_session_preset::AVCaptureSessionPreset};

extern "C" {
    pub static AVCaptureSessionRuntimeErrorNotification: &'static NSString;
    pub static AVCaptureSessionErrorKey: &'static NSString;
    pub static AVCaptureSessionDidStartRunningNotification: &'static NSString;
    pub static AVCaptureSessionDidStopRunningNotification: &'static NSString;
    pub static AVCaptureSessionWasInterruptedNotification: &'static NSString;
    pub static AVCaptureSessionInterruptionReasonKey: &'static NSString;
    pub static AVCaptureSessionInterruptionSystemPressureStateKey: &'static NSString;
    pub static AVCaptureSessionInterruptionEndedNotification: &'static NSString;
}

pub type AVCaptureSessionInterruptionReason = NSInteger;

pub const AVCaptureSessionInterruptionReasonVideoDeviceNotAvailableInBackground: AVCaptureSessionInterruptionReason = 1;
pub const AVCaptureSessionInterruptionReasonAudioDeviceInUseByAnotherClient: AVCaptureSessionInterruptionReason = 2;
pub const AVCaptureSessionInterruptionReasonVideoDeviceInUseByAnotherClient: AVCaptureSessionInterruptionReason = 3;
pub const AVCaptureSessionInterruptionReasonVideoDeviceNotAvailableWithMultipleForegroundApps: AVCaptureSessionInterruptionReason = 4;
pub const AVCaptureSessionInterruptionReasonVideoDeviceNotAvailableDueToSystemPressure: AVCaptureSessionInterruptionReason = 5;

pub type AVCaptureVideoOrientation = NSInteger;

pub const AVCaptureVideoOrientationPortrait: AVCaptureVideoOrientation = 1;
pub const AVCaptureVideoOrientationPortraitUpsideDown: AVCaptureVideoOrientation = 2;
pub const AVCaptureVideoOrientationLandscapeRight: AVCaptureVideoOrientation = 3;
pub const AVCaptureVideoOrientationLandscapeLeft: AVCaptureVideoOrientation = 4;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureSession;
);

unsafe impl NSObjectProtocol for AVCaptureSession {}

impl AVCaptureSession {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send![AVCaptureSession::class(), new] }
    }
    pub fn can_set_session_preset(&self, preset: &AVCaptureSessionPreset) -> bool {
        unsafe { msg_send![self, canSetSessionPreset: preset] }
    }

    pub fn get_session_preset(&self) -> Retained<AVCaptureSessionPreset> {
        unsafe { msg_send![self, sessionPreset] }
    }

    pub fn set_session_preset(&self, preset: &AVCaptureSessionPreset) {
        unsafe { msg_send![self, setSessionPreset: preset] }
    }

    pub fn inputs(&self) -> Retained<NSArray<AVCaptureInput>> {
        unsafe { msg_send![self, inputs] }
    }

    pub fn can_add_input(&self, input: &AVCaptureInput) -> bool {
        unsafe { msg_send![self, canAddInput: input] }
    }

    pub fn add_input(&self, input: &AVCaptureInput) {
        unsafe { msg_send![self, addInput: input] }
    }

    pub fn remove_input(&self, input: &AVCaptureInput) {
        unsafe { msg_send![self, removeInput: input] }
    }

    pub fn outputs(&self) -> Retained<NSArray<AVCaptureOutput>> {
        unsafe { msg_send![self, outputs] }
    }

    pub fn can_add_output(&self, output: &AVCaptureOutput) -> bool {
        unsafe { msg_send![self, canAddOutput: output] }
    }

    pub fn add_output(&self, output: &AVCaptureOutput) {
        unsafe { msg_send![self, addOutput: output] }
    }

    pub fn remove_output(&self, output: &AVCaptureOutput) {
        unsafe { msg_send![self, removeOutput: output] }
    }

    pub fn add_input_with_no_connections(&self, input: &AVCaptureInput) {
        unsafe { msg_send![self, addInputWithNoConnections: input] }
    }

    pub fn add_output_with_no_connections(&self, output: &AVCaptureOutput) {
        unsafe { msg_send![self, addOutputWithNoConnections: output] }
    }

    pub fn connections(&self) -> Retained<NSArray<AVCaptureConnection>> {
        unsafe { msg_send![self, connections] }
    }

    pub fn can_add_connection(&self, connection: &AVCaptureConnection) -> bool {
        unsafe { msg_send![self, canAddConnection: connection] }
    }

    pub fn add_connection(&self, connection: &AVCaptureConnection) {
        unsafe { msg_send![self, addConnection: connection] }
    }

    pub fn remove_connection(&self, connection: &AVCaptureConnection) {
        unsafe { msg_send![self, removeConnection: connection] }
    }

    pub fn is_running(&self) -> bool {
        unsafe { msg_send![self, isRunning] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_interrupted(&self) -> bool {
        unsafe { msg_send![self, isInterrupted] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_multitasking_camera_access_supported(&self) -> bool {
        unsafe { msg_send![self, isMultitaskingCameraAccessSupported] }
    }

    #[cfg(target_os = "ios")]
    pub fn get_uses_application_audio_session(&self) -> bool {
        unsafe { msg_send![self, usesApplicationAudioSession] }
    }

    #[cfg(target_os = "ios")]
    pub fn set_uses_application_audio_session(&self, uses_application_audio_session: bool) {
        unsafe { msg_send![self, setUsesApplicationAudioSession: uses_application_audio_session] }
    }

    pub fn begin_configuration(&self) {
        unsafe { msg_send![self, beginConfiguration] }
    }

    pub fn commit_configuration(&self) {
        unsafe { msg_send![self, commitConfiguration] }
    }

    pub fn start_running(&self) {
        unsafe { msg_send![self, startRunning] }
    }

    pub fn stop_running(&self) {
        unsafe { msg_send![self, stopRunning] }
    }
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureConnection;
);

unsafe impl NSObjectProtocol for AVCaptureConnection {}
