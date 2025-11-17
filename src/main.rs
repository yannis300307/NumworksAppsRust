#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#[macro_use]
mod eadk;

use crate::eadk::utils::wait_ok_released;

// The app name must be a C string and the app name size must include the end line NULL character
configure_app!(b"SampleApp\0", 10, "../target/icon.nwi", 745);

// Setup the heap allocator if you need one
setup_allocator!();

#[unsafe(no_mangle)]
fn main() {
    // You must call setup_allocator!() before
    init_heap!(); 
    wait_ok_released();

    // Your code here

    crate::eadk::utils::log(&["Hello, World!"]);
    crate::eadk::keyboard::wait_until_pressed(eadk::keyboard::Key::Ok);
}
