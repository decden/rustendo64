#[derive(Default)]
pub struct AudioInterface {
    dram_addr: u32,
    length: u32,
}

impl AudioInterface {
    pub fn read_dram_addr_reg(&self) -> u32 {
        self.dram_addr
    }

    pub fn write_dram_addr_reg(&mut self, value: u32) {
        self.dram_addr = value & 0x00ff_ffff;
    }

    pub fn read_len_reg(&self) -> u32 {
        self.length
    }

    pub fn write_len_reg(&mut self, value: u32) {
        self.length = value & 0x0003_fff8;
    }

    pub fn write_control_reg(&mut self, value: u32) {
        // TODO: 
        println!("WARNING: Writing to AI_CONTROL_REG {:08X}", value);
    }

    pub fn write_status_reg(&mut self, value: u32) {
        // TODO: 
        println!("WARNING: Writing to AI_STATUS_REG {:08X}", value);
    }

    pub fn write_dacrate_reg(&mut self, value: u32) {
        // TODO: 
        println!("WARNING: Writing to AI_DACRATE_REG {:08X}", value);
    }

    pub fn write_bitrate_reg(&mut self, value: u32) {
        // TODO: 
        println!("WARNING: Writing to AI_BITRATE_REG {:08X}", value);
    }
}
