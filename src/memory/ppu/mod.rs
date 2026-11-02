use bitfields::{bitfield, bitflag};
use lcd::Lcd;
use oam::Oam;
use vram::VRam;

use crate::memory::ppu::lcd::PPUMode;

pub mod lcd;
pub mod oam;
pub mod vram;

pub struct Ppu {
    pub vram: VRam, // Addresses 0x8000-0x9FFF
    pub oam: Oam,   // Addresses 0xFE00-0xFE9F
    pub lcd: Lcd,   // 0xFF40-FF4B LCD
    timer: u16,
    pub fb: [u8; 160 * 144], // Frame buffer
    fifo_bg: [u16; 16],      // Background Pixel FIFO
    fifo_obj: [u16; 16],     // Object Pixel FIFO
}

impl Ppu {
    pub fn init() -> Self {
        Self {
            vram: VRam::init(),
            oam: Oam::init(),
            lcd: Lcd::init(), // 0xFF40-FF4B LCD
            timer: 0,
            fb: [0; 160 * 144],
            fifo_bg: [0; 16],
            fifo_obj: [0; 16],
        }
    }

    pub fn cpu_vram_read(&self, addr: u16) -> u8 {
        if let PPUMode::Drawing = self.get_mode() {
            0xFF // Undefined data when VRAM is read during PPU Mode 3
        } else {
            self.vram.read(addr)
        }
    }

    pub fn cpu_vram_write(&mut self, addr: u16, val: u8) {
        if let PPUMode::Drawing = self.get_mode() {
            () // Write ignored when VRAM is read during PPU Mode 3
        } else {
            self.vram.write(addr, val);
        }
    }

    pub fn cpu_oam_read(&self, addr: u16) -> u8 {
        if let PPUMode::OAMSearch | PPUMode::Drawing = self.get_mode() {
            0xFF // Undefined data when OAM is read during PPU Modes 2 & 3
        } else {
            self.oam.read(addr)
        }
    }

    pub fn cpu_oam_write(&mut self, addr: u16, val: u8) {
        if let PPUMode::OAMSearch | PPUMode::Drawing = self.get_mode() {
            () // Write ignored when OAM is read during PPU Modes 2 & 3
        } else {
            self.oam.write(addr, val);
        }
    }

    pub fn advance(&mut self, cycles: u16) {
        self.timer = self.timer.wrapping_add(cycles);
    }

    fn get_mode(&self) -> PPUMode {
        self.lcd.stat.ppu_mode()
    }
}
