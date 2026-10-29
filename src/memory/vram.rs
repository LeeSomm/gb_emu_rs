use crate::ppu::{Tile, TileMap};

pub struct VRam {
    pub data: [Tile; 384], // Addresses 0x8000-0x97FF
    pub map1: TileMap,     // Addresses 0x9800-9BFF
    pub map2: TileMap,     // Addresses 0x9C00-9FFF
}
