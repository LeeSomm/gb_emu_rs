pub struct Oam {
    data: [u8; 0xA0], // Addresses 0xFE00-0xFE9F
}

impl Oam {
    pub fn init() -> Self {
        Oam { data: [0; 0xA0] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize - 0xFE00]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.data[addr as usize - 0xFE00] = val;
    }

    /// Returns the 4 bytes of an object attribute, given an index
    pub fn object_attribute(&self, idx: usize) -> &[u8; 4] {
        let base = idx * 4;
        self.data[base..base + 4].try_into().unwrap()
    }
}

pub struct ObjectAttribute {
    pub y: u8,
    pub x: u8,
    pub tile_idx: u8,
    pub flags: u8,
}
