use super::Interconnect;
use super::dma::DMARequest;

#[derive(Default)]
pub struct PeripheralInterface
{
    dram_addr: u32,
    cart_addr: u32,

    dma_write: DMARequest,
}

impl PeripheralInterface {

    pub fn get_dma_write_chunk(&mut self) -> DMARequest {
        self.dma_write.get_chunk(0x10000) // Completely arbitrary chunk size
    }

    pub fn write_dram_addr_reg(&mut self, value: u32) {
        self.dram_addr = value & 0x00ff_ffff;
    }

    pub fn write_cart_addr_reg(&mut self, value: u32) {
        self.cart_addr = value;
    }

    pub fn write_wr_len_reg(&mut self, value: u32) {
        self.dma_write = DMARequest {
            from: self.cart_addr,
            to: self.dram_addr,
            length: (value & 0x00ff_fffe) + 2,
        };
    }

    pub fn read_status_reg(&self) -> u32 {
        // TODO: Verify if this is correct
        if self.dma_write.is_pending() { 1 } else { 0 }
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
