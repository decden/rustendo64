use std::fmt;

use super::opcode::{Cop0CoOpcode, Cop0Opcode, Cop1Opcode, Opcode, RegImmOpcode, SpecialOpcode};

use num::FromPrimitive;

#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

impl Instruction {
    #[inline(always)]
    pub fn opcode(&self) -> Opcode {
        let value = (self.0 >> 26) & 0b111111;
        Opcode::from_u32(value).unwrap_or_else(|| panic!("Unrecognized instruction: {:#010x} (op: {:#08b})", self.0, value))
    }

    #[inline(always)]
    pub fn rs(&self) -> usize {
        ((self.0 >> 21) & 0b11111) as usize
    }

    #[inline(always)]
    pub fn rt(&self) -> usize {
        ((self.0 >> 16) & 0b11111) as usize
    }

    #[inline(always)]
    pub fn fmt(&self) -> usize {
        self.rs()
    }
    #[inline(always)]
    pub fn ft(&self) -> usize {
        ((self.0 >> 16) & 0b11111) as usize
    }
    #[inline(always)]
    pub fn fs(&self) -> usize {
        ((self.0 >> 11) & 0b11111) as usize
    }
    #[inline(always)]
    pub fn fd(&self) -> usize {
        ((self.0 >> 6) & 0b11111) as usize
    }

    #[inline(always)]
    pub fn rd(&self) -> u32 {
        (self.0 >> 11) & 0b11111
    }

    #[inline(always)]
    pub fn sa(&self) -> u32 {
        (self.0 >> 6) & 0b11111
    }

    #[inline(always)]
    pub fn imm(&self) -> u32 {
        self.0 & 0xffff
    }

    #[inline(always)]
    pub fn imm_sign_extended(&self) -> u64 {
        (self.imm() as i16) as u64
    }

    #[inline(always)]
    pub fn target(&self) -> u32 {
        self.0 & 0x3FFFFFF
    }

    #[inline(always)]
    pub fn offset(&self) -> u32 {
        self.imm()
    }

    #[inline(always)]
    pub fn offset_sign_extended(&self) -> u64 {
        (self.offset() as i16) as u64
    }

    #[inline(always)]
    pub fn special_op(&self) -> SpecialOpcode {
        let value = self.0 & 0b111111;
        SpecialOpcode::from_u32(value).unwrap_or_else(|| panic!("Unrecognized special opcode: {:#010x} (op: {:#08b})", self.0, value))
    }

    #[inline(always)]
    pub fn reg_imm_op(&self) -> RegImmOpcode {
        let value = (self.0 >> 16) & 0b11111;
        RegImmOpcode::from_u32(value).unwrap_or_else(|| panic!("Unrecognized reg imm opcode: {:#010x} (op: {:#07b})", self.0, value))
    }

    #[inline(always)]
    pub fn cop0_op(&self) -> Cop0Opcode {
        let value = (self.0 >> 21) & 0b11111;
        Cop0Opcode::from_u32(value).unwrap_or_else(|| panic!("Unrecognized COP0 opcode: {:#010x} (op: {:#07b})", self.0, value))
    }

    #[inline(always)]
    pub fn cop0_co_op(&self) -> Cop0CoOpcode {
        let value = self.0 & 0b111111;
        Cop0CoOpcode::from_u32(value).unwrap_or_else(|| panic!("Unrecognized COP0 CO opcode: {:#010x} (op: {:#08b})", self.0, value))
    }

    #[inline(always)]
    pub fn cop1_op(&self) -> Cop1Opcode {
        let value = self.0 & 0b111111;
        Cop1Opcode::from_u32(value).unwrap_or_else(|| panic!("Unrecognized COP1 opcode: {:#010x} (op: {:#08b})", self.0, value))
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.opcode() {
            Opcode::Special => write!(f, "{:?}", self.special_op()),
            Opcode::RegImm => write!(f, "{:?}", self.reg_imm_op()),
            _ => write!(f, "{:?}", self.opcode()),
        }
    }
}
