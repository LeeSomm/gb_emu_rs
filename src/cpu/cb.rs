use crate::cpu::Cpu;
use crate::cpu::register::Register8;
use crate::memory::MemoryBus;

impl Cpu {
    // Routines
    fn rlc_r8(&mut self, reg: Register8) {
        self.clock.advance_clock(4);
        self.register.clear_flags();
        let reg_val = self.register.read_r8(reg.clone());
        let bit7 = (reg_val & 0b10000000) >> 7;
        let val = (reg_val << 1) + bit7;

        self.clock.advance_clock(4);
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }

        self.register.write_r8(reg, val);
    }

    fn rrc_r8(&mut self, reg: Register8) {
        todo!()
    }

    fn rl_r8(&mut self, reg: Register8) {
        todo!()
    }

    fn rr_r8(&mut self, reg: Register8) {
        todo!()
    }

    fn sla_r8(&mut self, reg: Register8) {
        todo!()
    }

    fn sra_r8(&mut self, reg: Register8) {
        todo!()
    }

    fn swap_r8(&mut self, reg: Register8) {
        todo!()
    }

    fn srl_r8(&mut self, reg: Register8) {
        todo!()
    }

    fn bit_b3_r8(&mut self, reg: Register8) {
        todo!()
    }

    fn res_b3_r8(&mut self, reg: Register8) {
        todo!()
    }

    fn set_b3_r8(&mut self, reg: Register8) {
        todo!()
    }

    // RLC r8
    pub fn cb_rlc_a(&mut self, _bus: &mut MemoryBus) {
        self.rlc_r8(Register8::A);
    }

    pub fn cb_rlc_b(&mut self, _bus: &mut MemoryBus) {
        self.rlc_r8(Register8::B);
    }

    pub fn cb_rlc_c(&mut self, _bus: &mut MemoryBus) {
        self.rlc_r8(Register8::C);
    }

    pub fn cb_rlc_d(&mut self, _bus: &mut MemoryBus) {
        self.rlc_r8(Register8::D);
    }

    pub fn cb_rlc_e(&mut self, _bus: &mut MemoryBus) {
        self.rlc_r8(Register8::E);
    }

    pub fn cb_rlc_h(&mut self, _bus: &mut MemoryBus) {
        self.rlc_r8(Register8::H);
    }

    pub fn cb_rlc_l(&mut self, _bus: &mut MemoryBus) {
        self.rlc_r8(Register8::L);
    }

    pub fn cb_rlc_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
        self.register.clear_flags();
        let bit7 = (val & 0b10000000) >> 7;
        let val = (val << 1) + bit7;

        self.clock.advance_clock(4);
        if val == 0 {
            self.register.set_flag_z_zero();
        }
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }

        self.clock.advance_clock(4);
        bus.write(self.register.get_hl(), val);
    }
}
