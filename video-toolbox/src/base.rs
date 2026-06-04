use core_foundation::base::OSStatus;

/// Converts an `OSStatus` into a `Result`.
#[inline]
pub(crate) fn status_to_result(status: OSStatus) -> Result<(), OSStatus> {
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTInt32Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTInt32Size {
    pub width: i32,
    pub height: i32,
}
