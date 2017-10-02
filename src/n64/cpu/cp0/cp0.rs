use super::{reg_config, reg_cause, reg_status};

#[derive(Debug, Default)]
pub struct Cp0 {
    reg_count: u32,
    reg_compare: u32,
    reg_status: reg_status::RegStatus,
    reg_config: reg_config::RegConfig,
    reg_cause: reg_cause::RegCause,

    reg_watch_lo: u32,
    reg_watch_hi: u32,

    reg_tag_lo: u32,
    reg_tag_hi: u32,
}

impl Cp0 {
    pub fn write_reg(&mut self, index: u32, data: u64) {
        println!("CP0 Instr {:?}, {:#016}", index, data);
        match index {
             0 => { println!("WARNING: Writing to Index CP0 Reg {:08X}", data) }
             2 => { println!("WARNING: Writing to EntryLo0 CP0 Reg {:08X}", data) }
             3 => { println!("WARNING: Writing to EntryLo1 CP0 Reg {:08X}", data) }
             5 => { println!("WARNING: Writing to PageMask CP0 Reg {:08X}", data) }
             9 => self.reg_count = data as u32,
            11 => {
                self.reg_compare = data as u32;
                self.reg_cause.clearTimerInterruptPending();
                println!("A timer interrupt is set to trigger when the counter reaches {:08X}", data as u32);
            },
            10 => { println!("WARNING: Writing to EntryHi CP0 Reg {:08X}", data) }
            12 => self.reg_status = (data as u32).into(),
            13 => {
                // Only software interrupt pending flags can be written to
                self.reg_cause.setSoftwareInterruptPendingFields(data as u32);
            }
            16 => self.reg_config = (data as u32).into(),
            18 => self.reg_watch_lo = (data as u32),
            29 => self.reg_watch_hi = (data as u32),
            28 => self.reg_tag_lo = (data as u32),
            29 => self.reg_tag_hi = (data as u32),
            _ => panic!("Unrecognized Cp0 reg: {}, {:#018x}", index, data),
        }
    }
}
