use super::{reg_config, reg_cause, reg_status};

#[derive(Debug, Default)]
pub struct Cp0 {
    reg_index: u64,
    reg_entry_lo0: u64,
    reg_entry_lo1: u64,
    reg_entry_hi: u64,
    reg_page_mask: u64,

    reg_count: u32,
    reg_compare: u32,
    reg_status: reg_status::RegStatus,
    reg_cause: reg_cause::RegCause,
    reg_epc: u64, // Exception program counter
    reg_config: reg_config::RegConfig,

    reg_watch_lo: u32,
    reg_watch_hi: u32,

    reg_tag_lo: u32,
    reg_tag_hi: u32,

    tlb_entries: [[u64; 4]; 32],
}

impl Cp0 {
    pub fn store_tlb_entry(&mut self) {
        let index = (self.reg_index & 0b11111) as usize;

        // TODO: Veriy that this is correct
        self.tlb_entries[index][0] = self.reg_page_mask;
        self.tlb_entries[index][1] = (self.reg_entry_hi & !self.reg_page_mask)
            | if ((self.reg_entry_lo0 & self.reg_entry_lo1) & 0b1) != 0 { 0b1000000000000 } else { 0 }
            | self.reg_entry_hi & 0xff;
        self.tlb_entries[index][2] = self.reg_entry_lo0 & 0xffffffff_fffffffe;
        self.tlb_entries[index][3] = self.reg_entry_hi & 0xffffffff_fffffffe;
    }

    pub fn write_reg(&mut self, index: u32, data: u64) {
        println!("CP0 Instr {:2}, {:#018X}", index, data);
        match index {
             0 => { self.reg_index = (data & 0x8000_003F); }
             2 => { self.reg_entry_lo0 = data; }
             3 => { self.reg_entry_lo1 = data; }
             5 => { self.reg_page_mask = data; }
             9 => self.reg_count = data as u32,
            10 => { self.reg_entry_hi = data; }
            11 => {
                self.reg_compare = data as u32;
                self.reg_cause.clearTimerInterruptPending();
                println!("A timer interrupt is set to trigger when the counter reaches {:08X}", data as u32);
            }
            10 => { println!("WARNING: Writing to EntryHi CP0 Reg {:08X}", data) }
            12 => self.reg_status = (data as u32).into(),
            13 => {
                // Only software interrupt pending flags can be written to
                self.reg_cause.setSoftwareInterruptPendingFields(data as u32);
            }
            14 => { self.reg_epc = data; }
            16 => self.reg_config = (data as u32).into(),
            18 => self.reg_watch_lo = (data as u32),
            29 => self.reg_watch_hi = (data as u32),
            28 => self.reg_tag_lo = (data as u32),
            29 => self.reg_tag_hi = (data as u32),
            _ => panic!("Unrecognized CP0 reg: {}, {:#018x}", index, data),
        }
    }
    pub fn read_reg(&mut self, index: u32) -> u64 {
        println!("CP0 Read {:2}", index);
        match index {
            10 => { 0 /* TODO */ },
            12 => { 0 /* TODO */ }
            13 => { self.reg_cause.to_u32() as u64 }
            14 => { self.reg_epc }
            _ => panic!("Trying to read CP0 reg {}", index)
        }
    }
}
