use crate::memory::MemoryBus;

use super::Cpu;

pub struct InstructionSet {
    pub normal: [CpuInstruction; 256],
    pub cb: [CpuInstruction; 256],
}

pub struct CpuInstruction {
    pub disassembly: &'static str,
    pub operand_length: u8, // In bytes
    pub operation: fn(&mut Cpu, &mut MemoryBus),
}

pub const CALL_A16: CpuInstruction = CpuInstruction {
    disassembly: "CALL a16",
    operand_length: 2,
    operation: Cpu::call_a16,
};

pub const DEC_BC: CpuInstruction = CpuInstruction {
    disassembly: "DEC BC",
    operand_length: 0,
    operation: Cpu::dec_bc,
};

pub const DEC_DE: CpuInstruction = CpuInstruction {
    disassembly: "DEC DE",
    operand_length: 0,
    operation: Cpu::dec_de,
};

pub const DEC_HL: CpuInstruction = CpuInstruction {
    disassembly: "DEC HL",
    operand_length: 0,
    operation: Cpu::dec_hl,
};

pub const DEC_SP: CpuInstruction = CpuInstruction {
    disassembly: "DEC SP",
    operand_length: 0,
    operation: Cpu::dec_sp,
};

pub const DI: CpuInstruction = CpuInstruction {
    disassembly: "DI",
    operand_length: 0,
    operation: Cpu::di,
};

pub const INC_A: CpuInstruction = CpuInstruction {
    disassembly: "INC A",
    operand_length: 0,
    operation: Cpu::inc_a,
};

pub const INC_B: CpuInstruction = CpuInstruction {
    disassembly: "INC B",
    operand_length: 0,
    operation: Cpu::inc_b,
};

pub const INC_BC: CpuInstruction = CpuInstruction {
    disassembly: "INC BC",
    operand_length: 0,
    operation: Cpu::inc_bc,
};

pub const INC_C: CpuInstruction = CpuInstruction {
    disassembly: "INC C",
    operand_length: 0,
    operation: Cpu::inc_c,
};

pub const INC_D: CpuInstruction = CpuInstruction {
    disassembly: "INC D",
    operand_length: 0,
    operation: Cpu::inc_d,
};

pub const INC_DE: CpuInstruction = CpuInstruction {
    disassembly: "INC DE",
    operand_length: 0,
    operation: Cpu::inc_de,
};

pub const INC_E: CpuInstruction = CpuInstruction {
    disassembly: "INC E",
    operand_length: 0,
    operation: Cpu::inc_e,
};

pub const INC_H: CpuInstruction = CpuInstruction {
    disassembly: "INC H",
    operand_length: 0,
    operation: Cpu::inc_h,
};

pub const INC_HL: CpuInstruction = CpuInstruction {
    disassembly: "INC HL",
    operand_length: 0,
    operation: Cpu::inc_hl,
};

pub const INC_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "INC [HL]",
    operand_length: 0,
    operation: Cpu::inc_hl_indirect,
};

pub const INC_L: CpuInstruction = CpuInstruction {
    disassembly: "INC L",
    operand_length: 0,
    operation: Cpu::inc_l,
};

pub const INC_SP: CpuInstruction = CpuInstruction {
    disassembly: "INC SP",
    operand_length: 0,
    operation: Cpu::inc_sp,
};

pub const JP_A16: CpuInstruction = CpuInstruction {
    disassembly: "JP a16",
    operand_length: 2,
    operation: Cpu::jp_a16,
};

pub const JR_NZ_E8: CpuInstruction = CpuInstruction {
    disassembly: "JR NZ, e8",
    operand_length: 1,
    operation: Cpu::jr_nz_e8,
};

pub const LD_A_B: CpuInstruction = CpuInstruction {
    disassembly: "LD A, B",
    operand_length: 0,
    operation: Cpu::ld_a_b,
};

pub const LD_A_C: CpuInstruction = CpuInstruction {
    disassembly: "LD A, C",
    operand_length: 0,
    operation: Cpu::ld_a_c,
};

pub const LD_A_D: CpuInstruction = CpuInstruction {
    disassembly: "LD A, D",
    operand_length: 0,
    operation: Cpu::ld_a_d,
};

pub const LD_A_E: CpuInstruction = CpuInstruction {
    disassembly: "LD A, E",
    operand_length: 0,
    operation: Cpu::ld_a_e,
};

pub const LD_A_H: CpuInstruction = CpuInstruction {
    disassembly: "LD A, H",
    operand_length: 0,
    operation: Cpu::ld_a_h,
};

pub const LD_A_L: CpuInstruction = CpuInstruction {
    disassembly: "LD A, L",
    operand_length: 0,
    operation: Cpu::ld_a_l,
};

pub const LD_A_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD A, n8",
    operand_length: 1,
    operation: Cpu::ld_a_n8,
};

pub const LD_A16_A: CpuInstruction = CpuInstruction {
    disassembly: "LD [a16], A",
    operand_length: 2,
    operation: Cpu::ld_a16_a,
};

pub const LD_B_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD B, n8",
    operand_length: 1,
    operation: Cpu::ld_b_n8,
};

pub const LD_BC_N16: CpuInstruction = CpuInstruction {
    disassembly: "LD BC, n16",
    operand_length: 2,
    operation: Cpu::ld_bc_n16,
};

pub const LD_BC_A: CpuInstruction = CpuInstruction {
    disassembly: "LD BC, A",
    operand_length: 0,
    operation: Cpu::ld_bc_a,
};

pub const LD_C_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD C, n8",
    operand_length: 1,
    operation: Cpu::ld_c_n8,
};

pub const LD_D_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD D, n8",
    operand_length: 1,
    operation: Cpu::ld_d_n8,
};

pub const LD_DE_N16: CpuInstruction = CpuInstruction {
    disassembly: "LD DE, n16",
    operand_length: 2,
    operation: Cpu::ld_de_n16,
};

pub const LD_DE_A: CpuInstruction = CpuInstruction {
    disassembly: "LD DE, A",
    operand_length: 0,
    operation: Cpu::ld_de_a,
};

pub const LD_E_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD E, n8",
    operand_length: 1,
    operation: Cpu::ld_e_n8,
};

pub const LD_H_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD H, n8",
    operand_length: 1,
    operation: Cpu::ld_h_n8,
};

pub const LD_HLI_A: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL+], A",
    operand_length: 0,
    operation: Cpu::ld_hli_a,
};

pub const LD_HL_N16: CpuInstruction = CpuInstruction {
    disassembly: "LD HL, n16",
    operand_length: 2,
    operation: Cpu::ld_hl_n16,
};

pub const LD_L_A: CpuInstruction = CpuInstruction {
    disassembly: "LD L, A",
    operand_length: 0,
    operation: Cpu::ld_l_a,
};

pub const LD_L_B: CpuInstruction = CpuInstruction {
    disassembly: "LD L, B",
    operand_length: 0,
    operation: Cpu::ld_l_b,
};

pub const LD_L_C: CpuInstruction = CpuInstruction {
    disassembly: "LD L, C",
    operand_length: 0,
    operation: Cpu::ld_l_c,
};

pub const LD_L_D: CpuInstruction = CpuInstruction {
    disassembly: "LD L, D",
    operand_length: 0,
    operation: Cpu::ld_l_d,
};

pub const LD_L_E: CpuInstruction = CpuInstruction {
    disassembly: "LD L, E",
    operand_length: 0,
    operation: Cpu::ld_l_e,
};

pub const LD_L_H: CpuInstruction = CpuInstruction {
    disassembly: "LD L, H",
    operand_length: 0,
    operation: Cpu::ld_l_h,
};

pub const LD_L_L: CpuInstruction = CpuInstruction {
    disassembly: "LD L, L",
    operand_length: 0,
    operation: Cpu::ld_l_l,
};

pub const LD_L_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD L, n8",
    operand_length: 1,
    operation: Cpu::ld_l_n8,
};

pub const LD_SP_N16: CpuInstruction = CpuInstruction {
    disassembly: "LD SP, n16",
    operand_length: 2,
    operation: Cpu::ld_sp_n16,
};

pub const NOOP: CpuInstruction = CpuInstruction {
    disassembly: "NOP",
    operand_length: 0,
    operation: Cpu::noop,
};

pub const OR_A_A: CpuInstruction = CpuInstruction {
    disassembly: "OR A, A",
    operand_length: 0,
    operation: Cpu::or_a_a,
};

pub const OR_A_B: CpuInstruction = CpuInstruction {
    disassembly: "OR A, B",
    operand_length: 0,
    operation: Cpu::or_a_b,
};

pub const OR_A_C: CpuInstruction = CpuInstruction {
    disassembly: "OR A, C",
    operand_length: 0,
    operation: Cpu::or_a_c,
};

pub const OR_A_D: CpuInstruction = CpuInstruction {
    disassembly: "OR A, D",
    operand_length: 0,
    operation: Cpu::or_a_d,
};

pub const OR_A_E: CpuInstruction = CpuInstruction {
    disassembly: "OR A, E",
    operand_length: 0,
    operation: Cpu::or_a_e,
};

pub const OR_A_H: CpuInstruction = CpuInstruction {
    disassembly: "OR A, H",
    operand_length: 0,
    operation: Cpu::or_a_h,
};

pub const OR_A_L: CpuInstruction = CpuInstruction {
    disassembly: "OR A, L",
    operand_length: 0,
    operation: Cpu::or_a_l,
};

pub const OR_A_HL: CpuInstruction = CpuInstruction {
    disassembly: "OR A, [HL]",
    operand_length: 0,
    operation: Cpu::or_a_hl,
};

pub const OR_A_N8: CpuInstruction = CpuInstruction {
    disassembly: "OR A, n8",
    operand_length: 1,
    operation: Cpu::or_a_n8,
};

pub const RET: CpuInstruction = CpuInstruction {
    disassembly: "RET",
    operand_length: 0,
    operation: Cpu::ret,
};

pub const XOR_A_A: CpuInstruction = CpuInstruction {
    disassembly: "XOR A, A",
    operand_length: 0,
    operation: Cpu::xor_a_a,
};

// TODO: Remove when all instructions are implemented.
pub const PLACEHOLDER: CpuInstruction = CpuInstruction {
    disassembly: "Not yet implemented",
    operand_length: 0,
    operation: Cpu::noop,
};

pub const INSTRUCTION_SET: InstructionSet = InstructionSet {
    normal: [
        /* 0x00 */ NOOP,
        /* 0x01 */ LD_BC_N16,
        /* 0x02 */ LD_BC_A,
        /* 0x03 */ INC_BC,
        /* 0x04 */ INC_B,
        /* 0x05 */ PLACEHOLDER,
        /* 0x06 */ LD_B_N8,
        /* 0x07 */ PLACEHOLDER,
        /* 0x08 */ PLACEHOLDER,
        /* 0x09 */ PLACEHOLDER,
        /* 0x0A */ PLACEHOLDER,
        /* 0x0B */ DEC_BC,
        /* 0x0C */ INC_C,
        /* 0x0D */ PLACEHOLDER,
        /* 0x0E */ LD_C_N8,
        /* 0x0F */ PLACEHOLDER,
        /* 0x10 */ PLACEHOLDER,
        /* 0x11 */ LD_DE_N16,
        /* 0x12 */ LD_DE_A,
        /* 0x13 */ INC_DE,
        /* 0x14 */ INC_D,
        /* 0x15 */ PLACEHOLDER,
        /* 0x16 */ LD_D_N8,
        /* 0x17 */ PLACEHOLDER,
        /* 0x18 */ PLACEHOLDER,
        /* 0x19 */ PLACEHOLDER,
        /* 0x1A */ PLACEHOLDER,
        /* 0x1B */ DEC_DE,
        /* 0x1C */ INC_E,
        /* 0x1D */ PLACEHOLDER,
        /* 0x1E */ LD_E_N8,
        /* 0x1F */ PLACEHOLDER,
        /* 0x20 */ JR_NZ_E8,
        /* 0x21 */ LD_HL_N16,
        /* 0x22 */ LD_HLI_A,
        /* 0x23 */ INC_HL,
        /* 0x24 */ INC_H,
        /* 0x25 */ PLACEHOLDER,
        /* 0x26 */ LD_H_N8,
        /* 0x27 */ PLACEHOLDER,
        /* 0x28 */ PLACEHOLDER,
        /* 0x29 */ PLACEHOLDER,
        /* 0x2A */ PLACEHOLDER,
        /* 0x2B */ DEC_HL,
        /* 0x2C */ INC_L,
        /* 0x2D */ PLACEHOLDER,
        /* 0x2E */ LD_L_N8,
        /* 0x2F */ PLACEHOLDER,
        /* 0x30 */ PLACEHOLDER,
        /* 0x31 */ LD_SP_N16,
        /* 0x32 */ PLACEHOLDER,
        /* 0x33 */ INC_SP,
        /* 0x34 */ INC_HL_INDIRECT,
        /* 0x35 */ PLACEHOLDER,
        /* 0x36 */ PLACEHOLDER,
        /* 0x37 */ PLACEHOLDER,
        /* 0x38 */ PLACEHOLDER,
        /* 0x39 */ PLACEHOLDER,
        /* 0x3A */ PLACEHOLDER,
        /* 0x3B */ DEC_SP,
        /* 0x3C */ INC_A,
        /* 0x3D */ PLACEHOLDER,
        /* 0x3E */ LD_A_N8,
        /* 0x3F */ PLACEHOLDER,
        /* 0x40 */ PLACEHOLDER,
        /* 0x41 */ PLACEHOLDER,
        /* 0x42 */ PLACEHOLDER,
        /* 0x43 */ PLACEHOLDER,
        /* 0x44 */ PLACEHOLDER,
        /* 0x45 */ PLACEHOLDER,
        /* 0x46 */ PLACEHOLDER,
        /* 0x47 */ PLACEHOLDER,
        /* 0x48 */ PLACEHOLDER,
        /* 0x49 */ PLACEHOLDER,
        /* 0x4A */ PLACEHOLDER,
        /* 0x4B */ PLACEHOLDER,
        /* 0x4C */ PLACEHOLDER,
        /* 0x4D */ PLACEHOLDER,
        /* 0x4E */ PLACEHOLDER,
        /* 0x4F */ PLACEHOLDER,
        /* 0x50 */ PLACEHOLDER,
        /* 0x51 */ PLACEHOLDER,
        /* 0x52 */ PLACEHOLDER,
        /* 0x53 */ PLACEHOLDER,
        /* 0x54 */ PLACEHOLDER,
        /* 0x55 */ PLACEHOLDER,
        /* 0x56 */ PLACEHOLDER,
        /* 0x57 */ PLACEHOLDER,
        /* 0x58 */ PLACEHOLDER,
        /* 0x59 */ PLACEHOLDER,
        /* 0x5A */ PLACEHOLDER,
        /* 0x5B */ PLACEHOLDER,
        /* 0x5C */ PLACEHOLDER,
        /* 0x5D */ PLACEHOLDER,
        /* 0x5E */ PLACEHOLDER,
        /* 0x5F */ PLACEHOLDER,
        /* 0x60 */ PLACEHOLDER,
        /* 0x61 */ PLACEHOLDER,
        /* 0x62 */ PLACEHOLDER,
        /* 0x63 */ PLACEHOLDER,
        /* 0x64 */ PLACEHOLDER,
        /* 0x65 */ PLACEHOLDER,
        /* 0x66 */ PLACEHOLDER,
        /* 0x67 */ PLACEHOLDER,
        /* 0x68 */ LD_L_B,
        /* 0x69 */ LD_L_C,
        /* 0x6A */ LD_L_D,
        /* 0x6B */ LD_L_E,
        /* 0x6C */ LD_L_H,
        /* 0x6D */ LD_L_L,
        /* 0x6E */ PLACEHOLDER,
        /* 0x6F */ LD_L_A,
        /* 0x70 */ PLACEHOLDER,
        /* 0x71 */ PLACEHOLDER,
        /* 0x72 */ PLACEHOLDER,
        /* 0x73 */ PLACEHOLDER,
        /* 0x74 */ PLACEHOLDER,
        /* 0x75 */ PLACEHOLDER,
        /* 0x76 */ PLACEHOLDER,
        /* 0x77 */ PLACEHOLDER,
        /* 0x78 */ LD_A_B,
        /* 0x79 */ LD_A_C,
        /* 0x7A */ LD_A_D,
        /* 0x7B */ LD_A_E,
        /* 0x7C */ LD_A_H,
        /* 0x7D */ LD_A_L,
        /* 0x7E */ PLACEHOLDER,
        /* 0x7F */ PLACEHOLDER,
        /* 0x80 */ PLACEHOLDER,
        /* 0x81 */ PLACEHOLDER,
        /* 0x82 */ PLACEHOLDER,
        /* 0x83 */ PLACEHOLDER,
        /* 0x84 */ PLACEHOLDER,
        /* 0x85 */ PLACEHOLDER,
        /* 0x86 */ PLACEHOLDER,
        /* 0x87 */ PLACEHOLDER,
        /* 0x88 */ PLACEHOLDER,
        /* 0x89 */ PLACEHOLDER,
        /* 0x8A */ PLACEHOLDER,
        /* 0x8B */ PLACEHOLDER,
        /* 0x8C */ PLACEHOLDER,
        /* 0x8D */ PLACEHOLDER,
        /* 0x8E */ PLACEHOLDER,
        /* 0x8F */ PLACEHOLDER,
        /* 0x90 */ PLACEHOLDER,
        /* 0x91 */ PLACEHOLDER,
        /* 0x92 */ PLACEHOLDER,
        /* 0x93 */ PLACEHOLDER,
        /* 0x94 */ PLACEHOLDER,
        /* 0x95 */ PLACEHOLDER,
        /* 0x96 */ PLACEHOLDER,
        /* 0x97 */ PLACEHOLDER,
        /* 0x98 */ PLACEHOLDER,
        /* 0x99 */ PLACEHOLDER,
        /* 0x9A */ PLACEHOLDER,
        /* 0x9B */ PLACEHOLDER,
        /* 0x9C */ PLACEHOLDER,
        /* 0x9D */ PLACEHOLDER,
        /* 0x9E */ PLACEHOLDER,
        /* 0x9F */ PLACEHOLDER,
        /* 0xA0 */ PLACEHOLDER,
        /* 0xA1 */ PLACEHOLDER,
        /* 0xA2 */ PLACEHOLDER,
        /* 0xA3 */ PLACEHOLDER,
        /* 0xA4 */ PLACEHOLDER,
        /* 0xA5 */ PLACEHOLDER,
        /* 0xA6 */ PLACEHOLDER,
        /* 0xA7 */ PLACEHOLDER,
        /* 0xA8 */ PLACEHOLDER,
        /* 0xA9 */ PLACEHOLDER,
        /* 0xAA */ PLACEHOLDER,
        /* 0xAB */ PLACEHOLDER,
        /* 0xAC */ PLACEHOLDER,
        /* 0xAD */ PLACEHOLDER,
        /* 0xAE */ PLACEHOLDER,
        /* 0xAF */ XOR_A_A,
        /* 0xB0 */ OR_A_B,
        /* 0xB1 */ OR_A_C,
        /* 0xB2 */ OR_A_D,
        /* 0xB3 */ OR_A_E,
        /* 0xB4 */ OR_A_H,
        /* 0xB5 */ OR_A_L,
        /* 0xB6 */ OR_A_HL,
        /* 0xB7 */ OR_A_A,
        /* 0xB8 */ PLACEHOLDER,
        /* 0xB9 */ PLACEHOLDER,
        /* 0xBA */ PLACEHOLDER,
        /* 0xBB */ PLACEHOLDER,
        /* 0xBC */ PLACEHOLDER,
        /* 0xBD */ PLACEHOLDER,
        /* 0xBE */ PLACEHOLDER,
        /* 0xBF */ PLACEHOLDER,
        /* 0xC0 */ PLACEHOLDER,
        /* 0xC1 */ PLACEHOLDER,
        /* 0xC2 */ PLACEHOLDER,
        /* 0xC3 */ JP_A16,
        /* 0xC4 */ PLACEHOLDER,
        /* 0xC5 */ PLACEHOLDER,
        /* 0xC6 */ PLACEHOLDER,
        /* 0xC7 */ PLACEHOLDER,
        /* 0xC8 */ PLACEHOLDER,
        /* 0xC9 */ RET,
        /* 0xCA */ PLACEHOLDER,
        /* 0xCB */ PLACEHOLDER,
        /* 0xCC */ PLACEHOLDER,
        /* 0xCD */ CALL_A16,
        /* 0xCE */ PLACEHOLDER,
        /* 0xCF */ PLACEHOLDER,
        /* 0xD0 */ PLACEHOLDER,
        /* 0xD1 */ PLACEHOLDER,
        /* 0xD2 */ PLACEHOLDER,
        /* 0xD3 */ PLACEHOLDER,
        /* 0xD4 */ PLACEHOLDER,
        /* 0xD5 */ PLACEHOLDER,
        /* 0xD6 */ PLACEHOLDER,
        /* 0xD7 */ PLACEHOLDER,
        /* 0xD8 */ PLACEHOLDER,
        /* 0xD9 */ PLACEHOLDER,
        /* 0xDA */ PLACEHOLDER,
        /* 0xDB */ PLACEHOLDER,
        /* 0xDC */ PLACEHOLDER,
        /* 0xDD */ PLACEHOLDER,
        /* 0xDE */ PLACEHOLDER,
        /* 0xDF */ PLACEHOLDER,
        /* 0xE0 */ PLACEHOLDER,
        /* 0xE1 */ PLACEHOLDER,
        /* 0xE2 */ PLACEHOLDER,
        /* 0xE3 */ PLACEHOLDER,
        /* 0xE4 */ PLACEHOLDER,
        /* 0xE5 */ PLACEHOLDER,
        /* 0xE6 */ PLACEHOLDER,
        /* 0xE7 */ PLACEHOLDER,
        /* 0xE8 */ PLACEHOLDER,
        /* 0xE9 */ PLACEHOLDER,
        /* 0xEA */ LD_A16_A,
        /* 0xEB */ PLACEHOLDER,
        /* 0xEC */ PLACEHOLDER,
        /* 0xED */ PLACEHOLDER,
        /* 0xEE */ PLACEHOLDER,
        /* 0xEF */ PLACEHOLDER,
        /* 0xF0 */ PLACEHOLDER,
        /* 0xF1 */ PLACEHOLDER,
        /* 0xF2 */ PLACEHOLDER,
        /* 0xF3 */ DI,
        /* 0xF4 */ PLACEHOLDER,
        /* 0xF5 */ PLACEHOLDER,
        /* 0xF6 */ OR_A_N8,
        /* 0xF7 */ PLACEHOLDER,
        /* 0xF8 */ PLACEHOLDER,
        /* 0xF9 */ PLACEHOLDER,
        /* 0xFA */ PLACEHOLDER,
        /* 0xFB */ PLACEHOLDER,
        /* 0xFC */ PLACEHOLDER,
        /* 0xFD */ PLACEHOLDER,
        /* 0xFE */ PLACEHOLDER,
        /* 0xFF */ PLACEHOLDER,
    ],
    cb: [
        /* 0x00 */ PLACEHOLDER,
        /* 0x01 */ PLACEHOLDER,
        /* 0x02 */ PLACEHOLDER,
        /* 0x03 */ PLACEHOLDER,
        /* 0x04 */ PLACEHOLDER,
        /* 0x05 */ PLACEHOLDER,
        /* 0x06 */ PLACEHOLDER,
        /* 0x07 */ PLACEHOLDER,
        /* 0x08 */ PLACEHOLDER,
        /* 0x09 */ PLACEHOLDER,
        /* 0x0A */ PLACEHOLDER,
        /* 0x0B */ PLACEHOLDER,
        /* 0x0C */ PLACEHOLDER,
        /* 0x0D */ PLACEHOLDER,
        /* 0x0E */ PLACEHOLDER,
        /* 0x0F */ PLACEHOLDER,
        /* 0x10 */ PLACEHOLDER,
        /* 0x11 */ PLACEHOLDER,
        /* 0x12 */ PLACEHOLDER,
        /* 0x13 */ PLACEHOLDER,
        /* 0x14 */ PLACEHOLDER,
        /* 0x15 */ PLACEHOLDER,
        /* 0x16 */ PLACEHOLDER,
        /* 0x17 */ PLACEHOLDER,
        /* 0x18 */ PLACEHOLDER,
        /* 0x19 */ PLACEHOLDER,
        /* 0x1A */ PLACEHOLDER,
        /* 0x1B */ PLACEHOLDER,
        /* 0x1C */ PLACEHOLDER,
        /* 0x1D */ PLACEHOLDER,
        /* 0x1E */ PLACEHOLDER,
        /* 0x1F */ PLACEHOLDER,
        /* 0x20 */ PLACEHOLDER,
        /* 0x21 */ PLACEHOLDER,
        /* 0x22 */ PLACEHOLDER,
        /* 0x23 */ PLACEHOLDER,
        /* 0x24 */ PLACEHOLDER,
        /* 0x25 */ PLACEHOLDER,
        /* 0x26 */ PLACEHOLDER,
        /* 0x27 */ PLACEHOLDER,
        /* 0x28 */ PLACEHOLDER,
        /* 0x29 */ PLACEHOLDER,
        /* 0x2A */ PLACEHOLDER,
        /* 0x2B */ PLACEHOLDER,
        /* 0x2C */ PLACEHOLDER,
        /* 0x2D */ PLACEHOLDER,
        /* 0x2E */ PLACEHOLDER,
        /* 0x2F */ PLACEHOLDER,
        /* 0x30 */ PLACEHOLDER,
        /* 0x31 */ PLACEHOLDER,
        /* 0x32 */ PLACEHOLDER,
        /* 0x33 */ PLACEHOLDER,
        /* 0x34 */ PLACEHOLDER,
        /* 0x35 */ PLACEHOLDER,
        /* 0x36 */ PLACEHOLDER,
        /* 0x37 */ PLACEHOLDER,
        /* 0x38 */ PLACEHOLDER,
        /* 0x39 */ PLACEHOLDER,
        /* 0x3A */ PLACEHOLDER,
        /* 0x3B */ PLACEHOLDER,
        /* 0x3C */ PLACEHOLDER,
        /* 0x3D */ PLACEHOLDER,
        /* 0x3E */ PLACEHOLDER,
        /* 0x3F */ PLACEHOLDER,
        /* 0x40 */ PLACEHOLDER,
        /* 0x41 */ PLACEHOLDER,
        /* 0x42 */ PLACEHOLDER,
        /* 0x43 */ PLACEHOLDER,
        /* 0x44 */ PLACEHOLDER,
        /* 0x45 */ PLACEHOLDER,
        /* 0x46 */ PLACEHOLDER,
        /* 0x47 */ PLACEHOLDER,
        /* 0x48 */ PLACEHOLDER,
        /* 0x49 */ PLACEHOLDER,
        /* 0x4A */ PLACEHOLDER,
        /* 0x4B */ PLACEHOLDER,
        /* 0x4C */ PLACEHOLDER,
        /* 0x4D */ PLACEHOLDER,
        /* 0x4E */ PLACEHOLDER,
        /* 0x4F */ PLACEHOLDER,
        /* 0x50 */ PLACEHOLDER,
        /* 0x51 */ PLACEHOLDER,
        /* 0x52 */ PLACEHOLDER,
        /* 0x53 */ PLACEHOLDER,
        /* 0x54 */ PLACEHOLDER,
        /* 0x55 */ PLACEHOLDER,
        /* 0x56 */ PLACEHOLDER,
        /* 0x57 */ PLACEHOLDER,
        /* 0x58 */ PLACEHOLDER,
        /* 0x59 */ PLACEHOLDER,
        /* 0x5A */ PLACEHOLDER,
        /* 0x5B */ PLACEHOLDER,
        /* 0x5C */ PLACEHOLDER,
        /* 0x5D */ PLACEHOLDER,
        /* 0x5E */ PLACEHOLDER,
        /* 0x5F */ PLACEHOLDER,
        /* 0x60 */ PLACEHOLDER,
        /* 0x61 */ PLACEHOLDER,
        /* 0x62 */ PLACEHOLDER,
        /* 0x63 */ PLACEHOLDER,
        /* 0x64 */ PLACEHOLDER,
        /* 0x65 */ PLACEHOLDER,
        /* 0x66 */ PLACEHOLDER,
        /* 0x67 */ PLACEHOLDER,
        /* 0x68 */ PLACEHOLDER,
        /* 0x69 */ PLACEHOLDER,
        /* 0x6A */ PLACEHOLDER,
        /* 0x6B */ PLACEHOLDER,
        /* 0x6C */ PLACEHOLDER,
        /* 0x6D */ PLACEHOLDER,
        /* 0x6E */ PLACEHOLDER,
        /* 0x6F */ PLACEHOLDER,
        /* 0x70 */ PLACEHOLDER,
        /* 0x71 */ PLACEHOLDER,
        /* 0x72 */ PLACEHOLDER,
        /* 0x73 */ PLACEHOLDER,
        /* 0x74 */ PLACEHOLDER,
        /* 0x75 */ PLACEHOLDER,
        /* 0x76 */ PLACEHOLDER,
        /* 0x77 */ PLACEHOLDER,
        /* 0x78 */ PLACEHOLDER,
        /* 0x79 */ PLACEHOLDER,
        /* 0x7A */ PLACEHOLDER,
        /* 0x7B */ PLACEHOLDER,
        /* 0x7C */ PLACEHOLDER,
        /* 0x7D */ PLACEHOLDER,
        /* 0x7E */ PLACEHOLDER,
        /* 0x7F */ PLACEHOLDER,
        /* 0x80 */ PLACEHOLDER,
        /* 0x81 */ PLACEHOLDER,
        /* 0x82 */ PLACEHOLDER,
        /* 0x83 */ PLACEHOLDER,
        /* 0x84 */ PLACEHOLDER,
        /* 0x85 */ PLACEHOLDER,
        /* 0x86 */ PLACEHOLDER,
        /* 0x87 */ PLACEHOLDER,
        /* 0x88 */ PLACEHOLDER,
        /* 0x89 */ PLACEHOLDER,
        /* 0x8A */ PLACEHOLDER,
        /* 0x8B */ PLACEHOLDER,
        /* 0x8C */ PLACEHOLDER,
        /* 0x8D */ PLACEHOLDER,
        /* 0x8E */ PLACEHOLDER,
        /* 0x8F */ PLACEHOLDER,
        /* 0x90 */ PLACEHOLDER,
        /* 0x91 */ PLACEHOLDER,
        /* 0x92 */ PLACEHOLDER,
        /* 0x93 */ PLACEHOLDER,
        /* 0x94 */ PLACEHOLDER,
        /* 0x95 */ PLACEHOLDER,
        /* 0x96 */ PLACEHOLDER,
        /* 0x97 */ PLACEHOLDER,
        /* 0x98 */ PLACEHOLDER,
        /* 0x99 */ PLACEHOLDER,
        /* 0x9A */ PLACEHOLDER,
        /* 0x9B */ PLACEHOLDER,
        /* 0x9C */ PLACEHOLDER,
        /* 0x9D */ PLACEHOLDER,
        /* 0x9E */ PLACEHOLDER,
        /* 0x9F */ PLACEHOLDER,
        /* 0xA0 */ PLACEHOLDER,
        /* 0xA1 */ PLACEHOLDER,
        /* 0xA2 */ PLACEHOLDER,
        /* 0xA3 */ PLACEHOLDER,
        /* 0xA4 */ PLACEHOLDER,
        /* 0xA5 */ PLACEHOLDER,
        /* 0xA6 */ PLACEHOLDER,
        /* 0xA7 */ PLACEHOLDER,
        /* 0xA8 */ PLACEHOLDER,
        /* 0xA9 */ PLACEHOLDER,
        /* 0xAA */ PLACEHOLDER,
        /* 0xAB */ PLACEHOLDER,
        /* 0xAC */ PLACEHOLDER,
        /* 0xAD */ PLACEHOLDER,
        /* 0xAE */ PLACEHOLDER,
        /* 0xAF */ PLACEHOLDER,
        /* 0xB0 */ PLACEHOLDER,
        /* 0xB1 */ PLACEHOLDER,
        /* 0xB2 */ PLACEHOLDER,
        /* 0xB3 */ PLACEHOLDER,
        /* 0xB4 */ PLACEHOLDER,
        /* 0xB5 */ PLACEHOLDER,
        /* 0xB6 */ PLACEHOLDER,
        /* 0xB7 */ PLACEHOLDER,
        /* 0xB8 */ PLACEHOLDER,
        /* 0xB9 */ PLACEHOLDER,
        /* 0xBA */ PLACEHOLDER,
        /* 0xBB */ PLACEHOLDER,
        /* 0xBC */ PLACEHOLDER,
        /* 0xBD */ PLACEHOLDER,
        /* 0xBE */ PLACEHOLDER,
        /* 0xBF */ PLACEHOLDER,
        /* 0xC0 */ PLACEHOLDER,
        /* 0xC1 */ PLACEHOLDER,
        /* 0xC2 */ PLACEHOLDER,
        /* 0xC3 */ PLACEHOLDER,
        /* 0xC4 */ PLACEHOLDER,
        /* 0xC5 */ PLACEHOLDER,
        /* 0xC6 */ PLACEHOLDER,
        /* 0xC7 */ PLACEHOLDER,
        /* 0xC8 */ PLACEHOLDER,
        /* 0xC9 */ PLACEHOLDER,
        /* 0xCA */ PLACEHOLDER,
        /* 0xCB */ PLACEHOLDER,
        /* 0xCC */ PLACEHOLDER,
        /* 0xCD */ PLACEHOLDER,
        /* 0xCE */ PLACEHOLDER,
        /* 0xCF */ PLACEHOLDER,
        /* 0xD0 */ PLACEHOLDER,
        /* 0xD1 */ PLACEHOLDER,
        /* 0xD2 */ PLACEHOLDER,
        /* 0xD3 */ PLACEHOLDER,
        /* 0xD4 */ PLACEHOLDER,
        /* 0xD5 */ PLACEHOLDER,
        /* 0xD6 */ PLACEHOLDER,
        /* 0xD7 */ PLACEHOLDER,
        /* 0xD8 */ PLACEHOLDER,
        /* 0xD9 */ PLACEHOLDER,
        /* 0xDA */ PLACEHOLDER,
        /* 0xDB */ PLACEHOLDER,
        /* 0xDC */ PLACEHOLDER,
        /* 0xDD */ PLACEHOLDER,
        /* 0xDE */ PLACEHOLDER,
        /* 0xDF */ PLACEHOLDER,
        /* 0xE0 */ PLACEHOLDER,
        /* 0xE1 */ PLACEHOLDER,
        /* 0xE2 */ PLACEHOLDER,
        /* 0xE3 */ PLACEHOLDER,
        /* 0xE4 */ PLACEHOLDER,
        /* 0xE5 */ PLACEHOLDER,
        /* 0xE6 */ PLACEHOLDER,
        /* 0xE7 */ PLACEHOLDER,
        /* 0xE8 */ PLACEHOLDER,
        /* 0xE9 */ PLACEHOLDER,
        /* 0xEA */ PLACEHOLDER,
        /* 0xEB */ PLACEHOLDER,
        /* 0xEC */ PLACEHOLDER,
        /* 0xED */ PLACEHOLDER,
        /* 0xEE */ PLACEHOLDER,
        /* 0xEF */ PLACEHOLDER,
        /* 0xF0 */ PLACEHOLDER,
        /* 0xF1 */ PLACEHOLDER,
        /* 0xF2 */ PLACEHOLDER,
        /* 0xF3 */ PLACEHOLDER,
        /* 0xF4 */ PLACEHOLDER,
        /* 0xF5 */ PLACEHOLDER,
        /* 0xF6 */ PLACEHOLDER,
        /* 0xF7 */ PLACEHOLDER,
        /* 0xF8 */ PLACEHOLDER,
        /* 0xF9 */ PLACEHOLDER,
        /* 0xFA */ PLACEHOLDER,
        /* 0xFB */ PLACEHOLDER,
        /* 0xFC */ PLACEHOLDER,
        /* 0xFD */ PLACEHOLDER,
        /* 0xFE */ PLACEHOLDER,
        /* 0xFF */ PLACEHOLDER,
    ],
};
