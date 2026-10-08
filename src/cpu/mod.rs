pub mod clock;
pub mod instruction;
pub mod register;

use clock::Clock;
use register::Register;

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

    pub fn fetch_and_execute(&mut self) {
        // Fetch
        // ! TODO: Need to imlement a way for Cpu to read Cartridge data
        // ! such as from a Memory bus. This will require a method at the
        // ! Runtime level to pass the MemoryBus struct to the Cpu.
        // self.op_code =
        let curr_instr = &instruction::INSTRUCTION_SET.normal[self.op_code as usize];
        if curr_instr.disassembly.eq("Not yet implemented") {
            panic!("No instruction configured for {:#2x?}", self.op_code)
        }

        println!(
            "Executing Op Code {}: {}",
            self.op_code, curr_instr.disassembly
        );
        // Increment program counter
        self.register.pc += 1;
        // Execute
        (curr_instr.operation)(self)
    }

    pub fn reset(&mut self) {
        self.register.reset();
    }

    pub fn cpu_noop(&mut self) {
        self.clock.advance_clock(4);
    }

    pub fn cpu_ld_bc_n16(&mut self) {
        instruction::cpu_routine_ld_16(&mut self.register.b, &mut self.register.c);
    }
}
