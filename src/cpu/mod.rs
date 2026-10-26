pub mod clock;
pub mod instruction;
pub mod operation;
pub mod register;

use clock::Clock;
use register::Register;

use crate::{cpu::instruction::CpuInstruction, memory::MemoryBus};

pub struct Cpu {
    pub register: Register,
    pub op_code: u8,
    // pub instr_counter: u32,
    pub clock: Clock,
    cb: bool,           // Whether the next operation is from CB Op table
    pub ime: bool,      // Interrupt Master Enable flag
    ime_delay: bool,    // Set IME to true after next operation
    pub halted: bool,   // Do nothing until interrupted
    pub halt_bug: bool, // Catch the HALT bug
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
            ime_delay: bool::default(),
            halted: bool::default(),
            halt_bug: bool::default(),
        };
        cpu.reset();
        cpu
    }

    pub fn fetch_and_execute(&mut self, bus: &mut MemoryBus) {
        // Interrupts

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

        // Increment program counter from op-code fetch
        if self.halt_bug {
            self.halt_bug = false
        } else {
            self.inc_pc();
        }
        // Execute
        (curr_instr.operation)(self, bus);

        // IME Delay logic
        if self.ime_delay {
            self.ime_delay = false;
            self.ime = true;
        }
    }

    pub fn reset(&mut self) {
        self.register.reset();
    }

    fn inc_pc(&mut self) {
        self.register.pc = self.register.pc.wrapping_add(1);
    }
}
