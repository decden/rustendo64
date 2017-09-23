#[derive(Debug)]
pub struct MipsIntrMaskReg
{
    sp_intr_mask: bool,
    si_intr_mask: bool,
    ai_intr_mask: bool,
    vi_intr_mask: bool,
    pi_intr_mask: bool,
    dp_intr_mask: bool,
}

pub struct MipsInterface
{
    init_length: u8,
    init_mode: bool,
    ebus_test_mode: bool,
    rdram_reg_mode: bool,

    intr_mask: MipsIntrMaskReg,
}

impl MipsInterface {
    pub fn new() -> MipsInterface {
        MipsInterface {
            init_length: 0,
            init_mode: false,
            ebus_test_mode: false,
            rdram_reg_mode: false,

            intr_mask: MipsIntrMaskReg {
                sp_intr_mask: false,
                si_intr_mask: false,
                ai_intr_mask: false,
                vi_intr_mask: false,
                pi_intr_mask: false,
                dp_intr_mask: false,
            }
        }
    }

    pub fn read_mode_reg(&self) -> u32 {
        self.init_length as u32 |
        if self.init_mode      { 0b0000_0000_1000_0000 } else { 0 } |
        if self.ebus_test_mode { 0b0000_0001_0000_0000 } else { 0 } |
        if self.rdram_reg_mode { 0b0000_0010_0000_0000 } else { 0 }
    }

    pub fn read_version_reg(&self) -> u32 {
        let io_version: u32 = 1;
        let rac_version: u32 = 1;
        let rdp_version: u32 = 1;
        let rsp_version: u32 = 1;
        io_version | (rac_version << 8) | (rdp_version << 16) | (rsp_version << 24)
    }

    pub fn write_mode_reg(&mut self, value: u32) {
        self.init_length = (value & 0x7f) as u8;
        if ((value >>  7) & 0x01) != 0 { self.init_mode = false; }
        if ((value >>  8) & 0x01) != 0 { self.init_mode = true; }
        if ((value >>  9) & 0x01) != 0 { self.ebus_test_mode = false; }
        if ((value >> 10) & 0x01) != 0 { self.ebus_test_mode = true; }
        if ((value >> 11) & 0x01) != 0 { /* clear dp interrupt */ }
        if ((value >> 12) & 0x01) != 0 { self.rdram_reg_mode = false; }
        if ((value >> 13) & 0x01) != 0 { self.rdram_reg_mode = true; }
        println!("WARNING: Stub for write MI mode register {:08X}", value);
    }

    pub fn write_intr_mask_reg(&mut self, value: u32) {
        if (value & (1 <<  0)) != 0  { self.intr_mask.sp_intr_mask = false; }
        if (value & (1 <<  1)) != 0  { self.intr_mask.sp_intr_mask = true; }

        if (value & (1 <<  2)) != 0  { self.intr_mask.si_intr_mask = false; }
        if (value & (1 <<  3)) != 0  { self.intr_mask.si_intr_mask = true; }

        if (value & (1 <<  4)) != 0  { self.intr_mask.ai_intr_mask = false; }
        if (value & (1 <<  5)) != 0  { self.intr_mask.ai_intr_mask = true; }

        if (value & (1 <<  6)) != 0  { self.intr_mask.vi_intr_mask = false; }
        if (value & (1 <<  7)) != 0  { self.intr_mask.vi_intr_mask = true; }

        if (value & (1 <<  8)) != 0  { self.intr_mask.pi_intr_mask = false; }
        if (value & (1 <<  9)) != 0  { self.intr_mask.pi_intr_mask = true; }

        if (value & (1 << 10)) != 0  { self.intr_mask.dp_intr_mask = false; }
        if (value & (1 << 11)) != 0  { self.intr_mask.dp_intr_mask = true; }
        println!("WARNING: Writing to MI_INTR_MASK_REG {:?}", self.intr_mask);
    }
}
