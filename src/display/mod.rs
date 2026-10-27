use minifb::{self, Window, WindowOptions};

const W: usize = 160;
const H: usize = 144;

pub struct Display {
    pub window: Window,
    pub buffer: Vec<u16>,
}

impl Display {
    pub fn init() -> Self {
        let window = Window::new("gb_emu_rs", W, H, WindowOptions::default()).unwrap();

        Self {
            window,
            buffer: vec![0; W * H],
        }
    }
}
