#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[macro_use]
mod nadk;

use crate::nadk::display::{Color565, SCREEN_RECT, ScreenPoint, draw_string, push_rect_uniform};
use crate::nadk::keyboard::{Key, wait_until_pressed};
use crate::nadk::storage::{CalculatorModel, get_calculator_model};
use crate::nadk::utils::wait_ok_released;

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

    push_rect_uniform(SCREEN_RECT, Color565::from_rgb888(255, 255, 255));
    draw_string(
        "Exact same codebase.",
        ScreenPoint::new(20, 20),
        true,
        Color565::from_rgb888(0, 0, 0),
        Color565::new(255, 255, 255),
    );
    match get_calculator_model() {
        CalculatorModel::Upsilon => draw_string(
            "Hello from Upsilon!",
            ScreenPoint::new(20, 50),
            true,
            Color565::from_rgb888(0, 0, 0),
            Color565::new(255, 255, 255),
        ),
        CalculatorModel::Simulator => draw_string(
            "Hello from the simulator!",
            ScreenPoint::new(20, 50),
            true,
            Color565::from_rgb888(0, 0, 0),
            Color565::new(255, 255, 255),
        ),
        _ => draw_string(
            "Hello from Epsilon!",
            ScreenPoint::new(20, 50),
            true,
            Color565::from_rgb888(0, 0, 0),
            Color565::new(255, 255, 255),
        ),
    };
    wait_until_pressed(Key::Ok);
}
