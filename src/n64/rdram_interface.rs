#[derive(Default)]
pub struct RdramInterface;

impl RdramInterface {

    pub fn read_select_reg(&self) -> u32 {
        println!("WARNING: Stub for read RI select register");
        0
    }

    pub fn read_refresh_reg(&self) -> u32 {
        println!("WARNING: Stub for read RI refresh register");
        0
    }

    pub fn write_mode_reg(&self, value: u32) {
        println!("WARNING: Stub for write RI mode register {:08X}", value);
    }

    pub fn write_config_reg(&self, value: u32) {
        println!("WARNING: Stub for write RI config register {:08X}", value);
    }

    pub fn write_current_load_reg(&self, value: u32) {
        println!("WARNING: Stub for write RI current load register {:08X}", value);
    }

    pub fn write_select_reg(&self, value: u32) {
        println!("WARNING: Stub for write RI select register {:08X}", value);
    }

    pub fn write_refresh_reg(&self, value: u32) {
        println!("WARNING: Stub for write RI refresh register {:08X}", value);
    }



    pub fn read_rdram_mode_reg(&self) -> u32 {
        println!("WARNING: Stub for read RI mode register");
        0
    }
    
    pub fn write_rdram_mode_reg(&self, value: u32) {
        println!("WARNING: Stub for write RDRAM mode register {:08X}", value);
    }
}
