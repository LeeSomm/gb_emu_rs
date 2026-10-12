use crate::cart::Cartridge;

pub struct MemoryBus {
    cart: Cartridge,    // Addresses 0x0000-0x7FFF & 0xA000-0xBFFF
    vram: [u8; 0x2000], // Addresses 0x8000-0x9FFF
    wram: [u8; 0x2000], // Addresses 0xC000-DFFF & 0xE000-FDFF (via Echo RAM)
    oam: [u8; 0xA0],    // Addresses 0xFE00-0xFE9F
    io: [u8; 0x80],     // Address 0xFF00-0xFF7F (Consider a dedicated struct)
    hram: [u8; 0x7F],   // Address 0xFF80-FFFE
    ie: u8,             // Address 0xFFFF
}

impl MemoryBus {
    pub fn init(cart: Cartridge) -> Self {
        MemoryBus {
            cart,
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0..=0x3FFF => self.cart.data[addr as usize],
            0x4000..=0x7FFF => {
                if self.cart.header.cartridge_type == 0x0 {
                    return self.cart.data[addr as usize];
                } else if self.cart.header.cartridge_type == 0x3 {
                    // TODO: Implement read from switchable bank
                    return 1;
                } else {
                    panic!("Unsupported cartridge type")
                }
            }
            0x8000..=0x9FFF => todo!("Implement read for: {:#x?}", addr),
            0xA000..=0xBFFF => todo!("Implement read for: {:#x?}", addr),
            0xC000..=0xCFFF => todo!("Implement read for: {:#x?}", addr),
            0xD000..=0xDFFF => todo!("Implement read for: {:#x?}", addr),
            0xE000..=0xFDFF => todo!("Implement read for: {:#x?}", addr),
            0xFE00..=0xFE9F => todo!("Implement read for: {:#x?}", addr),
            0xFEA0..=0xFEFF => todo!("Implement read for: {:#x?}", addr),
            0xFF00..=0xFF7F => todo!("Implement read for: {:#x?}", addr),
            0xFF80..=0xFFFE => todo!("Implement read for: {:#x?}", addr),
            0xFFFF => self.ie,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0..=0x1FFF => self.cart.mbc.set_ramg(val),
            0x2000..=0x3FFF => self.cart.mbc.set_bank1(val),
            0x4000..=0x5FFF => self.cart.mbc.set_bank2(val),
            0x6000..=0x7FFF => self.cart.mbc.set_mode(val),
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000] = val,
            0xA000..=0xBFFF => todo!("Implement writes for Cartridge SRAM; addr: {:#x?}", addr),
            0xC000..=0xCFFF => self.wram[addr as usize - 0xC000] = val,
            0xD000..=0xDFFF => todo!("Implement write logic for addr: {:#x?}", addr),
            0xE000..=0xFDFF => self.wram[addr as usize - 0xE000] = val, // Echo RAM writes are effectively 0xC000-0xDDFF
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00] = val,
            0xFEA0..=0xFEFF => panic!("Writes to address block 0xFEA0-0xFEFF are forbidden"),
            0xFF00..=0xFF7F => self.io[addr as usize - 0xFF00] = val,
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80] = val,
            0xFFFF => self.ie = val,
        }
    }
}
