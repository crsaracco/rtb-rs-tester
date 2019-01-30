use log::*;
use rtb_rs::window::{Window, Size};
use std::ffi::c_void;
use std::sync::Arc;

use crate::parameters::Parameters;
use crate::event_handler::EventHandler;

pub struct Editor {
    window: Option<Window>,
    parameters: Arc<Parameters>,
    is_open: bool,
}

impl Editor {
    pub fn new(parameters: Arc<Parameters>) -> Self {
        let editor = Self {
            window: None,
            parameters,
            is_open: false,
        };

        editor
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

        let mut event_handler = Box::new(EventHandler::new(self.parameters.clone()));

        self.window = Some(Window::attach(
            parent,
            size,
            "derp",
            event_handler,
        ));

        true
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}