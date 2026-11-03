const TILE_LENGTH: usize = 16;

pub struct VRam {
    data: [u8; 0x2000], // Addresses 0x8000-9FFF
                        // pub tiles: [Tile; 384], // Addresses 0x8000-0x97FF
                        // pub map1: TileMap,      // Addresses 0x9800-9BFF
                        // pub map2: TileMap,      // Addresses 0x9C00-9FFF
}

impl VRam {
    pub fn init() -> Self {
        Self { data: [0; 0x2000] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize - 0x8000]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.data[addr as usize - 0x8000] = val;
    }

    /// Returns the 16 bytes of a tile, given an index from a tile map.
    pub fn tile(&self, index: u8, unsigned_mode: bool) -> &[u8; 16] {
        let base = if unsigned_mode {
            index as usize * 16
        } else {
            (0x1000 + (index as i8 as isize * 16)) as usize
        };
        self.data[base..base + 16].try_into().unwrap()
    }

    /// Returns the tile index at (x, y) in one of the two 32x32 maps.
    pub fn tile_index(&self, map_offset: usize, x: usize, y: usize) -> u8 {
        self.data[map_offset + y * 32 + x]
    }
}

pub struct Tile {
    // Make sure to use from_le_bytes
    pub data: [u8; 16], // 8x8 pixel data, 2bits per pixel
}

pub struct TileMap {
    pub data: [u8; 1024], // Tile maps are 32x32 bytes where each byte is a tile index
}
