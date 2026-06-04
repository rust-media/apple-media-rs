use std::ptr::null_mut;

use block2::RcBlock;
use core_foundation::{base::TCFType, string::CFStringRef};
use core_graphics::{color::CGColor, geometry::CGRect};
use core_media::{sample_buffer::CMSampleBufferRef, time::CMTime, OSType};
use dispatch2::{DispatchObject, DispatchQueue};
use libc::size_t;
use objc2::{
    encode::{Encode, Encoding},
    extern_class, extern_protocol, msg_send,
    rc::{Allocated, Retained},
    runtime::ProtocolObject,
    ClassType,
};
use objc2_foundation::{NSArray, NSError, NSInteger, NSObject, NSObjectProtocol, NSString};

use crate::{
    encode,
    shareable_content::{SCDisplay, SCRunningApplication, SCWindow},
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SCStreamOutputType(pub NSInteger);

impl SCStreamOutputType {
    #[doc(alias = "SCStreamOutputTypeScreen")]
    pub const Screen: Self = Self(0);
    #[doc(alias = "SCStreamOutputTypeAudio")]
    pub const Audio: Self = Self(1);
}

unsafe impl Encode for SCStreamOutputType {
    const ENCODING: Encoding = Encoding::Int;
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SCFrameStatus(pub NSInteger);

impl SCFrameStatus {
    #[doc(alias = "SCFrameStatusComplete")]
    pub const Complete: Self = Self(0);
    #[doc(alias = "SCFrameStatusIdle")]
    pub const Idle: Self = Self(1);
    #[doc(alias = "SCFrameStatusBlank")]
    pub const Blank: Self = Self(2);
    #[doc(alias = "SCFrameStatusSuspended")]
    pub const Suspended: Self = Self(3);
    #[doc(alias = "SCFrameStatusStarted")]
    pub const Started: Self = Self(4);
    #[doc(alias = "SCFrameStatusStopped")]
    pub const Stopped: Self = Self(5);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCContentFilter;
);

unsafe impl NSObjectProtocol for SCContentFilter {}

impl SCContentFilter {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send![SCContentFilter::class(), new] }
    }

    pub fn init_with_desktop_independent_window(this: Allocated<Self>, window: &SCWindow) -> Retained<Self> {
        unsafe { msg_send![this, initWithDesktopIndependentWindow: window] }
    }

    pub fn init_with_display_exclude_windows(this: Allocated<Self>, display: &SCDisplay, excluded: &NSArray<SCWindow>) -> Retained<Self> {
        unsafe { msg_send![this, initWithDisplay: display, excludingWindows: excluded] }
    }

    pub fn init_with_display_include_windows(this: Allocated<Self>, display: &SCDisplay, included: &NSArray<SCWindow>) -> Retained<Self> {
        unsafe { msg_send![this, initWithDisplay: display, includingWindows: included] }
    }

    pub fn init_with_display_exclude_applications(
        this: Allocated<Self>,
        display: &SCDisplay,
        applications: &NSArray<SCRunningApplication>,
        excepting_windows: &NSArray<SCWindow>,
    ) -> Retained<Self> {
        unsafe { msg_send![this, initWithDisplay: display, excludingApplications: applications, exceptingWindows: excepting_windows] }
    }

    pub fn init_with_display_include_applications(
        this: Allocated<Self>,
        display: &SCDisplay,
        applications: &NSArray<SCRunningApplication>,
        excepting_windows: &NSArray<SCWindow>,
    ) -> Retained<Self> {
        unsafe { msg_send![this, initWithDisplay: display, includingApplications: applications, exceptingWindows: excepting_windows] }
    }
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCStreamConfiguration;
);

unsafe impl NSObjectProtocol for SCStreamConfiguration {}

impl SCStreamConfiguration {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send![SCStreamConfiguration::class(), new] }
    }

    pub fn get_height(&self) -> size_t {
        unsafe { msg_send![self, height] }
    }

    pub fn set_height(&self, height: size_t) {
        unsafe { msg_send![self, setHeight: height] }
    }

    pub fn get_width(&self) -> size_t {
        unsafe { msg_send![self, width] }
    }

    pub fn set_width(&self, width: size_t) {
        unsafe { msg_send![self, setWidth: width] }
    }

    pub fn get_minimum_frame_interval(&self) -> CMTime {
        unsafe { msg_send![self, minimumFrameInterval] }
    }

    pub fn set_minimum_frame_interval(&self, interval: CMTime) {
        unsafe { msg_send![self, setMinimumFrameInterval: interval] }
    }

    pub fn get_pixel_format(&self) -> OSType {
        unsafe { msg_send![self, pixelFormat] }
    }

    pub fn set_pixel_format(&self, format: OSType) {
        unsafe { msg_send![self, setPixelFormat: format] }
    }

    pub fn get_scales_to_fit(&self) -> bool {
        unsafe { msg_send![self, scalesToFit] }
    }

    pub fn set_scales_to_fit(&self, scales_to_fit: bool) {
        unsafe { msg_send![self, setScalesToFit: scales_to_fit] }
    }

    pub fn get_show_cursor(&self) -> bool {
        unsafe { msg_send![self, showCursor] }
    }

    pub fn set_show_cursor(&self, show_cursor: bool) {
        unsafe { msg_send![self, setShowCursor: show_cursor] }
    }

    pub fn get_background_color(&self) -> CGColor {
        unsafe { CGColor::wrap_under_get_rule(msg_send![self, backgroundColor]) }
    }

    pub fn set_background_color(&self, color: CGColor) {
        unsafe { msg_send![self, setBackgroundColor: color.as_concrete_TypeRef()] }
    }

    pub fn get_source_rect(&self) -> CGRect {
        unsafe { msg_send![self, sourceRect] }
    }

    pub fn set_source_rect(&self, rect: CGRect) {
        unsafe { msg_send![self, setSourceRect: rect] }
    }

    pub fn get_destination_rect(&self) -> CGRect {
        unsafe { msg_send![self, destinationRect] }
    }

    pub fn set_destination_rect(&self, rect: CGRect) {
        unsafe { msg_send![self, setDestinationRect: rect] }
    }

    pub fn get_queue_depth(&self) -> NSInteger {
        unsafe { msg_send![self, queueDepth] }
    }

    pub fn set_queue_depth(&self, depth: NSInteger) {
        unsafe { msg_send![self, setQueueDepth: depth] }
    }

    pub fn get_color_matrix(&self) -> CFStringRef {
        unsafe {
            let color_matrix: encode::CFStringRef = msg_send![self, colorMatrix];
            color_matrix as CFStringRef
        }
    }

    pub fn set_color_matrix(&self, matrix: CFStringRef) {
        unsafe { msg_send![self, setColorMatrix: matrix as encode::CFStringRef] }
    }

    pub fn get_color_space_name(&self) -> CFStringRef {
        unsafe {
            let color_space_name: encode::CFStringRef = msg_send![self, colorSpaceName];
            color_space_name as CFStringRef
        }
    }

    pub fn set_color_space_name(&self, name: CFStringRef) {
        unsafe { msg_send![self, setColorSpaceName: name as encode::CFStringRef] }
    }

    pub fn get_captures_audio(&self) -> bool {
        unsafe { msg_send![self, capturesAudio] }
    }

    pub fn set_captures_audio(&self, captures_audio: bool) {
        unsafe { msg_send![self, setCapturesAudio: captures_audio] }
    }

    pub fn get_sample_rate(&self) -> f64 {
        unsafe { msg_send![self, sampleRate] }
    }

    pub fn set_sample_rate(&self, rate: f64) {
        unsafe { msg_send![self, setSampleRate: rate] }
    }

    pub fn get_channel_count(&self) -> size_t {
        unsafe { msg_send![self, channelCount] }
    }

    pub fn set_channel_count(&self, count: size_t) {
        unsafe { msg_send![self, setChannelCount: count] }
    }

    pub fn get_excludes_current_process_audio(&self) -> bool {
        unsafe { msg_send![self, excludesCurrentProcessAudio] }
    }

    pub fn set_excludes_current_process_audio(&self, excludes_current_process_audio: bool) {
        unsafe { msg_send![self, setExcludesCurrentProcessAudio: excludes_current_process_audio] }
    }
}

pub type SCStreamFrameInfo = NSString;

extern "C" {
    pub static SCStreamFrameInfoStatus: &'static NSString;
    pub static SCStreamFrameInfoDisplayTime: &'static NSString;
    pub static SCStreamFrameInfoScaleFactor: &'static NSString;
    pub static SCStreamFrameInfoContentScale: &'static NSString;
    pub static SCStreamFrameInfoContentRect: &'static NSString;
    pub static SCStreamFrameInfoDirtyRects: &'static NSString;
    pub static SCStreamFrameInfoScreenRect: &'static NSString;
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCStream;
);

unsafe impl NSObjectProtocol for SCStream {}

type CompletionHandler = RcBlock<dyn Fn(*mut NSError)>;

impl SCStream {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send![SCStream::class(), new] }
    }

    pub fn init_with_filter(
        this: Allocated<Self>,
        filter: &SCContentFilter,
        configuration: &SCStreamConfiguration,
        delegate: &ProtocolObject<dyn SCStreamDelegate>,
    ) -> Retained<Self> {
        unsafe { msg_send![this, initWithFilter: filter, configuration: configuration, delegate: delegate] }
    }

    pub fn add_stream_output(
        &self,
        output: &ProtocolObject<dyn SCStreamOutput>,
        output_type: SCStreamOutputType,
        queue: &DispatchQueue,
    ) -> Result<bool, Retained<NSError>> {
        let mut error: *mut NSError = null_mut();
        let result = unsafe {
            msg_send![self, addStreamOutput: output, type: output_type.0, sampleHandlerQueue: queue.as_raw().as_ptr() as *const NSObject, error: &mut error]
        };
        if result {
            Ok(result)
        } else {
            Err(unsafe { Retained::retain(error).unwrap() })
        }
    }

    pub fn remove_stream_output(
        &self,
        output: &ProtocolObject<dyn SCStreamOutput>,
        output_type: SCStreamOutputType,
    ) -> Result<bool, Retained<NSError>> {
        let mut error: *mut NSError = null_mut();
        let result = unsafe { msg_send![self, removeStreamOutput: output, type: output_type.0, error: &mut error] };
        if result {
            Ok(result)
        } else {
            Err(unsafe { Retained::retain(error).unwrap() })
        }
    }

    fn new_completion_handler<F>(closure: F) -> CompletionHandler
    where
        F: Fn(Option<Retained<NSError>>) + 'static,
    {
        RcBlock::new(move |error: *mut NSError| {
            closure(if error.is_null() {
                None
            } else {
                unsafe { Retained::retain(error) }
            });
        })
    }

    pub fn update_content_filter<F>(&self, content_filter: &SCContentFilter, closure: F)
    where
        F: Fn(Option<Retained<NSError>>) + 'static,
    {
        let handler = Self::new_completion_handler(closure);
        unsafe { msg_send![self, updateContentFilter: content_filter, completionHandler: &*handler] }
    }

    pub fn update_configuration<F>(&self, stream_config: &SCStreamConfiguration, closure: F)
    where
        F: Fn(Option<Retained<NSError>>) + 'static,
    {
        let handler = Self::new_completion_handler(closure);
        unsafe { msg_send![self, updateConfiguration: stream_config, completionHandler: &*handler] }
    }

    pub fn start_capture<F>(&self, closure: F)
    where
        F: Fn(Option<Retained<NSError>>) + 'static,
    {
        let handler = Self::new_completion_handler(closure);
        unsafe { msg_send![self, startCaptureWithCompletionHandler: &*handler] }
    }

    pub fn stop_capture<F>(&self, closure: F)
    where
        F: Fn(Option<Retained<NSError>>) + 'static,
    {
        let handler = Self::new_completion_handler(closure);
        unsafe { msg_send![self, stopCaptureWithCompletionHandler: &*handler] }
    }
}

extern_protocol!(
    pub unsafe trait SCStreamOutput: NSObjectProtocol {
        #[unsafe(method(stream:didOutputSampleBuffer:ofType:))]
        #[optional]
        unsafe fn stream_did_output_sample_buffer(&self, stream: &SCStream, sample_buffer: CMSampleBufferRef, of_type: SCStreamOutputType);
    }
);

extern_protocol!(
    pub unsafe trait SCStreamDelegate: NSObjectProtocol {
        #[unsafe(method(stream:didStopWithError:))]
        #[optional]
        unsafe fn stream_did_stop_with_error(&self, stream: &SCStream, error: &NSError);
    }
);
