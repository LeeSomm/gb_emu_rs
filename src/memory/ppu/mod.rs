use bitfields::{bitfield, bitflag};

#[derive(Default)]
pub struct Ppu {
    timer: u16,
}

impl Ppu {
    pub fn advance(&mut self, cycles: u16) {
        self.timer = self.timer.wrapping_add(cycles);
    }
}

pub struct Tile {
    // Make sure to use from_le_bytes
    pub data: [u8; 16], // 8x8 pixel data, 2bits per pixel
}

pub struct TileMap {
    pub data: [Tile; 1024], // Tile maps are 32x32 tiles (256 x 256 pixels)
}

pub struct ObjectAttribute {
    pub y: u8,
    pub x: u8,
    pub tile_idx: u8,
    pub flags: u8,
}

//FF40
#[bitfield(u8)]
pub struct LcdControl {
    bg_window_enable: bool, // False = Off, True = On
    obj_enable: bool,       // False = Off, True = On
    obj_size: bool,         // 0 = 8x8, 1 = 8x16
    #[bits(1)]
    bg_tilemap: TileMapRef,
    bg_window_tiles: bool, // 0 = 0x8800-97FF, 1 = 0x8000-8FFF (tile data in VRAM)
    window_enable: bool,   // False = Off, True = On (overridden by BG_WINDOW_ENABLE)
    #[bits(1)]
    window_tilemap: TileMapRef,
    lcd_ppu_enable: bool, // False = Off, True = On
}

#[bitflag(u8)]
pub enum TileMapRef {
    #[base]
    TileMap1 = 0,
    TileMap2 = 1,
}

// FF41
#[bitfield(u8)]
pub struct LcdStatus {
    #[bits(2)]
    ppu_mode: PPUMode, // PPU's current status
    lyc_eq_ly: bool,         // Set when LYC == LY
    m_zero_int_select: bool, // If set, selects Mode 0 condition for STAT interrupt
    m_one_int_select: bool,  // If set, selects Mode 1 condition for STAT interrupt
    m_two_int_select: bool,  // If set, selects Mode 2 condition for STAT interrupt
    lyc_int_select: bool,    // If set, select LYC == LY condition for STAT interrupt
    __: bool,                // Unused
}

#[bitflag(u8)]
pub enum PPUMode {
    #[base]
    HBlank = 0x0,
    VBlank = 0x1,
    OAMSearch = 0x2,
    Drawing = 0x3,
}
