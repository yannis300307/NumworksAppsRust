calc_use!(alloc::ffi::CString);
calc_use!(alloc::vec::Vec);

#[cfg(not(target_os = "none"))]
use std::ffi::CString;

use core::ffi::c_char;

pub const SCREEN_RECT: ScreenRect = ScreenRect {
    x: 0,
    y: 0,
    width: 320,
    height: 240,
};
pub const COLOR_BLACK: Color565 = Color565::from_rgb888(0, 0, 0);
pub const COLOR_WHITE: Color565 = Color565::from_rgb888(255, 255, 255);
pub const COLOR_RED: Color565 = Color565::from_rgb888(255, 0, 0);
pub const COLOR_GREEN: Color565 = Color565::from_rgb888(0, 255, 0);
pub const COLOR_BLUE: Color565 = Color565::from_rgb888(0, 0, 255);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color565 {
    pub value: u16,
}

impl Color565 {
    #[inline]
    pub const fn new(r: u16, g: u16, b: u16) -> Self {
        Color565 {
            value: r << 11 | g << 5 | b,
        }
    }

    pub const fn from_rgb888(r: u16, g: u16, b: u16) -> Color565 {
        Color565 {
            value: ((r & 0b11111000) << 8) | ((g & 0b11111100) << 3) | (b >> 3),
        }
    }

    pub const fn get_components(&self) -> (u16, u16, u16) {
        let r = self.value >> 0xB;
        let g = (self.value & 0x7E0) >> 5;
        let b = self.value & 0x1F;

        (r, g, b)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ScreenRect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ScreenPoint {
    pub x: u16,
    pub y: u16,
}

pub fn push_rect(rect: ScreenRect, pixels: &[Color565]) {
    unsafe {
        eadk_display_push_rect(rect, pixels.as_ptr());
    }
}

pub fn pull_rect(rect: ScreenRect) -> Vec<Color565> {
    let size = rect.width as usize * rect.height as usize;
    let mut vec: Vec<Color565> = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(COLOR_BLACK);
    }

    unsafe {
        eadk_display_pull_rect(rect, vec.as_mut_slice().as_mut_ptr());
    }
    vec
}

pub fn push_rect_uniform(rect: ScreenRect, color: Color565) {
    unsafe {
        eadk_display_push_rect_uniform(rect, color);
    }
}

pub fn wait_for_vblank() {
    unsafe {
        eadk_display_wait_for_vblank();
    }
}

pub fn draw_string(
    text: &str,
    point: ScreenPoint,
    large_font: bool,
    text_color: Color565,
    background_color: Color565,
) -> Option<()> {
    let c_string = CString::new(text).ok()?;
    unsafe {
        eadk_display_draw_string(
            c_string.as_ptr(),
            point,
            large_font,
            text_color,
            background_color,
        )
    }
    Some(())
}

unsafe extern "C" {
    fn eadk_display_push_rect_uniform(rect: ScreenRect, color: Color565);
    fn eadk_display_push_rect(rect: ScreenRect, color: *const Color565);
    fn eadk_display_wait_for_vblank();
    fn eadk_display_pull_rect(rect: ScreenRect, color: *mut Color565);
    fn eadk_display_draw_string(
        text: *const c_char,
        point: ScreenPoint,
        large_font: bool,
        text_color: Color565,
        background_color: Color565,
    );
}
