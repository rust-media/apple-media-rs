use objc2::{extern_class, msg_send, rc::Retained};
use objc2_foundation::{NSArray, NSInteger, NSObject, NSObjectProtocol, NSString};

use crate::capture_session::AVCaptureConnection;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureOutput;
);

unsafe impl NSObjectProtocol for AVCaptureOutput {}

impl AVCaptureOutput {
    pub fn connections(&self) -> Retained<NSArray<AVCaptureConnection>> {
        unsafe { msg_send![self, connections] }
    }

    pub fn connection_with_media_type(&self, media_type: &NSString) -> Option<Retained<AVCaptureConnection>> {
        unsafe { msg_send![self, connectionWithMediaType: media_type] }
    }
}

pub type AVCaptureOutputDataDroppedReason = NSInteger;

pub const AVCaptureOutputDataDroppedReasonNone: AVCaptureOutputDataDroppedReason = 0;
pub const AVCaptureOutputDataDroppedReasonLateData: AVCaptureOutputDataDroppedReason = 1;
pub const AVCaptureOutputDataDroppedReasonOutdatedData: AVCaptureOutputDataDroppedReason = 2;
pub const AVCaptureOutputDataDroppedReasonDiscontinuity: AVCaptureOutputDataDroppedReason = 3;
