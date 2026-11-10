use bitfields::{bitfield, bitflag};

#[derive(Default)]
pub struct Lcd {
    // Regsiters
    pub lcdc: LcdControl, // 0xFF40 LCD control
    pub stat: LcdStatus,  // 0xFF41 LCD status
    pub scy: u8,          // 0xFF42 Background viewport Y
    pub scx: u8,          // 0xFF43 Background viewport X
    pub ly: u8,           // 0xFF44 LCD Y coordinate (read-only), values from 0-153
    pub lyc: u8,          // 0xFF45 LY compare
    pub od_trans: u8,     // 0xFF46 OAM DMA transfer
    pub bgp: u8,          // 0xFF47 BG palette data
    pub obp0: u8,         // 0xFF48 OBJ palette 0 data
    pub obp1: u8,         // 0xFF49 OBJ palette 1 data
    pub wy: u8,           // 0xFF4A Window Y position
    pub wx: u8,           // 0xFF4B Window X position plus 7
    // Processing state
    pub dots: u16,                   // 0-455, overflow moves to next scanline
    pub output_x: u8,                // X value of current LCD output (0-159)
    pub window_y_condition: bool,    // Track whether a window has begun rendering
    pub drew_window_this_line: bool, // Tracks whether a window pixel was drawn this line
    pub window_line: u8,             // Tracks which line of window is being rendered
    // Pixel FIFO state
    pub drawing_complete: bool, // Signals end of PpuMode 3
}

impl Lcd {
    pub fn init() -> Self {
        Self {
            lcdc: LcdControl::from_bits(0x91),
            stat: LcdStatus::from_bits(0x85),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            od_trans: 0xFF,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            dots: 0,
            output_x: 0,
            window_y_condition: false,
            drew_window_this_line: false,
            window_line: 0,
            drawing_complete: false,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.lcdc.into_bits(),
            0xFF41 => self.stat.into_bits(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => 0x90, // TODO: Modify once PPU is implemented
            0xFF45 => self.lyc,
            0xFF46 => self.od_trans,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => panic!("Attempted to access invalid audio register: {:#x?}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF40 => {
                if (val >> 7) == 0 && self.lcdc.lcd_ppu_enable() {
                    self.lcd_disable_clear_state();
                }
                self.lcdc.write_bits(val);
            }
            0xFF41 => self.stat.write_bits(val),
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => panic!("Attempted to write to read-only LCD LY register"),
            0xFF45 => self.lyc = val,
            0xFF46 => unreachable!("Writes to 0xFF46 handled at MemoryBus level."),
            0xFF47 => self.bgp = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val,
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,
            _ => panic!("Attempted to access invalid audio register: {:#x?}", addr),
        }
    }

    fn lcd_disable_clear_state(&mut self) {
        self.dots = 0;
        self.ly = 0;
        self.stat.set_ppu_mode(PpuMode::HBlank);
        self.output_x = 0;
        self.window_y_condition = false;
        self.drew_window_this_line = false;
        self.drawing_complete = false;
    }

    /// Increment to next scanline. If final scanline, return true for completed frame.
    pub fn next_scanline(&mut self) -> bool {
        let ly = self.ly + 1;
        let frame_complete: bool;

        if ly > 153 {
            self.ly = 0;
            self.window_line = 0;
            frame_complete = true;
        } else {
            self.ly = ly;
            if self.drew_window_this_line {
                self.window_line += 1;
            }
            frame_complete = false;
        }

        self.output_x = 0;
        self.drew_window_this_line = false;

        if self.ly == self.wy {
            self.window_y_condition = true; // Set false each frame during VBlank
        }

        frame_complete
    }
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

// FF41
#[bitfield(u8)]
pub struct LcdStatus {
    #[bits(2)]
    ppu_mode: PpuMode, // PPU's current status
    lyc_eq_ly: bool,         // Set when LYC == LY
    m_zero_int_select: bool, // If set, selects Mode 0 condition for STAT interrupt
    m_one_int_select: bool,  // If set, selects Mode 1 condition for STAT interrupt
    m_two_int_select: bool,  // If set, selects Mode 2 condition for STAT interrupt
    lyc_int_select: bool,    // If set, select LYC == LY condition for STAT interrupt
    __: bool,                // Unused
}

#[bitflag(u8)]
pub enum TileMapRef {
    #[base]
    TileMap1 = 0,
    TileMap2 = 1,
}

impl TileMapRef {
    pub fn vram_offset(self) -> usize {
        match self {
            TileMapRef::TileMap1 => 0x1800,
            TileMapRef::TileMap2 => 0x1C00,
        }
    }
}

#[bitflag(u8)]
pub enum PpuMode {
    #[base]
    HBlank = 0x0,
    VBlank = 0x1,
    OAMSearch = 0x2,
    Drawing = 0x3,
}
