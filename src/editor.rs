use log::*;
use rtb_rs::platform;
use rtb_rs::window::{Window, Size};
use std::ffi::c_void;

pub struct EventHandler {

}

impl EventHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn event_handler(&mut self, event: crate::rtb_rs::Event) {
        info!("Got event!");
    }
}

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

        let mut event_handler = EventHandler::new();

        self.window = Some(Window::attach(parent, size, "derp", move |event: crate::rtb_rs::Event| {
            event_handler.event_handler(event);
        }));
        true
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}


