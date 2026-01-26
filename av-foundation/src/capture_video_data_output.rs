use core_media::sample_buffer::CMSampleBufferRef;
use dispatch2::Queue;
use objc2::{
    extern_class, extern_protocol, msg_send, msg_send_id, mutability::InteriorMutable, rc::Id, runtime::ProtocolObject, ClassType, ProtocolType,
};
use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSObject, NSObjectProtocol, NSString};

use crate::{capture_output_base::AVCaptureOutput, capture_session::AVCaptureConnection};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureVideoDataOutput;

    unsafe impl ClassType for AVCaptureVideoDataOutput {
        type Super = AVCaptureOutput;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureVideoDataOutput {}

impl AVCaptureVideoDataOutput {
    pub fn new() -> Id<Self> {
        unsafe { msg_send_id![AVCaptureVideoDataOutput::class(), new] }
    }

    pub fn get_sample_buffer_delegate(&self) -> Option<Id<ProtocolObject<dyn AVCaptureVideoDataOutputSampleBufferDelegate>>> {
        unsafe { msg_send_id![self, sampleBufferDelegate] }
    }

    pub fn set_sample_buffer_delegate(&self, delegate: &ProtocolObject<dyn AVCaptureVideoDataOutputSampleBufferDelegate>, queue: &Queue) {
        unsafe { msg_send![self, setSampleBufferDelegate: delegate queue: queue.as_raw() as *const NSObject] }
    }

    pub fn get_video_settings(&self) -> Option<Id<NSDictionary<NSString, NSObject>>> {
        unsafe { msg_send_id![self, videoSettings] }
    }

    pub fn set_video_settings(&self, video_settings: &NSDictionary<NSString, NSObject>) {
        unsafe { msg_send![self, setVideoSettings: video_settings] }
    }

    pub fn get_available_video_cv_pixel_format_types(&self) -> Id<NSArray<NSNumber>> {
        unsafe { msg_send_id![self, availableVideoCVPixelFormatTypes] }
    }

    pub fn get_available_video_data_format_types(&self) -> Id<NSArray<NSNumber>> {
        unsafe { msg_send_id![self, availableVideoDataFormatTypes] }
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
        #[method(captureOutput:didOutputSampleBuffer:fromConnection:)]
        #[optional]
        unsafe fn capture_output_did_output_sample_buffer(
            &self,
            capture_output: &AVCaptureOutput,
            sample_buffer: CMSampleBufferRef,
            connection: &AVCaptureConnection,
        );
        #[method(captureOutput:didDropSampleBuffer:fromConnection:)]
        #[optional]
        unsafe fn capture_output_did_drop_sample_buffer(
            &self,
            capture_output: &AVCaptureOutput,
            sample_buffer: CMSampleBufferRef,
            connection: &AVCaptureConnection,
        );
    }

    unsafe impl ProtocolType for dyn AVCaptureVideoDataOutputSampleBufferDelegate {}
);
