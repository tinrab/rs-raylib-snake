use crate::raylib;

pub struct Window;

impl Window {
    pub fn new(width: i32, height: i32, title: &str) -> Self {
        raylib::init_window(width, height, title);

        Self {}
    }

    pub fn should_close(&self) -> bool {
        raylib::window_should_close()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        raylib::close_window();
    }
}
