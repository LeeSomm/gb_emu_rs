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
            0xFF07 => self.tac = val & 0b111, // Only bottom 3 bits
            _ => unreachable!(),
        }
    }

    // Returns true if a timer interrupt is requested
    pub fn advance(&mut self, cycles: u16) -> bool {
        let ctrl_bit_mask: u16 = 1 << self.timer_control_bit();
        let ctrl_bit = self.timer & ctrl_bit_mask;

        self.timer = self.timer.wrapping_add(cycles);
        self.div = (self.timer >> 8) as u8;

        if self.is_timer_enabled() {
            if self.timer & ctrl_bit_mask < ctrl_bit {
                // Control bit overflow (1 -> 0) triggers TIMA increment
                let tima_overflow: bool;
                (self.tima, tima_overflow) = self.tima.overflowing_add(1);
                // println!("TIMA: {:?}", self.tima);
                if tima_overflow {
                    self.tima = self.tma;
                    return true;
                }
            }
        }
        false
    }

    fn is_timer_enabled(&self) -> bool {
        self.tac >> 2 == 1
    }

    fn timer_control_bit(&self) -> u8 {
        match self.tac & 0b11 {
            0x00 => 9, // 1024 clocks
            0x01 => 3, // 16 clocks
            0x10 => 5, // 64 clocks
            0x11 => 7, // 256 clocks
            _ => unreachable!(),
        }
    }
}
