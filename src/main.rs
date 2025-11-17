#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#[macro_use]
mod eadk;

use crate::eadk::utils::wait_ok_released;

configure_app!(b"test\0", 5, "../target/icon.nwi", 745);
setup_allocator!();

#[unsafe(no_mangle)]
fn main() {
    // Init the heap
    #[cfg(target_os = "none")]
    {
        let heap_size: usize = heap_size();
        unsafe { HEAP.init(HEAP_START as usize, heap_size) }
    }

    wait_ok_released();

    // Your code here
}
