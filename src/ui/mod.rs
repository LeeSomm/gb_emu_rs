use minifb::{self, Scale, Window, WindowOptions};

pub const W: usize = 160;
pub const H: usize = 144;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Palette {
    B00 = 0x0004380f, // Darkest
    B01 = 0x00306230,
    B10 = 0x0083a30e,
    B11 = 0x009bbc0f, // Brightest
}

pub struct UI {
    pub window: Window,
    pub buffer: Vec<u32>,
}

impl UI {
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
            buffer: vec![Palette::B11 as u32; W * H],
        }
    }

    pub fn present(&mut self) {
        self.window.update_with_buffer(&self.buffer, W, H).unwrap();
    }

    pub fn clear(&mut self, color: Palette) {
        self.buffer.iter_mut().for_each(|x| *x = color as u32)
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: Palette) {
        // Make sure pixel location is in bounds
        if x < W && y < H {
            self.buffer[(y * W) + x] = color as u32;
        } else {
            // Choosing to panic for debugging. Can replace with no op later.
            panic!("Attempted to update offscreen pixel.")
        }
    }

    pub fn debug(&mut self, offset: usize) {
        let mut color_idx = offset % 4;
        let colors = [Palette::B00, Palette::B01, Palette::B10, Palette::B11];
        let end = H * W;

        for idx in 0..end {
            let x = idx % W;
            let y = idx / W;
            self.put_pixel(x, y, colors[color_idx]);
            color_idx = (color_idx + 1) % 4;
        }
    }
}
