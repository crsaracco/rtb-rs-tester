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

        (250, 250)
    }

    fn position(&self) -> (i32, i32) {
        info!("Editor::position()");

        (0, 0)
    }

    fn close(&mut self) {
        info!("Editor::close()");
        self.is_open = false;
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        info!("Editor::open()");
        self.is_open = true;

        let size = Size{
            width: 1000,
            height: 1000,
        };

        self.window = Some(Window::attach(parent, size, "derp", asdf));
        true
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}

fn asdf(event: crate::rtb_rs::Event) {
    info!("Got event!");
}
