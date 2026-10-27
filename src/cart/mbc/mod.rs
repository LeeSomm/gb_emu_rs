#[derive(Debug)]
pub struct Mbc {
    pub ramg: bool,
    pub bank1: u8,
    pub bank2: u8,
    pub mode: BankingMode,
}

#[derive(Default, Debug)]
pub enum BankingMode {
    #[default]
    Locked, // Mode 0b0
    Advanced, // Mode 0b1
}

impl Default for Mbc {
    fn default() -> Self {
        Self {
            ramg: bool::default(),
            bank1: 1, // Must be set to 1
            bank2: u8::default(),
            mode: BankingMode::default(),
        }
    }
}

impl Mbc {
    pub fn set_ramg(&mut self, val: u8) {
        if val == 0b1010 {
            self.ramg = true;
        } else {
            self.ramg = false;
        }
    }

    pub fn set_bank1(&mut self, val: u8) {
        let val = val & 0b00011111;
        if val == 0x0 {
            self.bank1 = 0x1;
        } else {
            self.bank1 = val;
        }
    }

    pub fn set_bank2(&mut self, val: u8) {
        self.bank2 = val & 0b00000011;
    }

    pub fn set_mode(&mut self, val: u8) {
        let val = val & 0b00000001;
        if val == 1 {
            self.mode = BankingMode::Advanced;
        } else {
            self.mode = BankingMode::Locked;
        }
    }
}
