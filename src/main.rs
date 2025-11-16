#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#[macro_use]
mod eadk;

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

    // Avoid instant click on Ok
    while eadk::keyboard::KeyboardState::scan().key_down(eadk::keyboard::Key::Ok) {
        eadk::time::wait_milliseconds(50);
    }

    // Your code here
}
