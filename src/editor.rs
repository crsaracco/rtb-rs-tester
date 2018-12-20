use rtb_rs::Window;
use rtb_rs::WindowDimensions;
use rtb_rs::platform;
use std::ffi::c_void;
use log::*;

pub struct Editor {
    window: Option<Window>,

    // TODO: move this stuff into the rtb_rs::Window struct, if they exist there.
    is_open: bool,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    // End TODO
}

impl Editor {
    pub fn new() -> Self {
        Self {
            window: None,
            is_open: false,
            x: 0,
            y: 0,
            width: 1000,
            height: 1000,
        }
    }
}

impl vst::editor::Editor for Editor {
    fn size(&self) -> (i32, i32) {
        info!("Editor::size()");
        (self.width, self.height)
    }

    fn position(&self) -> (i32, i32) {
        info!("Editor::position()");
        (self.x, self.y)
    }

    fn close(&mut self) {
        info!("Editor::close()");
        self.is_open = false;
    }

    fn open(&mut self, parent: *mut c_void) {
        info!("Editor::open()");
        self.is_open = true;
        let parent_id = parent as u32;
        let window_dimensions = WindowDimensions {
            width: 1000,
            height: 1000,
        };
        let parent_window_handle: Option<platform::WindowHandle> = Some(platform::WindowHandle::new(parent_id));

        self.window = Some(Window::open_under(parent_window_handle, window_dimensions, "rtb-rs test window"));
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}