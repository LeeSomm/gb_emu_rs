// Note: The CPU clock also serves as the internal hardware
// divider (u16).

pub struct Timer {
    timer: u16,   // The actual 16bit timer. div points at top 8 bits.
    pub div: u8,  // 0xFF04 Divider register
    pub tima: u8, // 0xFF05 Timer counter
    pub tma: u8,  // 0xFF06 Timer modulo
    pub tac: u8,  // 0xFF07 Timer control
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            timer: 0xABCC, // Initialization value for DMG/MGB
            div: 0xAB,     // Upper 8 bits of timer
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }
}

impl Timer {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF04 => {
                // Any write to div results in 0
                self.timer = 0;
                self.div = 0;
            }
            0xFF05 => self.tima = val,
            0xFF06 => self.tma = val,
            0xFF07 => self.tac = val,
            _ => unreachable!(),
        }
    }

    pub fn advance_timer(&mut self, cycles: u16) {
        let overflow: bool;
        (self.timer, overflow) = self.timer.overflowing_add(cycles);
        self.div = (self.timer >> 8) as u8;

        // TODO: Implement overflow logic
    }
}
