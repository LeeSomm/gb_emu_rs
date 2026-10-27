pub mod instruction;
pub mod operation;
pub mod register;

use register::Register;

use crate::{cpu::instruction::CpuInstruction, memory::MemoryBus};

pub struct Cpu {
    pub register: Register,
    pub op_code: u8,
    // pub dbg_hits: u64,
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
            // dbg_hits: 0x0,
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
        // Fetch
        self.op_code = bus.read(self.register.pc);

        if !self.cb {
            println!(
                "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                self.register.a,
                self.register.f,
                self.register.b,
                self.register.c,
                self.register.d,
                self.register.e,
                self.register.h,
                self.register.l,
                self.register.sp,
                self.register.pc,
                bus.read(self.register.pc),
                bus.read(self.register.pc.wrapping_add(1)),
                bus.read(self.register.pc.wrapping_add(2)),
                bus.read(self.register.pc.wrapping_add(3)),
            );
        }

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

        // self.register.debug_print_registers();
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
