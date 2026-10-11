use crate::cart::Cartridge;

pub struct MemoryBus {
    cart: Cartridge,    // Addresses 0x0000-0x7FFF & 0xA000-0xBFFF
    vram: [u8; 0x2000], // Addresses 0x8000-0x9FFF
    wram: [u8; 0x2000], // Addresses 0xC000-DFFF & 0xE000-FDFF (via Echo RAM)
    oam: [u8; 0xA0],    // Addresses 0xFE00-0xFE9F
    io: [u8; 0x80],     // Address 0xFF00-0xFF7F (Consider a dedicated struct)
    hram: [u8; 0x7F],   // Address 0xFF80-FFFE
    ie: u8,
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
            0x8000..=0x9FFF => 1,
            0xA000..=0xBFFF => 1,
            0xC000..=0xCFFF => 1,
            0xD000..=0xDFFF => 1,
            0xE000..=0xFDFF => 1,
            0xFE00..=0xFE9F => 1,
            0xFEA0..=0xFEFF => 1,
            0xFF00..=0xFF7F => 1,
            0xFF80..=0xFFFE => 1,
            0xFFFF => 1,
        }
    }
}
