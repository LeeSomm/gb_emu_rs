use bitfields::bitflag;

#[bitflag(u8)]
pub enum Palette {
    #[base]
    Black = 0x0,
    DarkGray = 0x1,
    LightGray = 0x2,
    White = 0x3,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum PaletteRegister {
    BGP = 0xFF47,  // Full color range
    OBP0 = 0xFF48, // Color index 0 is transparent
    OBP1 = 0xFF49, // Color index 0 is transparent
}
