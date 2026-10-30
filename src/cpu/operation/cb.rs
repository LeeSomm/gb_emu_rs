use crate::cpu::Cpu;
use crate::cpu::register::Register8;
use crate::memory::MemoryBus;

#[repr(u8)]
enum Bit {
    B0 = 0,
    B1 = 1,
    B2 = 2,
    B3 = 3,
    B4 = 4,
    B5 = 5,
    B6 = 6,
    B7 = 7,
}

impl Cpu {
    // Routines
    fn rlc_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.tick(4);
        let reg_val = self.register.read_r8(reg);
        let bit7 = (reg_val & 0b10000000) >> 7;
        let val = (reg_val << 1) + bit7;

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }

        self.register.write_r8(reg, val);
    }

    fn rrc_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.tick(4);
        let reg_val = self.register.read_r8(reg);
        let bit0 = reg_val & 0b00000001;
        let val = (reg_val >> 1) + (bit0 << 7);

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }

        self.register.write_r8(reg, val);
    }

    fn rl_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.tick(4);
        let reg_val = self.register.read_r8(reg);
        let bit7 = (reg_val & 0b10000000) >> 7;
        let val = (reg_val << 1) + self.register.get_flag_c_carry();

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }

        self.register.write_r8(reg, val);
    }

    fn rr_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.tick(4);
        let reg_val = self.register.read_r8(reg);
        let bit0 = reg_val & 0b00000001;
        let val = (reg_val >> 1) + (self.register.get_flag_c_carry() << 7);

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }

        self.register.write_r8(reg, val);
    }

    fn sla_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.tick(4);
        let reg_val = self.register.read_r8(reg);
        let bit7 = (reg_val & 0b10000000) >> 7;
        let val = reg_val << 1;

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }

        self.register.write_r8(reg, val);
    }

    fn sra_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.tick(4);
        let reg_val = self.register.read_r8(reg);
        let bit0 = reg_val & 0b00000001;
        let bit7 = reg_val & 0b10000000;
        let val = (reg_val >> 1) + bit7;

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }

        self.register.write_r8(reg, val);
    }

    fn swap_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.tick(4);
        let reg_val = self.register.read_r8(reg);
        let hi = reg_val << 4;
        let lo = reg_val >> 4;
        let val = hi | lo;

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }

        self.register.write_r8(reg, val);
    }

    fn srl_r8(&mut self, bus: &mut MemoryBus, reg: Register8) {
        bus.tick(4);
        let reg_val = self.register.read_r8(reg);
        let bit0 = reg_val & 0b00000001;
        let val = reg_val >> 1;

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }

        self.register.write_r8(reg, val);
    }

    fn bit_b3_r8(&mut self, bus: &mut MemoryBus, bit: Bit, reg: Register8) {
        bus.tick(4);
        let bit = bit as u8;
        let mask = 0b00000001 << bit;
        let val = self.register.read_r8(reg) & mask;

        bus.tick(4);
        self.register.clear_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
    }

    fn res_b3_r8(&mut self, bus: &mut MemoryBus, bit: Bit, reg: Register8) {
        bus.tick(4);
        let bit = bit as u8;
        let mask = !(0b00000001 << bit); // Bitshift 1 to the desired bit and then invert
        let val = self.register.read_r8(reg) & mask;

        bus.tick(4);
        self.register.write_r8(reg, val);
    }

    fn set_b3_r8(&mut self, bus: &mut MemoryBus, bit: Bit, reg: Register8) {
        bus.tick(4);
        let bit = bit as u8;
        let mask = 0b00000001 << bit;
        let val = self.register.read_r8(reg) | mask;

        bus.tick(4);
        self.register.write_r8(reg, val);
    }

    // RLC r8
    pub fn cb_rlc_a(&mut self, bus: &mut MemoryBus) {
        self.rlc_r8(bus, Register8::A);
    }

    pub fn cb_rlc_b(&mut self, bus: &mut MemoryBus) {
        self.rlc_r8(bus, Register8::B);
    }

    pub fn cb_rlc_c(&mut self, bus: &mut MemoryBus) {
        self.rlc_r8(bus, Register8::C);
    }

    pub fn cb_rlc_d(&mut self, bus: &mut MemoryBus) {
        self.rlc_r8(bus, Register8::D);
    }

    pub fn cb_rlc_e(&mut self, bus: &mut MemoryBus) {
        self.rlc_r8(bus, Register8::E);
    }

    pub fn cb_rlc_h(&mut self, bus: &mut MemoryBus) {
        self.rlc_r8(bus, Register8::H);
    }

    pub fn cb_rlc_l(&mut self, bus: &mut MemoryBus) {
        self.rlc_r8(bus, Register8::L);
    }

    pub fn cb_rlc_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        self.register.clear_flags();
        let bit7 = (val & 0b10000000) >> 7;
        let val = (val << 1) + bit7;

        bus.tick(4);
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    // RRC r8
    pub fn cb_rrc_a(&mut self, bus: &mut MemoryBus) {
        self.rrc_r8(bus, Register8::A);
    }

    pub fn cb_rrc_b(&mut self, bus: &mut MemoryBus) {
        self.rrc_r8(bus, Register8::B);
    }

    pub fn cb_rrc_c(&mut self, bus: &mut MemoryBus) {
        self.rrc_r8(bus, Register8::C);
    }

    pub fn cb_rrc_d(&mut self, bus: &mut MemoryBus) {
        self.rrc_r8(bus, Register8::D);
    }

    pub fn cb_rrc_e(&mut self, bus: &mut MemoryBus) {
        self.rrc_r8(bus, Register8::E);
    }

    pub fn cb_rrc_h(&mut self, bus: &mut MemoryBus) {
        self.rrc_r8(bus, Register8::H);
    }

    pub fn cb_rrc_l(&mut self, bus: &mut MemoryBus) {
        self.rrc_r8(bus, Register8::L);
    }

    pub fn cb_rrc_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        self.register.clear_flags();
        let bit0 = val & 0b00000001;
        let val = (val >> 1) + (bit0 << 7);

        bus.tick(4);
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    // RL r8
    pub fn cb_rl_a(&mut self, bus: &mut MemoryBus) {
        self.rl_r8(bus, Register8::A);
    }

    pub fn cb_rl_b(&mut self, bus: &mut MemoryBus) {
        self.rl_r8(bus, Register8::B);
    }

    pub fn cb_rl_c(&mut self, bus: &mut MemoryBus) {
        self.rl_r8(bus, Register8::C);
    }

    pub fn cb_rl_d(&mut self, bus: &mut MemoryBus) {
        self.rl_r8(bus, Register8::D);
    }

    pub fn cb_rl_e(&mut self, bus: &mut MemoryBus) {
        self.rl_r8(bus, Register8::E);
    }

    pub fn cb_rl_h(&mut self, bus: &mut MemoryBus) {
        self.rl_r8(bus, Register8::H);
    }

    pub fn cb_rl_l(&mut self, bus: &mut MemoryBus) {
        self.rl_r8(bus, Register8::L);
    }

    pub fn cb_rl_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit7 = (val & 0b10000000) >> 7;
        let val = (val << 1) + self.register.get_flag_c_carry();

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    // RR r8
    pub fn cb_rr_a(&mut self, bus: &mut MemoryBus) {
        self.rr_r8(bus, Register8::A);
    }

    pub fn cb_rr_b(&mut self, bus: &mut MemoryBus) {
        self.rr_r8(bus, Register8::B);
    }

    pub fn cb_rr_c(&mut self, bus: &mut MemoryBus) {
        self.rr_r8(bus, Register8::C);
    }

    pub fn cb_rr_d(&mut self, bus: &mut MemoryBus) {
        self.rr_r8(bus, Register8::D);
    }

    pub fn cb_rr_e(&mut self, bus: &mut MemoryBus) {
        self.rr_r8(bus, Register8::E);
    }

    pub fn cb_rr_h(&mut self, bus: &mut MemoryBus) {
        self.rr_r8(bus, Register8::H);
    }

    pub fn cb_rr_l(&mut self, bus: &mut MemoryBus) {
        self.rr_r8(bus, Register8::L);
    }

    pub fn cb_rr_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit0 = val & 0b00000001;
        let val = (val >> 1) + (self.register.get_flag_c_carry() << 7);

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    // SLA r8
    pub fn cb_sla_a(&mut self, bus: &mut MemoryBus) {
        self.sla_r8(bus, Register8::A);
    }

    pub fn cb_sla_b(&mut self, bus: &mut MemoryBus) {
        self.sla_r8(bus, Register8::B);
    }

    pub fn cb_sla_c(&mut self, bus: &mut MemoryBus) {
        self.sla_r8(bus, Register8::C);
    }

    pub fn cb_sla_d(&mut self, bus: &mut MemoryBus) {
        self.sla_r8(bus, Register8::D);
    }

    pub fn cb_sla_e(&mut self, bus: &mut MemoryBus) {
        self.sla_r8(bus, Register8::E);
    }

    pub fn cb_sla_h(&mut self, bus: &mut MemoryBus) {
        self.sla_r8(bus, Register8::H);
    }

    pub fn cb_sla_l(&mut self, bus: &mut MemoryBus) {
        self.sla_r8(bus, Register8::L);
    }

    pub fn cb_sla_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit7 = (val & 0b10000000) >> 7;
        let val = val << 1;

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    // SRA r8
    pub fn cb_sra_a(&mut self, bus: &mut MemoryBus) {
        self.sra_r8(bus, Register8::A);
    }

    pub fn cb_sra_b(&mut self, bus: &mut MemoryBus) {
        self.sra_r8(bus, Register8::B);
    }

    pub fn cb_sra_c(&mut self, bus: &mut MemoryBus) {
        self.sra_r8(bus, Register8::C);
    }

    pub fn cb_sra_d(&mut self, bus: &mut MemoryBus) {
        self.sra_r8(bus, Register8::D);
    }

    pub fn cb_sra_e(&mut self, bus: &mut MemoryBus) {
        self.sra_r8(bus, Register8::E);
    }

    pub fn cb_sra_h(&mut self, bus: &mut MemoryBus) {
        self.sra_r8(bus, Register8::H);
    }

    pub fn cb_sra_l(&mut self, bus: &mut MemoryBus) {
        self.sra_r8(bus, Register8::L);
    }

    pub fn cb_sra_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit0 = val & 0b00000001;
        let bit7 = val & 0b10000000;
        let val = (val >> 1) + bit7;

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    // SWAP r8
    pub fn cb_swap_a(&mut self, bus: &mut MemoryBus) {
        self.swap_r8(bus, Register8::A);
    }

    pub fn cb_swap_b(&mut self, bus: &mut MemoryBus) {
        self.swap_r8(bus, Register8::B);
    }

    pub fn cb_swap_c(&mut self, bus: &mut MemoryBus) {
        self.swap_r8(bus, Register8::C);
    }

    pub fn cb_swap_d(&mut self, bus: &mut MemoryBus) {
        self.swap_r8(bus, Register8::D);
    }

    pub fn cb_swap_e(&mut self, bus: &mut MemoryBus) {
        self.swap_r8(bus, Register8::E);
    }

    pub fn cb_swap_h(&mut self, bus: &mut MemoryBus) {
        self.swap_r8(bus, Register8::H);
    }

    pub fn cb_swap_l(&mut self, bus: &mut MemoryBus) {
        self.swap_r8(bus, Register8::L);
    }

    pub fn cb_swap_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let hi = val << 4;
        let lo = val >> 4;
        let val = hi | lo;

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    // SRL r8
    pub fn cb_srl_a(&mut self, bus: &mut MemoryBus) {
        self.srl_r8(bus, Register8::A);
    }

    pub fn cb_srl_b(&mut self, bus: &mut MemoryBus) {
        self.srl_r8(bus, Register8::B);
    }

    pub fn cb_srl_c(&mut self, bus: &mut MemoryBus) {
        self.srl_r8(bus, Register8::C);
    }

    pub fn cb_srl_d(&mut self, bus: &mut MemoryBus) {
        self.srl_r8(bus, Register8::D);
    }

    pub fn cb_srl_e(&mut self, bus: &mut MemoryBus) {
        self.srl_r8(bus, Register8::E);
    }

    pub fn cb_srl_h(&mut self, bus: &mut MemoryBus) {
        self.srl_r8(bus, Register8::H);
    }

    pub fn cb_srl_l(&mut self, bus: &mut MemoryBus) {
        self.srl_r8(bus, Register8::L);
    }

    pub fn cb_srl_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit0 = val & 0b00000001;
        let val = val >> 1;

        bus.tick(4);
        self.register.clear_flags();
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    // BIT b3, r8
    pub fn cb_bit_0_a(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B0, Register8::A);
    }

    pub fn cb_bit_0_b(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B0, Register8::B);
    }

    pub fn cb_bit_0_c(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B0, Register8::C);
    }

    pub fn cb_bit_0_d(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B0, Register8::D);
    }

    pub fn cb_bit_0_e(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B0, Register8::E);
    }

    pub fn cb_bit_0_h(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B0, Register8::H);
    }

    pub fn cb_bit_0_l(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B0, Register8::L);
    }

    pub fn cb_bit_0_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let mask = 0b00000001 << (Bit::B0 as u8);
        let val = val & mask;

        bus.tick(4);
        self.register.clear_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
    }

    pub fn cb_bit_1_a(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B1, Register8::A);
    }

    pub fn cb_bit_1_b(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B1, Register8::B);
    }

    pub fn cb_bit_1_c(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B1, Register8::C);
    }

    pub fn cb_bit_1_d(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B1, Register8::D);
    }

    pub fn cb_bit_1_e(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B1, Register8::E);
    }

    pub fn cb_bit_1_h(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B1, Register8::H);
    }

    pub fn cb_bit_1_l(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B1, Register8::L);
    }

    pub fn cb_bit_1_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let mask = 0b00000001 << (Bit::B1 as u8);
        let val = val & mask;

        bus.tick(4);
        self.register.clear_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
    }

    pub fn cb_bit_2_a(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B2, Register8::A);
    }

    pub fn cb_bit_2_b(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B2, Register8::B);
    }

    pub fn cb_bit_2_c(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B2, Register8::C);
    }

    pub fn cb_bit_2_d(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B2, Register8::D);
    }

    pub fn cb_bit_2_e(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B2, Register8::E);
    }

    pub fn cb_bit_2_h(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B2, Register8::H);
    }

    pub fn cb_bit_2_l(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B2, Register8::L);
    }

    pub fn cb_bit_2_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let mask = 0b00000001 << (Bit::B2 as u8);
        let val = val & mask;

        bus.tick(4);
        self.register.clear_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
    }

    pub fn cb_bit_3_a(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B3, Register8::A);
    }

    pub fn cb_bit_3_b(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B3, Register8::B);
    }

    pub fn cb_bit_3_c(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B3, Register8::C);
    }

    pub fn cb_bit_3_d(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B3, Register8::D);
    }

    pub fn cb_bit_3_e(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B3, Register8::E);
    }

    pub fn cb_bit_3_h(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B3, Register8::H);
    }

    pub fn cb_bit_3_l(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B3, Register8::L);
    }

    pub fn cb_bit_3_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let mask = 0b00000001 << (Bit::B3 as u8);
        let val = val & mask;

        bus.tick(4);
        self.register.clear_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
    }

    pub fn cb_bit_4_a(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B4, Register8::A);
    }

    pub fn cb_bit_4_b(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B4, Register8::B);
    }

    pub fn cb_bit_4_c(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B4, Register8::C);
    }

    pub fn cb_bit_4_d(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B4, Register8::D);
    }

    pub fn cb_bit_4_e(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B4, Register8::E);
    }

    pub fn cb_bit_4_h(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B4, Register8::H);
    }

    pub fn cb_bit_4_l(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B4, Register8::L);
    }

    pub fn cb_bit_4_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let mask = 0b00000001 << (Bit::B4 as u8);
        let val = val & mask;

        bus.tick(4);
        self.register.clear_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
    }

    pub fn cb_bit_5_a(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B5, Register8::A);
    }

    pub fn cb_bit_5_b(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B5, Register8::B);
    }

    pub fn cb_bit_5_c(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B5, Register8::C);
    }

    pub fn cb_bit_5_d(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B5, Register8::D);
    }

    pub fn cb_bit_5_e(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B5, Register8::E);
    }

    pub fn cb_bit_5_h(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B5, Register8::H);
    }

    pub fn cb_bit_5_l(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B5, Register8::L);
    }

    pub fn cb_bit_5_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let mask = 0b00000001 << (Bit::B5 as u8);
        let val = val & mask;

        bus.tick(4);
        self.register.clear_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
    }

    pub fn cb_bit_6_a(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B6, Register8::A);
    }

    pub fn cb_bit_6_b(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B6, Register8::B);
    }

    pub fn cb_bit_6_c(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B6, Register8::C);
    }

    pub fn cb_bit_6_d(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B6, Register8::D);
    }

    pub fn cb_bit_6_e(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B6, Register8::E);
    }

    pub fn cb_bit_6_h(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B6, Register8::H);
    }

    pub fn cb_bit_6_l(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B6, Register8::L);
    }

    pub fn cb_bit_6_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let mask = 0b00000001 << (Bit::B6 as u8);
        let val = val & mask;

        bus.tick(4);
        self.register.clear_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
    }

    pub fn cb_bit_7_a(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B7, Register8::A);
    }

    pub fn cb_bit_7_b(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B7, Register8::B);
    }

    pub fn cb_bit_7_c(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B7, Register8::C);
    }

    pub fn cb_bit_7_d(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B7, Register8::D);
    }

    pub fn cb_bit_7_e(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B7, Register8::E);
    }

    pub fn cb_bit_7_h(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B7, Register8::H);
    }

    pub fn cb_bit_7_l(&mut self, bus: &mut MemoryBus) {
        self.bit_b3_r8(bus, Bit::B7, Register8::L);
    }

    pub fn cb_bit_7_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let mask = 0b00000001 << (Bit::B7 as u8);
        let val = val & mask;

        bus.tick(4);
        self.register.clear_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        if val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
    }

    pub fn cb_res_0_a(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B0, Register8::A);
    }

    pub fn cb_res_0_b(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B0, Register8::B);
    }

    pub fn cb_res_0_c(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B0, Register8::C);
    }

    pub fn cb_res_0_d(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B0, Register8::D);
    }

    pub fn cb_res_0_e(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B0, Register8::E);
    }

    pub fn cb_res_0_h(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B0, Register8::H);
    }

    pub fn cb_res_0_l(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B0, Register8::L);
    }

    pub fn cb_res_0_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B0 as u8;
        let mask = !(0b00000001 << bit); // Bitshift 1 to the desired bit and then invert

        bus.tick(4);
        let val = val & mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_res_1_a(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B1, Register8::A);
    }

    pub fn cb_res_1_b(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B1, Register8::B);
    }

    pub fn cb_res_1_c(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B1, Register8::C);
    }

    pub fn cb_res_1_d(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B1, Register8::D);
    }

    pub fn cb_res_1_e(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B1, Register8::E);
    }

    pub fn cb_res_1_h(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B1, Register8::H);
    }

    pub fn cb_res_1_l(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B1, Register8::L);
    }

    pub fn cb_res_1_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B1 as u8;
        let mask = !(0b00000001 << bit); // Bitshift 1 to the desired bit and then invert

        bus.tick(4);
        let val = val & mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_res_2_a(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B2, Register8::A);
    }

    pub fn cb_res_2_b(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B2, Register8::B);
    }

    pub fn cb_res_2_c(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B2, Register8::C);
    }

    pub fn cb_res_2_d(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B2, Register8::D);
    }

    pub fn cb_res_2_e(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B2, Register8::E);
    }

    pub fn cb_res_2_h(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B2, Register8::H);
    }

    pub fn cb_res_2_l(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B2, Register8::L);
    }

    pub fn cb_res_2_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B2 as u8;
        let mask = !(0b00000001 << bit); // Bitshift 1 to the desired bit and then invert

        bus.tick(4);
        let val = val & mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_res_3_a(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B3, Register8::A);
    }

    pub fn cb_res_3_b(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B3, Register8::B);
    }

    pub fn cb_res_3_c(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B3, Register8::C);
    }

    pub fn cb_res_3_d(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B3, Register8::D);
    }

    pub fn cb_res_3_e(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B3, Register8::E);
    }

    pub fn cb_res_3_h(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B3, Register8::H);
    }

    pub fn cb_res_3_l(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B3, Register8::L);
    }

    pub fn cb_res_3_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B3 as u8;
        let mask = !(0b00000001 << bit); // Bitshift 1 to the desired bit and then invert

        bus.tick(4);
        let val = val & mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_res_4_a(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B4, Register8::A);
    }

    pub fn cb_res_4_b(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B4, Register8::B);
    }

    pub fn cb_res_4_c(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B4, Register8::C);
    }

    pub fn cb_res_4_d(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B4, Register8::D);
    }

    pub fn cb_res_4_e(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B4, Register8::E);
    }

    pub fn cb_res_4_h(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B4, Register8::H);
    }

    pub fn cb_res_4_l(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B4, Register8::L);
    }

    pub fn cb_res_4_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B4 as u8;
        let mask = !(0b00000001 << bit); // Bitshift 1 to the desired bit and then invert

        bus.tick(4);
        let val = val & mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_res_5_a(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B5, Register8::A);
    }

    pub fn cb_res_5_b(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B5, Register8::B);
    }

    pub fn cb_res_5_c(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B5, Register8::C);
    }

    pub fn cb_res_5_d(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B5, Register8::D);
    }

    pub fn cb_res_5_e(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B5, Register8::E);
    }

    pub fn cb_res_5_h(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B5, Register8::H);
    }

    pub fn cb_res_5_l(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B5, Register8::L);
    }

    pub fn cb_res_5_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B5 as u8;
        let mask = !(0b00000001 << bit); // Bitshift 1 to the desired bit and then invert

        bus.tick(4);
        let val = val & mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_res_6_a(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B6, Register8::A);
    }

    pub fn cb_res_6_b(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B6, Register8::B);
    }

    pub fn cb_res_6_c(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B6, Register8::C);
    }

    pub fn cb_res_6_d(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B6, Register8::D);
    }

    pub fn cb_res_6_e(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B6, Register8::E);
    }

    pub fn cb_res_6_h(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B6, Register8::H);
    }

    pub fn cb_res_6_l(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B6, Register8::L);
    }

    pub fn cb_res_6_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B6 as u8;
        let mask = !(0b00000001 << bit); // Bitshift 1 to the desired bit and then invert

        bus.tick(4);
        let val = val & mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_res_7_a(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B7, Register8::A);
    }

    pub fn cb_res_7_b(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B7, Register8::B);
    }

    pub fn cb_res_7_c(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B7, Register8::C);
    }

    pub fn cb_res_7_d(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B7, Register8::D);
    }

    pub fn cb_res_7_e(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B7, Register8::E);
    }

    pub fn cb_res_7_h(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B7, Register8::H);
    }

    pub fn cb_res_7_l(&mut self, bus: &mut MemoryBus) {
        self.res_b3_r8(bus, Bit::B7, Register8::L);
    }

    pub fn cb_res_7_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B7 as u8;
        let mask = !(0b00000001 << bit); // Bitshift 1 to the desired bit and then invert

        bus.tick(4);
        let val = val & mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_set_0_a(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B0, Register8::A);
    }

    pub fn cb_set_0_b(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B0, Register8::B);
    }

    pub fn cb_set_0_c(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B0, Register8::C);
    }

    pub fn cb_set_0_d(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B0, Register8::D);
    }

    pub fn cb_set_0_e(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B0, Register8::E);
    }

    pub fn cb_set_0_h(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B0, Register8::H);
    }

    pub fn cb_set_0_l(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B0, Register8::L);
    }

    pub fn cb_set_0_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B0 as u8;
        let mask = 0b00000001 << bit;

        bus.tick(4);
        let val = val | mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_set_1_a(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B1, Register8::A);
    }

    pub fn cb_set_1_b(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B1, Register8::B);
    }

    pub fn cb_set_1_c(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B1, Register8::C);
    }

    pub fn cb_set_1_d(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B1, Register8::D);
    }

    pub fn cb_set_1_e(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B1, Register8::E);
    }

    pub fn cb_set_1_h(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B1, Register8::H);
    }

    pub fn cb_set_1_l(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B1, Register8::L);
    }

    pub fn cb_set_1_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B1 as u8;
        let mask = 0b00000001 << bit;

        bus.tick(4);
        let val = val | mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_set_2_a(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B2, Register8::A);
    }

    pub fn cb_set_2_b(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B2, Register8::B);
    }

    pub fn cb_set_2_c(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B2, Register8::C);
    }

    pub fn cb_set_2_d(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B2, Register8::D);
    }

    pub fn cb_set_2_e(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B2, Register8::E);
    }

    pub fn cb_set_2_h(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B2, Register8::H);
    }

    pub fn cb_set_2_l(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B2, Register8::L);
    }

    pub fn cb_set_2_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B2 as u8;
        let mask = 0b00000001 << bit;

        bus.tick(4);
        let val = val | mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_set_3_a(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B3, Register8::A);
    }

    pub fn cb_set_3_b(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B3, Register8::B);
    }

    pub fn cb_set_3_c(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B3, Register8::C);
    }

    pub fn cb_set_3_d(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B3, Register8::D);
    }

    pub fn cb_set_3_e(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B3, Register8::E);
    }

    pub fn cb_set_3_h(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B3, Register8::H);
    }

    pub fn cb_set_3_l(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B3, Register8::L);
    }

    pub fn cb_set_3_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B3 as u8;
        let mask = 0b00000001 << bit;

        bus.tick(4);
        let val = val | mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_set_4_a(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B4, Register8::A);
    }

    pub fn cb_set_4_b(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B4, Register8::B);
    }

    pub fn cb_set_4_c(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B4, Register8::C);
    }

    pub fn cb_set_4_d(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B4, Register8::D);
    }

    pub fn cb_set_4_e(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B4, Register8::E);
    }

    pub fn cb_set_4_h(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B4, Register8::H);
    }

    pub fn cb_set_4_l(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B4, Register8::L);
    }

    pub fn cb_set_4_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B4 as u8;
        let mask = 0b00000001 << bit;

        bus.tick(4);
        let val = val | mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_set_5_a(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B5, Register8::A);
    }

    pub fn cb_set_5_b(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B5, Register8::B);
    }

    pub fn cb_set_5_c(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B5, Register8::C);
    }

    pub fn cb_set_5_d(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B5, Register8::D);
    }

    pub fn cb_set_5_e(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B5, Register8::E);
    }

    pub fn cb_set_5_h(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B5, Register8::H);
    }

    pub fn cb_set_5_l(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B5, Register8::L);
    }

    pub fn cb_set_5_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B5 as u8;
        let mask = 0b00000001 << bit;

        bus.tick(4);
        let val = val | mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_set_6_a(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B6, Register8::A);
    }

    pub fn cb_set_6_b(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B6, Register8::B);
    }

    pub fn cb_set_6_c(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B6, Register8::C);
    }

    pub fn cb_set_6_d(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B6, Register8::D);
    }

    pub fn cb_set_6_e(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B6, Register8::E);
    }

    pub fn cb_set_6_h(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B6, Register8::H);
    }

    pub fn cb_set_6_l(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B6, Register8::L);
    }

    pub fn cb_set_6_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B6 as u8;
        let mask = 0b00000001 << bit;

        bus.tick(4);
        let val = val | mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn cb_set_7_a(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B7, Register8::A);
    }

    pub fn cb_set_7_b(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B7, Register8::B);
    }

    pub fn cb_set_7_c(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B7, Register8::C);
    }

    pub fn cb_set_7_d(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B7, Register8::D);
    }

    pub fn cb_set_7_e(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B7, Register8::E);
    }

    pub fn cb_set_7_h(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B7, Register8::H);
    }

    pub fn cb_set_7_l(&mut self, bus: &mut MemoryBus) {
        self.set_b3_r8(bus, Bit::B7, Register8::L);
    }

    pub fn cb_set_7_hl_indirect(&mut self, bus: &mut MemoryBus) {
        bus.tick(4);
        let val = bus.read(self.register.get_hl());

        bus.tick(4);
        let bit = Bit::B7 as u8;
        let mask = 0b00000001 << bit;

        bus.tick(4);
        let val = val | mask;

        bus.tick(4);
        bus.write(self.register.get_hl(), val);
    }
}
