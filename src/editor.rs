use log::*;
use rtb_rs::platform;
use rtb_rs::window::{Window, Size};
use std::ffi::c_void;

pub struct Editor {
    window: Option<Window>,
    is_open: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            window: None,
            is_open: false,
        }
    }
}

impl vst::editor::Editor for Editor {
    fn size(&self) -> (i32, i32) {
        info!("Editor::size()");

        (1000, 1000)
    }

    fn position(&self) -> (i32, i32) {
        info!("Editor::position()");

        (0, 0)
    }

    fn close(&mut self) {
        info!("Editor::close()");
        self.is_open = false;
    }

    fn open(&mut self, parent: *mut c_void) {
        info!("Editor::open()");
        self.is_open = true;

        let size = Size{
            width: 1000,
            height: 1000,
        };

        self.window = Some(Window::attach(parent, size, "derp"));
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}
