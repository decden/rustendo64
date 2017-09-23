use std::cmp;
use super::Interconnect;

#[derive(Debug)]
pub struct DMAWrite {
    pub from_cart_addr: u32,
    pub to_dram_addr: u32,
    pub length: u32,
}

#[derive(Default)]
pub struct PeripheralInterface
{
    dram_addr: u32,
    cart_addr: u32,

    pending_wr_dma_op: Option<DMAWrite>,
}

impl PeripheralInterface {

    pub fn get_dma_chunk(&mut self) -> Option<DMAWrite>
    {
        let mut clear_dma = false;
        let chunk = match self.pending_wr_dma_op.as_mut() {
            None => None,
            Some(ref mut dma) => {
                let chunk = DMAWrite {
                    from_cart_addr: dma.from_cart_addr,
                    to_dram_addr:   dma.to_dram_addr,
                    length:         cmp::min(256, dma.length), // Completely arbitrary chunk size...
                };
                dma.from_cart_addr += chunk.length;
                dma.to_dram_addr += chunk.length;
                dma.length -= chunk.length;
                if dma.length == 0 { clear_dma = true; }
                Some(chunk)
            }
        };

        if clear_dma { self.pending_wr_dma_op = None; }

        chunk
    }

    pub fn write_dram_addr_reg(&mut self, value: u32) {
        self.dram_addr = value & 0x00ff_ffff;
    }

    pub fn write_cart_addr_reg(&mut self, value: u32) {
        self.cart_addr = value;
    }

    pub fn write_wr_len_reg(&mut self, value: u32) {
        self.pending_wr_dma_op = Some(DMAWrite {
            from_cart_addr: self.cart_addr,
            to_dram_addr: self.dram_addr,
            length: (value & 0x00ff_fffe) + 2,
        });
    }

    pub fn read_status_reg(&self) -> u32 {
        // TODO: Verify if this is correct
        if let Some(_) = self.pending_wr_dma_op { 1 } else { 0 }
    }

    pub fn write_status_reg(&mut self, value: u32) {
        if (value & (1 << 0)) != 0 {
            println!("WARNING: PI reset controller bit written but not yet implemented");
        }

        if (value & (1 << 1)) != 0 {
            // TODO: Affect MI_INTR_REG
            println!("WARNING: PI clear intr bit written but not yet implemented");
        }
    }

    pub fn read_bsd_dom1_lat_reg(&self) -> u32 {
        // TODO: Proper impl (probably not necessary)
        0
    }

    pub fn write_bsd_dom1_lat_reg(&mut self, value: u32) {
        // TODO: Proper impl (probably not necessary)
        println!("PI_BSD_DOM1_LAT_REG written: {:#x}", value);
    }

    pub fn read_bsd_dom1_pwd_reg(&self) -> u32 {
        // TODO: Proper impl (probably not necessary)
        0
    }

    pub fn write_bsd_dom1_pwd_reg(&mut self, value: u32) {
        // TODO: Proper impl (probably not necessary)
        println!("PI_BSD_DOM1_PWD_REG written: {:#x}", value);
    }

    pub fn read_bsd_dom1_pgs_reg(&self) -> u32 {
        // TODO: Proper impl (probably not necessary)
        0
    }

    pub fn write_bsd_dom1_pgs_reg(&mut self, value: u32) {
        // TODO: Proper impl (probably not necessary)
        println!("PI_BSD_DOM1_PGS_REG written: {:#x}", value);
    }

    pub fn read_bsd_dom1_rls_reg(&self) -> u32 {
        // TODO: Proper impl (probably not necessary)
        0
    }

    pub fn write_bsd_dom1_rls_reg(&mut self, value: u32) {
        // TODO: Proper impl (probably not necessary)
        println!("PI_BSD_DOM1_RLS_REG written: {:#x}", value);
    }
}
