use objc2_core_audio as sys;

#[inline]
pub fn current_host_time() -> u64 {
    unsafe { sys::AudioGetCurrentHostTime() }
}

#[inline]
pub fn host_clock_frequency() -> f64 {
    unsafe { sys::AudioGetHostClockFrequency() }
}

#[inline]
pub fn host_clock_minimum_time_delta() -> u32 {
    unsafe { sys::AudioGetHostClockMinimumTimeDelta() }
}

#[inline]
pub fn host_time_to_nanos(host_time: u64) -> u64 {
    unsafe { sys::AudioConvertHostTimeToNanos(host_time) }
}

#[inline]
pub fn nanos_to_host_time(nanos: u64) -> u64 {
    unsafe { sys::AudioConvertNanosToHostTime(nanos) }
}
