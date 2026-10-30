use crate::{memory::MemoryBus, ppu::Tile};
use minifb::{self, Scale, Window, WindowOptions};

pub const W: usize = 160;
pub const H: usize = 144;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Palette {
    Black = 0x0004380f, // Darkest
    DarkGray = 0x00306230,
    LightGray = 0x0083a30e,
    White = 0x009bbc0f, // Brightest
}

impl From<u8> for Palette {
    fn from(id: u8) -> Self {
        match id {
            0 => Palette::White,
            1 => Palette::LightGray,
            2 => Palette::DarkGray,
            _ => Palette::Black,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum PaletteRegister {
    BGP = 0xFF47,  // Full color range
    OBP0 = 0xFF48, // Color index 0 is transparent
    OBP1 = 0xFF49, // Color index 0 is transparent
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
            buffer: vec![Palette::Black as u32; W * H],
        }
    }

    pub fn present(&mut self) {
        self.window.update_with_buffer(&self.buffer, W, H).unwrap();
    }

    pub fn clear(&mut self, color: Palette) {
        self.buffer.iter_mut().for_each(|x| *x = color as u32)
    }

    pub fn decode_tile(&self, tile: &Tile) -> [u8; 64] {
        let mut pixel_data = [0u8; 64];
        for row in 0..8 {
            let lo = tile.data[row * 2];
            let hi = tile.data[row * 2 + 1];
            for col in 0..8 {
                let bit = 7 - col;
                let color_id = (((hi >> bit) & 1) << 1) | ((lo >> bit) & 1);
                pixel_data[(row * 8) + col] = color_id;
            }
        }
        pixel_data
    }

    pub fn draw_tile(
        &mut self,
        bus: &MemoryBus,
        register: PaletteRegister,
        x: usize,
        y: usize,
        tile: &Tile,
    ) {
        let tile_data = self.decode_tile(tile);

        for ax in x..x + 8 {
            for ay in y..y + 8 {
                let color_id = tile_data[((ay - y) * 8) + (ax - x)];
                match register {
                    PaletteRegister::BGP => {
                        let bgp = bus.read(register as u16);
                        let shade = (bgp >> (color_id * 2)) & 0b11;
                        self.put_pixel(ax, ay, Palette::from(shade));
                    }
                    PaletteRegister::OBP0 | PaletteRegister::OBP1 => {
                        // Color Index 0 is transparent for OBP
                        if color_id != 0 {
                            let obp = bus.read(register as u16);
                            let shade = (obp >> (color_id * 2)) & 0b11;
                            self.put_pixel(ax, ay, Palette::from(shade));
                        }
                    }
                }
            }
        }
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

    pub fn debug(&mut self, bus: &MemoryBus, offset: u8) {
        let mut color_idx = u8::from(offset % 4);

        let end = H * W;

        for idx in 0..end {
            let x = idx % W;
            let y = idx / W;
            self.put_pixel(x, y, Palette::from(color_idx));
            color_idx = (color_idx + 1) % 4;
        }

        let tile = Tile {
            data: [
                0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56,
                0x38, 0x7C,
            ],
        };

        self.draw_tile(bus, PaletteRegister::BGP, 5, 10, &tile);
    }
}
