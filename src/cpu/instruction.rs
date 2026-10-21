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

// AND
pub const AND_A_A: CpuInstruction = CpuInstruction {
    disassembly: "AND A, A",
    operand_length: 0,
    operation: Cpu::and_a_a,
};

pub const AND_A_B: CpuInstruction = CpuInstruction {
    disassembly: "AND A, B",
    operand_length: 0,
    operation: Cpu::and_a_b,
};

pub const AND_A_C: CpuInstruction = CpuInstruction {
    disassembly: "AND A, C",
    operand_length: 0,
    operation: Cpu::and_a_c,
};

pub const AND_A_D: CpuInstruction = CpuInstruction {
    disassembly: "AND A, D",
    operand_length: 0,
    operation: Cpu::and_a_d,
};

pub const AND_A_E: CpuInstruction = CpuInstruction {
    disassembly: "AND A, E",
    operand_length: 0,
    operation: Cpu::and_a_e,
};

pub const AND_A_H: CpuInstruction = CpuInstruction {
    disassembly: "AND A, H",
    operand_length: 0,
    operation: Cpu::and_a_h,
};

pub const AND_A_L: CpuInstruction = CpuInstruction {
    disassembly: "AND A, L",
    operand_length: 0,
    operation: Cpu::and_a_l,
};

pub const AND_A_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "AND A, [HL]",
    operand_length: 0,
    operation: Cpu::and_a_hl_indirect,
};

pub const AND_A_N8: CpuInstruction = CpuInstruction {
    disassembly: "AND A, n8",
    operand_length: 1,
    operation: Cpu::and_a_n8,
};

// CALL
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

pub const JP_C_A16: CpuInstruction = CpuInstruction {
    disassembly: "JP C, a16",
    operand_length: 0,
    operation: Cpu::jp_c_a16,
};

pub const JP_NC_A16: CpuInstruction = CpuInstruction {
    disassembly: "JP NC, a16",
    operand_length: 0,
    operation: Cpu::jp_nc_a16,
};

pub const JP_Z_A16: CpuInstruction = CpuInstruction {
    disassembly: "JP Z, a16",
    operand_length: 0,
    operation: Cpu::jp_z_a16,
};

pub const JP_NZ_A16: CpuInstruction = CpuInstruction {
    disassembly: "JP NZ, a16",
    operand_length: 0,
    operation: Cpu::jp_nz_a16,
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
        /* 0xA0 */ AND_A_B,
        /* 0xA1 */ AND_A_C,
        /* 0xA2 */ AND_A_D,
        /* 0xA3 */ AND_A_E,
        /* 0xA4 */ AND_A_H,
        /* 0xA5 */ AND_A_L,
        /* 0xA6 */ AND_A_HL_INDIRECT,
        /* 0xA7 */ AND_A_A,
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
        /* 0xC2 */ JP_NZ_A16,
        /* 0xC3 */ JP_A16,
        /* 0xC4 */ PLACEHOLDER,
        /* 0xC5 */ PLACEHOLDER,
        /* 0xC6 */ ADD_A_N8,
        /* 0xC7 */ PLACEHOLDER,
        /* 0xC8 */ RET_Z,
        /* 0xC9 */ RET,
        /* 0xCA */ JP_Z_A16,
        /* 0xCB */ PLACEHOLDER,
        /* 0xCC */ PLACEHOLDER,
        /* 0xCD */ CALL_A16,
        /* 0xCE */ ADC_A_N8,
        /* 0xCF */ PLACEHOLDER,
        /* 0xD0 */ RET_NC,
        /* 0xD1 */ POP_DE,
        /* 0xD2 */ JP_NC_A16,
        /* 0xD3 */ PLACEHOLDER,
        /* 0xD4 */ PLACEHOLDER,
        /* 0xD5 */ PLACEHOLDER,
        /* 0xD6 */ SUB_A_N8,
        /* 0xD7 */ PLACEHOLDER,
        /* 0xD8 */ RET_C,
        /* 0xD9 */ PLACEHOLDER,
        /* 0xDA */ JP_C_A16,
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
        /* 0xE6 */ AND_A_N8,
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
        /* 0x00 */ RLC_B,
        /* 0x01 */ RLC_C,
        /* 0x02 */ RLC_D,
        /* 0x03 */ RLC_E,
        /* 0x04 */ RLC_H,
        /* 0x05 */ RLC_L,
        /* 0x06 */ RLC_HL_INDIRECT,
        /* 0x07 */ RLC_A,
        /* 0x08 */ RRC_B,
        /* 0x09 */ RRC_C,
        /* 0x0A */ RRC_D,
        /* 0x0B */ RRC_E,
        /* 0x0C */ RRC_H,
        /* 0x0D */ RRC_L,
        /* 0x0E */ RRC_HL_INDIRECT,
        /* 0x0F */ RRC_A,
        /* 0x10 */ RL_B,
        /* 0x11 */ RL_C,
        /* 0x12 */ RL_D,
        /* 0x13 */ RL_E,
        /* 0x14 */ RL_H,
        /* 0x15 */ RL_L,
        /* 0x16 */ RL_HL_INDIRECT,
        /* 0x17 */ RL_A,
        /* 0x18 */ RR_B,
        /* 0x19 */ RR_C,
        /* 0x1A */ RR_D,
        /* 0x1B */ RR_E,
        /* 0x1C */ RR_H,
        /* 0x1D */ RR_L,
        /* 0x1E */ RR_HL_INDIRECT,
        /* 0x1F */ RR_A,
        /* 0x20 */ SLA_B,
        /* 0x21 */ SLA_C,
        /* 0x22 */ SLA_D,
        /* 0x23 */ SLA_E,
        /* 0x24 */ SLA_H,
        /* 0x25 */ SLA_L,
        /* 0x26 */ SLA_HL_INDIRECT,
        /* 0x27 */ SLA_A,
        /* 0x28 */ SRA_B,
        /* 0x29 */ SRA_C,
        /* 0x2A */ SRA_D,
        /* 0x2B */ SRA_E,
        /* 0x2C */ SRA_H,
        /* 0x2D */ SRA_L,
        /* 0x2E */ SRA_HL_INDIRECT,
        /* 0x2F */ SRA_A,
        /* 0x30 */ SWAP_B,
        /* 0x31 */ SWAP_C,
        /* 0x32 */ SWAP_D,
        /* 0x33 */ SWAP_E,
        /* 0x34 */ SWAP_H,
        /* 0x35 */ SWAP_L,
        /* 0x36 */ SWAP_HL_INDIRECT,
        /* 0x37 */ SWAP_A,
        /* 0x38 */ SRL_B,
        /* 0x39 */ SRL_C,
        /* 0x3A */ SRL_D,
        /* 0x3B */ SRL_E,
        /* 0x3C */ SRL_H,
        /* 0x3D */ SRL_L,
        /* 0x3E */ SRL_HL_INDIRECT,
        /* 0x3F */ SRL_A,
        /* 0x40 */ BIT_0_B,
        /* 0x41 */ BIT_0_C,
        /* 0x42 */ BIT_0_D,
        /* 0x43 */ BIT_0_E,
        /* 0x44 */ BIT_0_H,
        /* 0x45 */ BIT_0_L,
        /* 0x46 */ BIT_0_HL_INDIRECT,
        /* 0x47 */ BIT_0_A,
        /* 0x48 */ BIT_1_B,
        /* 0x49 */ BIT_1_C,
        /* 0x4A */ BIT_1_D,
        /* 0x4B */ BIT_1_E,
        /* 0x4C */ BIT_1_H,
        /* 0x4D */ BIT_1_L,
        /* 0x4E */ BIT_1_HL_INDIRECT,
        /* 0x4F */ BIT_1_A,
        /* 0x50 */ BIT_2_B,
        /* 0x51 */ BIT_2_C,
        /* 0x52 */ BIT_2_D,
        /* 0x53 */ BIT_2_E,
        /* 0x54 */ BIT_2_H,
        /* 0x55 */ BIT_2_L,
        /* 0x56 */ BIT_2_HL_INDIRECT,
        /* 0x57 */ BIT_2_A,
        /* 0x58 */ BIT_3_B,
        /* 0x59 */ BIT_3_C,
        /* 0x5A */ BIT_3_D,
        /* 0x5B */ BIT_3_E,
        /* 0x5C */ BIT_3_H,
        /* 0x5D */ BIT_3_L,
        /* 0x5E */ BIT_3_HL_INDIRECT,
        /* 0x5F */ BIT_3_A,
        /* 0x60 */ BIT_4_B,
        /* 0x61 */ BIT_4_C,
        /* 0x62 */ BIT_4_D,
        /* 0x63 */ BIT_4_E,
        /* 0x64 */ BIT_4_H,
        /* 0x65 */ BIT_4_L,
        /* 0x66 */ BIT_4_HL_INDIRECT,
        /* 0x67 */ BIT_4_A,
        /* 0x68 */ BIT_5_B,
        /* 0x69 */ BIT_5_C,
        /* 0x6A */ BIT_5_D,
        /* 0x6B */ BIT_5_E,
        /* 0x6C */ BIT_5_H,
        /* 0x6D */ BIT_5_L,
        /* 0x6E */ BIT_5_HL_INDIRECT,
        /* 0x6F */ BIT_5_A,
        /* 0x70 */ BIT_6_B,
        /* 0x71 */ BIT_6_C,
        /* 0x72 */ BIT_6_D,
        /* 0x73 */ BIT_6_E,
        /* 0x74 */ BIT_6_H,
        /* 0x75 */ BIT_6_L,
        /* 0x76 */ BIT_6_HL_INDIRECT,
        /* 0x77 */ BIT_6_A,
        /* 0x78 */ BIT_7_B,
        /* 0x79 */ BIT_7_C,
        /* 0x7A */ BIT_7_D,
        /* 0x7B */ BIT_7_E,
        /* 0x7C */ BIT_7_H,
        /* 0x7D */ BIT_7_L,
        /* 0x7E */ BIT_7_HL_INDIRECT,
        /* 0x7F */ BIT_7_A,
        /* 0x80 */ RES_0_B,
        /* 0x81 */ RES_0_C,
        /* 0x82 */ RES_0_D,
        /* 0x83 */ RES_0_E,
        /* 0x84 */ RES_0_H,
        /* 0x85 */ RES_0_L,
        /* 0x86 */ RES_0_HL_INDIRECT,
        /* 0x87 */ RES_0_A,
        /* 0x88 */ RES_1_B,
        /* 0x89 */ RES_1_C,
        /* 0x8A */ RES_1_D,
        /* 0x8B */ RES_1_E,
        /* 0x8C */ RES_1_H,
        /* 0x8D */ RES_1_L,
        /* 0x8E */ RES_1_HL_INDIRECT,
        /* 0x8F */ RES_1_A,
        /* 0x90 */ RES_2_B,
        /* 0x91 */ RES_2_C,
        /* 0x92 */ RES_2_D,
        /* 0x93 */ RES_2_E,
        /* 0x94 */ RES_2_H,
        /* 0x95 */ RES_2_L,
        /* 0x96 */ RES_2_HL_INDIRECT,
        /* 0x97 */ RES_2_A,
        /* 0x98 */ RES_3_B,
        /* 0x99 */ RES_3_C,
        /* 0x9A */ RES_3_D,
        /* 0x9B */ RES_3_E,
        /* 0x9C */ RES_3_H,
        /* 0x9D */ RES_3_L,
        /* 0x9E */ RES_3_HL_INDIRECT,
        /* 0x9F */ RES_3_A,
        /* 0xA0 */ RES_4_B,
        /* 0xA1 */ RES_4_C,
        /* 0xA2 */ RES_4_D,
        /* 0xA3 */ RES_4_E,
        /* 0xA4 */ RES_4_H,
        /* 0xA5 */ RES_4_L,
        /* 0xA6 */ RES_4_HL_INDIRECT,
        /* 0xA7 */ RES_4_A,
        /* 0xA8 */ RES_5_B,
        /* 0xA9 */ RES_5_C,
        /* 0xAA */ RES_5_D,
        /* 0xAB */ RES_5_E,
        /* 0xAC */ RES_5_H,
        /* 0xAD */ RES_5_L,
        /* 0xAE */ RES_5_HL_INDIRECT,
        /* 0xAF */ RES_5_A,
        /* 0xB0 */ RES_6_B,
        /* 0xB1 */ RES_6_C,
        /* 0xB2 */ RES_6_D,
        /* 0xB3 */ RES_6_E,
        /* 0xB4 */ RES_6_H,
        /* 0xB5 */ RES_6_L,
        /* 0xB6 */ RES_6_HL_INDIRECT,
        /* 0xB7 */ RES_6_A,
        /* 0xB8 */ RES_7_B,
        /* 0xB9 */ RES_7_C,
        /* 0xBA */ RES_7_D,
        /* 0xBB */ RES_7_E,
        /* 0xBC */ RES_7_H,
        /* 0xBD */ RES_7_L,
        /* 0xBE */ RES_7_HL_INDIRECT,
        /* 0xBF */ RES_7_A,
        /* 0xC0 */ SET_0_B,
        /* 0xC1 */ SET_0_C,
        /* 0xC2 */ SET_0_D,
        /* 0xC3 */ SET_0_E,
        /* 0xC4 */ SET_0_H,
        /* 0xC5 */ SET_0_L,
        /* 0xC6 */ SET_0_HL_INDIRECT,
        /* 0xC7 */ SET_0_A,
        /* 0xC8 */ SET_1_B,
        /* 0xC9 */ SET_1_C,
        /* 0xCA */ SET_1_D,
        /* 0xCB */ SET_1_E,
        /* 0xCC */ SET_1_H,
        /* 0xCD */ SET_1_L,
        /* 0xCE */ SET_1_HL_INDIRECT,
        /* 0xCF */ SET_1_A,
        /* 0xD0 */ SET_2_B,
        /* 0xD1 */ SET_2_C,
        /* 0xD2 */ SET_2_D,
        /* 0xD3 */ SET_2_E,
        /* 0xD4 */ SET_2_H,
        /* 0xD5 */ SET_2_L,
        /* 0xD6 */ SET_2_HL_INDIRECT,
        /* 0xD7 */ SET_2_A,
        /* 0xD8 */ SET_3_B,
        /* 0xD9 */ SET_3_C,
        /* 0xDA */ SET_3_D,
        /* 0xDB */ SET_3_E,
        /* 0xDC */ SET_3_H,
        /* 0xDD */ SET_3_L,
        /* 0xDE */ SET_3_HL_INDIRECT,
        /* 0xDF */ SET_3_A,
        /* 0xE0 */ SET_4_B,
        /* 0xE1 */ SET_4_C,
        /* 0xE2 */ SET_4_D,
        /* 0xE3 */ SET_4_E,
        /* 0xE4 */ SET_4_H,
        /* 0xE5 */ SET_4_L,
        /* 0xE6 */ SET_4_HL_INDIRECT,
        /* 0xE7 */ SET_4_A,
        /* 0xE8 */ SET_5_B,
        /* 0xE9 */ SET_5_C,
        /* 0xEA */ SET_5_D,
        /* 0xEB */ SET_5_E,
        /* 0xEC */ SET_5_H,
        /* 0xED */ SET_5_L,
        /* 0xEE */ SET_5_HL_INDIRECT,
        /* 0xEF */ SET_5_A,
        /* 0xF0 */ SET_6_B,
        /* 0xF1 */ SET_6_C,
        /* 0xF2 */ SET_6_D,
        /* 0xF3 */ SET_6_E,
        /* 0xF4 */ SET_6_H,
        /* 0xF5 */ SET_6_L,
        /* 0xF6 */ SET_6_HL_INDIRECT,
        /* 0xF7 */ SET_6_A,
        /* 0xF8 */ SET_7_B,
        /* 0xF9 */ SET_7_C,
        /* 0xFA */ SET_7_D,
        /* 0xFB */ SET_7_E,
        /* 0xFC */ SET_7_H,
        /* 0xFD */ SET_7_L,
        /* 0xFE */ SET_7_HL_INDIRECT,
        /* 0xFF */ SET_7_A,
    ],
};

// CB Instructions
pub const RLC_A: CpuInstruction = CpuInstruction {
    disassembly: "RLC A",
    operand_length: 0,
    operation: Cpu::cb_rlc_a,
};

pub const RLC_B: CpuInstruction = CpuInstruction {
    disassembly: "RLC B",
    operand_length: 0,
    operation: Cpu::cb_rlc_b,
};

pub const RLC_C: CpuInstruction = CpuInstruction {
    disassembly: "RLC C",
    operand_length: 0,
    operation: Cpu::cb_rlc_c,
};

pub const RLC_D: CpuInstruction = CpuInstruction {
    disassembly: "RLC D",
    operand_length: 0,
    operation: Cpu::cb_rlc_d,
};

pub const RLC_E: CpuInstruction = CpuInstruction {
    disassembly: "RLC E",
    operand_length: 0,
    operation: Cpu::cb_rlc_e,
};

pub const RLC_H: CpuInstruction = CpuInstruction {
    disassembly: "RLC H",
    operand_length: 0,
    operation: Cpu::cb_rlc_h,
};

pub const RLC_L: CpuInstruction = CpuInstruction {
    disassembly: "RLC L",
    operand_length: 0,
    operation: Cpu::cb_rlc_l,
};

pub const RLC_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RLC [HL]",
    operand_length: 0,
    operation: Cpu::cb_rlc_hl_indirect,
};

pub const RRC_A: CpuInstruction = CpuInstruction {
    disassembly: "RRC A",
    operand_length: 0,
    operation: Cpu::cb_rrc_a,
};

pub const RRC_B: CpuInstruction = CpuInstruction {
    disassembly: "RRC B",
    operand_length: 0,
    operation: Cpu::cb_rrc_b,
};

pub const RRC_C: CpuInstruction = CpuInstruction {
    disassembly: "RRC C",
    operand_length: 0,
    operation: Cpu::cb_rrc_c,
};

pub const RRC_D: CpuInstruction = CpuInstruction {
    disassembly: "RRC D",
    operand_length: 0,
    operation: Cpu::cb_rrc_d,
};

pub const RRC_E: CpuInstruction = CpuInstruction {
    disassembly: "RRC E",
    operand_length: 0,
    operation: Cpu::cb_rrc_e,
};

pub const RRC_H: CpuInstruction = CpuInstruction {
    disassembly: "RRC H",
    operand_length: 0,
    operation: Cpu::cb_rrc_h,
};

pub const RRC_L: CpuInstruction = CpuInstruction {
    disassembly: "RRC L",
    operand_length: 0,
    operation: Cpu::cb_rrc_l,
};

pub const RRC_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RRC [HL]",
    operand_length: 0,
    operation: Cpu::cb_rrc_hl_indirect,
};

pub const RL_A: CpuInstruction = CpuInstruction {
    disassembly: "RL A",
    operand_length: 0,
    operation: Cpu::cb_rl_a,
};

pub const RL_B: CpuInstruction = CpuInstruction {
    disassembly: "RL B",
    operand_length: 0,
    operation: Cpu::cb_rl_b,
};

pub const RL_C: CpuInstruction = CpuInstruction {
    disassembly: "RL C",
    operand_length: 0,
    operation: Cpu::cb_rl_c,
};

pub const RL_D: CpuInstruction = CpuInstruction {
    disassembly: "RL D",
    operand_length: 0,
    operation: Cpu::cb_rl_d,
};

pub const RL_E: CpuInstruction = CpuInstruction {
    disassembly: "RL E",
    operand_length: 0,
    operation: Cpu::cb_rl_e,
};

pub const RL_H: CpuInstruction = CpuInstruction {
    disassembly: "RL H",
    operand_length: 0,
    operation: Cpu::cb_rl_h,
};

pub const RL_L: CpuInstruction = CpuInstruction {
    disassembly: "RL L",
    operand_length: 0,
    operation: Cpu::cb_rl_l,
};

pub const RL_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RL [HL]",
    operand_length: 0,
    operation: Cpu::cb_rl_hl_indirect,
};

pub const RR_A: CpuInstruction = CpuInstruction {
    disassembly: "RR A",
    operand_length: 0,
    operation: Cpu::cb_rr_a,
};

pub const RR_B: CpuInstruction = CpuInstruction {
    disassembly: "RR B",
    operand_length: 0,
    operation: Cpu::cb_rr_b,
};

pub const RR_C: CpuInstruction = CpuInstruction {
    disassembly: "RR C",
    operand_length: 0,
    operation: Cpu::cb_rr_c,
};

pub const RR_D: CpuInstruction = CpuInstruction {
    disassembly: "RR D",
    operand_length: 0,
    operation: Cpu::cb_rr_d,
};

pub const RR_E: CpuInstruction = CpuInstruction {
    disassembly: "RR E",
    operand_length: 0,
    operation: Cpu::cb_rr_e,
};

pub const RR_H: CpuInstruction = CpuInstruction {
    disassembly: "RR H",
    operand_length: 0,
    operation: Cpu::cb_rr_h,
};

pub const RR_L: CpuInstruction = CpuInstruction {
    disassembly: "RR L",
    operand_length: 0,
    operation: Cpu::cb_rr_l,
};

pub const RR_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RR [HL]",
    operand_length: 0,
    operation: Cpu::cb_rr_hl_indirect,
};

pub const SLA_A: CpuInstruction = CpuInstruction {
    disassembly: "SLA A",
    operand_length: 0,
    operation: Cpu::cb_sla_a,
};

pub const SLA_B: CpuInstruction = CpuInstruction {
    disassembly: "SLA B",
    operand_length: 0,
    operation: Cpu::cb_sla_b,
};

pub const SLA_C: CpuInstruction = CpuInstruction {
    disassembly: "SLA C",
    operand_length: 0,
    operation: Cpu::cb_sla_c,
};

pub const SLA_D: CpuInstruction = CpuInstruction {
    disassembly: "SLA D",
    operand_length: 0,
    operation: Cpu::cb_sla_d,
};

pub const SLA_E: CpuInstruction = CpuInstruction {
    disassembly: "SLA E",
    operand_length: 0,
    operation: Cpu::cb_sla_e,
};

pub const SLA_H: CpuInstruction = CpuInstruction {
    disassembly: "SLA H",
    operand_length: 0,
    operation: Cpu::cb_sla_h,
};

pub const SLA_L: CpuInstruction = CpuInstruction {
    disassembly: "SLA L",
    operand_length: 0,
    operation: Cpu::cb_sla_l,
};

pub const SLA_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SLA [HL]",
    operand_length: 0,
    operation: Cpu::cb_sla_hl_indirect,
};

pub const SRA_A: CpuInstruction = CpuInstruction {
    disassembly: "SRA A",
    operand_length: 0,
    operation: Cpu::cb_sra_a,
};

pub const SRA_B: CpuInstruction = CpuInstruction {
    disassembly: "SRA B",
    operand_length: 0,
    operation: Cpu::cb_sra_b,
};

pub const SRA_C: CpuInstruction = CpuInstruction {
    disassembly: "SRA C",
    operand_length: 0,
    operation: Cpu::cb_sra_c,
};

pub const SRA_D: CpuInstruction = CpuInstruction {
    disassembly: "SRA D",
    operand_length: 0,
    operation: Cpu::cb_sra_d,
};

pub const SRA_E: CpuInstruction = CpuInstruction {
    disassembly: "SRA E",
    operand_length: 0,
    operation: Cpu::cb_sra_e,
};

pub const SRA_H: CpuInstruction = CpuInstruction {
    disassembly: "SRA H",
    operand_length: 0,
    operation: Cpu::cb_sra_h,
};

pub const SRA_L: CpuInstruction = CpuInstruction {
    disassembly: "SRA L",
    operand_length: 0,
    operation: Cpu::cb_sra_l,
};

pub const SRA_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SRA [HL]",
    operand_length: 0,
    operation: Cpu::cb_sra_hl_indirect,
};

pub const SWAP_A: CpuInstruction = CpuInstruction {
    disassembly: "SWAP A",
    operand_length: 0,
    operation: Cpu::cb_swap_a,
};

pub const SWAP_B: CpuInstruction = CpuInstruction {
    disassembly: "SWAP B",
    operand_length: 0,
    operation: Cpu::cb_swap_b,
};

pub const SWAP_C: CpuInstruction = CpuInstruction {
    disassembly: "SWAP C",
    operand_length: 0,
    operation: Cpu::cb_swap_c,
};

pub const SWAP_D: CpuInstruction = CpuInstruction {
    disassembly: "SWAP D",
    operand_length: 0,
    operation: Cpu::cb_swap_d,
};

pub const SWAP_E: CpuInstruction = CpuInstruction {
    disassembly: "SWAP E",
    operand_length: 0,
    operation: Cpu::cb_swap_e,
};

pub const SWAP_H: CpuInstruction = CpuInstruction {
    disassembly: "SWAP H",
    operand_length: 0,
    operation: Cpu::cb_swap_h,
};

pub const SWAP_L: CpuInstruction = CpuInstruction {
    disassembly: "SWAP L",
    operand_length: 0,
    operation: Cpu::cb_swap_l,
};

pub const SWAP_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SWAP [HL]",
    operand_length: 0,
    operation: Cpu::cb_swap_hl_indirect,
};

pub const SRL_A: CpuInstruction = CpuInstruction {
    disassembly: "SRL A",
    operand_length: 0,
    operation: Cpu::cb_srl_a,
};

pub const SRL_B: CpuInstruction = CpuInstruction {
    disassembly: "SRL B",
    operand_length: 0,
    operation: Cpu::cb_srl_b,
};

pub const SRL_C: CpuInstruction = CpuInstruction {
    disassembly: "SRL C",
    operand_length: 0,
    operation: Cpu::cb_srl_c,
};

pub const SRL_D: CpuInstruction = CpuInstruction {
    disassembly: "SRL D",
    operand_length: 0,
    operation: Cpu::cb_srl_d,
};

pub const SRL_E: CpuInstruction = CpuInstruction {
    disassembly: "SRL E",
    operand_length: 0,
    operation: Cpu::cb_srl_e,
};

pub const SRL_H: CpuInstruction = CpuInstruction {
    disassembly: "SRL H",
    operand_length: 0,
    operation: Cpu::cb_srl_h,
};

pub const SRL_L: CpuInstruction = CpuInstruction {
    disassembly: "SRL L",
    operand_length: 0,
    operation: Cpu::cb_srl_l,
};

pub const SRL_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SRL [HL]",
    operand_length: 0,
    operation: Cpu::cb_srl_hl_indirect,
};

pub const BIT_0_B: CpuInstruction = CpuInstruction {
    disassembly: "BIT 0, B",
    operand_length: 0,
    operation: Cpu::cb_bit_0_b,
};

pub const BIT_0_C: CpuInstruction = CpuInstruction {
    disassembly: "BIT 0, C",
    operand_length: 0,
    operation: Cpu::cb_bit_0_c,
};

pub const BIT_0_D: CpuInstruction = CpuInstruction {
    disassembly: "BIT 0, D",
    operand_length: 0,
    operation: Cpu::cb_bit_0_d,
};

pub const BIT_0_E: CpuInstruction = CpuInstruction {
    disassembly: "BIT 0, E",
    operand_length: 0,
    operation: Cpu::cb_bit_0_e,
};

pub const BIT_0_H: CpuInstruction = CpuInstruction {
    disassembly: "BIT 0, H",
    operand_length: 0,
    operation: Cpu::cb_bit_0_h,
};

pub const BIT_0_L: CpuInstruction = CpuInstruction {
    disassembly: "BIT 0, L",
    operand_length: 0,
    operation: Cpu::cb_bit_0_l,
};

pub const BIT_0_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "BIT 0, [HL]",
    operand_length: 0,
    operation: Cpu::cb_bit_0_hl_indirect,
};

pub const BIT_0_A: CpuInstruction = CpuInstruction {
    disassembly: "BIT 0, A",
    operand_length: 0,
    operation: Cpu::cb_bit_0_a,
};

pub const BIT_1_B: CpuInstruction = CpuInstruction {
    disassembly: "BIT 1, B",
    operand_length: 0,
    operation: Cpu::cb_bit_1_b,
};

pub const BIT_1_C: CpuInstruction = CpuInstruction {
    disassembly: "BIT 1, C",
    operand_length: 0,
    operation: Cpu::cb_bit_1_c,
};

pub const BIT_1_D: CpuInstruction = CpuInstruction {
    disassembly: "BIT 1, D",
    operand_length: 0,
    operation: Cpu::cb_bit_1_d,
};

pub const BIT_1_E: CpuInstruction = CpuInstruction {
    disassembly: "BIT 1, E",
    operand_length: 0,
    operation: Cpu::cb_bit_1_e,
};

pub const BIT_1_H: CpuInstruction = CpuInstruction {
    disassembly: "BIT 1, H",
    operand_length: 0,
    operation: Cpu::cb_bit_1_h,
};

pub const BIT_1_L: CpuInstruction = CpuInstruction {
    disassembly: "BIT 1, L",
    operand_length: 0,
    operation: Cpu::cb_bit_1_l,
};

pub const BIT_1_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "BIT 1, [HL]",
    operand_length: 0,
    operation: Cpu::cb_bit_1_hl_indirect,
};

pub const BIT_1_A: CpuInstruction = CpuInstruction {
    disassembly: "BIT 1, A",
    operand_length: 0,
    operation: Cpu::cb_bit_1_a,
};

pub const BIT_2_B: CpuInstruction = CpuInstruction {
    disassembly: "BIT 2, B",
    operand_length: 0,
    operation: Cpu::cb_bit_2_b,
};

pub const BIT_2_C: CpuInstruction = CpuInstruction {
    disassembly: "BIT 2, C",
    operand_length: 0,
    operation: Cpu::cb_bit_2_c,
};

pub const BIT_2_D: CpuInstruction = CpuInstruction {
    disassembly: "BIT 2, D",
    operand_length: 0,
    operation: Cpu::cb_bit_2_d,
};

pub const BIT_2_E: CpuInstruction = CpuInstruction {
    disassembly: "BIT 2, E",
    operand_length: 0,
    operation: Cpu::cb_bit_2_e,
};

pub const BIT_2_H: CpuInstruction = CpuInstruction {
    disassembly: "BIT 2, H",
    operand_length: 0,
    operation: Cpu::cb_bit_2_h,
};

pub const BIT_2_L: CpuInstruction = CpuInstruction {
    disassembly: "BIT 2, L",
    operand_length: 0,
    operation: Cpu::cb_bit_2_l,
};

pub const BIT_2_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "BIT 2, [HL]",
    operand_length: 0,
    operation: Cpu::cb_bit_2_hl_indirect,
};

pub const BIT_2_A: CpuInstruction = CpuInstruction {
    disassembly: "BIT 2, A",
    operand_length: 0,
    operation: Cpu::cb_bit_2_a,
};

pub const BIT_3_B: CpuInstruction = CpuInstruction {
    disassembly: "BIT 3, B",
    operand_length: 0,
    operation: Cpu::cb_bit_3_b,
};

pub const BIT_3_C: CpuInstruction = CpuInstruction {
    disassembly: "BIT 3, C",
    operand_length: 0,
    operation: Cpu::cb_bit_3_c,
};

pub const BIT_3_D: CpuInstruction = CpuInstruction {
    disassembly: "BIT 3, D",
    operand_length: 0,
    operation: Cpu::cb_bit_3_d,
};

pub const BIT_3_E: CpuInstruction = CpuInstruction {
    disassembly: "BIT 3, E",
    operand_length: 0,
    operation: Cpu::cb_bit_3_e,
};

pub const BIT_3_H: CpuInstruction = CpuInstruction {
    disassembly: "BIT 3, H",
    operand_length: 0,
    operation: Cpu::cb_bit_3_h,
};

pub const BIT_3_L: CpuInstruction = CpuInstruction {
    disassembly: "BIT 3, L",
    operand_length: 0,
    operation: Cpu::cb_bit_3_l,
};

pub const BIT_3_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "BIT 3, [HL]",
    operand_length: 0,
    operation: Cpu::cb_bit_3_hl_indirect,
};

pub const BIT_3_A: CpuInstruction = CpuInstruction {
    disassembly: "BIT 3, A",
    operand_length: 0,
    operation: Cpu::cb_bit_3_a,
};

pub const BIT_4_B: CpuInstruction = CpuInstruction {
    disassembly: "BIT 4, B",
    operand_length: 0,
    operation: Cpu::cb_bit_4_b,
};

pub const BIT_4_C: CpuInstruction = CpuInstruction {
    disassembly: "BIT 4, C",
    operand_length: 0,
    operation: Cpu::cb_bit_4_c,
};

pub const BIT_4_D: CpuInstruction = CpuInstruction {
    disassembly: "BIT 4, D",
    operand_length: 0,
    operation: Cpu::cb_bit_4_d,
};

pub const BIT_4_E: CpuInstruction = CpuInstruction {
    disassembly: "BIT 4, E",
    operand_length: 0,
    operation: Cpu::cb_bit_4_e,
};

pub const BIT_4_H: CpuInstruction = CpuInstruction {
    disassembly: "BIT 4, H",
    operand_length: 0,
    operation: Cpu::cb_bit_4_h,
};

pub const BIT_4_L: CpuInstruction = CpuInstruction {
    disassembly: "BIT 4, L",
    operand_length: 0,
    operation: Cpu::cb_bit_4_l,
};

pub const BIT_4_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "BIT 4, [HL]",
    operand_length: 0,
    operation: Cpu::cb_bit_4_hl_indirect,
};

pub const BIT_4_A: CpuInstruction = CpuInstruction {
    disassembly: "BIT 4, A",
    operand_length: 0,
    operation: Cpu::cb_bit_4_a,
};

pub const BIT_5_B: CpuInstruction = CpuInstruction {
    disassembly: "BIT 5, B",
    operand_length: 0,
    operation: Cpu::cb_bit_5_b,
};

pub const BIT_5_C: CpuInstruction = CpuInstruction {
    disassembly: "BIT 5, C",
    operand_length: 0,
    operation: Cpu::cb_bit_5_c,
};

pub const BIT_5_D: CpuInstruction = CpuInstruction {
    disassembly: "BIT 5, D",
    operand_length: 0,
    operation: Cpu::cb_bit_5_d,
};

pub const BIT_5_E: CpuInstruction = CpuInstruction {
    disassembly: "BIT 5, E",
    operand_length: 0,
    operation: Cpu::cb_bit_5_e,
};

pub const BIT_5_H: CpuInstruction = CpuInstruction {
    disassembly: "BIT 5, H",
    operand_length: 0,
    operation: Cpu::cb_bit_5_h,
};

pub const BIT_5_L: CpuInstruction = CpuInstruction {
    disassembly: "BIT 5, L",
    operand_length: 0,
    operation: Cpu::cb_bit_5_l,
};

pub const BIT_5_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "BIT 5, [HL]",
    operand_length: 0,
    operation: Cpu::cb_bit_5_hl_indirect,
};

pub const BIT_5_A: CpuInstruction = CpuInstruction {
    disassembly: "BIT 5, A",
    operand_length: 0,
    operation: Cpu::cb_bit_5_a,
};

pub const BIT_6_B: CpuInstruction = CpuInstruction {
    disassembly: "BIT 6, B",
    operand_length: 0,
    operation: Cpu::cb_bit_6_b,
};

pub const BIT_6_C: CpuInstruction = CpuInstruction {
    disassembly: "BIT 6, C",
    operand_length: 0,
    operation: Cpu::cb_bit_6_c,
};

pub const BIT_6_D: CpuInstruction = CpuInstruction {
    disassembly: "BIT 6, D",
    operand_length: 0,
    operation: Cpu::cb_bit_6_d,
};

pub const BIT_6_E: CpuInstruction = CpuInstruction {
    disassembly: "BIT 6, E",
    operand_length: 0,
    operation: Cpu::cb_bit_6_e,
};

pub const BIT_6_H: CpuInstruction = CpuInstruction {
    disassembly: "BIT 6, H",
    operand_length: 0,
    operation: Cpu::cb_bit_6_h,
};

pub const BIT_6_L: CpuInstruction = CpuInstruction {
    disassembly: "BIT 6, L",
    operand_length: 0,
    operation: Cpu::cb_bit_6_l,
};

pub const BIT_6_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "BIT 6, [HL]",
    operand_length: 0,
    operation: Cpu::cb_bit_6_hl_indirect,
};

pub const BIT_6_A: CpuInstruction = CpuInstruction {
    disassembly: "BIT 6, A",
    operand_length: 0,
    operation: Cpu::cb_bit_6_a,
};

pub const BIT_7_B: CpuInstruction = CpuInstruction {
    disassembly: "BIT 7, B",
    operand_length: 0,
    operation: Cpu::cb_bit_7_b,
};

pub const BIT_7_C: CpuInstruction = CpuInstruction {
    disassembly: "BIT 7, C",
    operand_length: 0,
    operation: Cpu::cb_bit_7_c,
};

pub const BIT_7_D: CpuInstruction = CpuInstruction {
    disassembly: "BIT 7, D",
    operand_length: 0,
    operation: Cpu::cb_bit_7_d,
};

pub const BIT_7_E: CpuInstruction = CpuInstruction {
    disassembly: "BIT 7, E",
    operand_length: 0,
    operation: Cpu::cb_bit_7_e,
};

pub const BIT_7_H: CpuInstruction = CpuInstruction {
    disassembly: "BIT 7, H",
    operand_length: 0,
    operation: Cpu::cb_bit_7_h,
};

pub const BIT_7_L: CpuInstruction = CpuInstruction {
    disassembly: "BIT 7, L",
    operand_length: 0,
    operation: Cpu::cb_bit_7_l,
};

pub const BIT_7_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "BIT 7, [HL]",
    operand_length: 0,
    operation: Cpu::cb_bit_7_hl_indirect,
};

pub const BIT_7_A: CpuInstruction = CpuInstruction {
    disassembly: "BIT 7, A",
    operand_length: 0,
    operation: Cpu::cb_bit_7_a,
};

pub const RES_0_B: CpuInstruction = CpuInstruction {
    disassembly: "RES 0, B",
    operand_length: 0,
    operation: Cpu::cb_res_0_b,
};

pub const RES_0_C: CpuInstruction = CpuInstruction {
    disassembly: "RES 0, C",
    operand_length: 0,
    operation: Cpu::cb_res_0_c,
};

pub const RES_0_D: CpuInstruction = CpuInstruction {
    disassembly: "RES 0, D",
    operand_length: 0,
    operation: Cpu::cb_res_0_d,
};

pub const RES_0_E: CpuInstruction = CpuInstruction {
    disassembly: "RES 0, E",
    operand_length: 0,
    operation: Cpu::cb_res_0_e,
};

pub const RES_0_H: CpuInstruction = CpuInstruction {
    disassembly: "RES 0, H",
    operand_length: 0,
    operation: Cpu::cb_res_0_h,
};

pub const RES_0_L: CpuInstruction = CpuInstruction {
    disassembly: "RES 0, L",
    operand_length: 0,
    operation: Cpu::cb_res_0_l,
};

pub const RES_0_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RES 0, [HL]",
    operand_length: 0,
    operation: Cpu::cb_res_0_hl_indirect,
};

pub const RES_0_A: CpuInstruction = CpuInstruction {
    disassembly: "RES 0, A",
    operand_length: 0,
    operation: Cpu::cb_res_0_a,
};

pub const RES_1_B: CpuInstruction = CpuInstruction {
    disassembly: "RES 1, B",
    operand_length: 0,
    operation: Cpu::cb_res_1_b,
};

pub const RES_1_C: CpuInstruction = CpuInstruction {
    disassembly: "RES 1, C",
    operand_length: 0,
    operation: Cpu::cb_res_1_c,
};

pub const RES_1_D: CpuInstruction = CpuInstruction {
    disassembly: "RES 1, D",
    operand_length: 0,
    operation: Cpu::cb_res_1_d,
};

pub const RES_1_E: CpuInstruction = CpuInstruction {
    disassembly: "RES 1, E",
    operand_length: 0,
    operation: Cpu::cb_res_1_e,
};

pub const RES_1_H: CpuInstruction = CpuInstruction {
    disassembly: "RES 1, H",
    operand_length: 0,
    operation: Cpu::cb_res_1_h,
};

pub const RES_1_L: CpuInstruction = CpuInstruction {
    disassembly: "RES 1, L",
    operand_length: 0,
    operation: Cpu::cb_res_1_l,
};

pub const RES_1_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RES 1, [HL]",
    operand_length: 0,
    operation: Cpu::cb_res_1_hl_indirect,
};

pub const RES_1_A: CpuInstruction = CpuInstruction {
    disassembly: "RES 1, A",
    operand_length: 0,
    operation: Cpu::cb_res_1_a,
};

pub const RES_2_B: CpuInstruction = CpuInstruction {
    disassembly: "RES 2, B",
    operand_length: 0,
    operation: Cpu::cb_res_2_b,
};

pub const RES_2_C: CpuInstruction = CpuInstruction {
    disassembly: "RES 2, C",
    operand_length: 0,
    operation: Cpu::cb_res_2_c,
};

pub const RES_2_D: CpuInstruction = CpuInstruction {
    disassembly: "RES 2, D",
    operand_length: 0,
    operation: Cpu::cb_res_2_d,
};

pub const RES_2_E: CpuInstruction = CpuInstruction {
    disassembly: "RES 2, E",
    operand_length: 0,
    operation: Cpu::cb_res_2_e,
};

pub const RES_2_H: CpuInstruction = CpuInstruction {
    disassembly: "RES 2, H",
    operand_length: 0,
    operation: Cpu::cb_res_2_h,
};

pub const RES_2_L: CpuInstruction = CpuInstruction {
    disassembly: "RES 2, L",
    operand_length: 0,
    operation: Cpu::cb_res_2_l,
};

pub const RES_2_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RES 2, [HL]",
    operand_length: 0,
    operation: Cpu::cb_res_2_hl_indirect,
};

pub const RES_2_A: CpuInstruction = CpuInstruction {
    disassembly: "RES 2, A",
    operand_length: 0,
    operation: Cpu::cb_res_2_a,
};

pub const RES_3_B: CpuInstruction = CpuInstruction {
    disassembly: "RES 3, B",
    operand_length: 0,
    operation: Cpu::cb_res_3_b,
};

pub const RES_3_C: CpuInstruction = CpuInstruction {
    disassembly: "RES 3, C",
    operand_length: 0,
    operation: Cpu::cb_res_3_c,
};

pub const RES_3_D: CpuInstruction = CpuInstruction {
    disassembly: "RES 3, D",
    operand_length: 0,
    operation: Cpu::cb_res_3_d,
};

pub const RES_3_E: CpuInstruction = CpuInstruction {
    disassembly: "RES 3, E",
    operand_length: 0,
    operation: Cpu::cb_res_3_e,
};

pub const RES_3_H: CpuInstruction = CpuInstruction {
    disassembly: "RES 3, H",
    operand_length: 0,
    operation: Cpu::cb_res_3_h,
};

pub const RES_3_L: CpuInstruction = CpuInstruction {
    disassembly: "RES 3, L",
    operand_length: 0,
    operation: Cpu::cb_res_3_l,
};

pub const RES_3_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RES 3, [HL]",
    operand_length: 0,
    operation: Cpu::cb_res_3_hl_indirect,
};

pub const RES_3_A: CpuInstruction = CpuInstruction {
    disassembly: "RES 3, A",
    operand_length: 0,
    operation: Cpu::cb_res_3_a,
};

pub const RES_4_B: CpuInstruction = CpuInstruction {
    disassembly: "RES 4, B",
    operand_length: 0,
    operation: Cpu::cb_res_4_b,
};

pub const RES_4_C: CpuInstruction = CpuInstruction {
    disassembly: "RES 4, C",
    operand_length: 0,
    operation: Cpu::cb_res_4_c,
};

pub const RES_4_D: CpuInstruction = CpuInstruction {
    disassembly: "RES 4, D",
    operand_length: 0,
    operation: Cpu::cb_res_4_d,
};

pub const RES_4_E: CpuInstruction = CpuInstruction {
    disassembly: "RES 4, E",
    operand_length: 0,
    operation: Cpu::cb_res_4_e,
};

pub const RES_4_H: CpuInstruction = CpuInstruction {
    disassembly: "RES 4, H",
    operand_length: 0,
    operation: Cpu::cb_res_4_h,
};

pub const RES_4_L: CpuInstruction = CpuInstruction {
    disassembly: "RES 4, L",
    operand_length: 0,
    operation: Cpu::cb_res_4_l,
};

pub const RES_4_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RES 4, [HL]",
    operand_length: 0,
    operation: Cpu::cb_res_4_hl_indirect,
};

pub const RES_4_A: CpuInstruction = CpuInstruction {
    disassembly: "RES 4, A",
    operand_length: 0,
    operation: Cpu::cb_res_4_a,
};

pub const RES_5_B: CpuInstruction = CpuInstruction {
    disassembly: "RES 5, B",
    operand_length: 0,
    operation: Cpu::cb_res_5_b,
};

pub const RES_5_C: CpuInstruction = CpuInstruction {
    disassembly: "RES 5, C",
    operand_length: 0,
    operation: Cpu::cb_res_5_c,
};

pub const RES_5_D: CpuInstruction = CpuInstruction {
    disassembly: "RES 5, D",
    operand_length: 0,
    operation: Cpu::cb_res_5_d,
};

pub const RES_5_E: CpuInstruction = CpuInstruction {
    disassembly: "RES 5, E",
    operand_length: 0,
    operation: Cpu::cb_res_5_e,
};

pub const RES_5_H: CpuInstruction = CpuInstruction {
    disassembly: "RES 5, H",
    operand_length: 0,
    operation: Cpu::cb_res_5_h,
};

pub const RES_5_L: CpuInstruction = CpuInstruction {
    disassembly: "RES 5, L",
    operand_length: 0,
    operation: Cpu::cb_res_5_l,
};

pub const RES_5_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RES 5, [HL]",
    operand_length: 0,
    operation: Cpu::cb_res_5_hl_indirect,
};

pub const RES_5_A: CpuInstruction = CpuInstruction {
    disassembly: "RES 5, A",
    operand_length: 0,
    operation: Cpu::cb_res_5_a,
};

pub const RES_6_B: CpuInstruction = CpuInstruction {
    disassembly: "RES 6, B",
    operand_length: 0,
    operation: Cpu::cb_res_6_b,
};

pub const RES_6_C: CpuInstruction = CpuInstruction {
    disassembly: "RES 6, C",
    operand_length: 0,
    operation: Cpu::cb_res_6_c,
};

pub const RES_6_D: CpuInstruction = CpuInstruction {
    disassembly: "RES 6, D",
    operand_length: 0,
    operation: Cpu::cb_res_6_d,
};

pub const RES_6_E: CpuInstruction = CpuInstruction {
    disassembly: "RES 6, E",
    operand_length: 0,
    operation: Cpu::cb_res_6_e,
};

pub const RES_6_H: CpuInstruction = CpuInstruction {
    disassembly: "RES 6, H",
    operand_length: 0,
    operation: Cpu::cb_res_6_h,
};

pub const RES_6_L: CpuInstruction = CpuInstruction {
    disassembly: "RES 6, L",
    operand_length: 0,
    operation: Cpu::cb_res_6_l,
};

pub const RES_6_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RES 6, [HL]",
    operand_length: 0,
    operation: Cpu::cb_res_6_hl_indirect,
};

pub const RES_6_A: CpuInstruction = CpuInstruction {
    disassembly: "RES 6, A",
    operand_length: 0,
    operation: Cpu::cb_res_6_a,
};

pub const RES_7_B: CpuInstruction = CpuInstruction {
    disassembly: "RES 7, B",
    operand_length: 0,
    operation: Cpu::cb_res_7_b,
};

pub const RES_7_C: CpuInstruction = CpuInstruction {
    disassembly: "RES 7, C",
    operand_length: 0,
    operation: Cpu::cb_res_7_c,
};

pub const RES_7_D: CpuInstruction = CpuInstruction {
    disassembly: "RES 7, D",
    operand_length: 0,
    operation: Cpu::cb_res_7_d,
};

pub const RES_7_E: CpuInstruction = CpuInstruction {
    disassembly: "RES 7, E",
    operand_length: 0,
    operation: Cpu::cb_res_7_e,
};

pub const RES_7_H: CpuInstruction = CpuInstruction {
    disassembly: "RES 7, H",
    operand_length: 0,
    operation: Cpu::cb_res_7_h,
};

pub const RES_7_L: CpuInstruction = CpuInstruction {
    disassembly: "RES 7, L",
    operand_length: 0,
    operation: Cpu::cb_res_7_l,
};

pub const RES_7_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "RES 7, [HL]",
    operand_length: 0,
    operation: Cpu::cb_res_7_hl_indirect,
};

pub const RES_7_A: CpuInstruction = CpuInstruction {
    disassembly: "RES 7, A",
    operand_length: 0,
    operation: Cpu::cb_res_7_a,
};

pub const SET_0_B: CpuInstruction = CpuInstruction {
    disassembly: "SET 0, B",
    operand_length: 0,
    operation: Cpu::cb_set_0_b,
};

pub const SET_0_C: CpuInstruction = CpuInstruction {
    disassembly: "SET 0, C",
    operand_length: 0,
    operation: Cpu::cb_set_0_c,
};

pub const SET_0_D: CpuInstruction = CpuInstruction {
    disassembly: "SET 0, D",
    operand_length: 0,
    operation: Cpu::cb_set_0_d,
};

pub const SET_0_E: CpuInstruction = CpuInstruction {
    disassembly: "SET 0, E",
    operand_length: 0,
    operation: Cpu::cb_set_0_e,
};

pub const SET_0_H: CpuInstruction = CpuInstruction {
    disassembly: "SET 0, H",
    operand_length: 0,
    operation: Cpu::cb_set_0_h,
};

pub const SET_0_L: CpuInstruction = CpuInstruction {
    disassembly: "SET 0, L",
    operand_length: 0,
    operation: Cpu::cb_set_0_l,
};

pub const SET_0_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SET 0, [HL]",
    operand_length: 0,
    operation: Cpu::cb_set_0_hl_indirect,
};

pub const SET_0_A: CpuInstruction = CpuInstruction {
    disassembly: "SET 0, A",
    operand_length: 0,
    operation: Cpu::cb_set_0_a,
};

pub const SET_1_B: CpuInstruction = CpuInstruction {
    disassembly: "SET 1, B",
    operand_length: 0,
    operation: Cpu::cb_set_1_b,
};

pub const SET_1_C: CpuInstruction = CpuInstruction {
    disassembly: "SET 1, C",
    operand_length: 0,
    operation: Cpu::cb_set_1_c,
};

pub const SET_1_D: CpuInstruction = CpuInstruction {
    disassembly: "SET 1, D",
    operand_length: 0,
    operation: Cpu::cb_set_1_d,
};

pub const SET_1_E: CpuInstruction = CpuInstruction {
    disassembly: "SET 1, E",
    operand_length: 0,
    operation: Cpu::cb_set_1_e,
};

pub const SET_1_H: CpuInstruction = CpuInstruction {
    disassembly: "SET 1, H",
    operand_length: 0,
    operation: Cpu::cb_set_1_h,
};

pub const SET_1_L: CpuInstruction = CpuInstruction {
    disassembly: "SET 1, L",
    operand_length: 0,
    operation: Cpu::cb_set_1_l,
};

pub const SET_1_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SET 1, [HL]",
    operand_length: 0,
    operation: Cpu::cb_set_1_hl_indirect,
};

pub const SET_1_A: CpuInstruction = CpuInstruction {
    disassembly: "SET 1, A",
    operand_length: 0,
    operation: Cpu::cb_set_1_a,
};

pub const SET_2_B: CpuInstruction = CpuInstruction {
    disassembly: "SET 2, B",
    operand_length: 0,
    operation: Cpu::cb_set_2_b,
};

pub const SET_2_C: CpuInstruction = CpuInstruction {
    disassembly: "SET 2, C",
    operand_length: 0,
    operation: Cpu::cb_set_2_c,
};

pub const SET_2_D: CpuInstruction = CpuInstruction {
    disassembly: "SET 2, D",
    operand_length: 0,
    operation: Cpu::cb_set_2_d,
};

pub const SET_2_E: CpuInstruction = CpuInstruction {
    disassembly: "SET 2, E",
    operand_length: 0,
    operation: Cpu::cb_set_2_e,
};

pub const SET_2_H: CpuInstruction = CpuInstruction {
    disassembly: "SET 2, H",
    operand_length: 0,
    operation: Cpu::cb_set_2_h,
};

pub const SET_2_L: CpuInstruction = CpuInstruction {
    disassembly: "SET 2, L",
    operand_length: 0,
    operation: Cpu::cb_set_2_l,
};

pub const SET_2_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SET 2, [HL]",
    operand_length: 0,
    operation: Cpu::cb_set_2_hl_indirect,
};

pub const SET_2_A: CpuInstruction = CpuInstruction {
    disassembly: "SET 2, A",
    operand_length: 0,
    operation: Cpu::cb_set_2_a,
};

pub const SET_3_B: CpuInstruction = CpuInstruction {
    disassembly: "SET 3, B",
    operand_length: 0,
    operation: Cpu::cb_set_3_b,
};

pub const SET_3_C: CpuInstruction = CpuInstruction {
    disassembly: "SET 3, C",
    operand_length: 0,
    operation: Cpu::cb_set_3_c,
};

pub const SET_3_D: CpuInstruction = CpuInstruction {
    disassembly: "SET 3, D",
    operand_length: 0,
    operation: Cpu::cb_set_3_d,
};

pub const SET_3_E: CpuInstruction = CpuInstruction {
    disassembly: "SET 3, E",
    operand_length: 0,
    operation: Cpu::cb_set_3_e,
};

pub const SET_3_H: CpuInstruction = CpuInstruction {
    disassembly: "SET 3, H",
    operand_length: 0,
    operation: Cpu::cb_set_3_h,
};

pub const SET_3_L: CpuInstruction = CpuInstruction {
    disassembly: "SET 3, L",
    operand_length: 0,
    operation: Cpu::cb_set_3_l,
};

pub const SET_3_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SET 3, [HL]",
    operand_length: 0,
    operation: Cpu::cb_set_3_hl_indirect,
};

pub const SET_3_A: CpuInstruction = CpuInstruction {
    disassembly: "SET 3, A",
    operand_length: 0,
    operation: Cpu::cb_set_3_a,
};

pub const SET_4_B: CpuInstruction = CpuInstruction {
    disassembly: "SET 4, B",
    operand_length: 0,
    operation: Cpu::cb_set_4_b,
};

pub const SET_4_C: CpuInstruction = CpuInstruction {
    disassembly: "SET 4, C",
    operand_length: 0,
    operation: Cpu::cb_set_4_c,
};

pub const SET_4_D: CpuInstruction = CpuInstruction {
    disassembly: "SET 4, D",
    operand_length: 0,
    operation: Cpu::cb_set_4_d,
};

pub const SET_4_E: CpuInstruction = CpuInstruction {
    disassembly: "SET 4, E",
    operand_length: 0,
    operation: Cpu::cb_set_4_e,
};

pub const SET_4_H: CpuInstruction = CpuInstruction {
    disassembly: "SET 4, H",
    operand_length: 0,
    operation: Cpu::cb_set_4_h,
};

pub const SET_4_L: CpuInstruction = CpuInstruction {
    disassembly: "SET 4, L",
    operand_length: 0,
    operation: Cpu::cb_set_4_l,
};

pub const SET_4_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SET 4, [HL]",
    operand_length: 0,
    operation: Cpu::cb_set_4_hl_indirect,
};

pub const SET_4_A: CpuInstruction = CpuInstruction {
    disassembly: "SET 4, A",
    operand_length: 0,
    operation: Cpu::cb_set_4_a,
};

pub const SET_5_B: CpuInstruction = CpuInstruction {
    disassembly: "SET 5, B",
    operand_length: 0,
    operation: Cpu::cb_set_5_b,
};

pub const SET_5_C: CpuInstruction = CpuInstruction {
    disassembly: "SET 5, C",
    operand_length: 0,
    operation: Cpu::cb_set_5_c,
};

pub const SET_5_D: CpuInstruction = CpuInstruction {
    disassembly: "SET 5, D",
    operand_length: 0,
    operation: Cpu::cb_set_5_d,
};

pub const SET_5_E: CpuInstruction = CpuInstruction {
    disassembly: "SET 5, E",
    operand_length: 0,
    operation: Cpu::cb_set_5_e,
};

pub const SET_5_H: CpuInstruction = CpuInstruction {
    disassembly: "SET 5, H",
    operand_length: 0,
    operation: Cpu::cb_set_5_h,
};

pub const SET_5_L: CpuInstruction = CpuInstruction {
    disassembly: "SET 5, L",
    operand_length: 0,
    operation: Cpu::cb_set_5_l,
};

pub const SET_5_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SET 5, [HL]",
    operand_length: 0,
    operation: Cpu::cb_set_5_hl_indirect,
};

pub const SET_5_A: CpuInstruction = CpuInstruction {
    disassembly: "SET 5, A",
    operand_length: 0,
    operation: Cpu::cb_set_5_a,
};

pub const SET_6_B: CpuInstruction = CpuInstruction {
    disassembly: "SET 6, B",
    operand_length: 0,
    operation: Cpu::cb_set_6_b,
};

pub const SET_6_C: CpuInstruction = CpuInstruction {
    disassembly: "SET 6, C",
    operand_length: 0,
    operation: Cpu::cb_set_6_c,
};

pub const SET_6_D: CpuInstruction = CpuInstruction {
    disassembly: "SET 6, D",
    operand_length: 0,
    operation: Cpu::cb_set_6_d,
};

pub const SET_6_E: CpuInstruction = CpuInstruction {
    disassembly: "SET 6, E",
    operand_length: 0,
    operation: Cpu::cb_set_6_e,
};

pub const SET_6_H: CpuInstruction = CpuInstruction {
    disassembly: "SET 6, H",
    operand_length: 0,
    operation: Cpu::cb_set_6_h,
};

pub const SET_6_L: CpuInstruction = CpuInstruction {
    disassembly: "SET 6, L",
    operand_length: 0,
    operation: Cpu::cb_set_6_l,
};

pub const SET_6_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SET 6, [HL]",
    operand_length: 0,
    operation: Cpu::cb_set_6_hl_indirect,
};

pub const SET_6_A: CpuInstruction = CpuInstruction {
    disassembly: "SET 6, A",
    operand_length: 0,
    operation: Cpu::cb_set_6_a,
};

pub const SET_7_B: CpuInstruction = CpuInstruction {
    disassembly: "SET 7, B",
    operand_length: 0,
    operation: Cpu::cb_set_7_b,
};

pub const SET_7_C: CpuInstruction = CpuInstruction {
    disassembly: "SET 7, C",
    operand_length: 0,
    operation: Cpu::cb_set_7_c,
};

pub const SET_7_D: CpuInstruction = CpuInstruction {
    disassembly: "SET 7, D",
    operand_length: 0,
    operation: Cpu::cb_set_7_d,
};

pub const SET_7_E: CpuInstruction = CpuInstruction {
    disassembly: "SET 7, E",
    operand_length: 0,
    operation: Cpu::cb_set_7_e,
};

pub const SET_7_H: CpuInstruction = CpuInstruction {
    disassembly: "SET 7, H",
    operand_length: 0,
    operation: Cpu::cb_set_7_h,
};

pub const SET_7_L: CpuInstruction = CpuInstruction {
    disassembly: "SET 7, L",
    operand_length: 0,
    operation: Cpu::cb_set_7_l,
};

pub const SET_7_HL_INDIRECT: CpuInstruction = CpuInstruction {
    disassembly: "SET 7, [HL]",
    operand_length: 0,
    operation: Cpu::cb_set_7_hl_indirect,
};

pub const SET_7_A: CpuInstruction = CpuInstruction {
    disassembly: "SET 7, A",
    operand_length: 0,
    operation: Cpu::cb_set_7_a,
};
