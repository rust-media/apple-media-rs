use core_media::sample_buffer::CMSampleBufferRef;
use dispatch2::{DispatchObject, DispatchQueue};
use objc2::{extern_class, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject, ClassType};
use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSObject, NSObjectProtocol, NSString};

use crate::{capture_output_base::AVCaptureOutput, capture_session::AVCaptureConnection};

extern_class!(
    #[unsafe(super(AVCaptureOutput))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureVideoDataOutput;
);

unsafe impl NSObjectProtocol for AVCaptureVideoDataOutput {}

impl AVCaptureVideoDataOutput {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send![AVCaptureVideoDataOutput::class(), new] }
    }

    pub fn get_sample_buffer_delegate(&self) -> Option<Retained<ProtocolObject<dyn AVCaptureVideoDataOutputSampleBufferDelegate>>> {
        unsafe { msg_send![self, sampleBufferDelegate] }
    }

    pub fn set_sample_buffer_delegate(&self, delegate: &ProtocolObject<dyn AVCaptureVideoDataOutputSampleBufferDelegate>, queue: &DispatchQueue) {
        unsafe { msg_send![self, setSampleBufferDelegate: delegate, queue: queue.as_raw().as_ptr() as *const NSObject] }
    }

    pub fn get_video_settings(&self) -> Option<Retained<NSDictionary<NSString, NSObject>>> {
        unsafe { msg_send![self, videoSettings] }
    }

    pub fn set_video_settings(&self, video_settings: &NSDictionary<NSString, NSObject>) {
        unsafe { msg_send![self, setVideoSettings: video_settings] }
    }

    pub fn get_available_video_cv_pixel_format_types(&self) -> Retained<NSArray<NSNumber>> {
        unsafe { msg_send![self, availableVideoCVPixelFormatTypes] }
    }

    pub fn get_available_video_data_format_types(&self) -> Retained<NSArray<NSNumber>> {
        unsafe { msg_send![self, availableVideoDataFormatTypes] }
    }

    pub fn get_always_discards_late_video_frames(&self) -> bool {
        unsafe { msg_send![self, alwaysDiscardsLateVideoFrames] }
    }

    pub fn set_always_discards_late_video_frames(&self, always_discards_late_video_frames: bool) {
        unsafe { msg_send![self, setAlwaysDiscardsLateVideoFrames: always_discards_late_video_frames] }
    }
}

extern_protocol!(
    pub unsafe trait AVCaptureVideoDataOutputSampleBufferDelegate: NSObjectProtocol {
        #[unsafe(method(captureOutput:didOutputSampleBuffer:fromConnection:))]
        #[optional]
        unsafe fn capture_output_did_output_sample_buffer(
            &self,
            capture_output: &AVCaptureOutput,
            sample_buffer: CMSampleBufferRef,
            connection: &AVCaptureConnection,
        );
        #[unsafe(method(captureOutput:didDropSampleBuffer:fromConnection:))]
        #[optional]
        unsafe fn capture_output_did_drop_sample_buffer(
            &self,
            capture_output: &AVCaptureOutput,
            sample_buffer: CMSampleBufferRef,
            connection: &AVCaptureConnection,
        );
    }
);
