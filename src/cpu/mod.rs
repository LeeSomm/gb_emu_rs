pub mod clock;
pub mod instruction;
pub mod register;
pub mod routine;

use clock::Clock;
use register::Register;

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

    pub fn dec_bc(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r16("bc");
    }

    pub fn dec_de(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r16("de");
    }

    pub fn dec_hl(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r16("hl");
    }

    pub fn dec_sp(&mut self, _bus: &mut MemoryBus) {
        self.routine_dec_r16("sp");
    }

    pub fn di(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.ime = false;
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

    pub fn ld_a_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("a", "b");
    }

    pub fn ld_a_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("a", "c");
    }

    pub fn ld_a_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("a", "d");
    }

    pub fn ld_a_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("a", "e");
    }

    pub fn ld_a_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("a", "h");
    }

    pub fn ld_a_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("a", "l");
    }

    pub fn ld_a_n8(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let val = bus.read(self.register.pc);
        self.inc_pc();

        self.clock.advance_clock(4);
        self.register.a = val;
    }

    pub fn ld_a16_a(&mut self, bus: &mut MemoryBus) {
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

    pub fn ld_bc_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, "bc");
    }

    pub fn ld_de_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, "de");
    }

    pub fn ld_hli_a(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let hl = self.register.get_hl();
        bus.write(hl, self.register.a);

        self.clock.advance_clock(4);
        self.register.inc_hl();
    }

    pub fn ld_hl_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, "hl");
    }

    pub fn ld_l_a(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("l", "a");
    }

    pub fn ld_l_b(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("l", "b");
    }

    pub fn ld_l_c(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("l", "c");
    }

    pub fn ld_l_d(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("l", "d");
    }

    pub fn ld_l_e(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("l", "e");
    }

    pub fn ld_l_h(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("l", "h");
    }

    pub fn ld_l_l(&mut self, _bus: &mut MemoryBus) {
        self.routine_ld_r8_r8("l", "l");
    }

    pub fn ld_sp_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, "sp");
    }

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

    pub fn or_a_hl(&mut self, bus: &mut MemoryBus) {
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

    pub fn xor_a_a(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.a = 0; // XOR'ing a value against itself will always be 0
        self.register.clear_flags();
        self.register.set_flag_z_zero();
    }
}
