use lcd::Lcd;
use oam::Oam;
use palette::{Palette, PaletteRegister};
use vram::VRam;

use crate::memory::ppu::lcd::{PpuMode, TileMapRef};

pub mod lcd;
pub mod oam;
pub mod palette;
pub mod vram;

const DISCARDED_PIXELS: u8 = 7;

pub struct Ppu {
    pub vram: VRam, // Addresses 0x8000-0x9FFF
    pub oam: Oam,   // Addresses 0xFE00-0xFE9F
    pub lcd: Lcd,   // 0xFF40-FF4B LCD
    // timer: u16, //TODO: Consider removal
    // Display
    pub fb: [Palette; 160 * 144], // Frame buffer
    pub fb_ready: bool,
}

impl Ppu {
    pub fn init() -> Self {
        Self {
            vram: VRam::init(),
            oam: Oam::init(),
            lcd: Lcd::init(), // 0xFF40-FF4B LCD
            // timer: 0,
            fb: [Palette::from_bits(0x0); 160 * 144],
            fb_ready: false,
        }
    }

    pub fn cpu_vram_read(&self, addr: u16) -> u8 {
        if let PpuMode::Drawing = self.get_mode()
            && !self.lcd.lcdc.lcd_ppu_enable()
        {
            0xFF // Undefined data when VRAM is read during PPU Mode 3
        } else {
            self.vram.read(addr)
        }
    }

    pub fn cpu_vram_write(&mut self, addr: u16, val: u8) {
        if let PpuMode::Drawing = self.get_mode()
            && !self.lcd.lcdc.lcd_ppu_enable()
        {
            () // Write ignored when VRAM is read during PPU Mode 3
        } else {
            self.vram.write(addr, val);
        }
    }

    pub fn cpu_oam_read(&self, addr: u16) -> u8 {
        if let PpuMode::OAMSearch | PpuMode::Drawing = self.get_mode()
            && !self.lcd.lcdc.lcd_ppu_enable()
        {
            0xFF // Undefined data when OAM is read during PPU Modes 2 & 3
        } else {
            self.oam.read(addr)
        }
    }

    pub fn cpu_oam_write(&mut self, addr: u16, val: u8) {
        if let PpuMode::OAMSearch | PpuMode::Drawing = self.get_mode()
            && !self.lcd.lcdc.lcd_ppu_enable()
        {
            () // Write ignored when OAM is read during PPU Modes 2 & 3
        } else {
            self.oam.write(addr, val);
        }
    }

    pub fn advance(&mut self, cycles: u16) {
        // self.timer = self.timer.wrapping_add(cycles);
        for _dot in 0..cycles {
            self.lcd.dots += 1;
            if self.lcd.dots > 455 {
                self.lcd.dots = self.lcd.dots % 456;
                // If frame complete, signal fb is ready.
                if self.lcd.next_scanline() {
                    self.fb_ready = true;
                }
            }
            self.set_mode();
            if let PpuMode::Drawing = self.get_mode() {
                self.fifo_pixel_fetcher();
            }
            // Interrupt flags
            self.lcd.stat.set_lyc_eq_ly(self.lcd.ly == self.lcd.lyc);
        }
    }

    fn get_mode(&self) -> PpuMode {
        self.lcd.stat.ppu_mode()
    }

    fn set_mode(&mut self) {
        if (144..=153).contains(&self.lcd.ly) {
            self.lcd.stat.set_ppu_mode(PpuMode::VBlank);
            // Reset window_y_condition
            if self.lcd.window_y_condition {
                self.lcd.window_y_condition = false;
            }
        } else if (0..80).contains(&self.lcd.dots) {
            self.lcd.stat.set_ppu_mode(PpuMode::OAMSearch);
            // Reset drawing_complete flag
            if self.lcd.drawing_complete {
                self.lcd.drawing_complete = false
            }
        } else if self.lcd.drawing_complete {
            self.lcd.stat.set_ppu_mode(PpuMode::HBlank);
        } else {
            self.lcd.stat.set_ppu_mode(PpuMode::Drawing);
        }
    }

    // Currently pixel by pixel.
    fn fifo_pixel_fetcher(&mut self) {
        // Determine whether Window or BG tile
        let tile_type: TileType;
        let tile_map: TileMapRef;
        if self.lcd.lcdc.window_enable()
            && self.lcd.window_y_condition
            // Won't overflow b/c output_x wraps at 160
            && (self.lcd.output_x + DISCARDED_PIXELS) >= self.lcd.wx
        {
            // If drawing inside window
            tile_map = self.lcd.lcdc.window_tilemap();
            tile_type = TileType::Window;
            self.lcd.drew_window_this_line = true;
        } else {
            // If drawing outside of window
            tile_map = self.lcd.lcdc.bg_tilemap();
            tile_type = TileType::Bg;
        }

        // Calculate xy coordinate of window/bg tile
        let (idx_x, tile_x, idx_y, tile_y) = self.pixel_to_map_and_tile_coords(tile_type);

        let tile_idx = self.vram.tile_index(tile_map.vram_offset(), idx_x, idx_y);

        let tile = self.vram.tile(tile_idx, self.lcd.lcdc.bg_window_tiles());

        let lo_byte = tile[tile_y * 2];
        let hi_byte = tile[tile_y * 2 + 1];
        let bit = 7 - tile_x;
        let color_id = (((hi_byte >> bit) & 1) << 1) | ((lo_byte >> bit) & 1);

        if let Some(palette) = self.convert_to_palette(PaletteRegister::BGP, color_id) {
            self.fb[(self.lcd.ly as u16 * 160 + self.lcd.output_x as u16) as usize] = palette;
        }
        self.lcd.output_x = (self.lcd.output_x + 1) % 160;
        if self.lcd.output_x == 0 {
            self.lcd.drawing_complete = true;
        }
    }

    fn pixel_to_map_and_tile_coords(&self, tile_type: TileType) -> (usize, usize, usize, usize) {
        let x: usize;
        let y: usize;
        match tile_type {
            TileType::Bg => {
                x = self.lcd.output_x.wrapping_add(self.lcd.scx) as usize;
                y = self.lcd.ly.wrapping_add(self.lcd.scy) as usize;
            }
            TileType::Window => {
                // Won't underflow given (self.lcd.output_x + DISCARDED_PIXELS) >= self.lcd.wx
                x = (self.lcd.output_x + DISCARDED_PIXELS - self.lcd.wx) as usize;
                y = self.lcd.window_line as usize;
            }
        }
        let idx_x = x / 8;
        let tile_x = x % 8;
        let idx_y = y / 8;
        let tile_y = y % 8;
        (idx_x, tile_x, idx_y, tile_y)
    }

    // None means transparency
    fn convert_to_palette(&self, register: PaletteRegister, color_id: u8) -> Option<Palette> {
        let pal_reg: u8;
        match register {
            PaletteRegister::BGP => {
                pal_reg = self.lcd.bgp;
            }
            PaletteRegister::OBP0 => {
                // Color Index 0 is transparent for OBP
                if color_id != 0 {
                    pal_reg = self.lcd.obp0;
                } else {
                    return None;
                }
            }
            PaletteRegister::OBP1 => {
                // Color Index 0 is transparent for OBP
                if color_id != 0 {
                    pal_reg = self.lcd.obp1;
                } else {
                    return None;
                }
            }
        }
        let shade = (pal_reg >> (color_id * 2)) & 0b11;
        return Some(Palette::from_bits(shade));
    }
}

enum TileType {
    Bg,
    Window,
}
