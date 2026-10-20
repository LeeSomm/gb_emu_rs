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

// ADC r8, r8/n8
pub const ADC_A_A: CpuInstruction = CpuInstruction {
    disassembly: "ADC A, A",
    operand_length: 0,
    operation: Cpu::adc_a_a,
};

pub const ADC_A_B: CpuInstruction = CpuInstruction {
    disassembly: "ADC A, B",
    operand_length: 0,
    operation: Cpu::adc_a_b,
};

pub const ADC_A_C: CpuInstruction = CpuInstruction {
    disassembly: "ADC A, C",
    operand_length: 0,
    operation: Cpu::adc_a_c,
};

pub const ADC_A_D: CpuInstruction = CpuInstruction {
    disassembly: "ADC A, D",
    operand_length: 0,
    operation: Cpu::adc_a_d,
};

pub const ADC_A_E: CpuInstruction = CpuInstruction {
    disassembly: "ADC A, E",
    operand_length: 0,
    operation: Cpu::adc_a_e,
};

pub const ADC_A_H: CpuInstruction = CpuInstruction {
    disassembly: "ADC A, H",
    operand_length: 0,
    operation: Cpu::adc_a_h,
};

pub const ADC_A_L: CpuInstruction = CpuInstruction {
    disassembly: "ADC A, L",
    operand_length: 0,
    operation: Cpu::adc_a_l,
};

pub const ADC_A_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "ADC A, [HL]",
    operand_length: 0,
    operation: Cpu::adc_a_hl_indirect,
};

pub const ADC_A_N8: CpuInstruction = CpuInstruction {
    disassembly: "ADC A, n8",
    operand_length: 1,
    operation: Cpu::adc_a_n8,
};

// ADD r8, r8/n8
pub const ADD_A_A: CpuInstruction = CpuInstruction {
    disassembly: "ADD A, A",
    operand_length: 0,
    operation: Cpu::add_a_a,
};

pub const ADD_A_B: CpuInstruction = CpuInstruction {
    disassembly: "ADD A, B",
    operand_length: 0,
    operation: Cpu::add_a_b,
};

pub const ADD_A_C: CpuInstruction = CpuInstruction {
    disassembly: "ADD A, C",
    operand_length: 0,
    operation: Cpu::add_a_c,
};

pub const ADD_A_D: CpuInstruction = CpuInstruction {
    disassembly: "ADD A, D",
    operand_length: 0,
    operation: Cpu::add_a_d,
};

pub const ADD_A_E: CpuInstruction = CpuInstruction {
    disassembly: "ADD A, E",
    operand_length: 0,
    operation: Cpu::add_a_e,
};

pub const ADD_A_H: CpuInstruction = CpuInstruction {
    disassembly: "ADD A, H",
    operand_length: 0,
    operation: Cpu::add_a_h,
};

pub const ADD_A_L: CpuInstruction = CpuInstruction {
    disassembly: "ADD A, L",
    operand_length: 0,
    operation: Cpu::add_a_l,
};

pub const ADD_A_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "ADD A, [HL]",
    operand_length: 0,
    operation: Cpu::add_a_hl_indirect,
};

pub const ADD_A_N8: CpuInstruction = CpuInstruction {
    disassembly: "ADD A, n8",
    operand_length: 1,
    operation: Cpu::add_a_n8,
};

// ADD r16, r16
pub const ADD_HL_BC: CpuInstruction = CpuInstruction {
    disassembly: "ADD HL, BC",
    operand_length: 0,
    operation: Cpu::add_hl_bc,
};

pub const ADD_HL_DE: CpuInstruction = CpuInstruction {
    disassembly: "ADD HL, DE",
    operand_length: 0,
    operation: Cpu::add_hl_de,
};

pub const ADD_HL_HL: CpuInstruction = CpuInstruction {
    disassembly: "ADD HL, HL",
    operand_length: 0,
    operation: Cpu::add_hl_hl,
};

pub const ADD_HL_SP: CpuInstruction = CpuInstruction {
    disassembly: "ADD HL, SP",
    operand_length: 0,
    operation: Cpu::add_hl_sp,
};

pub const CALL_A16: CpuInstruction = CpuInstruction {
    disassembly: "CALL a16",
    operand_length: 2,
    operation: Cpu::call_a16,
};

// Carry Flag Instructions
pub const CCF: CpuInstruction = CpuInstruction {
    disassembly: "CCF",
    operand_length: 0,
    operation: Cpu::ccf,
};

pub const SCF: CpuInstruction = CpuInstruction {
    disassembly: "SCF",
    operand_length: 0,
    operation: Cpu::scf,
};

// CP A, r8/n8
pub const CP_A_A: CpuInstruction = CpuInstruction {
    disassembly: "CP A, A",
    operand_length: 0,
    operation: Cpu::cp_a_a,
};

pub const CP_A_B: CpuInstruction = CpuInstruction {
    disassembly: "CP A, B",
    operand_length: 0,
    operation: Cpu::cp_a_b,
};

pub const CP_A_C: CpuInstruction = CpuInstruction {
    disassembly: "CP A, C",
    operand_length: 0,
    operation: Cpu::cp_a_c,
};

pub const CP_A_D: CpuInstruction = CpuInstruction {
    disassembly: "CP A, D",
    operand_length: 0,
    operation: Cpu::cp_a_d,
};

pub const CP_A_E: CpuInstruction = CpuInstruction {
    disassembly: "CP A, E",
    operand_length: 0,
    operation: Cpu::cp_a_e,
};

pub const CP_A_H: CpuInstruction = CpuInstruction {
    disassembly: "CP A, H",
    operand_length: 0,
    operation: Cpu::cp_a_h,
};

pub const CP_A_L: CpuInstruction = CpuInstruction {
    disassembly: "CP A, L",
    operand_length: 0,
    operation: Cpu::cp_a_l,
};

pub const CP_A_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "CP A, [HL]",
    operand_length: 0,
    operation: Cpu::cp_a_hl_indirect,
};

pub const CP_A_N8: CpuInstruction = CpuInstruction {
    disassembly: "CP A, n8",
    operand_length: 1,
    operation: Cpu::cp_a_n8,
};

// CPL
pub const CPL: CpuInstruction = CpuInstruction {
    disassembly: "CPL",
    operand_length: 0,
    operation: Cpu::cpl,
};

// DAA
pub const DAA: CpuInstruction = CpuInstruction {
    disassembly: "DAA",
    operand_length: 0,
    operation: Cpu::daa,
};

// DEC r8/n8
pub const DEC_A: CpuInstruction = CpuInstruction {
    disassembly: "DEC A",
    operand_length: 0,
    operation: Cpu::dec_a,
};

pub const DEC_B: CpuInstruction = CpuInstruction {
    disassembly: "DEC B",
    operand_length: 0,
    operation: Cpu::dec_b,
};

pub const DEC_C: CpuInstruction = CpuInstruction {
    disassembly: "DEC C",
    operand_length: 0,
    operation: Cpu::dec_c,
};

pub const DEC_D: CpuInstruction = CpuInstruction {
    disassembly: "DEC D",
    operand_length: 0,
    operation: Cpu::dec_d,
};

pub const DEC_E: CpuInstruction = CpuInstruction {
    disassembly: "DEC E",
    operand_length: 0,
    operation: Cpu::dec_e,
};

pub const DEC_H: CpuInstruction = CpuInstruction {
    disassembly: "DEC H",
    operand_length: 0,
    operation: Cpu::dec_h,
};

pub const DEC_L: CpuInstruction = CpuInstruction {
    disassembly: "DEC L",
    operand_length: 0,
    operation: Cpu::dec_l,
};

pub const DEC_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "DEC [HL]",
    operand_length: 0,
    operation: Cpu::dec_hl_indirect,
};

// DEC r16
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

// DI
pub const DI: CpuInstruction = CpuInstruction {
    disassembly: "DI",
    operand_length: 0,
    operation: Cpu::di,
};

// HALT
pub const HALT: CpuInstruction = CpuInstruction {
    disassembly: "HALT",
    operand_length: 0,
    operation: Cpu::halt,
};

// INC r8/n8
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

pub const INC_L: CpuInstruction = CpuInstruction {
    disassembly: "INC L",
    operand_length: 0,
    operation: Cpu::inc_l,
};

pub const INC_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "INC [HL]",
    operand_length: 0,
    operation: Cpu::inc_hl_indirect,
};

// INC r16
pub const INC_BC: CpuInstruction = CpuInstruction {
    disassembly: "INC BC",
    operand_length: 0,
    operation: Cpu::inc_bc,
};

pub const INC_DE: CpuInstruction = CpuInstruction {
    disassembly: "INC DE",
    operand_length: 0,
    operation: Cpu::inc_de,
};

pub const INC_HL: CpuInstruction = CpuInstruction {
    disassembly: "INC HL",
    operand_length: 0,
    operation: Cpu::inc_hl,
};

pub const INC_SP: CpuInstruction = CpuInstruction {
    disassembly: "INC SP",
    operand_length: 0,
    operation: Cpu::inc_sp,
};

// JP
pub const JP_A16: CpuInstruction = CpuInstruction {
    disassembly: "JP a16",
    operand_length: 2,
    operation: Cpu::jp_a16,
};

pub const JP_HL: CpuInstruction = CpuInstruction {
    disassembly: "JP HL",
    operand_length: 0,
    operation: Cpu::jp_hl,
};

// JR
pub const JR_E8: CpuInstruction = CpuInstruction {
    disassembly: "JR e8",
    operand_length: 1,
    operation: Cpu::jr_e8,
};

pub const JR_C_E8: CpuInstruction = CpuInstruction {
    disassembly: "JR C, e8",
    operand_length: 1,
    operation: Cpu::jr_c_e8,
};

pub const JR_NC_E8: CpuInstruction = CpuInstruction {
    disassembly: "JR NC, e8",
    operand_length: 1,
    operation: Cpu::jr_nc_e8,
};

pub const JR_Z_E8: CpuInstruction = CpuInstruction {
    disassembly: "JR Z, e8",
    operand_length: 1,
    operation: Cpu::jr_z_e8,
};

pub const JR_NZ_E8: CpuInstruction = CpuInstruction {
    disassembly: "JR NZ, e8",
    operand_length: 1,
    operation: Cpu::jr_nz_e8,
};

// LD r8, r8/n8
pub const LD_A_A: CpuInstruction = CpuInstruction {
    disassembly: "LD A, A",
    operand_length: 0,
    operation: Cpu::ld_a_a,
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

pub const LD_B_A: CpuInstruction = CpuInstruction {
    disassembly: "LD B, A",
    operand_length: 0,
    operation: Cpu::ld_b_a,
};

pub const LD_B_B: CpuInstruction = CpuInstruction {
    disassembly: "LD B, B",
    operand_length: 0,
    operation: Cpu::ld_b_b,
};

pub const LD_B_C: CpuInstruction = CpuInstruction {
    disassembly: "LD B, C",
    operand_length: 0,
    operation: Cpu::ld_b_c,
};

pub const LD_B_D: CpuInstruction = CpuInstruction {
    disassembly: "LD B, D",
    operand_length: 0,
    operation: Cpu::ld_b_d,
};

pub const LD_B_E: CpuInstruction = CpuInstruction {
    disassembly: "LD B, E",
    operand_length: 0,
    operation: Cpu::ld_b_e,
};

pub const LD_B_H: CpuInstruction = CpuInstruction {
    disassembly: "LD B, H",
    operand_length: 0,
    operation: Cpu::ld_b_h,
};

pub const LD_B_L: CpuInstruction = CpuInstruction {
    disassembly: "LD B, L",
    operand_length: 0,
    operation: Cpu::ld_b_l,
};

pub const LD_B_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD B, n8",
    operand_length: 1,
    operation: Cpu::ld_b_n8,
};

pub const LD_C_A: CpuInstruction = CpuInstruction {
    disassembly: "LD C, A",
    operand_length: 0,
    operation: Cpu::ld_c_a,
};

pub const LD_C_B: CpuInstruction = CpuInstruction {
    disassembly: "LD C, B",
    operand_length: 0,
    operation: Cpu::ld_c_b,
};

pub const LD_C_C: CpuInstruction = CpuInstruction {
    disassembly: "LD C, C",
    operand_length: 0,
    operation: Cpu::ld_c_c,
};

pub const LD_C_D: CpuInstruction = CpuInstruction {
    disassembly: "LD C, D",
    operand_length: 0,
    operation: Cpu::ld_c_d,
};

pub const LD_C_E: CpuInstruction = CpuInstruction {
    disassembly: "LD C, E",
    operand_length: 0,
    operation: Cpu::ld_c_e,
};

pub const LD_C_H: CpuInstruction = CpuInstruction {
    disassembly: "LD C, H",
    operand_length: 0,
    operation: Cpu::ld_c_h,
};

pub const LD_C_L: CpuInstruction = CpuInstruction {
    disassembly: "LD C, L",
    operand_length: 0,
    operation: Cpu::ld_c_l,
};

pub const LD_C_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD C, n8",
    operand_length: 1,
    operation: Cpu::ld_c_n8,
};

pub const LD_D_A: CpuInstruction = CpuInstruction {
    disassembly: "LD D, A",
    operand_length: 0,
    operation: Cpu::ld_d_a,
};

pub const LD_D_B: CpuInstruction = CpuInstruction {
    disassembly: "LD D, B",
    operand_length: 0,
    operation: Cpu::ld_d_b,
};

pub const LD_D_C: CpuInstruction = CpuInstruction {
    disassembly: "LD D, C",
    operand_length: 0,
    operation: Cpu::ld_d_c,
};

pub const LD_D_D: CpuInstruction = CpuInstruction {
    disassembly: "LD D, D",
    operand_length: 0,
    operation: Cpu::ld_d_d,
};

pub const LD_D_E: CpuInstruction = CpuInstruction {
    disassembly: "LD D, E",
    operand_length: 0,
    operation: Cpu::ld_d_e,
};

pub const LD_D_H: CpuInstruction = CpuInstruction {
    disassembly: "LD D, H",
    operand_length: 0,
    operation: Cpu::ld_d_h,
};

pub const LD_D_L: CpuInstruction = CpuInstruction {
    disassembly: "LD D, L",
    operand_length: 0,
    operation: Cpu::ld_d_l,
};

pub const LD_D_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD D, n8",
    operand_length: 1,
    operation: Cpu::ld_d_n8,
};

pub const LD_E_A: CpuInstruction = CpuInstruction {
    disassembly: "LD E, A",
    operand_length: 0,
    operation: Cpu::ld_e_a,
};

pub const LD_E_B: CpuInstruction = CpuInstruction {
    disassembly: "LD E, B",
    operand_length: 0,
    operation: Cpu::ld_e_b,
};

pub const LD_E_C: CpuInstruction = CpuInstruction {
    disassembly: "LD E, C",
    operand_length: 0,
    operation: Cpu::ld_e_c,
};

pub const LD_E_D: CpuInstruction = CpuInstruction {
    disassembly: "LD E, D",
    operand_length: 0,
    operation: Cpu::ld_e_d,
};

pub const LD_E_E: CpuInstruction = CpuInstruction {
    disassembly: "LD E, E",
    operand_length: 0,
    operation: Cpu::ld_e_e,
};

pub const LD_E_H: CpuInstruction = CpuInstruction {
    disassembly: "LD E, H",
    operand_length: 0,
    operation: Cpu::ld_e_h,
};

pub const LD_E_L: CpuInstruction = CpuInstruction {
    disassembly: "LD E, L",
    operand_length: 0,
    operation: Cpu::ld_e_l,
};

pub const LD_E_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD E, n8",
    operand_length: 1,
    operation: Cpu::ld_e_n8,
};

pub const LD_H_A: CpuInstruction = CpuInstruction {
    disassembly: "LD H, A",
    operand_length: 0,
    operation: Cpu::ld_h_a,
};

pub const LD_H_B: CpuInstruction = CpuInstruction {
    disassembly: "LD H, B",
    operand_length: 0,
    operation: Cpu::ld_h_b,
};

pub const LD_H_C: CpuInstruction = CpuInstruction {
    disassembly: "LD H, C",
    operand_length: 0,
    operation: Cpu::ld_h_c,
};

pub const LD_H_D: CpuInstruction = CpuInstruction {
    disassembly: "LD H, D",
    operand_length: 0,
    operation: Cpu::ld_h_d,
};

pub const LD_H_E: CpuInstruction = CpuInstruction {
    disassembly: "LD H, E",
    operand_length: 0,
    operation: Cpu::ld_h_e,
};

pub const LD_H_H: CpuInstruction = CpuInstruction {
    disassembly: "LD H, H",
    operand_length: 0,
    operation: Cpu::ld_h_h,
};

pub const LD_H_L: CpuInstruction = CpuInstruction {
    disassembly: "LD H, L",
    operand_length: 0,
    operation: Cpu::ld_h_l,
};

pub const LD_H_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD H, n8",
    operand_length: 1,
    operation: Cpu::ld_h_n8,
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

// LD [a16/r16], r8/n8
pub const LD_A16_INDIRECT_A: CpuInstruction = CpuInstruction {
    disassembly: "LD [a16], A",
    operand_length: 2,
    operation: Cpu::ld_a16_indirect_a,
};

pub const LD_A16_INDIRECT_SP: CpuInstruction = CpuInstruction {
    disassembly: "LD [a16], SP",
    operand_length: 2,
    operation: Cpu::ld_a16_indirect_sp,
};

pub const LD_BC_INDIRECT_A: CpuInstruction = CpuInstruction {
    disassembly: "LD [BC], A",
    operand_length: 0,
    operation: Cpu::ld_bc_indirect_a,
};

pub const LD_DE_INDIRECT_A: CpuInstruction = CpuInstruction {
    disassembly: "LD [DE], A",
    operand_length: 0,
    operation: Cpu::ld_de_indirect_a,
};

pub const LD_HL_INDIRECT_A: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL], A",
    operand_length: 0,
    operation: Cpu::ld_hl_indirect_a,
};

pub const LD_HL_INDIRECT_B: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL], B",
    operand_length: 0,
    operation: Cpu::ld_hl_indirect_b,
};

pub const LD_HL_INDIRECT_C: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL], C",
    operand_length: 0,
    operation: Cpu::ld_hl_indirect_c,
};

pub const LD_HL_INDIRECT_D: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL], D",
    operand_length: 0,
    operation: Cpu::ld_hl_indirect_d,
};

pub const LD_HL_INDIRECT_E: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL], E",
    operand_length: 0,
    operation: Cpu::ld_hl_indirect_e,
};

pub const LD_HL_INDIRECT_H: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL], H",
    operand_length: 0,
    operation: Cpu::ld_hl_indirect_h,
};

pub const LD_HL_INDIRECT_L: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL], L",
    operand_length: 0,
    operation: Cpu::ld_hl_indirect_l,
};

pub const LD_HL_INDIRECT_N8: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL], n8",
    operand_length: 1,
    operation: Cpu::ld_hl_indirect_n8,
};

pub const LD_HLI_INDIRECT_A: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL+], A",
    operand_length: 0,
    operation: Cpu::ld_hli_indirect_a,
};

pub const LD_HLD_INDIRECT_A: CpuInstruction = CpuInstruction {
    disassembly: "LD [HL-], A",
    operand_length: 0,
    operation: Cpu::ld_hld_indirect_a,
};

// LD r8, [a16/r16]
pub const LD_A_BC_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD A, [BC]",
    operand_length: 0,
    operation: Cpu::ld_a_bc_indirect,
};

pub const LD_A_DE_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD A, [DE]",
    operand_length: 0,
    operation: Cpu::ld_a_de_indirect,
};

pub const LD_A_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD A, [HL]",
    operand_length: 0,
    operation: Cpu::ld_a_hl_indirect,
};

pub const LD_A_HLI_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD A, [HL+]",
    operand_length: 0,
    operation: Cpu::ld_a_hli_indirect,
};

pub const LD_A_A16_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD A, [a16]",
    operand_length: 2,
    operation: Cpu::ld_a_a16_indirect,
};

pub const LD_A_HLD_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD A, [HL-]",
    operand_length: 0,
    operation: Cpu::ld_a_hld_indirect,
};

pub const LD_B_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD B, [HL]",
    operand_length: 0,
    operation: Cpu::ld_b_hl_indirect,
};

pub const LD_C_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD C, [HL]",
    operand_length: 0,
    operation: Cpu::ld_c_hl_indirect,
};

pub const LD_D_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD D, [HL]",
    operand_length: 0,
    operation: Cpu::ld_d_hl_indirect,
};

pub const LD_E_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD E, [HL]",
    operand_length: 0,
    operation: Cpu::ld_e_hl_indirect,
};

pub const LD_H_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD H, [HL]",
    operand_length: 0,
    operation: Cpu::ld_h_hl_indirect,
};

pub const LD_L_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LD L, [HL]",
    operand_length: 0,
    operation: Cpu::ld_l_hl_indirect,
};

// LD r16, n16
pub const LD_BC_N16: CpuInstruction = CpuInstruction {
    disassembly: "LD BC, n16",
    operand_length: 2,
    operation: Cpu::ld_bc_n16,
};

pub const LD_DE_N16: CpuInstruction = CpuInstruction {
    disassembly: "LD DE, n16",
    operand_length: 2,
    operation: Cpu::ld_de_n16,
};

pub const LD_HL_N16: CpuInstruction = CpuInstruction {
    disassembly: "LD HL, n16",
    operand_length: 2,
    operation: Cpu::ld_hl_n16,
};

pub const LD_SP_N16: CpuInstruction = CpuInstruction {
    disassembly: "LD SP, n16",
    operand_length: 2,
    operation: Cpu::ld_sp_n16,
};

// LDH
pub const LDH_A8_INDIRECT_A: CpuInstruction = CpuInstruction {
    disassembly: "LDH [a8], A",
    operand_length: 1,
    operation: Cpu::ldh_a8_indirect_a,
};

pub const LDH_A_A8_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LDH A, [a8]",
    operand_length: 1,
    operation: Cpu::ldh_a_a8_indirect,
};

pub const LDH_C_INDIRECT_A: CpuInstruction = CpuInstruction {
    disassembly: "LDH [C], A",
    operand_length: 1,
    operation: Cpu::ldh_c_indirect_a,
};

pub const LDH_A_C_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "LDH A, [C]",
    operand_length: 1,
    operation: Cpu::ldh_a_c_indirect,
};

// NOOP
pub const NOOP: CpuInstruction = CpuInstruction {
    disassembly: "NOP",
    operand_length: 0,
    operation: Cpu::noop,
};

// OR R8, R8/n8
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

pub const OR_A_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "OR A, [HL]",
    operand_length: 0,
    operation: Cpu::or_a_hl_indirect,
};

pub const OR_A_N8: CpuInstruction = CpuInstruction {
    disassembly: "OR A, n8",
    operand_length: 1,
    operation: Cpu::or_a_n8,
};

// POP
pub const POP_AF: CpuInstruction = CpuInstruction {
    disassembly: "POP AF",
    operand_length: 0,
    operation: Cpu::pop_af,
};

pub const POP_BC: CpuInstruction = CpuInstruction {
    disassembly: "POP BC",
    operand_length: 0,
    operation: Cpu::pop_bc,
};

pub const POP_DE: CpuInstruction = CpuInstruction {
    disassembly: "POP DE",
    operand_length: 0,
    operation: Cpu::pop_de,
};

pub const POP_HL: CpuInstruction = CpuInstruction {
    disassembly: "POP HL",
    operand_length: 0,
    operation: Cpu::pop_hl,
};

// RET
pub const RET: CpuInstruction = CpuInstruction {
    disassembly: "RET",
    operand_length: 0,
    operation: Cpu::ret,
};

pub const RET_Z: CpuInstruction = CpuInstruction {
    disassembly: "RET Z",
    operand_length: 0,
    operation: Cpu::ret_z,
};

pub const RET_NZ: CpuInstruction = CpuInstruction {
    disassembly: "RET NZ",
    operand_length: 0,
    operation: Cpu::ret_nz,
};

pub const RET_C: CpuInstruction = CpuInstruction {
    disassembly: "RET C",
    operand_length: 0,
    operation: Cpu::ret_c,
};

pub const RET_NC: CpuInstruction = CpuInstruction {
    disassembly: "RET NC",
    operand_length: 0,
    operation: Cpu::ret_nc,
};

// Rotate Register
pub const RLA: CpuInstruction = CpuInstruction {
    disassembly: "RLA",
    operand_length: 0,
    operation: Cpu::rla,
};

pub const RLCA: CpuInstruction = CpuInstruction {
    disassembly: "RLCA",
    operand_length: 0,
    operation: Cpu::rlca,
};

pub const RRA: CpuInstruction = CpuInstruction {
    disassembly: "RRA",
    operand_length: 0,
    operation: Cpu::rra,
};

pub const RRCA: CpuInstruction = CpuInstruction {
    disassembly: "RRCA",
    operand_length: 0,
    operation: Cpu::rrca,
};

// STOP
pub const STOP_N8: CpuInstruction = CpuInstruction {
    disassembly: "STOP n8",
    operand_length: 1,
    operation: Cpu::stop_n8,
};

// SUB
pub const SUB_A_A: CpuInstruction = CpuInstruction {
    disassembly: "SUB A, A",
    operand_length: 0,
    operation: Cpu::sub_a_a,
};

pub const SUB_A_B: CpuInstruction = CpuInstruction {
    disassembly: "SUB A, B",
    operand_length: 0,
    operation: Cpu::sub_a_b,
};

pub const SUB_A_C: CpuInstruction = CpuInstruction {
    disassembly: "SUB A, C",
    operand_length: 0,
    operation: Cpu::sub_a_c,
};

pub const SUB_A_D: CpuInstruction = CpuInstruction {
    disassembly: "SUB A, D",
    operand_length: 0,
    operation: Cpu::sub_a_d,
};

pub const SUB_A_E: CpuInstruction = CpuInstruction {
    disassembly: "SUB A, E",
    operand_length: 0,
    operation: Cpu::sub_a_e,
};

pub const SUB_A_H: CpuInstruction = CpuInstruction {
    disassembly: "SUB A, H",
    operand_length: 0,
    operation: Cpu::sub_a_h,
};

pub const SUB_A_L: CpuInstruction = CpuInstruction {
    disassembly: "SUB A, L",
    operand_length: 0,
    operation: Cpu::sub_a_l,
};

pub const SUB_A_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SUB A, [HL]",
    operand_length: 0,
    operation: Cpu::sub_a_hl_indirect,
};

pub const SUB_A_N8: CpuInstruction = CpuInstruction {
    disassembly: "SUB A, n8",
    operand_length: 1,
    operation: Cpu::sub_a_n8,
};

// SBC
pub const SBC_A_A: CpuInstruction = CpuInstruction {
    disassembly: "SBC A, A",
    operand_length: 0,
    operation: Cpu::sbc_a_a,
};

pub const SBC_A_B: CpuInstruction = CpuInstruction {
    disassembly: "SBC A, B",
    operand_length: 0,
    operation: Cpu::sbc_a_b,
};

pub const SBC_A_C: CpuInstruction = CpuInstruction {
    disassembly: "SBC A, C",
    operand_length: 0,
    operation: Cpu::sbc_a_c,
};

pub const SBC_A_D: CpuInstruction = CpuInstruction {
    disassembly: "SBC A, D",
    operand_length: 0,
    operation: Cpu::sbc_a_d,
};

pub const SBC_A_E: CpuInstruction = CpuInstruction {
    disassembly: "SBC A, E",
    operand_length: 0,
    operation: Cpu::sbc_a_e,
};

pub const SBC_A_H: CpuInstruction = CpuInstruction {
    disassembly: "SBC A, H",
    operand_length: 0,
    operation: Cpu::sbc_a_h,
};

pub const SBC_A_L: CpuInstruction = CpuInstruction {
    disassembly: "SBC A, L",
    operand_length: 0,
    operation: Cpu::sbc_a_l,
};

pub const SBC_A_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SBC A, [HL]",
    operand_length: 0,
    operation: Cpu::sbc_a_hl_indirect,
};

pub const SBC_A_N8: CpuInstruction = CpuInstruction {
    disassembly: "SBC A, n8",
    operand_length: 1,
    operation: Cpu::sbc_a_n8,
};

// XOR A, r8/n8
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
        /* 0x02 */ LD_BC_INDIRECT_A,
        /* 0x03 */ INC_BC,
        /* 0x04 */ INC_B,
        /* 0x05 */ DEC_B,
        /* 0x06 */ LD_B_N8,
        /* 0x07 */ RLCA,
        /* 0x08 */ LD_A16_INDIRECT_SP,
        /* 0x09 */ ADD_HL_BC,
        /* 0x0A */ LD_A_BC_INDIRECT,
        /* 0x0B */ DEC_BC,
        /* 0x0C */ INC_C,
        /* 0x0D */ DEC_C,
        /* 0x0E */ LD_C_N8,
        /* 0x0F */ RRCA,
        /* 0x10 */ STOP_N8,
        /* 0x11 */ LD_DE_N16,
        /* 0x12 */ LD_DE_INDIRECT_A,
        /* 0x13 */ INC_DE,
        /* 0x14 */ INC_D,
        /* 0x15 */ DEC_D,
        /* 0x16 */ LD_D_N8,
        /* 0x17 */ RLA,
        /* 0x18 */ JR_E8,
        /* 0x19 */ ADD_HL_DE,
        /* 0x1A */ LD_A_DE_INDIRECT,
        /* 0x1B */ DEC_DE,
        /* 0x1C */ INC_E,
        /* 0x1D */ DEC_E,
        /* 0x1E */ LD_E_N8,
        /* 0x1F */ RRA,
        /* 0x20 */ JR_NZ_E8,
        /* 0x21 */ LD_HL_N16,
        /* 0x22 */ LD_HLI_INDIRECT_A,
        /* 0x23 */ INC_HL,
        /* 0x24 */ INC_H,
        /* 0x25 */ DEC_H,
        /* 0x26 */ LD_H_N8,
        /* 0x27 */ DAA,
        /* 0x28 */ JR_Z_E8,
        /* 0x29 */ ADD_HL_HL,
        /* 0x2A */ LD_A_HLI_INDIRECT,
        /* 0x2B */ DEC_HL,
        /* 0x2C */ INC_L,
        /* 0x2D */ DEC_L,
        /* 0x2E */ LD_L_N8,
        /* 0x2F */ CPL,
        /* 0x30 */ JR_NC_E8,
        /* 0x31 */ LD_SP_N16,
        /* 0x32 */ LD_HLD_INDIRECT_A,
        /* 0x33 */ INC_SP,
        /* 0x34 */ INC_HL_INDIRECT,
        /* 0x35 */ DEC_HL_INDIRECT,
        /* 0x36 */ LD_HL_INDIRECT_N8,
        /* 0x37 */ SCF,
        /* 0x38 */ JR_C_E8,
        /* 0x39 */ ADD_HL_SP,
        /* 0x3A */ LD_A_HLD_INDIRECT,
        /* 0x3B */ DEC_SP,
        /* 0x3C */ INC_A,
        /* 0x3D */ DEC_A,
        /* 0x3E */ LD_A_N8,
        /* 0x3F */ CCF,
        /* 0x40 */ LD_B_B,
        /* 0x41 */ LD_B_C,
        /* 0x42 */ LD_B_D,
        /* 0x43 */ LD_B_E,
        /* 0x44 */ LD_B_H,
        /* 0x45 */ LD_B_L,
        /* 0x46 */ LD_B_HL_INDIRECT,
        /* 0x47 */ LD_B_A,
        /* 0x48 */ LD_C_B,
        /* 0x49 */ LD_C_C,
        /* 0x4A */ LD_C_D,
        /* 0x4B */ LD_C_E,
        /* 0x4C */ LD_C_H,
        /* 0x4D */ LD_C_L,
        /* 0x4E */ LD_C_HL_INDIRECT,
        /* 0x4F */ LD_C_A,
        /* 0x50 */ LD_D_B,
        /* 0x51 */ LD_D_C,
        /* 0x52 */ LD_D_D,
        /* 0x53 */ LD_D_E,
        /* 0x54 */ LD_D_H,
        /* 0x55 */ LD_D_L,
        /* 0x56 */ LD_D_HL_INDIRECT,
        /* 0x57 */ LD_D_A,
        /* 0x58 */ LD_E_B,
        /* 0x59 */ LD_E_C,
        /* 0x5A */ LD_E_D,
        /* 0x5B */ LD_E_E,
        /* 0x5C */ LD_E_H,
        /* 0x5D */ LD_E_L,
        /* 0x5E */ LD_E_HL_INDIRECT,
        /* 0x5F */ LD_E_A,
        /* 0x60 */ LD_H_B,
        /* 0x61 */ LD_H_C,
        /* 0x62 */ LD_H_D,
        /* 0x63 */ LD_H_E,
        /* 0x64 */ LD_H_H,
        /* 0x65 */ LD_H_L,
        /* 0x66 */ LD_H_HL_INDIRECT,
        /* 0x67 */ LD_H_A,
        /* 0x68 */ LD_L_B,
        /* 0x69 */ LD_L_C,
        /* 0x6A */ LD_L_D,
        /* 0x6B */ LD_L_E,
        /* 0x6C */ LD_L_H,
        /* 0x6D */ LD_L_L,
        /* 0x6E */ LD_L_HL_INDIRECT,
        /* 0x6F */ LD_L_A,
        /* 0x70 */ LD_HL_INDIRECT_B,
        /* 0x71 */ LD_HL_INDIRECT_C,
        /* 0x72 */ LD_HL_INDIRECT_D,
        /* 0x73 */ LD_HL_INDIRECT_E,
        /* 0x74 */ LD_HL_INDIRECT_H,
        /* 0x75 */ LD_HL_INDIRECT_L,
        /* 0x76 */ HALT,
        /* 0x77 */ LD_HL_INDIRECT_A,
        /* 0x78 */ LD_A_B,
        /* 0x79 */ LD_A_C,
        /* 0x7A */ LD_A_D,
        /* 0x7B */ LD_A_E,
        /* 0x7C */ LD_A_H,
        /* 0x7D */ LD_A_L,
        /* 0x7E */ LD_A_HL_INDIRECT,
        /* 0x7F */ LD_A_A,
        /* 0x80 */ ADD_A_B,
        /* 0x81 */ ADD_A_C,
        /* 0x82 */ ADD_A_D,
        /* 0x83 */ ADD_A_E,
        /* 0x84 */ ADD_A_H,
        /* 0x85 */ ADD_A_L,
        /* 0x86 */ ADD_A_HL_INDIRECT,
        /* 0x87 */ ADD_A_A,
        /* 0x88 */ ADC_A_B,
        /* 0x89 */ ADC_A_C,
        /* 0x8A */ ADC_A_D,
        /* 0x8B */ ADC_A_E,
        /* 0x8C */ ADC_A_H,
        /* 0x8D */ ADC_A_L,
        /* 0x8E */ ADC_A_HL_INDIRECT,
        /* 0x8F */ ADC_A_A,
        /* 0x90 */ SUB_A_B,
        /* 0x91 */ SUB_A_C,
        /* 0x92 */ SUB_A_D,
        /* 0x93 */ SUB_A_E,
        /* 0x94 */ SUB_A_H,
        /* 0x95 */ SUB_A_L,
        /* 0x96 */ SUB_A_HL_INDIRECT,
        /* 0x97 */ SUB_A_A,
        /* 0x98 */ SBC_A_B,
        /* 0x99 */ SBC_A_C,
        /* 0x9A */ SBC_A_D,
        /* 0x9B */ SBC_A_E,
        /* 0x9C */ SBC_A_H,
        /* 0x9D */ SBC_A_L,
        /* 0x9E */ SBC_A_HL_INDIRECT,
        /* 0x9F */ SBC_A_A,
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
        /* 0xB6 */ OR_A_HL_INDIRECT,
        /* 0xB7 */ OR_A_A,
        /* 0xB8 */ CP_A_B,
        /* 0xB9 */ CP_A_C,
        /* 0xBA */ CP_A_D,
        /* 0xBB */ CP_A_E,
        /* 0xBC */ CP_A_H,
        /* 0xBD */ CP_A_L,
        /* 0xBE */ CP_A_HL_INDIRECT,
        /* 0xBF */ CP_A_A,
        /* 0xC0 */ RET_NZ,
        /* 0xC1 */ POP_BC,
        /* 0xC2 */ PLACEHOLDER,
        /* 0xC3 */ JP_A16,
        /* 0xC4 */ PLACEHOLDER,
        /* 0xC5 */ PLACEHOLDER,
        /* 0xC6 */ ADD_A_N8,
        /* 0xC7 */ PLACEHOLDER,
        /* 0xC8 */ RET_Z,
        /* 0xC9 */ RET,
        /* 0xCA */ PLACEHOLDER,
        /* 0xCB */ PLACEHOLDER,
        /* 0xCC */ PLACEHOLDER,
        /* 0xCD */ CALL_A16,
        /* 0xCE */ ADC_A_N8,
        /* 0xCF */ PLACEHOLDER,
        /* 0xD0 */ RET_NC,
        /* 0xD1 */ POP_DE,
        /* 0xD2 */ PLACEHOLDER,
        /* 0xD3 */ PLACEHOLDER,
        /* 0xD4 */ PLACEHOLDER,
        /* 0xD5 */ PLACEHOLDER,
        /* 0xD6 */ SUB_A_N8,
        /* 0xD7 */ PLACEHOLDER,
        /* 0xD8 */ RET_C,
        /* 0xD9 */ PLACEHOLDER,
        /* 0xDA */ PLACEHOLDER,
        /* 0xDB */ PLACEHOLDER,
        /* 0xDC */ PLACEHOLDER,
        /* 0xDD */ PLACEHOLDER,
        /* 0xDE */ SBC_A_N8,
        /* 0xDF */ PLACEHOLDER,
        /* 0xE0 */ LDH_A8_INDIRECT_A,
        /* 0xE1 */ POP_HL,
        /* 0xE2 */ LDH_C_INDIRECT_A,
        /* 0xE3 */ PLACEHOLDER,
        /* 0xE4 */ PLACEHOLDER,
        /* 0xE5 */ PLACEHOLDER,
        /* 0xE6 */ PLACEHOLDER,
        /* 0xE7 */ PLACEHOLDER,
        /* 0xE8 */ PLACEHOLDER,
        /* 0xE9 */ JP_HL,
        /* 0xEA */ LD_A16_INDIRECT_A,
        /* 0xEB */ PLACEHOLDER,
        /* 0xEC */ PLACEHOLDER,
        /* 0xED */ PLACEHOLDER,
        /* 0xEE */ PLACEHOLDER,
        /* 0xEF */ PLACEHOLDER,
        /* 0xF0 */ LDH_A_A8_INDIRECT,
        /* 0xF1 */ POP_AF,
        /* 0xF2 */ LDH_A_C_INDIRECT,
        /* 0xF3 */ DI,
        /* 0xF4 */ PLACEHOLDER,
        /* 0xF5 */ PLACEHOLDER,
        /* 0xF6 */ OR_A_N8,
        /* 0xF7 */ PLACEHOLDER,
        /* 0xF8 */ PLACEHOLDER,
        /* 0xF9 */ PLACEHOLDER,
        /* 0xFA */ LD_A_A16_INDIRECT,
        /* 0xFB */ PLACEHOLDER,
        /* 0xFC */ PLACEHOLDER,
        /* 0xFD */ PLACEHOLDER,
        /* 0xFE */ CP_A_N8,
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
