use core_foundation::base::OSStatus;

/// Converts an `OSStatus` into a `Result`
#[inline]
pub(crate) fn status_to_result(status: OSStatus) -> Result<(), OSStatus> {
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}
