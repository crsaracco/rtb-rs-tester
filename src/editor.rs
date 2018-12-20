use log::*;
use rtb_rs::platform;
use rtb_rs::Window;
use rtb_rs::WindowDimensions;
use std::ffi::c_void;

pub struct Editor {
    window: Option<Window>,
    window_dimensions: WindowDimensions,
    is_open: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            window: None,
            window_dimensions: WindowDimensions {
                width: 1000,
                height: 1000,
                x: 0,
                y: 0,
            },
            is_open: false,
        }
    }
}

impl vst::editor::Editor for Editor {
    fn size(&self) -> (i32, i32) {
        info!("Editor::size()");
        (
            self.window_dimensions.width as i32,
            self.window_dimensions.height as i32,
        )
    }

    fn position(&self) -> (i32, i32) {
        info!("Editor::position()");
        (
            self.window_dimensions.x as i32,
            self.window_dimensions.y as i32,
        )
    }

    fn close(&mut self) {
        info!("Editor::close()");
        self.is_open = false;
    }

    fn open(&mut self, parent: *mut c_void) {
        info!("Editor::open()");
        self.is_open = true;
        let parent_id = parent as u32;
        let parent_window_handle: Option<platform::WindowHandle> =
            Some(platform::WindowHandle::new(parent_id));

        // TODO: maybe need a `WindowDimensions::clone()`, or maybe I'm just doing it wrong.
        let window_dimensions = WindowDimensions {
            width: self.window_dimensions.width,
            height: self.window_dimensions.height,
            x: self.window_dimensions.x,
            y: self.window_dimensions.y,
        };

        self.window = Some(Window::open_under(
            parent_window_handle,
            window_dimensions,
            "rtb-rs test window",
        ));
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}
