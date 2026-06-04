use block2::RcBlock;
use core_graphics::{display::CGDirectDisplayID, geometry::CGRect, window::CGWindowID};
use libc::pid_t;
use objc2::{class, extern_class, msg_send, rc::Retained, ClassType};
use objc2_foundation::{NSArray, NSError, NSInteger, NSObject, NSObjectProtocol, NSString};

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCRunningApplication;
);

unsafe impl NSObjectProtocol for SCRunningApplication {}

impl SCRunningApplication {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send![SCRunningApplication::class(), new] }
    }

    pub fn bundle_identifier(&self) -> Retained<NSString> {
        unsafe { msg_send![self, bundleIdentifier] }
    }

    pub fn application_name(&self) -> Retained<NSString> {
        unsafe { msg_send![self, applicationName] }
    }

    pub fn process_id(&self) -> pid_t {
        unsafe { msg_send![self, processID] }
    }
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCWindow;
);

unsafe impl NSObjectProtocol for SCWindow {}

impl SCWindow {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send![SCWindow::class(), new] }
    }

    pub fn window_id(&self) -> CGWindowID {
        unsafe { msg_send![self, windowID] }
    }

    pub fn frame(&self) -> CGRect {
        unsafe { msg_send![self, frame] }
    }

    pub fn title(&self) -> Option<Retained<NSString>> {
        unsafe { msg_send![self, title] }
    }

    pub fn window_layer(&self) -> NSInteger {
        unsafe { msg_send![self, windowLayer] }
    }

    pub fn owning_application(&self) -> Option<Retained<SCRunningApplication>> {
        unsafe { msg_send![self, owningApplication] }
    }

    pub fn on_screen(&self) -> bool {
        unsafe { msg_send![self, isOnScreen] }
    }

    pub fn active(&self) -> bool {
        unsafe { msg_send![self, isActive] }
    }
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCDisplay;
);

unsafe impl NSObjectProtocol for SCDisplay {}

impl SCDisplay {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send![SCDisplay::class(), new] }
    }

    pub fn display_id(&self) -> CGDirectDisplayID {
        unsafe { msg_send![self, displayID] }
    }

    pub fn width(&self) -> NSInteger {
        unsafe { msg_send![self, width] }
    }

    pub fn height(&self) -> NSInteger {
        unsafe { msg_send![self, height] }
    }

    pub fn frame(&self) -> CGRect {
        unsafe { msg_send![self, frame] }
    }
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCShareableContent;
);

unsafe impl NSObjectProtocol for SCShareableContent {}

type CompletionHandler = RcBlock<dyn Fn(*mut SCShareableContent, *mut NSError)>;

impl SCShareableContent {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send![SCShareableContent::class(), new] }
    }

    fn new_completion_handler<F>(closure: F) -> CompletionHandler
    where
        F: Fn(Option<Retained<SCShareableContent>>, Option<Retained<NSError>>) + 'static,
    {
        RcBlock::new(move |sc: *mut Self, error: *mut NSError| {
            closure(
                if sc.is_null() {
                    None
                } else {
                    unsafe { Retained::retain(sc) }
                },
                if error.is_null() {
                    None
                } else {
                    unsafe { Retained::retain(error) }
                },
            );
        })
    }

    pub fn get_shareable_content_with_completion_closure<F>(closure: F)
    where
        F: Fn(Option<Retained<SCShareableContent>>, Option<Retained<NSError>>) + 'static,
    {
        let handler = Self::new_completion_handler(closure);
        unsafe { msg_send![class!(SCShareableContent), getShareableContentWithCompletionHandler: &*handler] }
    }

    pub fn get_shareable_content_excluding_desktop_windows<F>(exclude_desktop_windows: bool, on_screen_windows_only: bool, closure: F)
    where
        F: Fn(Option<Retained<SCShareableContent>>, Option<Retained<NSError>>) + 'static,
    {
        let handler = Self::new_completion_handler(closure);
        unsafe {
            msg_send![class!(SCShareableContent), getShareableContentExcludingDesktopWindows: exclude_desktop_windows, onScreenWindowsOnly: on_screen_windows_only, completionHandler: &*handler]
        }
    }

    pub fn get_shareable_content_excluding_desktop_windows_below_window<F>(exclude_desktop_windows: bool, window: &SCWindow, closure: F)
    where
        F: Fn(Option<Retained<SCShareableContent>>, Option<Retained<NSError>>) + 'static,
    {
        let handler = Self::new_completion_handler(closure);
        unsafe {
            msg_send![class!(SCShareableContent), getShareableContentExcludingDesktopWindows: exclude_desktop_windows, onScreenWindowsOnlyBelowWindow: window, completionHandler: &*handler]
        }
    }

    pub fn get_shareable_content_excluding_desktop_windows_above_window<F>(&self, exclude_desktop_windows: bool, window: &SCWindow, closure: F)
    where
        F: Fn(Option<Retained<SCShareableContent>>, Option<Retained<NSError>>) + 'static,
    {
        let handler = Self::new_completion_handler(closure);
        unsafe {
            msg_send![class!(SCShareableContent), getShareableContentExcludingDesktopWindows: exclude_desktop_windows, onScreenWindowsOnlyAboveWindow: window, completionHandler: &*handler]
        }
    }

    pub fn windows(&self) -> Retained<NSArray<SCWindow>> {
        unsafe { msg_send![self, windows] }
    }

    pub fn displays(&self) -> Retained<NSArray<SCDisplay>> {
        unsafe { msg_send![self, displays] }
    }

    pub fn applications(&self) -> Retained<NSArray<SCRunningApplication>> {
        unsafe { msg_send![self, applications] }
    }
}
