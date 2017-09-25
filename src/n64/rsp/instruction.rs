use std::fmt;

use super::opcode::{RspOpcode, RspSpecialOpcode};

use num::FromPrimitive;

#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

impl Instruction {
    #[inline(always)]
    pub fn opcode(&self) -> RspOpcode {
        let value = (self.0 >> 26) & 0b111111;
        RspOpcode::from_u32(value).unwrap_or_else(|| {
            panic!("Unrecognized RSP instruction: {:#010x} (op: {:#08b})", self.0, value)
        })
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
    pub fn target(&self) -> u32 {
        self.0 & 0x3FFFFFF
    }

    #[inline(always)]
    pub fn offset(&self) -> u32 {
        self.imm()
    }

    #[inline(always)]
    pub fn offset_sign_extended(&self) -> u32 {
        (self.offset() as i16) as u32
    }

    #[inline(always)]
    pub fn special_op(&self) -> RspSpecialOpcode {
        let value = self.0 & 0b111111;
        RspSpecialOpcode::from_u32(value).unwrap_or_else(|| {
            panic!("Unrecognized RSP special opcode: {:#010x} (op: {:#08b})", self.0, value)
        })
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.opcode() {
            _ => write!(f, "{:?}", self.opcode()),
        }
    }
}
