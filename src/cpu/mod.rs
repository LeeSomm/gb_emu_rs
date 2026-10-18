pub mod clock;
pub mod instruction;
pub mod register;
pub mod routine;

use clock::Clock;
use register::{Register, Register8, Register16};

use crate::memory::MemoryBus;

pub struct Cpu {
    pub register: Register,
    pub op_code: u8,
    // pub instr_counter: u32,
    clock: Clock,
    ime: bool, // Interrupt Master Enable flag
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            register: Register::new(),
            op_code: 0x0,
            // instr_counter: 0x0,
            clock: Clock::default(),
            ime: bool::default(),
        };
        cpu.reset();
        cpu
    }

    pub fn fetch_and_execute(&mut self, bus: &mut MemoryBus) {
        // Fetch
        self.op_code = bus.read(self.register.pc);
        let curr_instr = &instruction::INSTRUCTION_SET.normal[self.op_code as usize];
        if curr_instr.disassembly.eq("Not yet implemented") {
            panic!("No instruction configured for {:#2x?}", self.op_code)
        }

        println!(
            "Executing Op Code {:#2x?}: {}",
            self.op_code, curr_instr.disassembly
        );
        // Increment program counter
        self.inc_pc();
        // Execute
        (curr_instr.operation)(self, bus)
    }

    pub fn reset(&mut self) {
        self.register.reset();
    }

    fn inc_pc(&mut self) {
        self.register.pc = self.register.pc.wrapping_add(1);
    }

    // CPU Instructions
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

    pub fn call_a16(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        self.register.sp -= 1;

        self.clock.advance_clock(4);
        bus.write(self.register.sp, hi);

        self.clock.advance_clock(4);
        self.register.sp -= 1;
        bus.write(self.register.sp, lo);

        self.clock.advance_clock(4);
        self.register.pc = ((hi as u16) << 8) | lo as u16;
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

    pub fn dec_hl_indirect(&mut self, bus: &mut MemoryBus) {}

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

    pub fn inc_hl_indirect(&mut self, bus: &mut MemoryBus) {}

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

    pub fn jp_a16(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();
        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc) as u16;
        self.clock.advance_clock(4);
        self.register.pc = (hi << 8) | lo;
        self.clock.advance_clock(4);
        println!("Jumping to: {:#x?}", self.register.pc);
    }

    // JR Instructions
    pub fn jr_nz_e8(&mut self, bus: &mut MemoryBus) {
        let e: i8 = bus.read(self.register.pc) as i8;
        self.inc_pc();
        self.routine_jr_cc_e8("nz", e);
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

    // LD r8, [r16]
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

    pub fn ld_c_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::C, Register16::HL);
    }

    pub fn ld_e_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::E, Register16::HL);
    }

    pub fn ld_l_hl_indirect(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r8_r16_indirect(bus, Register8::L, Register16::HL);
    }

    // LD [a16], r8
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

    // LD r16, n16
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

    // LD [r16], r8
    pub fn ld_bc_indirect_a(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_bc(), Register8::A);
    }

    pub fn ld_de_indirect_a(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_a16_indirect_r8(bus, self.register.get_de(), Register8::A);
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

    // Pop r16
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

    // Ret
    pub fn ret(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.sp) as u16;
        self.register.sp += 1;

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.sp) as u16;
        self.register.sp += 1;

        self.clock.advance_clock(4);
        self.register.pc = (hi << 8) | lo;

        self.clock.advance_clock(4);
    }

    // Rotate Registers
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

    pub fn xor_a_a(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.a = 0; // XOR'ing a value against itself will always be 0
        self.register.clear_flags();
        self.register.set_flag_z_zero();
    }
}
