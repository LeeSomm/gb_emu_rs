pub mod clock;
pub mod instruction;
pub mod register;

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
        self.register.pc += 1;
    }

    // CPU Generic Operations

    fn routine_ld_r16_n16(&mut self, bus: &mut MemoryBus, register: &str) {
        self.clock.advance_clock(4);
        let lo = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let hi = bus.read(self.register.pc) as u16;
        self.inc_pc();

        self.clock.advance_clock(4);
        let val = (hi << 8) | lo;
        match register {
            "bc" => self.register.set_bc(val),
            "de" => self.register.set_de(val),
            "sp" => self.register.sp = val,
            "hl" => self.register.set_hl(val),
            _ => panic!("Attempted to load to invalid 16bit register."),
        }
    }

    // CPU Instructions
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

    pub fn ld_hl_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, "hl");
    }

    pub fn ld_sp_n16(&mut self, bus: &mut MemoryBus) {
        self.routine_ld_r16_n16(bus, "sp");
    }

    pub fn noop(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
    }

    pub fn xor_a_a(&mut self, _bus: &mut MemoryBus) {
        self.clock.advance_clock(4);
        self.register.a = 0; // XOR'ing a value against itself will always be 0
        self.register.set_flag_z_zero();
    }
}
