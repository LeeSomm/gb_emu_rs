pub mod instruction;
pub mod register;

use register::Register;

pub struct Cpu {
    pub register: Register,
    pub op_code: u8,
    pub instr_counter: u32,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            register: Register::new(),
            op_code: 0x0,
            instr_counter: 0x0,
        };
        cpu.reset();
        cpu
    }

    pub fn fetch_and_execute(&mut self) {
        // Fetch
        let curr_instr = &instruction::GB_CPU_INSTRUCTION_SET[self.op_code as usize];
        // Execute
        (curr_instr.operation)(self)
    }

    pub fn reset(&mut self) {
        self.register.reset();
    }

    pub fn cpu_noop(&mut self) {}

    pub fn cpu_ld_bc_n16(&mut self) {
        instruction::cpu_routine_ld_16(&mut self.register.b, &mut self.register.c);
    }
}
