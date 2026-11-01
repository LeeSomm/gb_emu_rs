use crate::{
    cart::{Cartridge, mbc::BankingMode},
    interrupt::{Interrupt, interrupt_flag_raise},
};
use io::Io;
use ppu::Ppu;

pub mod io;
pub mod ppu;
pub mod timer;
pub mod vram;

pub struct MemoryBus {
    cart: Cartridge,    // Addresses 0x0000-0x7FFF & 0xA000-0xBFFF
    vram: [u8; 0x2000], // Addresses 0x8000-0x9FFF
    wram: [u8; 0x2000], // Addresses 0xC000-DFFF & 0xE000-FDFF (via Echo RAM)
    oam: [u8; 0xA0],    // Addresses 0xFE00-0xFE9F
    io: Io,             // Address 0xFF00-0xFF7F
    hram: [u8; 0x7F],   // Address 0xFF80-FFFE
    ie: u8,             // Address 0xFFFF
    ppu: Ppu,
}

impl MemoryBus {
    pub fn init(cart: Cartridge) -> Self {
        MemoryBus {
            cart,
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io: Io::init(),
            hram: [0; 0x7F],
            ie: 0,
            ppu: Ppu::default(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0..=0x7FFF => self.cart_rom_read(addr),
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000],
            0xA000..=0xBFFF => self.cart_ram_read(addr),
            // If CGB mode, must modify 0xD000-0xDFFF for switchable banks
            0xC000..=0xDFFF => self.wram[addr as usize - 0xC000],
            0xE000..=0xFDFF => self.wram[addr as usize - 0xE000],
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00],
            0xFEA0..=0xFEFF => 0, // Unusable memory location
            0xFF00..=0xFF7F => self.io.read(addr),
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80],
            0xFFFF => self.ie,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        // For test ROMs to print out results
        // if addr == 0xFF02 && val == 0x81 {
        //     let c = self.read(0xFF01);
        //     print!("{}", c as char)
        // }

        match addr {
            0x0..=0x1FFF => self.cart.mbc.set_ramg(val),
            0x2000..=0x3FFF => self.cart.mbc.set_bank1(val),
            0x4000..=0x5FFF => self.cart.mbc.set_bank2(val),
            0x6000..=0x7FFF => self.cart.mbc.set_mode(val),
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000] = val,
            0xA000..=0xBFFF => self.cart_ram_write(addr, val),
            // If CGB mode, must modify 0xD000-0xDFFF for switchable banks
            0xC000..=0xDFFF => self.wram[addr as usize - 0xC000] = val,
            0xE000..=0xFDFF => self.wram[addr as usize - 0xE000] = val, // Echo RAM writes are effectively 0xC000-0xDDFF
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00] = val,
            0xFEA0..=0xFEFF => (), // Writes to this location have no effect
            0xFF00..=0xFF45 => self.io.write(addr, val),
            0xFF46 => {
                // OAM DMA Transfer requires full MemoryBus
                self.io.lcd.od_trans = val;
                self.oam_dma_transfer(val);
            }
            0xFF47..=0xFF7F => self.io.write(addr, val),
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80] = val,
            0xFFFF => self.ie = val,
        };
        // println!("{:?}", self.cart.mbc);
    }

    fn cart_rom_read(&self, addr: u16) -> u8 {
        // Initial check if addr is within bounds of ROM data
        self.valid_address_check(addr as usize);
        match self.cart.header.cartridge_type {
            // No MBC
            0x0 | 0x8 | 0x9 => return self.cart.data[addr as usize],
            // MBC 1
            0x1..=0x3 => {
                if (0x0..=0x3FFF).contains(&addr) {
                    match self.cart.mbc.mode {
                        BankingMode::Advanced => {
                            let addr = ((self.cart.mbc.bank2 as u32) << 19) | (addr as u32);
                            self.valid_address_check(addr as usize);
                            return self.cart.data[addr as usize];
                        }
                        BankingMode::Locked => {
                            return self.cart.data[addr as usize];
                        }
                    }
                }
                // 0x4000..=7FFF (Same behavior in both modes)
                let rom_mask = self.cart.data.len() - 1;
                let addr = (((self.cart.mbc.bank2 as usize) << 19)
                    | ((self.cart.mbc.bank1 as usize) << 14)
                    | (addr as usize & 0x3FFF))
                    & rom_mask;
                self.cart.data[addr]
            }
            _ => {
                todo!(
                    "Implement MBC logic for Cartridge Type {:?}",
                    self.cart.header.cartridge_type
                )
            }
        }
    }

    fn valid_address_check(&self, addr: usize) {
        // For simplicity, panics when an invalid address is found
        let len = self.cart.cart_data_len();
        if addr > len {
            panic!(
                "Attemped to write to address {:#x?}, but ROM length is only {:#x?}",
                addr, len
            )
        }
    }

    fn cart_ram_write(&mut self, addr: u16, _val: u8) {
        if !self.cart.mbc.ramg {
            ()
        } else {
            todo!("Implement writes for Cartridge RAM; addr: {:#x?}", addr)
        }
    }

    fn cart_ram_read(&self, addr: u16) -> u8 {
        if !self.cart.mbc.ramg {
            0xFF
        } else {
            todo!("Implement writes for Cartridge RAM; addr: {:#x?}", addr)
        }
    }

    pub fn tick(&mut self, cycles: u16) {
        if self.io.timer.advance(cycles) {
            interrupt_flag_raise(self, Interrupt::Timer);
        }
        self.ppu.advance(cycles);
        // TODO: Pass PPU interrupts up
    }

    fn oam_dma_transfer(&mut self, page: u8) {
        let src: u16 = u16::from(page) << 8;
        for i in 0..0xA0u16 {
            let byte = self.read(src + i);
            self.write(0xFE00 + i, byte);
        }
    }
}
