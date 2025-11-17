use crate::eadk::{keyboard, time};

pub fn wait_ok_released() {
    while keyboard::KeyboardState::scan().key_down(keyboard::Key::Ok) {
        time::wait_milliseconds(50);
    }
}
