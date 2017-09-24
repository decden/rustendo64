use byteorder::{BigEndian, ByteOrder};

use super::mem_map::{SP_DMEM_START};
use super::dma::DMARequest;
use super::Interconnect;

#[derive(Debug)]
pub struct RspStatusReg {
    pub halt: bool,
    pub broke: bool,
    pub interrupt_enable: bool,
    pub clear: bool,
    pub sstep: bool,
    pub interrupt_on_break: bool,
    pub signal0: bool,
    pub signal1: bool,
    pub signal2: bool,
    pub signal3: bool,
    pub signal4: bool,
    pub signal5: bool,
    pub signal6: bool,
    pub signal7: bool,
}

#[derive(Debug)]
pub struct RspRegs {
    pc: u32,

    status: RspStatusReg,

    dram_addr: u32,
    mem_addr: u32,
    dma_read: DMARequest,
}

#[derive(Debug)]
pub struct Rsp {
}

impl RspRegs {
    pub fn new() -> RspRegs {
        // TODO: Check for correct init hw state
        RspRegs {
            pc: 0,

            // I am not sure about these initial values
            status: RspStatusReg {
                halt: true,
                broke: false,
                interrupt_enable: false,
                clear: false,
                sstep: false,
                interrupt_on_break: false,
                signal0: false,
                signal1: false,
                signal2: false,
                signal3: false,
                signal4: false,
                signal5: false,
                signal6: false,
                signal7: false,
            },

            dram_addr: 0,
            mem_addr: SP_DMEM_START,
            dma_read: DMARequest::default(),
        }
    }

    pub fn get_dma_read_chunk(&mut self) -> DMARequest {
        self.dma_read.get_chunk(256) // Completely arbitrary chunk size...
    }

    pub fn write_mem_addr_reg(&mut self, value: u32) {
        self.mem_addr = value & 0x1fff + SP_DMEM_START;
    }

    pub fn write_dram_addr_reg(&mut self, value: u32) {
        self.dram_addr = value & 0x00ff_ffff;
    }

    pub fn write_rd_len_reg(&mut self, value: u32) {
        let count = (value >> 12) & 0xff;
        let skip = (value >> 20);

        if count != 0 { panic!("Multi-part DMAs not yet supported by RSP {} {}", count, skip); }

        self.dma_read = DMARequest {
            from: self.dram_addr,
            to: self.mem_addr,
            length: value & 0x0fff,
        }
    }

    // TODO: Read dma/single-step regs
    pub fn read_status_reg(&self) -> u32 {
        (if self.status.halt               { 1 } else { 0 } <<  0) |
        (if self.status.interrupt_enable   { 1 } else { 0 } <<  1) |
        (if self.dma_read.is_pending()     { 1 } else { 0 } <<  2) |
        // The following are still missing: [2] dma busy [3] dma full [4] io full [5] single step
        (if self.status.interrupt_on_break { 1 } else { 0 } <<  6) |
        (if self.status.signal0            { 1 } else { 0 } <<  7) |
        (if self.status.signal1            { 1 } else { 0 } <<  8) |
        (if self.status.signal2            { 1 } else { 0 } <<  9) |
        (if self.status.signal3            { 1 } else { 0 } << 10) |
        (if self.status.signal4            { 1 } else { 0 } << 11) |
        (if self.status.signal5            { 1 } else { 0 } << 12) |
        (if self.status.signal6            { 1 } else { 0 } << 13) |
        (if self.status.signal7            { 1 } else { 0 } << 14)
    }

    pub fn write_status_reg(&mut self, value: u32) {
        // TODO: What happens if both a set and clear bit are set?
        if (value & (1 <<  0)) != 0 { self.status.halt = false; }
        if (value & (1 <<  1)) != 0 { self.status.halt = true; }

        if (value & (1 <<  2)) != 0 { self.status.broke = false; }

        if (value & (1 <<  3)) != 0 { self.status.interrupt_enable = false; }
        if (value & (1 <<  4)) != 0 { self.status.interrupt_enable = true; }

        if (value & (1 <<  5)) != 0 { self.status.sstep = false; }
        if (value & (1 <<  6)) != 0 { self.status.sstep = true; }

        if (value & (1 <<  7)) != 0 { self.status.interrupt_on_break = false; }
        if (value & (1 <<  8)) != 0 { self.status.interrupt_on_break = true; }

        if (value & (1 <<  9)) != 0 { self.status.signal0 = false; }
        if (value & (1 << 10)) != 0 { self.status.signal0 = true; }
        if (value & (1 << 11)) != 0 { self.status.signal1 = false; }
        if (value & (1 << 12)) != 0 { self.status.signal1 = true; }
        if (value & (1 << 13)) != 0 { self.status.signal2 = false; }
        if (value & (1 << 14)) != 0 { self.status.signal2 = true; }
        if (value & (1 << 15)) != 0 { self.status.signal3 = false; }
        if (value & (1 << 16)) != 0 { self.status.signal3 = true; }
        if (value & (1 << 17)) != 0 { self.status.signal4 = false; }
        if (value & (1 << 18)) != 0 { self.status.signal4 = true; }
        if (value & (1 << 19)) != 0 { self.status.signal5 = false; }
        if (value & (1 << 20)) != 0 { self.status.signal5 = true; }
        if (value & (1 << 21)) != 0 { self.status.signal6 = false; }
        if (value & (1 << 22)) != 0 { self.status.signal6 = true; }
        if (value & (1 << 23)) != 0 { self.status.signal7 = false; }
        if (value & (1 << 24)) != 0 { self.status.signal7 = true; }

        println!("WARNING: RSP Status reg was written to {:?}", self.status);
    }

    pub fn read_dma_busy_reg(&self) -> u32 {
        println!("WARNING: Reading SP_DMA_BUSY_REG: {:#08X}", self.pc);
        0
    }

    pub fn write_dma_busy_reg(&self, value: u32) {
        panic!("Attempted write to SP_DMA_BUSY: {:#?}", value);
    }

    pub fn write_semaphore_reg(&self, value: u32) {
        println!("WARNING: Writing to SP_SEMAPHORE_REG to {:#08X}", value);
    }

    pub fn read_pc_reg(&self) -> u32 {
        println!("WARNING: Reading SP_PC_REG: {:#08X}", self.pc);
        self.pc
    }

    pub fn write_pc_reg(&mut self, value: u32) {
        self.pc = value;
        println!("WARNING: Writing to SP_PC_REG: {:#08X}", value);
    }
}

impl Rsp {
    pub fn new() -> Rsp {
        Rsp {
        }
    }

    pub fn step(&mut self, interconnect: &mut Interconnect) {
        if interconnect.rsp().status.halt {
            return;
        }
        panic!("AHHHHH RSP is running...");
    }
}