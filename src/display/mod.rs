use minifb::{self, Scale, Window, WindowOptions};

const W: usize = 160;
const H: usize = 144;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Palette {
    x00 = 0x0004380f,
    x01 = 0x00306230,
    x10 = 0x008bac0f,
    x11 = 0x009bbc0f,
}

pub struct Display {
    pub window: Window,
    pub buffer: Vec<u32>,
}

impl Display {
    pub fn init() -> Self {
        let opts = WindowOptions {
            scale: Scale::X4,
            resize: false,
            ..WindowOptions::default()
        };
        let mut window = Window::new("gb_emu_rs", W, H, opts).unwrap();
        window.set_target_fps(60);
        Self {
            window,
            buffer: vec![Palette::x11 as u32; W * H],
        }
    }

    pub fn present(&mut self) {
        self.window.update_with_buffer(&self.buffer, W, H).unwrap();
    }

    pub fn clear(&mut self, color: Palette) {
        self.buffer.iter_mut().for_each(|x| *x = color as u32)
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: Palette) {
        todo!()
    }
}
