use super::super::Interconnect;
use super::opcode::Opcode::*;
use super::opcode::RegImmOpcode::*;
use super::opcode::SpecialOpcode::*;
use super::opcode::{Cop0CoOpcode, Cop0Opcode, Cop1Opcode};
use super::{cp0, Instruction};

use extprim::i128::i128;
use extprim::u128::u128;

use std::fmt;

const NUM_GPR: usize = 32;

enum SignExtendResult {
    Yes,
    No,
}

enum WriteLink {
    Yes,
    No,
}

pub struct Cpu {
    reg_gpr: [u64; NUM_GPR],
    reg_fpr: [f64; NUM_GPR],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool, // TODO: Enum type

    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: cp0::Cp0,

    delay_slot_pc: Option<u64>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_gpr: [0; NUM_GPR],
            reg_fpr: [0.0; NUM_GPR],

            reg_pc: 0xffff_ffff_bfc0_0000, // TODO: Move to const

            reg_hi: 0,
            reg_lo: 0,

            reg_llbit: false,

            reg_fcr0: 0,
            reg_fcr31: 0,

            cp0: cp0::Cp0::default(),

            delay_slot_pc: None,
        }
    }

    pub fn current_pc_virt(&self) -> u64 {
        self.delay_slot_pc.unwrap_or(self.reg_pc)
    }

    pub fn current_pc_phys(&self) -> u64 {
        self.virt_addr_to_phys_addr(self.current_pc_virt())
    }

    pub fn will_execute_from_delay_slot(&self) -> bool {
        self.delay_slot_pc.is_some()
    }

    pub fn step(&mut self, interconnect: &mut Interconnect) {
        if let Some(pc) = self.delay_slot_pc {
            let instr = self.read_instruction(interconnect, pc);
            self.delay_slot_pc = None;

            self.execute_instruction(interconnect, instr);
        } else {
            let instr = self.read_instruction(interconnect, self.reg_pc);

            self.reg_pc += 4;
            self.execute_instruction(interconnect, instr);
        }
    }

    fn read_instruction(&self, interconnect: &mut Interconnect, addr: u64) -> Instruction {
        Instruction(self.read_word(interconnect, addr))
    }

    fn execute_instruction(&mut self, interconnect: &mut Interconnect, instr: Instruction) {
        match instr.opcode() {
            Special => {
                match instr.special_op() {
                    Sll => self.reg_instr(instr, SignExtendResult::Yes, |_, rt, sa| rt << sa),
                    Srl => self.reg_instr(instr, SignExtendResult::Yes, |_, rt, sa| ((rt as u32) >> sa) as u64),
                    Sra => self.reg_instr(instr, SignExtendResult::Yes, |_, rt, sa| ((rt as i32) >> sa) as u64),

                    Sllv => self.reg_instr(instr, SignExtendResult::Yes, |rs, rt, _| ((rt as u32) << (rs & 0b11111)) as u64),
                    Srlv => self.reg_instr(instr, SignExtendResult::Yes, |rs, rt, _| ((rt as u32) >> (rs & 0b11111)) as u64),
                    Srav => self.reg_instr(instr, SignExtendResult::Yes, |rs, rt, _| ((rt as i32) >> (rs & 0b11111)) as u64),

                    Jr => {
                        let delay_slot_pc = self.reg_pc;

                        // Update PC before executing delay slot instruction
                        self.reg_pc = self.read_reg_gpr(instr.rs());

                        self.delay_slot_pc = Some(delay_slot_pc);
                    }

                    Jalr => {
                        let delay_slot_pc = self.reg_pc;

                        let link_address = delay_slot_pc + 4;
                        self.write_reg_gpr(instr.rd() as usize, link_address);

                        self.delay_slot_pc = Some(delay_slot_pc);
                    }

                    Sync => {
                        // Basically a NOP on vr4300
                    }

                    Multu => {
                        let rs = self.read_reg_gpr(instr.rs()) as u32;
                        let rt = self.read_reg_gpr(instr.rt()) as u32;

                        let res = (rs as u64) * (rt as u64);

                        // TODO: Undefined if last 2 instructions were
                        //  MFHI or MFLO
                        self.reg_lo = (res as i32) as u64;
                        self.reg_hi = ((res >> 32) as i32) as u64;
                    }

                    Mfhi => {
                        let value = self.reg_hi;
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }
                    Mthi => {
                        let value = self.read_reg_gpr(instr.rs());
                        self.reg_hi = value;
                    }
                    Mflo => {
                        let value = self.reg_lo;
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }
                    Mtlo => {
                        let value = self.read_reg_gpr(instr.rs());
                        self.reg_lo = value;
                    }

                    Dsllv => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| rt << (rs & 0b111111)),
                    Dsrlv => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| rt >> (rs & 0b111111)),
                    Dsrav => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| ((rt as i64) >> (rs & 0b111111)) as u64),
                    Mult => {
                        let rs = self.read_reg_gpr(instr.rs()) as i32;
                        let rt = self.read_reg_gpr(instr.rt()) as i32;

                        let res = (rs as i64) * (rt as i64);

                        // TODO: Undefined if last 2 instructions were
                        //  MFHI or MFLO
                        self.reg_lo = (res as i32) as u64;
                        self.reg_hi = ((res >> 32) as i32) as u64;
                    }

                    Div => {
                        let rs = self.read_reg_gpr(instr.rs()) as i32;
                        let rt = self.read_reg_gpr(instr.rt()) as i32;

                        self.reg_lo = (rs / rt) as u64;
                        self.reg_hi = (rs % rt) as u64;
                    }
                    Divu => {
                        let rs = self.read_reg_gpr(instr.rs()) as u32;
                        let rt = self.read_reg_gpr(instr.rt()) as u32;

                        self.reg_lo = (rs / rt) as u64;
                        self.reg_hi = (rs % rt) as u64;
                    }
                    Dmult => {
                        let rs = i128::new(self.read_reg_gpr(instr.rs()) as i64);
                        let rt = i128::new(self.read_reg_gpr(instr.rt()) as i64);
                        let mul = rs * rt;

                        self.reg_lo = mul.low64();
                        self.reg_hi = mul.high64() as u64;
                    }
                    Dmultu => {
                        let rs = u128::new(self.read_reg_gpr(instr.rs()));
                        let rt = u128::new(self.read_reg_gpr(instr.rt()));
                        let mul = rs * rt;

                        self.reg_lo = mul.low64();
                        self.reg_hi = mul.high64();
                    }
                    Ddiv => {
                        let rs = self.read_reg_gpr(instr.rs()) as i64;
                        let rt = self.read_reg_gpr(instr.rt()) as i64;

                        self.reg_lo = (rs / rt) as u64;
                        self.reg_hi = (rs % rt) as u64;
                    }

                    Ddivu => {
                        let rs = self.read_reg_gpr(instr.rs());
                        let rt = self.read_reg_gpr(instr.rt());

                        self.reg_lo = rs / rt;
                        self.reg_hi = rs % rt;
                    }

                    Add => self.reg_instr(instr, SignExtendResult::Yes, |rs, rt, _| rs.wrapping_add(rt)), // Todo handle overflow exception
                    Addu => self.reg_instr(instr, SignExtendResult::Yes, |rs, rt, _| rs.wrapping_add(rt)),
                    Sub => self.reg_instr(instr, SignExtendResult::Yes, |rs, rt, _| rs.wrapping_sub(rt)), // Todo handle overflow exception
                    Subu => self.reg_instr(instr, SignExtendResult::Yes, |rs, rt, _| rs.wrapping_sub(rt)),

                    And => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| rs & rt),
                    Or => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| rs | rt),
                    Xor => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| rs ^ rt),
                    Nor => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| !(rs | rt)),

                    Slt => self.reg_instr(
                        instr,
                        SignExtendResult::Yes,
                        |rs, rt, _| {
                            if (rs as i64) < (rt as i64) {
                                1
                            } else {
                                0
                            }
                        },
                    ),
                    Sltu => self.reg_instr(instr, SignExtendResult::Yes, |rs, rt, _| if rs < rt { 1 } else { 0 }),

                    Dadd => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| rs.wrapping_add(rt)), // Todo handle overflow exception
                    Daddu => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| rs.wrapping_add(rt)),
                    Dsub => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| rs.wrapping_sub(rt)), // Todo handle overflow exception
                    Dsubu => self.reg_instr(instr, SignExtendResult::No, |rs, rt, _| rs.wrapping_sub(rt)),

                    Dsll => self.reg_instr(instr, SignExtendResult::No, |_, rt, sa| rt << sa),
                    Dsrl => self.reg_instr(instr, SignExtendResult::No, |_, rt, sa| rt >> sa),
                    Dsra => self.reg_instr(instr, SignExtendResult::No, |_, rt, sa| ((rt as i64) >> sa) as u64),
                    Dsll32 => self.reg_instr(instr, SignExtendResult::No, |_, rt, sa| rt << (sa + 32)),
                    Dsrl32 => self.reg_instr(instr, SignExtendResult::No, |_, rt, sa| rt >> (sa + 32)),
                    Dsra32 => self.reg_instr(instr, SignExtendResult::No, |_, rt, sa| ((rt as i64) >> (sa + 32)) as u64),
                }
            }

            RegImm => match instr.reg_imm_op() {
                Bltz => {
                    self.branch(instr, WriteLink::No, |rs, _| (rs as i64) < 0);
                }
                Bgez => {
                    self.branch(instr, WriteLink::No, |rs, _| (rs as i64) >= 0);
                }
                Bgezl => {
                    self.branch_likely(instr, |rs, _| (rs as i64) >= 0);
                }
                Bgezal => {
                    self.branch(instr, WriteLink::Yes, |rs, _| (rs as i64) >= 0);
                }
            },

            Cop0 => {
                match instr.cop0_op() {
                    Cop0Opcode::Mfc0 => {
                        let data = self.cp0.read_reg(instr.rd());
                        self.write_reg_gpr(instr.rt(), data);
                    }
                    Cop0Opcode::Mtc0 => {
                        let data = self.read_reg_gpr(instr.rt());
                        self.cp0.write_reg(instr.rd(), data);
                    }
                    Cop0Opcode::Co => {
                        match instr.cop0_co_op() {
                            Cop0CoOpcode::Tlbwi => self.cp0.store_tlb_entry(),
                            Cop0CoOpcode::Eret => {
                                self.reg_pc = self.cp0.read_reg(14 /* EPC */)
                            }
                        }
                    }
                }
            }

            Cop1 => {
                if instr.fmt() == 0b01000 {
                    if instr.ft() == 0b00010 {
                        let branch = (self.reg_fcr31 & 0x00400000) == 0;
                        self.branch_likely(instr, |_, _| branch);
                    } else if instr.ft() == 0b00000 {
                        let branch = (self.reg_fcr31 & 0x00400000) == 0;
                        self.branch(instr, WriteLink::No, |_, _| branch);
                    }
                } else {
                    match (instr.cop1_op(), instr.fmt()) {
                        (Cop1Opcode::Add, 16) => self.cp1_single_instr(instr, |fs, ft| fs + ft),
                        (Cop1Opcode::Add, 17) => self.cp1_double_instr(instr, |fs, ft| fs + ft),
                        (Cop1Opcode::Sub, 16) => self.cp1_single_instr(instr, |fs, ft| fs - ft),
                        (Cop1Opcode::Sub, 17) => self.cp1_double_instr(instr, |fs, ft| fs - ft),
                        (Cop1Opcode::Mul, 16) => self.cp1_single_instr(instr, |fs, ft| fs * ft),
                        (Cop1Opcode::Mul, 17) => self.cp1_double_instr(instr, |fs, ft| fs * ft),
                        (Cop1Opcode::Div, 16) => self.cp1_single_instr(instr, |fs, ft| fs / ft),
                        (Cop1Opcode::Div, 17) => self.cp1_double_instr(instr, |fs, ft| fs / ft),
                        (Cop1Opcode::Sqrt, 16) => self.cp1_single_instr(instr, |fs, _| fs.sqrt()),
                        (Cop1Opcode::Sqrt, 17) => self.cp1_double_instr(instr, |fs, _| fs.sqrt()),
                        (Cop1Opcode::Abs, 16) => self.cp1_single_instr(instr, |fs, _| fs.abs()),
                        (Cop1Opcode::Abs, 17) => self.cp1_double_instr(instr, |fs, _| fs.abs()),
                        (Cop1Opcode::Mov, 16) => self.cp1_single_instr(instr, |fs, _| fs),
                        (Cop1Opcode::Mov, 17) => self.cp1_double_instr(instr, |fs, _| fs),
                        (Cop1Opcode::Neg, 16) => self.cp1_single_instr(instr, |fs, _| -fs),
                        (Cop1Opcode::Neg, 17) => self.cp1_double_instr(instr, |fs, _| -fs),
                        (Cop1Opcode::RoundL, 16) => self.cp1_single_to_doubleword_instr(instr, |fs, _| fs.round() as i64 as u64),
                        (Cop1Opcode::RoundL, 17) => self.cp1_double_to_doubleword_instr(instr, |fs, _| fs.round() as i64 as u64),
                        (Cop1Opcode::TruncL, 16) => self.cp1_single_to_doubleword_instr(instr, |fs, _| fs.trunc() as i64 as u64),
                        (Cop1Opcode::TruncL, 17) => self.cp1_double_to_doubleword_instr(instr, |fs, _| fs.trunc() as i64 as u64),
                        (Cop1Opcode::CeilL, 16) => self.cp1_single_to_doubleword_instr(instr, |fs, _| fs.ceil() as i64 as u64),
                        (Cop1Opcode::CeilL, 17) => self.cp1_double_to_doubleword_instr(instr, |fs, _| fs.ceil() as i64 as u64),
                        (Cop1Opcode::FloorL, 16) => self.cp1_single_to_doubleword_instr(instr, |fs, _| fs.floor() as i64 as u64),
                        (Cop1Opcode::FloorL, 17) => self.cp1_double_to_doubleword_instr(instr, |fs, _| fs.floor() as i64 as u64),
                        (Cop1Opcode::RoundW, 16) => self.cp1_single_to_word_instr(instr, |fs, _| fs.round() as i32 as u32),
                        (Cop1Opcode::RoundW, 17) => self.cp1_double_to_word_instr(instr, |fs, _| fs.round() as i32 as u32),
                        (Cop1Opcode::TruncW, 16) => self.cp1_single_to_doubleword_instr(instr, |fs, _| fs.trunc() as i64 as u64),
                        (Cop1Opcode::TruncW, 17) => self.cp1_double_to_doubleword_instr(instr, |fs, _| fs.trunc() as i64 as u64),
                        (Cop1Opcode::CeilW, 16) => self.cp1_single_to_word_instr(instr, |fs, _| fs.ceil() as i32 as u32),
                        (Cop1Opcode::CeilW, 17) => self.cp1_double_to_word_instr(instr, |fs, _| fs.ceil() as i32 as u32),
                        (Cop1Opcode::FloorW, 16) => self.cp1_single_to_word_instr(instr, |fs, _| fs.floor() as i32 as u32),
                        (Cop1Opcode::FloorW, 17) => self.cp1_double_to_word_instr(instr, |fs, _| fs.floor() as i32 as u32),
                        (Cop1Opcode::CvtS, 17) => self.cp1_double_to_word_instr(instr, |fs, _| (fs as f32).to_bits()),
                        (Cop1Opcode::CvtS, 20) => self.cp1_single_to_word_instr(instr, |fs, _| (fs.to_bits() as i32 as f32).to_bits()),
                        (Cop1Opcode::CvtS, 21) => self.cp1_double_to_word_instr(instr, |fs, _| (fs.to_bits() as i64 as f32).to_bits()),
                        (Cop1Opcode::CvtD, 16) => self.cp1_single_to_doubleword_instr(instr, |fs, _| (fs as f64).to_bits()),
                        (Cop1Opcode::CvtD, 20) => {
                            self.cp1_single_to_doubleword_instr(instr, |fs, _| (fs.to_bits() as i32 as f64).to_bits())
                        }
                        (Cop1Opcode::CvtD, 21) => {
                            self.cp1_double_to_doubleword_instr(instr, |fs, _| (fs.to_bits() as i64 as f64).to_bits())
                        }
                        (Cop1Opcode::CvtW, 16) => self.cp1_single_to_word_instr(instr, |fs, _| (fs as i32 as u32)),
                        (Cop1Opcode::CvtW, 17) => self.cp1_double_to_word_instr(instr, |fs, _| (fs as i32 as u32)),
                        (Cop1Opcode::CvtL, 16) => self.cp1_single_to_doubleword_instr(instr, |fs, _| (fs as i64 as u64)),
                        (Cop1Opcode::CvtL, 17) => self.cp1_double_to_doubleword_instr(instr, |fs, _| (fs as i64 as u64)),
                        (Cop1Opcode::Ceq, 16) => self.cp1_single_cmp_instr(instr, |fs, ft| fs == ft),
                        (Cop1Opcode::Ceq, 17) => self.cp1_double_cmp_instr(instr, |fs, ft| fs == ft),
                        (Cop1Opcode::Cle, 16) => self.cp1_single_cmp_instr(instr, |fs, ft| fs <= ft),
                        (Cop1Opcode::Cle, 17) => self.cp1_double_cmp_instr(instr, |fs, ft| fs <= ft),
                        _ => panic!("Unsupported {:?} {:?} {} {:08X}", instr, instr.cop1_op(), instr.fmt(), instr.0),
                    }
                }
            }

            J => {
                let delay_slot_pc = self.reg_pc;
                let jump_to = (delay_slot_pc & 0xFFFFFFFF_F0000000) | ((instr.target() as u64) << 2);
                self.reg_pc = jump_to;
                self.delay_slot_pc = Some(delay_slot_pc);
            }

            Jal => {
                let delay_slot_pc = self.reg_pc;
                self.reg_gpr[31] = delay_slot_pc + 4;
                let jump_to = (delay_slot_pc & 0xFFFFFFFF_F0000000) | ((instr.target() as u64) << 2);
                self.reg_pc = jump_to;
                self.delay_slot_pc = Some(delay_slot_pc);
            }

            Blez => {
                self.branch(instr, WriteLink::No, |rs, _rt| (rs as i64) <= 0);
            }

            Addi => self.imm_instr(instr, SignExtendResult::Yes, |rs, _, imm_sign_extended| {
                // TODO: Handle overflow exception
                rs.wrapping_add(imm_sign_extended)
            }),
            Addiu => self.imm_instr(instr, SignExtendResult::Yes, |rs, _, imm_sign_extended| {
                rs.wrapping_add(imm_sign_extended)
            }),
            Slti => self.imm_instr(instr, SignExtendResult::No, |rs, _, imm_sign_extended| {
                if (rs as i64) < (imm_sign_extended as i64) {
                    1
                } else {
                    0
                }
            }),
            Sltiu => self.imm_instr(
                instr,
                SignExtendResult::No,
                |rs, _, imm_sign_extended| {
                    if rs < imm_sign_extended {
                        1
                    } else {
                        0
                    }
                },
            ),

            Andi => self.imm_instr(instr, SignExtendResult::No, |rs, imm, _| rs & imm),
            Ori => self.imm_instr(instr, SignExtendResult::No, |rs, imm, _| rs | imm),
            Xori => self.imm_instr(instr, SignExtendResult::No, |rs, imm, _| rs ^ imm),

            Lui => self.imm_instr(instr, SignExtendResult::Yes, |_, imm, _| imm << 16),

            Beq => {
                self.branch(instr, WriteLink::No, |rs, rt| rs == rt);
            }
            Bne => {
                self.branch(instr, WriteLink::No, |rs, rt| rs != rt);
            }
            Bgtz => {
                self.branch(instr, WriteLink::No, |rs, _rt| (rs as i64) > 0);
            }

            Beql => self.branch_likely(instr, |rs, rt| rs == rt),
            Bnel => self.branch_likely(instr, |rs, rt| rs != rt),
            Blezl => self.branch_likely(instr, |rs, _rt| (rs as i64) <= 0),

            Daddi => self.imm_instr(instr, SignExtendResult::No, |rs, _, imm_sign_extended| {
                rs.wrapping_add(imm_sign_extended)
            }), // TODO: Handle overflow
            Daddiu => self.imm_instr(instr, SignExtendResult::No, |rs, _, imm_sign_extended| {
                rs.wrapping_add(imm_sign_extended)
            }),

            Lb => {
                let byte = self.read_byte(interconnect, self.resolve_offset(instr));
                self.write_reg_gpr(instr.rt(), byte as i8 as u64);
            }

            Lh => {
                let halfword = self.read_halfword(interconnect, self.resolve_offset(instr));
                self.write_reg_gpr(instr.rt(), halfword as i16 as u64);
            }

            Lwl => {
                let base = instr.rs();
                let sign_extended_offset = instr.offset_sign_extended();
                let virt_addr = self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let aligned_addr = virt_addr & 0xffff_ffff_ffff_fffc;
                let mem = self.read_word(interconnect, aligned_addr) as u64;
                let reg = self.read_reg_gpr(instr.rt());

                let shift = virt_addr - aligned_addr;

                let value = match shift {
                    0 => mem,
                    1 => (mem << 8) | reg & 0xff,
                    2 => (mem << 16) | reg & 0xffff,
                    3 => (mem << 24) | reg & 0xffffff,
                    _ => unreachable!(),
                };

                self.write_reg_gpr(instr.rt(), value as i32 as u64);
            }

            Lw => {
                let word = self.read_word(interconnect, self.resolve_offset(instr));
                self.write_reg_gpr(instr.rt(), word as i32 as u64);
            }

            Lbu => {
                let byte = self.read_byte(interconnect, self.resolve_offset(instr));
                self.write_reg_gpr(instr.rt(), byte as u64);
            }

            Lhu => {
                let halfword = self.read_halfword(interconnect, self.resolve_offset(instr));
                self.write_reg_gpr(instr.rt(), halfword as u64);
            }

            Lwr => {
                let base = instr.rs();
                let sign_extended_offset = instr.offset_sign_extended();
                let virt_addr = self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let aligned_addr = virt_addr & 0xffff_ffff_ffff_fffC;

                let mem = self.read_word(interconnect, aligned_addr) as u64;
                let reg = self.read_reg_gpr(instr.rt());

                let shift = virt_addr - aligned_addr;
                let value = match shift {
                    0 => (mem & 0xff00_0000) >> 24 | reg & 0xffff_ffff_ffff_ff00,
                    1 => (mem & 0xffff_0000) >> 16 | reg & 0xffff_ffff_ffff_0000,
                    2 => (mem & 0xffff_ff00) >> 8 | reg & 0xffff_ffff_ff00_0000,
                    3 => (mem & 0xffff_ffff) >> 0 | reg & 0xffff_ffff_0000_0000,
                    _ => unreachable!(),
                };

                self.write_reg_gpr(instr.rt(), value);
            }

            Lwu => {
                let word = self.read_word(interconnect, self.resolve_offset(instr));
                self.write_reg_gpr(instr.rt(), word as u64);
            }

            Sb => {
                let byte = (self.read_reg_gpr(instr.rt()) & 0xff) as u8;
                let virtual_addr = self.resolve_offset(instr);
                self.write_byte(interconnect, virtual_addr, byte);
            }

            Sh => {
                let virt_addr = self.resolve_offset(instr);
                let halfword = (self.read_reg_gpr(instr.rt()) & 0xffff) as u16;
                self.write_byte(interconnect, virt_addr, (halfword >> 8) as u8);
                self.write_byte(interconnect, virt_addr + 1, (halfword & 0xff) as u8);
            }

            Swl => {
                let virt_addr = self.resolve_offset(instr);
                let word = self.read_reg_gpr(instr.rt());
                let base = virt_addr & 0b11;
                for i in base..4 {
                    let value = ((word >> ((3 - (i - base)) * 8)) & 0xff) as u8;
                    self.write_byte(interconnect, virt_addr + i, value);
                }
            }

            Sw => {
                let virt_addr = self.resolve_offset(instr);
                let word = self.read_reg_gpr(instr.rt()) as u32;
                self.write_word(interconnect, virt_addr, word);
            }

            Swr => {
                let virt_addr = self.resolve_offset(instr);
                let word = self.read_reg_gpr(instr.rt());
                let to = (virt_addr & 0b11) + 1;
                for i in 0..to {
                    let value = ((word >> ((to - i - 1) * 8)) & 0xff) as u8;
                    self.write_byte(interconnect, virt_addr + i, value);
                }
            }

            Cache => {
                // We are basically doing nothing here...
                let base = instr.rs();
                let _op = instr.rt();
                let sign_extended_offset = instr.offset_sign_extended();
                let _virt_addr = self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                // println!("Cache {:#016X} {:#08X}", virt_addr, op);
            }

            Lwc1 => {
                // TODO: Assuming here that FR is 1
                let base = instr.rs();

                let sign_extended_offset = instr.offset_sign_extended();
                let virt_addr = self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = self.read_word(interconnect, virt_addr);
                let reg = self.read_reg_fpr(instr.rt()).to_bits();
                let reg = (reg & 0xffffffff_00000000) | (mem as u64);

                self.write_reg_fpr(instr.rt(), f64::from_bits(reg));
            }

            Ldc1 => {
                let base = instr.rs();

                let sign_extended_offset = instr.offset_sign_extended();
                let virt_addr = self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = f64::from_bits(self.read_doubleword(interconnect, virt_addr));

                self.write_reg_fpr(instr.ft(), mem);
            }

            Swc1 => {
                // TODO: Assuming here that FR is 1
                let virt_addr = self.resolve_offset(instr);

                let word = self.read_reg_fpr(instr.rt()).to_bits();
                self.write_word(interconnect, virt_addr, word as u32);
            }

            Sdc1 => {
                let base = instr.rs();

                let sign_extended_offset = instr.offset_sign_extended();
                let virt_addr = self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = self.read_reg_fpr(instr.rt()).to_bits();

                self.write_doubleword(interconnect, virt_addr, mem)
            }

            Ld => {
                let base = instr.rs();

                let sign_extended_offset = instr.offset_sign_extended();
                let virt_addr = self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = self.read_doubleword(interconnect, virt_addr);

                self.write_reg_gpr(instr.rt(), mem);
            }

            Sd => {
                let base = instr.rs();

                let sign_extended_offset = instr.offset_sign_extended();
                let virt_addr = self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = self.read_reg_gpr(instr.rt());

                self.write_doubleword(interconnect, virt_addr, mem)
            }
        }
    }

    fn imm_instr<F>(&mut self, instr: Instruction, sign_extend_result: SignExtendResult, f: F)
    where
        F: FnOnce(u64, u64, u64) -> u64,
    {
        let rs = self.read_reg_gpr(instr.rs());
        let imm = instr.imm() as u64;
        let imm_sign_extended = instr.imm_sign_extended();
        let value = f(rs, imm, imm_sign_extended);
        let sign_extended_value = (value as i32) as u64;
        let value = match sign_extend_result {
            SignExtendResult::Yes => sign_extended_value,
            _ => value,
        };
        self.write_reg_gpr(instr.rt(), value);
    }

    fn reg_instr<F>(&mut self, instr: Instruction, sign_extend_result: SignExtendResult, f: F)
    where
        F: FnOnce(u64, u64, u32) -> u64,
    {
        let rs = self.read_reg_gpr(instr.rs());
        let rt = self.read_reg_gpr(instr.rt());
        let sa = instr.sa();
        let value = f(rs, rt, sa);
        let sign_extended_value = (value as i32) as u64;
        let value = match sign_extend_result {
            SignExtendResult::Yes => sign_extended_value,
            _ => value,
        };
        self.write_reg_gpr(instr.rd() as usize, value);
    }

    fn cp1_double_instr<F>(&mut self, instr: Instruction, f: F)
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let fs = self.read_reg_fpr(instr.fs());
        let ft = self.read_reg_fpr(instr.ft());
        self.write_reg_fpr(instr.fd(), f(fs, ft));
    }
    fn cp1_single_instr<F>(&mut self, instr: Instruction, f: F)
    where
        F: FnOnce(f32, f32) -> f32,
    {
        let fs = self.read_reg_fpr_single(instr.fs());
        let ft = self.read_reg_fpr_single(instr.ft());
        self.write_reg_fpr_single(instr.fd(), f(fs, ft));
    }
    fn cp1_double_to_doubleword_instr<F>(&mut self, instr: Instruction, f: F)
    where
        F: FnOnce(f64, f64) -> u64,
    {
        let fs = self.read_reg_fpr(instr.fs());
        let ft = self.read_reg_fpr(instr.ft());
        self.write_reg_fpr(instr.fd(), f64::from_bits(f(fs, ft)));
    }
    fn cp1_single_to_doubleword_instr<F>(&mut self, instr: Instruction, f: F)
    where
        F: FnOnce(f32, f32) -> u64,
    {
        let fs = self.read_reg_fpr_single(instr.fs());
        let ft = self.read_reg_fpr_single(instr.ft());
        self.write_reg_fpr(instr.fd(), f64::from_bits(f(fs, ft)));
    }
    fn cp1_double_to_word_instr<F>(&mut self, instr: Instruction, f: F)
    where
        F: FnOnce(f64, f64) -> u32,
    {
        let fs = self.read_reg_fpr(instr.fs());
        let ft = self.read_reg_fpr(instr.ft());
        self.write_reg_fpr_single(instr.fd(), f32::from_bits(f(fs, ft)));
    }
    fn cp1_single_to_word_instr<F>(&mut self, instr: Instruction, f: F)
    where
        F: FnOnce(f32, f32) -> u32,
    {
        let fs = self.read_reg_fpr_single(instr.fs());
        let ft = self.read_reg_fpr_single(instr.ft());
        self.write_reg_fpr_single(instr.fd(), f32::from_bits(f(fs, ft)));
    }
    fn cp1_double_cmp_instr<F>(&mut self, instr: Instruction, f: F)
    where
        F: FnOnce(f64, f64) -> bool,
    {
        let fs = self.read_reg_fpr(instr.fs());
        let ft = self.read_reg_fpr(instr.ft());
        let c = f(fs, ft);
        if c {
            self.reg_fcr31 = self.reg_fcr31 | 0x00400000;
        } else {
            self.reg_fcr31 = self.reg_fcr31 & 0xFFBFFFFF;
        }
    }
    fn cp1_single_cmp_instr<F>(&mut self, instr: Instruction, f: F)
    where
        F: FnOnce(f32, f32) -> bool,
    {
        let fs = self.read_reg_fpr_single(instr.fs());
        let ft = self.read_reg_fpr_single(instr.ft());
        let c = f(fs, ft);
        if c {
            self.reg_fcr31 = self.reg_fcr31 | 0x00400000;
        } else {
            self.reg_fcr31 = self.reg_fcr31 & 0xFFBFFFFF;
        }
    }

    fn resolve_offset(&self, instr: Instruction) -> u64 {
        let base = instr.rs();
        let sign_extended_offset = instr.offset_sign_extended();
        let virt_addr = self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
        virt_addr
    }

    fn branch<F>(&mut self, instr: Instruction, write_link: WriteLink, f: F) -> bool
    where
        F: FnOnce(u64, u64) -> bool,
    {
        let rs = self.read_reg_gpr(instr.rs());
        let rt = self.read_reg_gpr(instr.rt());
        let is_taken = f(rs, rt);

        let delay_slot_pc = self.reg_pc;

        if let WriteLink::Yes = write_link {
            let link_address = delay_slot_pc + 4;
            self.write_reg_gpr(31, link_address);
        }

        if is_taken {
            let sign_extended_offset = instr.offset_sign_extended() << 2;
            // Update PC before executing delay slot instruction
            self.reg_pc = self.reg_pc.wrapping_add(sign_extended_offset);

            self.delay_slot_pc = Some(delay_slot_pc);
        }

        is_taken
    }

    fn branch_likely<F>(&mut self, instr: Instruction, f: F)
    where
        F: FnOnce(u64, u64) -> bool,
    {
        if !self.branch(instr, WriteLink::No, f) {
            // Skip over delay slot instruction when not branching
            self.reg_pc = self.reg_pc.wrapping_add(4);
        }
    }

    fn read_doubleword(&self, interconnect: &mut Interconnect, virt_addr: u64) -> u64 {
        (self.read_word(interconnect, virt_addr + 0) as u64) << 32 | (self.read_word(interconnect, virt_addr + 4) as u64)
    }

    fn read_word(&self, interconnect: &mut Interconnect, virt_addr: u64) -> u32 {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        interconnect.read_word(phys_addr as u32)
    }

    fn read_halfword(&self, interconnect: &mut Interconnect, virt_addr: u64) -> u16 {
        let hi = self.read_byte(interconnect, virt_addr) as u16;
        let lo = self.read_byte(interconnect, virt_addr + 1) as u16;
        hi << 8 | lo
    }

    fn read_byte(&self, interconnect: &mut Interconnect, virt_addr: u64) -> u8 {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        interconnect.read_byte(phys_addr as u32)
    }

    fn write_word(&self, interconnect: &mut Interconnect, virt_addr: u64, value: u32) {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        interconnect.write_word(phys_addr as u32, value);
    }

    fn write_doubleword(&self, interconnect: &mut Interconnect, virt_addr: u64, value: u64) {
        self.write_word(interconnect, virt_addr + 0, (value >> 32) as u32);
        self.write_word(interconnect, virt_addr + 4, value as u32);
    }

    fn write_byte(&mut self, interconnect: &mut Interconnect, virt_addr: u64, value: u8) {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        interconnect.write_byte(phys_addr as u32, value);
    }

    fn virt_addr_to_phys_addr(&self, virt_addr: u64) -> u64 {
        // See Table 5-3 in the VR4300 User's Manual
        let addr_bit_values = (virt_addr >> 29) & 0b111;

        match addr_bit_values {
            // kseg0
            0b100 => virt_addr - 0xffff_ffff_8000_0000,
            // kseg1
            0b101 => virt_addr - 0xffff_ffff_a000_0000,
            // TODO
            _ => panic!("Unrecognized virtual address: {:#x}", virt_addr),
        }
    }

    fn write_reg_gpr(&mut self, index: usize, value: u64) {
        if index != 0 {
            self.reg_gpr[index] = value;
        }
    }

    fn read_reg_gpr(&self, index: usize) -> u64 {
        match index {
            0 => 0,
            _ => self.reg_gpr[index],
        }
    }

    fn write_reg_fpr(&mut self, index: usize, value: f64) {
        self.reg_fpr[index] = value;
    }
    fn write_reg_fpr_single(&mut self, index: usize, value: f32) {
        self.reg_fpr[index] = f64::from_bits(value.to_bits() as u64);
    }

    fn read_reg_fpr(&self, index: usize) -> f64 {
        self.reg_fpr[index]
    }
    fn read_reg_fpr_single(&self, index: usize) -> f32 {
        f32::from_bits(self.reg_fpr[index].to_bits() as u32)
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const REGS_PER_LINE: usize = 2;
        const REG_NAMES: [&'static str; NUM_GPR] = [
            "r0", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "s0", "s1", "s2", "s3", "s4",
            "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
        ];

        try!(write!(f, "\nCPU General Purpose Registers:"));
        for reg_num in 0..NUM_GPR {
            if (reg_num % REGS_PER_LINE) == 0 {
                try!(writeln!(f, ""));
            }
            try!(write!(
                f,
                "{reg_name}/gpr{num:02}: {value:#018X} ",
                num = reg_num,
                reg_name = REG_NAMES[reg_num],
                value = self.reg_gpr[reg_num],
            ));
        }

        try!(write!(f, "\n\nCPU Floating Point Registers:"));
        for reg_num in 0..NUM_GPR {
            if (reg_num % REGS_PER_LINE) == 0 {
                try!(writeln!(f, ""));
            }
            try!(write!(f, "fpr{num:02}: {value:21} ", num = reg_num, value = self.reg_fpr[reg_num]));
        }

        try!(writeln!(f, "\n\nCPU Special Registers:"));
        try!(writeln!(
            f,
            "\
             reg_pc: {:#018X}\n\
             reg_hi: {:#018X}\n\
             reg_lo: {:#018X}\n\
             reg_llbit: {}\n\
             reg_fcr0:  {:#010X}\n\
             reg_fcr31: {:#010X}\n\
             ",
            self.reg_pc, self.reg_hi, self.reg_lo, self.reg_llbit, self.reg_fcr0, self.reg_fcr31
        ));

        writeln!(f, "{:#?}", self.cp0)
    }
}
