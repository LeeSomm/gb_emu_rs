use crate::cpu::Cpu;
use crate::cpu::register::{FlagCondition, Register8, Register16};
use crate::memory::MemoryBus;

impl Cpu {
    // CPU Generic Operations
    pub fn routine_adc_a_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.advance_timer(4);
        let val = self.register.read_r8(reg);
        let c = self.register.get_flag_c_carry();
        let half_carry = (self.register.a & 0x0F) + (val & 0x0F) + c > 0x0F;
        let (tmp_result, carry1) = self.register.a.overflowing_add(val);
        let (result, carry2) = tmp_result.overflowing_add(c);

        if result == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }

        if carry1 || carry2 {
            self.register.set_flag_c_carry();
        } else {
            self.register.clear_flag_c_carry();
        }

        if half_carry {
            self.register.set_flag_h_half_carry();
        } else {
            self.register.clear_flag_h_half_carry();
        }

        self.register.clear_flag_n_subtraction();

        self.register.a = result;
    }

    pub fn routine_add_a_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.advance_timer(4);
        let val = self.register.read_r8(reg);
        let half_carry = (self.register.a & 0x0F) + (val & 0x0F) > 0x0F;
        let (result, carry) = self.register.a.overflowing_add(val);

        if result == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }

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

        self.register.clear_flag_n_subtraction();

        self.register.a = result;
    }

    pub fn routine_add_hl_r16(&mut self, bus: &mut MemoryBus, reg: Register16) {
        bus.advance_timer(4);
        let val = self.register.get_hl();
        let reg_val = self.register.read_r16(reg);
        let half_carry = (val & 0x0FFF) + (reg_val & 0x0FFF) > 0x0FFF;
        let (val, carry) = val.overflowing_add(reg_val);

        bus.advance_timer(4);
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

    pub fn routine_and_a_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.advance_timer(4);
        let result = self.register.a & self.register.read_r8(reg);
        self.register.a = result;
        self.register.clear_flags();
        if result == 0 {
            self.register.set_flag_z_zero();
        }
        self.register.set_flag_h_half_carry();
    }

    pub fn routine_call_cc_a16(&mut self, bus: &mut MemoryBus, cc: FlagCondition) {
        fn call_cc_match(cpu: &mut Cpu, bus: &mut MemoryBus, hi: u8, lo: u8) {
            bus.advance_timer(4);
            cpu.register.sp = cpu.register.sp.wrapping_sub(1);
            bus.write(cpu.register.sp, hi);

            bus.advance_timer(4);
            cpu.register.sp = cpu.register.sp.wrapping_sub(1);
            bus.write(cpu.register.sp, lo);

            bus.advance_timer(4);
            cpu.register.pc = ((hi as u16) << 8) | lo as u16;
        }

        bus.advance_timer(4);
        let lo = bus.read(self.register.pc);
        self.inc_pc();

        bus.advance_timer(4);
        let hi = bus.read(self.register.pc);
        self.inc_pc();

        bus.advance_timer(4);
        match cc {
            FlagCondition::C => {
                if self.register.get_flag_c_carry() == 1 {
                    call_cc_match(self, bus, hi, lo);
                }
            }
            FlagCondition::NC => {
                if self.register.get_flag_c_carry() == 0 {
                    call_cc_match(self, bus, hi, lo);
                }
            }
            FlagCondition::Z => {
                if self.register.get_flag_z_zero() == 1 {
                    call_cc_match(self, bus, hi, lo);
                }
            }
            FlagCondition::NZ => {
                if self.register.get_flag_z_zero() == 0 {
                    call_cc_match(self, bus, hi, lo);
                }
            }
        }
    }

    pub fn routine_cp_a_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.advance_timer(4);
        let val = self.register.read_r8(reg);
        let half_carry = (self.register.a & 0x0F) < val & 0x0F;
        let (result, carry) = self.register.a.overflowing_sub(val);

        if result == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }

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

        self.register.set_flag_n_subtraction();
    }

    pub fn routine_dec_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.advance_timer(4);
        let mut val = self.register.read_r8(reg);

        // Check half-carry flag
        if (val & 0x0F) == 0 {
            self.register.set_flag_h_half_carry();
        } else {
            self.register.clear_flag_h_half_carry();
        }
        self.register.set_flag_n_subtraction();
        val = val.wrapping_sub(1);
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }

        self.register.write_r8(reg, val);
    }

    pub fn routine_dec_r16(&mut self, bus: &mut MemoryBus, reg: Register16) {
        bus.advance_timer(4);
        let val = self.register.read_r16(reg).wrapping_sub(1);

        bus.advance_timer(4);
        self.register.write_r16(reg, val);
    }

    pub fn routine_inc_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.advance_timer(4);
        let mut val = self.register.read_r8(reg);

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

    pub fn routine_inc_r16(&mut self, bus: &mut MemoryBus, reg: Register16) {
        bus.advance_timer(4);
        let val = self.register.read_r16(reg).wrapping_add(1);

        bus.advance_timer(4);
        self.register.write_r16(reg, val);
    }

    pub fn routine_jp_cc_a16(&mut self, bus: &mut MemoryBus, cc: FlagCondition) {
        bus.advance_timer(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();

        bus.advance_timer(4);
        let hi = bus.read(self.register.pc) as u16;
        self.inc_pc();
        let addr = (hi << 8) | lo;

        match cc {
            // Carry
            FlagCondition::C => {
                if self.register.get_flag_c_carry() == 1 {
                    bus.advance_timer(4);
                    self.register.pc = addr;
                }
            }
            // Not Carry
            FlagCondition::NC => {
                if self.register.get_flag_c_carry() == 0 {
                    bus.advance_timer(4);
                    self.register.pc = addr;
                }
            }
            // Zero
            FlagCondition::Z => {
                if self.register.get_flag_z_zero() == 1 {
                    bus.advance_timer(4);
                    self.register.pc = addr;
                }
            }
            // Not Zero
            FlagCondition::NZ => {
                if self.register.get_flag_z_zero() == 0 {
                    bus.advance_timer(4);
                    self.register.pc = addr;
                }
            }
        };
        bus.advance_timer(4);
    }

    pub fn routine_jr_cc_e8(&mut self, bus: &mut MemoryBus, cc: FlagCondition, e: i8) {
        bus.advance_timer(4);
        match cc {
            // Carry
            FlagCondition::C => {
                if self.register.get_flag_c_carry() == 1 {
                    bus.advance_timer(4);
                    self.register.pc = self.register.pc.wrapping_add_signed(e as i16);
                }
            }
            // Not Carry
            FlagCondition::NC => {
                if self.register.get_flag_c_carry() == 0 {
                    bus.advance_timer(4);
                    self.register.pc = self.register.pc.wrapping_add_signed(e as i16);
                }
            }
            // Zero
            FlagCondition::Z => {
                if self.register.get_flag_z_zero() == 1 {
                    bus.advance_timer(4);
                    self.register.pc = self.register.pc.wrapping_add_signed(e as i16);
                }
            }
            // Not Zero
            FlagCondition::NZ => {
                if self.register.get_flag_z_zero() == 0 {
                    bus.advance_timer(4);
                    self.register.pc = self.register.pc.wrapping_add_signed(e as i16);
                }
            }
        };
        bus.advance_timer(4);
    }

    pub fn routine_ld_a16_indirect_r8(&mut self, bus: &mut MemoryBus, addr: u16, src: Register8) {
        bus.advance_timer(4);
        let val = self.register.read_r8(src);

        bus.advance_timer(4);
        bus.write(addr, val);
    }

    pub fn routine_ld_r8_r8(&mut self, bus: &mut MemoryBus, dst: Register8, src: Register8) {
        bus.advance_timer(4);
        let val = self.register.read_r8(src);
        self.register.write_r8(dst, val);
    }

    pub fn routine_ld_r8_r16_indirect(
        &mut self,
        bus: &mut MemoryBus,
        dst: Register8,
        src: Register16,
    ) {
        bus.advance_timer(4);
        let val = bus.read(self.register.read_r16(src));

        bus.advance_timer(4);
        self.register.write_r8(dst, val);
    }

    pub fn routine_ld_r8_n8(&mut self, bus: &mut MemoryBus, dst: Register8) {
        bus.advance_timer(4);
        self.register.write_r8(dst, bus.read(self.register.pc));

        bus.advance_timer(4);
        self.inc_pc();
    }

    pub fn routine_ld_r16_n16(&mut self, bus: &mut MemoryBus, reg: Register16) {
        bus.advance_timer(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();

        bus.advance_timer(4);
        let hi = bus.read(self.register.pc) as u16;
        self.inc_pc();

        bus.advance_timer(4);
        let val = (hi << 8) | lo;
        self.register.write_r16(reg, val);
    }

    pub fn routine_or_a_r8(&mut self, bus: &mut MemoryBus, val: u8) {
        bus.advance_timer(4);
        self.register.a |= val;
        self.register.clear_flags();
        if self.register.a == 0 {
            self.register.set_flag_z_zero();
        }
    }

    pub fn routine_pop_r16(&mut self, bus: &mut MemoryBus, reg: Register16) {
        bus.advance_timer(4);
        let lo = bus.read(self.register.sp) as u16;
        self.register.sp = self.register.sp.wrapping_add(1);

        bus.advance_timer(4);
        let hi = bus.read(self.register.sp) as u16;
        self.register.sp = self.register.sp.wrapping_add(1);

        bus.advance_timer(4);
        let val = (hi << 8) | lo;
        self.register.write_r16(reg, val);
    }

    pub fn routine_push_r16(&mut self, bus: &mut MemoryBus, reg: Register16) {
        let hi: u8;
        let lo: u8;
        match reg {
            Register16::AF => {
                hi = self.register.a;
                lo = self.register.f;
            }
            Register16::BC => {
                hi = self.register.b;
                lo = self.register.c;
            }
            Register16::DE => {
                hi = self.register.d;
                lo = self.register.e;
            }
            Register16::HL => {
                hi = self.register.h;
                lo = self.register.l;
            }
            Register16::SP => panic!("Attempted to push from invalid 16bit register."),
        }
        bus.advance_timer(4);
        self.register.sp = self.register.sp.wrapping_sub(1);

        bus.advance_timer(4);
        bus.write(self.register.sp, hi);

        bus.advance_timer(4);
        self.register.sp = self.register.sp.wrapping_sub(1);

        bus.advance_timer(4);
        bus.write(self.register.sp, lo);
    }

    pub fn routine_rst_n(&mut self, bus: &mut MemoryBus, n8: u8) {
        bus.advance_timer(4);
        let [hi, lo] = self.register.pc.to_be_bytes();

        bus.advance_timer(4);
        self.register.sp = self.register.sp.wrapping_sub(1);
        bus.write(self.register.sp, hi);

        bus.advance_timer(4);
        self.register.sp = self.register.sp.wrapping_sub(1);
        bus.write(self.register.sp, lo);

        bus.advance_timer(4);
        self.register.pc = n8 as u16;
    }

    pub fn routine_ret_cc(&mut self, bus: &mut MemoryBus, cc: FlagCondition) {
        fn ret_cc_match(cpu: &mut Cpu, bus: &mut MemoryBus) {
            bus.advance_timer(4);
            let lo = bus.read(cpu.register.sp) as u16;
            cpu.register.sp = cpu.register.sp.wrapping_add(1);

            bus.advance_timer(4);
            let hi = bus.read(cpu.register.sp) as u16;
            cpu.register.sp = cpu.register.sp.wrapping_add(1);

            bus.advance_timer(4);
            let addr = (hi << 8) | lo;
            cpu.register.pc = addr;
        }

        bus.advance_timer(4);
        match cc {
            FlagCondition::C => {
                if self.register.get_flag_c_carry() == 1 {
                    ret_cc_match(self, bus);
                }
            }
            FlagCondition::NC => {
                if self.register.get_flag_c_carry() == 0 {
                    ret_cc_match(self, bus);
                }
            }
            FlagCondition::Z => {
                if self.register.get_flag_z_zero() == 1 {
                    ret_cc_match(self, bus);
                }
            }
            FlagCondition::NZ => {
                if self.register.get_flag_z_zero() == 0 {
                    ret_cc_match(self, bus);
                }
            }
        }
        bus.advance_timer(4);
    }

    pub fn routine_sub_a_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.advance_timer(4);
        let val = self.register.read_r8(reg);
        let half_carry = (self.register.a & 0x0F) < val & 0x0F;
        let (result, carry) = self.register.a.overflowing_sub(val);
        self.register.a = result;

        if result == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }

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

        self.register.set_flag_n_subtraction();

        self.register.a = result;
    }

    pub fn routine_sbc_a_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.advance_timer(4);
        let val = self.register.read_r8(reg);
        let c = self.register.get_flag_c_carry();
        let half_carry = (self.register.a & 0x0F) < ((val & 0x0F) + c);
        let (tmp_result, carry1) = self.register.a.overflowing_sub(val);
        let (result, carry2) = tmp_result.overflowing_sub(c);

        if result == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }

        if carry1 || carry2 {
            self.register.set_flag_c_carry();
        } else {
            self.register.clear_flag_c_carry();
        }

        if half_carry {
            self.register.set_flag_h_half_carry();
        } else {
            self.register.clear_flag_h_half_carry();
        }

        self.register.set_flag_n_subtraction();

        self.register.a = result;
    }

    pub fn routine_xor_a_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.advance_timer(4);
        self.register.a = self.register.a ^ self.register.read_r8(reg);
        self.register.clear_flags();
        if self.register.a == 0 {
            self.register.set_flag_z_zero();
        }
    }
}
