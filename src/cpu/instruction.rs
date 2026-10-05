use super::Cpu;

pub struct CpuInstruction {
    pub disassembly: &'static str,
    pub operand_length: u8,
    pub operation: fn(&mut Cpu),
}

pub const GB_CPU_INSTRUCTION_SET: [CpuInstruction; 11] = [
    // 0x00
    CpuInstruction {
        disassembly: "NOP",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
    // 0x01
    CpuInstruction {
        disassembly: "LD BC, n16",
        operand_length: 0,
        operation: Cpu::cpu_ld_bc_n16,
    },
    // 0x02
    CpuInstruction {
        disassembly: "LD BC, A",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
    // 0x03
    CpuInstruction {
        disassembly: "",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
    // 0x04
    CpuInstruction {
        disassembly: "",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
    // 0x05
    CpuInstruction {
        disassembly: "",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
    // 0x06
    CpuInstruction {
        disassembly: "",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
    // 0x07
    CpuInstruction {
        disassembly: "",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
    // 0x08
    CpuInstruction {
        disassembly: "",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
    // 0x09
    CpuInstruction {
        disassembly: "",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
    // 0x0a
    CpuInstruction {
        disassembly: "",
        operand_length: 0,
        operation: Cpu::cpu_noop,
    },
];

// CPU Generic Operations
pub fn cpu_routine_ld_16(hi: &mut u8, lo: &mut u8) {
    // Advance cpu clocks by 4
    // Read into lo from memory bus
    // Advance cpu clocks by 4
    // Read into hi from memory bus
    // Advance cpu clocks by 4
}
