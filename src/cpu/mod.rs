pub mod cb;
pub mod clock;
pub mod instruction;
pub mod register;
pub mod routine;

use clock::Clock;
use register::{FlagCondition, Register, Register8, Register16};

use crate::{cpu::instruction::CpuInstruction, memory::MemoryBus};

pub struct Cpu {
    pub register: Register,
    pub op_code: u8,
    // pub instr_counter: u32,
    clock: Clock,
    cb: bool,       // Whether the next operation is from CB Op table
    ime: bool,      // Interrupt Master Enable flag
    ime_next: bool, // Set IME to true after next operation
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            register: Register::new(),
            op_code: 0x0,
            // instr_counter: 0x0,
            clock: Clock::default(),
            cb: bool::default(),
            ime: bool::default(),
            ime_next: bool::default(),
        };
        cpu.reset();
        cpu
    }

    pub fn fetch_and_execute(&mut self, bus: &mut MemoryBus) {
        // Fetch
        self.op_code = bus.read(self.register.pc);

        let curr_instr: &CpuInstruction;
        if !self.cb {
            curr_instr = &instruction::INSTRUCTION_SET.normal[self.op_code as usize];
        } else {
            curr_instr = &instruction::INSTRUCTION_SET.cb[self.op_code as usize];
            self.cb = false;
        }

        if curr_instr.disassembly.eq("Not yet implemented") {
            panic!("No instruction configured for {:#2x?}", self.op_code)
        }

        // println!(
        //     "Executing Op Code {:#2x?}: {}",
        //     self.op_code, curr_instr.disassembly
        // );

        // Increment program counter
        self.inc_pc();
        // Execute
        (curr_instr.operation)(self, bus);

        // IME Next logic
        if self.ime_next {
            self.ime_next = false;
            self.ime = true;
        }
    }

    pub fn reset(&mut self) {
        self.register.reset();
    }

    fn inc_pc(&mut self) {
        self.register.pc = self.register.pc.wrapping_add(1);
    }

    // CPU Instructions

    // ADC r8, r8/n8
    pub fn adc_a_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_adc_a_r8(Register8::A);
    }

    pub fn adc_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_adc_a_r8(Register8::B);
    }

    pub fn adc_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_adc_a_r8(Register8::C);
    }

    pub fn adc_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_adc_a_r8(Register8::D);
    }

    pub fn adc_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_adc_a_r8(Register8::E);
    }

    pub fn adc_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_adc_a_r8(Register8::H);
    }

    pub fn adc_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_adc_a_r8(Register8::L);
    }

    pub fn adc_a_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
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

    pub fn adc_a_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
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

    // ADD r8, r8/n8
    pub fn add_a_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_a_r8(Register8::A);
    }

    pub fn add_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_a_r8(Register8::B);
    }

    pub fn add_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_a_r8(Register8::C);
    }

    pub fn add_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_a_r8(Register8::D);
    }

    pub fn add_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_a_r8(Register8::E);
    }

    pub fn add_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_a_r8(Register8::H);
    }

    pub fn add_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_a_r8(Register8::L);
    }

    pub fn add_a_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
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

    pub fn add_a_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
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

    // ADD r16, r16
    pub fn add_hl_bc(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_hl_r16(Register16::BC);
    }

    pub fn add_hl_de(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_hl_r16(Register16::DE);
    }

    pub fn add_hl_hl(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_hl_r16(Register16::HL);
    }

    pub fn add_hl_sp(&mut self, _bus: &mut MemoryBus) {
        self.routine_add_hl_r16(Register16::SP);
    }

    // ADD SP, e8
    pub fn add_sp_e8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let e = bus.read(self.register.pc); // Will cast as i8 later
        self.inc_pc();

        self.clock.advance_clock(4);
        let half_carry = (self.register.sp & 0x0F) + (e & 0x0F) as u16 > 0x0F; // Works w/ u8 b/c 2's Complement

        self.clock.advance_clock(4);
        let carry = (self.register.sp & 0xFF) + e as u16 > 0xFF;

        self.clock.advance_clock(4);
        let result = self.register.sp.wrapping_add_signed(e as i8 as i16); // Must cast as i8 first for correct sign extension
        self.register.clear_flags();

        if carry {
            self.register.set_flag_c_carry();
        }

        if half_carry {
            self.register.set_flag_h_half_carry();
        }

        self.clock.advance_clock(4);
        self.register.sp = result;
    }

    // AND
    pub fn and_a_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_and_a_r8(Register8::A);
    }

    pub fn and_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_and_a_r8(Register8::B);
    }

    pub fn and_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_and_a_r8(Register8::C);
    }

    pub fn and_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_and_a_r8(Register8::D);
    }

    pub fn and_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_and_a_r8(Register8::E);
    }

    pub fn and_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_and_a_r8(Register8::H);
    }

    pub fn and_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_and_a_r8(Register8::L);
    }

    pub fn and_a_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
        let result = self.register.a & val;
        self.register.a = result;
        self.register.clear_flags();
        if result == 0 {
            self.register.set_flag_z_zero();
        }
        self.register.set_flag_h_half_carry();
    }

    pub fn and_a_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        let result = self.register.a & val;
        self.register.a = result;
        self.register.clear_flags();
        if result == 0 {
            self.register.set_flag_z_zero();
        }
        self.register.set_flag_h_half_carry();
    }

    // CALL
    pub fn call_a16(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        self.register.sp = self.register.sp.wrapping_sub(1);

        self.clock.advance_clock(4);
        bus.write(self.register.sp, hi);

        self.clock.advance_clock(4);
        self.register.sp = self.register.sp.wrapping_sub(1);
        bus.write(self.register.sp, lo);

        self.clock.advance_clock(4);
        self.register.pc = ((hi as u16) << 8) | lo as u16;
    }

    pub fn call_c_a16(&mut self, bus: &mut MemoryBus) {
        self.routine_call_cc_a16(bus, FlagCondition::C);
    }

    pub fn call_nc_a16(&mut self, bus: &mut MemoryBus) {
        self.routine_call_cc_a16(bus, FlagCondition::NC);
    }

    pub fn call_z_a16(&mut self, bus: &mut MemoryBus) {
        self.routine_call_cc_a16(bus, FlagCondition::Z);
    }

    pub fn call_nz_a16(&mut self, bus: &mut MemoryBus) {
        self.routine_call_cc_a16(bus, FlagCondition::NZ);
    }

    // CB
    pub fn cb(&mut self, _bus: &mut MemoryBus) {
        // Machine cycles handled within CB operation functions
        self.cb = true;
    }

    // Carry Flag Operations
    pub fn ccf(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.clear_flag_n_subtraction();
        self.register.clear_flag_h_half_carry();
        if self.register.get_flag_c_carry() == 1 {
            self.register.clear_flag_c_carry();
        } else {
            self.register.set_flag_c_carry();
        }
    }

    pub fn scf(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.clear_flag_n_subtraction();
        self.register.clear_flag_h_half_carry();
        self.register.set_flag_c_carry();
    }

    // CP A, r8/n8
    pub fn cp_a_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_cp_a_r8(Register8::A);
    }

    pub fn cp_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_cp_a_r8(Register8::B);
    }

    pub fn cp_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_cp_a_r8(Register8::C);
    }

    pub fn cp_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_cp_a_r8(Register8::D);
    }

    pub fn cp_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_cp_a_r8(Register8::E);
    }

    pub fn cp_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_cp_a_r8(Register8::H);
    }

    pub fn cp_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_cp_a_r8(Register8::L);
    }

    pub fn cp_a_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
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

    pub fn cp_a_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
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

    // CPL
    pub fn cpl(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.set_flag_n_subtraction();
        self.register.set_flag_h_half_carry();
        self.register.a = !self.register.a;
    }

    //DAA
    pub fn daa(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let a_val = self.register.a;
        let new_val: u8;
        let n = self.register.get_flag_n_subtraction();
        let h = self.register.get_flag_h_half_carry();
        let c = self.register.get_flag_c_carry();
        self.register.clear_flag_c_carry();
        self.register.clear_flag_h_half_carry();
        let mut adj: u8 = 0;
        if n == 1 {
            if h == 1 {
                adj += 0x6;
            }
            if c == 1 {
                adj += 0x60;
            }
            new_val = a_val.wrapping_sub(adj);
        } else {
            if h == 1 || (a_val & 0xF > 0x9) {
                adj += 0x6;
            }
            if c == 1 || a_val > 0x99 {
                adj += 0x60;
                self.register.set_flag_c_carry();
            }
            new_val = a_val.wrapping_add(adj);
        }

        if new_val == 0 {
            self.register.set_flag_z_zero();
        } else {
            self.register.clear_flag_z_zero();
        }
        self.register.a = new_val;
    }

    // DEC r8/n8
    pub fn dec_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r8(Register8::A);
    }

    pub fn dec_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r8(Register8::B);
    }

    pub fn dec_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r8(Register8::C);
    }

    pub fn dec_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r8(Register8::D);
    }

    pub fn dec_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r8(Register8::E);
    }

    pub fn dec_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r8(Register8::H);
    }

    pub fn dec_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r8(Register8::L);
    }

    pub fn dec_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let mut val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
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

        bus.write(self.register.get_hl(), val);
    }

    // DEC r16
    pub fn dec_bc(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r16(Register16::BC);
    }

    pub fn dec_de(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r16(Register16::DE);
    }

    pub fn dec_hl(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r16(Register16::HL);
    }

    pub fn dec_sp(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r16(Register16::SP);
    }

    // DI
    pub fn di(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.ime = false;
    }

    // EI
    pub fn ei(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.ime_next = true;
    }

    // HALT
    pub fn halt(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        todo!("Implement HALT operation. Requires IME, IE, and IF flags (i.e., I/O)")
        // https://rgbds.gbdev.io/docs/v1.0.4/gbz80.7#HALT
    }

    // INC r8/n8
    pub fn inc_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r8(Register8::A);
    }

    pub fn inc_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r8(Register8::B);
    }

    pub fn inc_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r8(Register8::C);
    }

    pub fn inc_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r8(Register8::D);
    }

    pub fn inc_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r8(Register8::E);
    }

    pub fn inc_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r8(Register8::H);
    }

    pub fn inc_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r8(Register8::L);
    }

    pub fn inc_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let mut val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
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

        self.clock.advance_clock(4);
        bus.write(self.register.get_hl(), val);
    }

    // INC r16
    pub fn inc_bc(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r16(Register16::BC)
    }

    pub fn inc_de(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r16(Register16::DE)
    }

    pub fn inc_hl(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r16(Register16::HL)
    }

    pub fn inc_sp(&mut self, _bus: &mut MemoryBus) {
        self.routine_inc_r16(Register16::SP)
    }

    // JP Instructions
    pub fn jp_a16(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();
        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc) as u16;
        self.clock.advance_clock(4);
        self.register.pc = (hi << 8) | lo;
        self.clock.advance_clock(4);
        // println!("Jumping to: {:#x?}", self.register.pc);
    }

    pub fn jp_hl(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.pc = self.register.get_hl();
    }

    pub fn jp_c_a16(&mut self, bus: &mut MemoryBus) {
        self.routine_jp_cc_a16(bus, FlagCondition::C);
    }

    pub fn jp_nc_a16(&mut self, bus: &mut MemoryBus) {
        self.routine_jp_cc_a16(bus, FlagCondition::NC);
    }

    pub fn jp_z_a16(&mut self, bus: &mut MemoryBus) {
        self.routine_jp_cc_a16(bus, FlagCondition::Z);
    }

    pub fn jp_nz_a16(&mut self, bus: &mut MemoryBus) {
        self.routine_jp_cc_a16(bus, FlagCondition::NZ);
    }

    // JR Instructions
    pub fn jr_e8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let e: i8 = bus.read(self.register.pc) as i8;
        self.inc_pc();

        self.clock.advance_clock(4);
        self.register.pc = self.register.pc.wrapping_add_signed(e as i16);

        self.clock.advance_clock(4);
    }

    pub fn jr_c_e8(&mut self, bus: &mut MemoryBus) {
        let e: i8 = bus.read(self.register.pc) as i8;
        self.inc_pc();
        self.routine_jr_cc_e8(FlagCondition::C, e);
    }

    pub fn jr_nc_e8(&mut self, bus: &mut MemoryBus) {
        let e: i8 = bus.read(self.register.pc) as i8;
        self.inc_pc();
        self.routine_jr_cc_e8(FlagCondition::NC, e);
    }

    pub fn jr_z_e8(&mut self, bus: &mut MemoryBus) {
        let e: i8 = bus.read(self.register.pc) as i8;
        self.inc_pc();
        self.routine_jr_cc_e8(FlagCondition::Z, e);
    }

    pub fn jr_nz_e8(&mut self, bus: &mut MemoryBus) {
        let e: i8 = bus.read(self.register.pc) as i8;
        self.inc_pc();
        self.routine_jr_cc_e8(FlagCondition::NZ, e);
    }

    // LD R8, R8/n8
    pub fn ld_a_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::A, Register8::A);
    }

    pub fn ld_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::A, Register8::B);
    }

    pub fn ld_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::A, Register8::C);
    }

    pub fn ld_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::A, Register8::D);
    }

    pub fn ld_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::A, Register8::E);
    }

    pub fn ld_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::A, Register8::H);
    }

    pub fn ld_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::A, Register8::L);
    }

    pub fn ld_a_n8(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_n8(bus, Register8::A);
    }

    pub fn ld_b_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::B, Register8::A);
    }

    pub fn ld_b_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::B, Register8::B);
    }

    pub fn ld_b_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::B, Register8::C);
    }

    pub fn ld_b_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::B, Register8::D);
    }

    pub fn ld_b_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::B, Register8::E);
    }

    pub fn ld_b_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::B, Register8::H);
    }

    pub fn ld_b_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::B, Register8::L);
    }

    pub fn ld_b_n8(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_n8(bus, Register8::B);
    }

    pub fn ld_c_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::C, Register8::A);
    }

    pub fn ld_c_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::C, Register8::B);
    }

    pub fn ld_c_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::C, Register8::C);
    }

    pub fn ld_c_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::C, Register8::D);
    }

    pub fn ld_c_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::C, Register8::E);
    }

    pub fn ld_c_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::C, Register8::H);
    }

    pub fn ld_c_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::C, Register8::L);
    }

    pub fn ld_c_n8(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_n8(bus, Register8::C);
    }

    pub fn ld_d_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::D, Register8::A);
    }

    pub fn ld_d_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::D, Register8::B);
    }

    pub fn ld_d_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::D, Register8::C);
    }

    pub fn ld_d_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::D, Register8::D);
    }

    pub fn ld_d_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::D, Register8::E);
    }

    pub fn ld_d_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::D, Register8::H);
    }

    pub fn ld_d_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::D, Register8::L);
    }

    pub fn ld_d_n8(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_n8(bus, Register8::D);
    }

    pub fn ld_e_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::E, Register8::A);
    }

    pub fn ld_e_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::E, Register8::B);
    }

    pub fn ld_e_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::E, Register8::C);
    }

    pub fn ld_e_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::E, Register8::D);
    }

    pub fn ld_e_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::E, Register8::E);
    }

    pub fn ld_e_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::E, Register8::H);
    }

    pub fn ld_e_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::E, Register8::L);
    }

    pub fn ld_e_n8(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_n8(bus, Register8::E);
    }

    pub fn ld_h_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::H, Register8::A);
    }

    pub fn ld_h_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::H, Register8::B);
    }

    pub fn ld_h_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::H, Register8::C);
    }

    pub fn ld_h_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::H, Register8::D);
    }

    pub fn ld_h_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::H, Register8::E);
    }

    pub fn ld_h_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::H, Register8::H);
    }

    pub fn ld_h_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::H, Register8::L);
    }

    pub fn ld_h_n8(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_n8(bus, Register8::H);
    }

    pub fn ld_l_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::L, Register8::A);
    }

    pub fn ld_l_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::L, Register8::B);
    }

    pub fn ld_l_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::L, Register8::C);
    }

    pub fn ld_l_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::L, Register8::D);
    }

    pub fn ld_l_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::L, Register8::E);
    }

    pub fn ld_l_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::L, Register8::H);
    }

    pub fn ld_l_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8(Register8::L, Register8::L);
    }

    pub fn ld_l_n8(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_n8(bus, Register8::L);
    }

    // LD [a16/r16], r8/n8
    pub fn ld_a16_indirect_a(&mut self, bus: &mut MemoryBus) {
        // Load the 8bit value from register A into the 16bit address
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let addr = (hi << 8) | lo;
        bus.write(addr, self.register.a);

        self.clock.advance_clock(4);
    }

    pub fn ld_a16_indirect_sp(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let addr = (hi << 8) | lo;

        self.clock.advance_clock(4);
        let [lsb, msb] = self.register.sp.to_le_bytes();
        bus.write(addr, lsb);

        self.clock.advance_clock(4);
        bus.write(addr + 1, msb);
    }

    pub fn ld_bc_indirect_a(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_bc(), Register8::A);
    }

    pub fn ld_de_indirect_a(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_de(), Register8::A);
    }

    pub fn ld_hl_indirect_a(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_hl(), Register8::A);
    }

    pub fn ld_hl_indirect_b(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_hl(), Register8::B);
    }

    pub fn ld_hl_indirect_c(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_hl(), Register8::C);
    }

    pub fn ld_hl_indirect_d(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_hl(), Register8::D);
    }

    pub fn ld_hl_indirect_e(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_hl(), Register8::E);
    }

    pub fn ld_hl_indirect_h(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_hl(), Register8::H);
    }

    pub fn ld_hl_indirect_l(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_hl(), Register8::L);
    }

    pub fn ld_hl_indirect_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        bus.write(self.register.get_hl(), val);
    }

    pub fn ld_hli_indirect_a(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let hl = self.register.get_hl();
        bus.write(hl, self.register.a);

        self.clock.advance_clock(4);
        self.register.inc_hl();
    }

    pub fn ld_hld_indirect_a(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let hl = self.register.get_hl();
        bus.write(hl, self.register.a);

        self.clock.advance_clock(4);
        self.register.dec_hl();
    }

    // LD r8, [r16/a16]
    pub fn ld_a_bc_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::A, Register16::BC);
    }

    pub fn ld_a_de_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::A, Register16::DE);
    }

    pub fn ld_a_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::A, Register16::HL);
    }

    pub fn ld_a_hli_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.read_r16(Register16::HL));

        self.clock.advance_clock(4);
        self.register.write_r8(Register8::A, val);
        self.register.inc_hl();
    }

    pub fn ld_a_hld_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.read_r16(Register16::HL));

        self.clock.advance_clock(4);
        self.register.write_r8(Register8::A, val);
        self.register.dec_hl();
    }

    pub fn ld_a_a16_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let addr = (hi << 8) | lo;
        let val = bus.read(addr);

        self.clock.advance_clock(4);
        self.register.a = val;
    }

    pub fn ld_b_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::B, Register16::HL);
    }

    pub fn ld_c_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::C, Register16::HL);
    }

    pub fn ld_d_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::D, Register16::HL);
    }

    pub fn ld_e_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::E, Register16::HL);
    }

    pub fn ld_h_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::H, Register16::HL);
    }

    pub fn ld_l_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::L, Register16::HL);
    }

    // LD r16, n16/r16
    pub fn ld_bc_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, Register16::BC);
    }

    pub fn ld_de_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, Register16::DE);
    }

    pub fn ld_hl_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, Register16::HL);
    }

    pub fn ld_sp_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, Register16::SP);
    }

    pub fn ld_sp_hl(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.sp = self.register.get_hl();

        self.clock.advance_clock(4);
    }

    pub fn ld_hl_sp_e8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let e = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        let half_carry = (self.register.sp & 0x0F) + (e & 0x0F) as u16 > 0x0F; // Works w/ u8 b/c 2's Complement
        let carry = (self.register.sp & 0xFF) + e as u16 > 0xFF;

        let result = self.register.sp.wrapping_add_signed(e as i8 as i16); // Must cast as i8 first for correct sign extension
        self.register.clear_flags();

        if carry {
            self.register.set_flag_c_carry();
        }

        if half_carry {
            self.register.set_flag_h_half_carry();
        }

        self.clock.advance_clock(4);
        self.register.set_hl(result);
    }

    // LDH
    pub fn ldh_a8_indirect_a(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let addr = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let addr = 0xFF00 | addr;

        self.clock.advance_clock(4);
        bus.write(addr, self.register.a);
    }

    pub fn ldh_a_a8_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let addr = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let addr = 0xFF00 | addr;

        self.clock.advance_clock(4);
        self.register.a = bus.read(addr);
    }

    pub fn ldh_c_indirect_a(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let addr = 0xFF00 | self.register.c as u16;

        self.clock.advance_clock(4);
        bus.write(addr, self.register.a);
    }

    pub fn ldh_a_c_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let addr = 0xFF00 | self.register.c as u16;

        self.clock.advance_clock(4);
        self.register.a = bus.read(addr);
    }

    // NOOP
    pub fn noop(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
    }

    pub fn or_a_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_or_a_r8(self.register.a);
    }

    pub fn or_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_or_a_r8(self.register.b);
    }

    pub fn or_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_or_a_r8(self.register.c);
    }

    pub fn or_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_or_a_r8(self.register.d);
    }

    pub fn or_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_or_a_r8(self.register.e);
    }

    pub fn or_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_or_a_r8(self.register.h);
    }

    pub fn or_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_or_a_r8(self.register.l);
    }

    pub fn or_a_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
        self.register.a |= val;
        self.register.clear_flags();
        if self.register.a == 0 {
            self.register.set_flag_z_zero();
        }
    }

    pub fn or_a_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        self.register.a |= val;
        if self.register.a == 0 {
            self.register.set_flag_z_zero();
        }
    }

    // POP r16
    pub fn pop_af(&mut self, bus: &mut MemoryBus) {
        // Note: This overwrites flags
        self.routine_pop_r16(bus, Register16::AF);
    }

    pub fn pop_bc(&mut self, bus: &mut MemoryBus) {
        self.routine_pop_r16(bus, Register16::BC);
    }

    pub fn pop_de(&mut self, bus: &mut MemoryBus) {
        self.routine_pop_r16(bus, Register16::DE);
    }

    pub fn pop_hl(&mut self, bus: &mut MemoryBus) {
        self.routine_pop_r16(bus, Register16::HL);
    }

    // PUSH r16
    pub fn push_af(&mut self, bus: &mut MemoryBus) {
        // Note: This overwrites flags
        self.routine_push_r16(bus, Register16::AF);
    }

    pub fn push_bc(&mut self, bus: &mut MemoryBus) {
        self.routine_push_r16(bus, Register16::BC);
    }

    pub fn push_de(&mut self, bus: &mut MemoryBus) {
        self.routine_push_r16(bus, Register16::DE);
    }

    pub fn push_hl(&mut self, bus: &mut MemoryBus) {
        self.routine_push_r16(bus, Register16::HL);
    }

    // RST n
    pub fn rst_00(&mut self, bus: &mut MemoryBus) {
        self.routine_rst_n(bus, 0x00);
    }

    pub fn rst_08(&mut self, bus: &mut MemoryBus) {
        self.routine_rst_n(bus, 0x08);
    }

    pub fn rst_10(&mut self, bus: &mut MemoryBus) {
        self.routine_rst_n(bus, 0x10);
    }

    pub fn rst_18(&mut self, bus: &mut MemoryBus) {
        self.routine_rst_n(bus, 0x18);
    }

    pub fn rst_20(&mut self, bus: &mut MemoryBus) {
        self.routine_rst_n(bus, 0x20);
    }

    pub fn rst_28(&mut self, bus: &mut MemoryBus) {
        self.routine_rst_n(bus, 0x28);
    }

    pub fn rst_30(&mut self, bus: &mut MemoryBus) {
        self.routine_rst_n(bus, 0x30);
    }

    pub fn rst_38(&mut self, bus: &mut MemoryBus) {
        self.routine_rst_n(bus, 0x38);
    }

    // RET
    pub fn ret(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.sp) as u16;
        self.register.sp = self.register.sp.wrapping_add(1);

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.sp) as u16;
        self.register.sp = self.register.sp.wrapping_add(1);

        self.clock.advance_clock(4);
        self.register.pc = (hi << 8) | lo;

        self.clock.advance_clock(4);
    }

    pub fn ret_z(&mut self, bus: &mut MemoryBus) {
        self.routine_ret_cc(bus, FlagCondition::Z)
    }

    pub fn ret_nz(&mut self, bus: &mut MemoryBus) {
        self.routine_ret_cc(bus, FlagCondition::NZ)
    }

    pub fn ret_c(&mut self, bus: &mut MemoryBus) {
        self.routine_ret_cc(bus, FlagCondition::C)
    }

    pub fn ret_nc(&mut self, bus: &mut MemoryBus) {
        self.routine_ret_cc(bus, FlagCondition::NC)
    }

    // RETI
    pub fn reti(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.sp) as u16;
        self.register.sp = self.register.sp.wrapping_add(1);

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.sp) as u16;
        self.register.sp = self.register.sp.wrapping_add(1);

        self.clock.advance_clock(4);
        self.register.pc = (hi << 8) | lo;

        self.clock.advance_clock(4);
        self.ime = true;
    }

    // Rotate Registers
    pub fn rla(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let bit7 = (self.register.a & 0b10000000) >> 7;
        let val = (self.register.a << 1) + self.register.get_flag_c_carry();
        self.register.clear_flags();
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }
        self.register.a = val;
    }

    pub fn rlca(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.clear_flags();
        let bit7 = (self.register.a & 0b10000000) >> 7;
        let val = (self.register.a << 1) + bit7;
        if bit7 == 1 {
            self.register.set_flag_c_carry();
        }
        self.register.a = val;
    }

    pub fn rra(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let bit0 = self.register.a & 0b00000001;
        let carry_into_7 = self.register.get_flag_c_carry() << 7;
        let val = (self.register.a >> 1) + carry_into_7;
        self.register.clear_flags();
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }
        self.register.a = val;
    }

    pub fn rrca(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.clear_flags();
        let bit0 = (self.register.a & 0b00000001) << 7;
        let val = (self.register.a >> 1) + bit0;
        if bit0 == 1 {
            self.register.set_flag_c_carry();
        }
        self.register.a = val;
    }

    // STOP
    pub fn stop_n8(&mut self, bus: &mut MemoryBus) {
        // Puts CPU into very low power mode
        self.clock.advance_clock(4);
        let _ = bus.read(self.register.pc); // Followed by a byte, but doesn't do anything.
        self.inc_pc();
    }

    // SUB A
    pub fn sub_a_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_sub_a_r8(Register8::A);
    }

    pub fn sub_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_sub_a_r8(Register8::B);
    }

    pub fn sub_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_sub_a_r8(Register8::C);
    }

    pub fn sub_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_sub_a_r8(Register8::D);
    }

    pub fn sub_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_sub_a_r8(Register8::E);
    }

    pub fn sub_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_sub_a_r8(Register8::H);
    }

    pub fn sub_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_sub_a_r8(Register8::L);
    }

    pub fn sub_a_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
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

        self.register.a = result;
    }

    pub fn sub_a_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
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

        self.register.a = result;
    }

    // SBC A
    pub fn sbc_a_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_sbc_a_r8(Register8::A);
    }

    pub fn sbc_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_sbc_a_r8(Register8::B);
    }

    pub fn sbc_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_sbc_a_r8(Register8::C);
    }

    pub fn sbc_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_sbc_a_r8(Register8::D);
    }

    pub fn sbc_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_sbc_a_r8(Register8::E);
    }

    pub fn sbc_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_sbc_a_r8(Register8::H);
    }

    pub fn sbc_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_sbc_a_r8(Register8::L);
    }

    pub fn sbc_a_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
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
    }

    pub fn sbc_a_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
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
    }

    // XOR A
    pub fn xor_a_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_xor_a_r8(Register8::A);
    }

    pub fn xor_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_xor_a_r8(Register8::B);
    }

    pub fn xor_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_xor_a_r8(Register8::C);
    }

    pub fn xor_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_xor_a_r8(Register8::D);
    }

    pub fn xor_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_xor_a_r8(Register8::E);
    }

    pub fn xor_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_xor_a_r8(Register8::H);
    }

    pub fn xor_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_xor_a_r8(Register8::L);
    }

    pub fn xor_a_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.get_hl());

        self.clock.advance_clock(4);
        self.register.a = self.register.a ^ val;
        self.register.clear_flags();
        if self.register.a == 0 {
            self.register.set_flag_z_zero();
        }
    }

    pub fn xor_a_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        self.register.a = self.register.a ^ val;
        self.register.clear_flags();
        if self.register.a == 0 {
            self.register.set_flag_z_zero();
        }
    }
}
