pub fn wait_microseconds(delay: u32) {
    unsafe {
        eadk_timing_usleep(delay);
    }
}

pub fn wait_seconds(delay: f32) {
    unsafe {
        eadk_timing_usleep((delay * 1000000.) as u32);
    }
}

pub fn wait_milliseconds(delay: u32) {
    unsafe {
        eadk_timing_msleep(delay);
    }
}

pub fn get_current_time_millis() -> u64 {
    unsafe { eadk_timing_millis() }
}

pub fn get_current_time_seconds() -> f32 {
    unsafe { eadk_timing_millis() as f32 * 0.001}
}

unsafe extern "C" {
    fn eadk_timing_usleep(delay: u32);
    fn eadk_timing_msleep(delay: u32);
    fn eadk_timing_millis() -> u64;
}
