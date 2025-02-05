use std::ffi::{c_char, c_int, CString};

#[allow(warnings)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::Color;

// Could use https://crates.io/crates/raylib instead.
mod api {
    use super::*;

    pub fn init_window(width: i32, height: i32, title: &str) {
        unsafe {
            ffi::InitWindow(
                width as c_int,
                height as c_int,
                CString::new(title).unwrap().as_ptr() as *const c_char,
            );
        }
    }

    pub fn window_should_close() -> bool {
        unsafe { ffi::WindowShouldClose() }
    }

    pub fn close_window() {
        unsafe { ffi::CloseWindow() }
    }

    pub fn is_window_ready() -> bool {
        unsafe { ffi::IsWindowReady() }
    }

    pub fn begin_drawing() {
        unsafe { ffi::BeginDrawing() }
    }

    pub fn end_drawing() {
        unsafe { ffi::EndDrawing() }
    }

    pub fn clear_background(color: Color) {
        unsafe { ffi::ClearBackground(color) }
    }

    pub fn draw_rectangle(x: i32, y: i32, width: i32, height: i32, color: Color) {
        unsafe {
            ffi::DrawRectangle(
                x as c_int,
                y as c_int,
                width as c_int,
                height as c_int,
                color,
            );
        }
    }

    pub fn draw_rectangle_lines(x: i32, y: i32, width: i32, height: i32, color: Color) {
        unsafe {
            ffi::DrawRectangleLines(
                x as c_int,
                y as c_int,
                width as c_int,
                height as c_int,
                color,
            );
        }
    }

    pub fn draw_text<T: Into<Vec<u8>>>(text: T, x: i32, y: i32, font_size: i32, color: Color) {
        unsafe {
            ffi::DrawText(
                CString::new(text).unwrap().as_ptr() as *const c_char,
                x as c_int,
                y as c_int,
                font_size as c_int,
                color,
            );
        }
    }

    pub fn draw_centered_text<T: Into<Vec<u8>>>(
        text: T,
        x: i32,
        y: i32,
        font_size: i32,
        color: Color,
    ) {
        let text: Vec<u8> = text.into();
        let text_size = measure_text(text.clone(), font_size);
        draw_text(text, x - text_size / 2, y, font_size, color);
    }

    pub fn get_frame_time() -> f32 {
        unsafe { ffi::GetFrameTime() }
    }

    pub fn get_time() -> f64 {
        unsafe { ffi::GetTime() }
    }

    pub fn is_key_down(key: KeyboardKey) -> bool {
        unsafe {
            match key {
                KeyboardKey::KeyUp => ffi::IsKeyDown(ffi::KeyboardKey_KEY_UP as c_int),
                KeyboardKey::KeyDown => ffi::IsKeyDown(ffi::KeyboardKey_KEY_DOWN as c_int),
                KeyboardKey::KeyLeft => ffi::IsKeyDown(ffi::KeyboardKey_KEY_LEFT as c_int),
                KeyboardKey::KeyRight => ffi::IsKeyDown(ffi::KeyboardKey_KEY_RIGHT as c_int),
            }
        }
    }

    pub fn measure_text<T: Into<Vec<u8>>>(text: T, font_size: i32) -> i32 {
        unsafe {
            ffi::MeasureText(
                CString::new(text).unwrap().as_ptr() as *const c_char,
                font_size as c_int,
            )
        }
    }
}

pub use api::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum KeyboardKey {
    KeyUp = 0,
    KeyDown = 1,
    KeyLeft = 2,
    KeyRight = 3,
}

impl Color {
    pub const BLACK: Self = Self::from_u32(0x00000000);
    pub const WHITE: Self = Self::from_u32(0xFFFFFFFF);
    pub const RED: Self = Self::from_u32(0xFF0000FF);
    pub const GREEN: Self = Self::from_u32(0x00FF00FF);
    pub const BLUE: Self = Self::from_u32(0x0000FFFF);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn from_u32(color: u32) -> Self {
        let r = ((color >> 24) & 0xFF) as u8;
        let g = ((color >> 16) & 0xFF) as u8;
        let b = ((color >> 8) & 0xFF) as u8;
        let a = (color & 0xFF) as u8;
        Self { r, g, b, a }
    }
}
