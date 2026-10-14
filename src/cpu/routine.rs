use crate::cpu::Cpu;
use crate::memory::MemoryBus;

impl Cpu {
    // CPU Generic Operations
    pub fn routine_add_a_r8(&mut self, r8: u8) {
        todo!()
    }

    pub fn routine_adc_a_r8(&mut self, r8: u8) {
        todo!()
    }

    pub fn routine_cp_a_r8(&mut self, r8: u8) {
        todo!()
    }

    pub fn routine_dec_r16(&mut self, register: &str) {
        self.clock.advance_clock(4);
        match register {
            "bc" => {
                let bc = self.register.get_bc();
                self.register.set_bc(bc - 1);
            }
            "de" => {
                let de = self.register.get_de();
                self.register.set_de(de - 1);
            }
            "hl" => {
                let hl = self.register.get_hl();
                self.register.set_hl(hl - 1);
            }
            "sp" => self.register.sp -= 1,
            _ => panic!("Attempted to decrement invalid 16bit register."),
        };

        self.clock.advance_clock(4);
    }

    pub fn routine_ld_r8_r8(&mut self, dst: &str, src: &str) {
        self.clock.advance_clock(4);
        let src = match src {
            "a" => self.register.a,
            "b" => self.register.b,
            "c" => self.register.c,
            "d" => self.register.d,
            "e" => self.register.e,
            "h" => self.register.h,
            "l" => self.register.l,
            _ => panic!("Attempted to load from invalid 8bit register"),
        };

        match dst {
            "a" => self.register.a = src,
            "b" => self.register.b = src,
            "c" => self.register.c = src,
            "d" => self.register.d = src,
            "e" => self.register.e = src,
            "h" => self.register.h = src,
            "l" => self.register.l = src,
            _ => panic!("Attempted to load into invalid 8bit register"),
        };
    }

    pub fn routine_ld_r16_n16(&mut self, bus: &mut MemoryBus, register: &str) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let val = (hi << 8) | lo;
        match register {
            "bc" => self.register.set_bc(val),
            "de" => self.register.set_de(val),
            "hl" => self.register.set_hl(val),
            "sp" => self.register.sp = val,
            _ => panic!("Attempted to load to invalid 16bit register."),
        }
    }

    pub fn routine_jr_cc_e8(&mut self, cc: &str, e: i8) {
        self.clock.advance_clock(4);
        match cc {
            // Not Zero
            "nz" => {
                if self.register.get_flag_z_zero() == 0 {
                    self.clock.advance_clock(4);
                    self.register.pc = self.register.pc.wrapping_add_signed(e as i16);
                }
            }
            _ => panic!("Unknown condition for relative jump"),
        };
        self.clock.advance_clock(4);
    }

    pub fn routine_or_a_r8(&mut self, val: u8) {
        self.clock.advance_clock(4);
        self.register.a |= val;
        self.register.clear_flags();
        if self.register.a == 0 {
            self.register.set_flag_z_zero();
        }
    }

    pub fn routine_sbc_a_r8(&mut self, r8: u8) {
        todo!()
    }

    pub fn routine_sub_a_r8(&mut self, r8: u8) {
        todo!()
    }

    pub fn routine_xor_a_r8(&mut self, r8: u8) {
        todo!()
    }
}
