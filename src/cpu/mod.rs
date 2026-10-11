pub mod clock;
pub mod instruction;
pub mod register;

use clock::Clock;
use register::Register;

use crate::memory::MemoryBus;

pub struct Cpu {
    pub register: Register,
    pub op_code: u8,
    pub instr_counter: u32,
    clock: Clock,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            register: Register::new(),
            op_code: 0x0,
            instr_counter: 0x0,
            clock: Clock::default(),
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
        self.register.pc += 1;
    }

    // CPU Generic Operations
    pub fn cpu_routine_ld_16(hi: &mut u8, lo: &mut u8) {
        // Advance cpu clocks by 4
        // Read into lo from memory bus
        // Advance cpu clocks by 4
        // Read into hi from memory bus
        // Advance cpu clocks by 4
    }

    // CPU Instructions
    pub fn cpu_noop(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
    }

    pub fn cpu_ld_bc_n16(&mut self, bus: &mut MemoryBus) {
        Cpu::cpu_routine_ld_16(&mut self.register.b, &mut self.register.c);
    }

    pub fn cpu_jp_a16(&mut self, bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();
        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc) as u16;
        self.clock.advance_clock(4);
        self.register.pc = (hi << 8) | lo;
        self.clock.advance_clock(4);
    }
}
