// Note: The CPU clock also serves as the internal hardware
// divider (u16).

pub struct Timer {
    pub div: u8,  // 0xFF04 Divider register
    pub tima: u8, // 0xFF05 Timer counter
    pub tma: u8,  // 0xFF06 Timer modulo
    pub tac: u8,  // 0xFF07 Timer control
}

impl Timer {}
