use crate::eadk::{display, keyboard, time};
calc_use!(alloc::string::String);
calc_use!(alloc::vec::Vec);

pub fn wait_ok_released() {
    while keyboard::KeyboardState::scan().key_down(keyboard::Key::Ok) {
        time::wait_milliseconds(50);
    }
}

pub fn log(text: &[&str]) {
    for i in 0..text.len() {
        display::draw_string(
            text[i],
            display::ScreenPoint::new(5, 5 + 14 * i as u16 as u16),
            false,
            display::Color565::from_rgb888(255, 255, 255),
            display::Color565::from_rgb888(0, 0, 0),
        );
    }
}

#[inline(always)]
pub fn refresh_simulator() {
    #[cfg(not(target_os = "none"))]
    crate::eadk::keyboard::KeyboardState::scan();
}
