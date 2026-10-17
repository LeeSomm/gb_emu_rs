use crate::cpu::Cpu;
use crate::cpu::register::{Register8, Register16};
use crate::memory::MemoryBus;

impl Cpu {
    // CPU Generic Operations
    pub fn routine_add_a_r8(&mut self, r8: u8) {
        todo!()
    }

    pub fn routine_add_hl_r16(&mut self, reg: Register16) {
        self.clock.advance_clock(4);
        let val = self.register.get_hl();
        let reg_val = self.register.read_r16(reg);
        let half_carry = (val & 0x0FFF) + (reg_val & 0x0FFF) > 0x0FFF;
        let (val, carry) = val.overflowing_add(reg_val);

        self.clock.advance_clock(4);
        self.register.set_hl(val);
        self.register.clear_flag_n_subtraction();
        if carry {
            self.register.set_flag_c_carry();
        } else {
            self.register.clear_flag_c_carry();
        }

        if half_carry {
            self.register.set_flag_h_half_carry();
        } else {
            self.register.clear_flag_h_half_carry();
        }
    }

    pub fn routine_adc_a_r8(&mut self, r8: u8) {
        todo!()
    }

    pub fn routine_cp_a_r8(&mut self, r8: u8) {
        todo!()
    }

    pub fn routine_dec_r16(&mut self, reg: Register16) {
        self.clock.advance_clock(4);
        let val = self.register.read_r16(reg.clone()).wrapping_sub(1);

        self.clock.advance_clock(4);
        self.register.write_r16(reg, val);
    }

    pub fn routine_inc_r8(&mut self, reg: Register8) {
        self.clock.advance_clock(4);
        let mut val = self.register.read_r8(reg.clone());

        // If bottom 4 bits are set, inc will cause overflow from bit 3.
        if (val & 0x0F) == 0x0F {
            self.register.set_flag_h_half_carry();
        } else {
            self.register.clear_flag_h_half_carry();
        }
        self.register.clear_flag_n_subtraction();
        val = val.wrapping_add(1);
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }

        self.register.write_r8(reg, val);
    }

    pub fn routine_inc_r16(&mut self, reg: Register16) {
        self.clock.advance_clock(4);
        let val = self.register.read_r16(reg.clone()).wrapping_add(1);

        self.clock.advance_clock(4);
        self.register.write_r16(reg, val);
    }

    pub fn routine_ld_a16_r8(&mut self, bus: &mut MemoryBus, addr: u16, src: Register8) {
        self.clock.advance_clock(4);
        let val = self.register.read_r8(src);

        self.clock.advance_clock(4);
        bus.write(addr, val);
    }

    pub fn routine_ld_r8_r8(&mut self, dst: Register8, src: Register8) {
        self.clock.advance_clock(4);
        let val = self.register.read_r8(src);
        self.register.write_r8(dst, val);
    }

    pub fn routine_ld_r8_n8(&mut self, bus: &MemoryBus, dst: Register8) {
        self.clock.advance_clock(4);
        self.register.write_r8(dst, bus.read(self.register.pc));

        self.clock.advance_clock(4);
        self.inc_pc();
    }

    pub fn routine_ld_r16_n16(&mut self, bus: &mut MemoryBus, reg: Register16) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let val = (hi << 8) | lo;
        self.register.write_r16(reg, val);
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
