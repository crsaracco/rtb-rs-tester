use rtb_rs::Window;
use rtb_rs::WindowDimensions;

pub struct Editor {
    window: Window,
}

impl Editor {
    pub fn new() -> Self {
        let window_dimensions = WindowDimensions {
            width: 1000,
            height: 1000,
        };

        Self {
            window: Window::open_under(None, window_dimensions, "rtb-rs test window"),
        }
    }
}