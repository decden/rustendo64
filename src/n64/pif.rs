use byteorder::{BigEndian, ByteOrder};

use super::mem_map::PIF_RAM_LENGTH;
use super::mem_map::PIF_ROM_LENGTH;

pub struct Pif {
    boot_rom: Box<[u8]>,
    ram: Box<[u8]>,
}

impl Pif {
    pub fn new(boot_rom: Box<[u8]>) -> Pif {
        let mut pif = Pif {
            boot_rom: boot_rom,

            ram: vec![0; PIF_RAM_LENGTH as usize].into_boxed_slice(),
        };
        pif.ram[0x26] = 0x3F;
        pif.ram[0x27] = 0x3F;
        pif
    }

    pub fn read_boot_rom(&self, addr: u32) -> u32 {
        BigEndian::read_u32(&self.boot_rom[addr as usize..])
    }

    pub fn read_ram(&self, addr: u32) -> u32 {
        let value = BigEndian::read_u32(&self.ram[addr as usize..]);
        println!("Reading PifRam: {:08X} {:08X}", addr, value);
        BigEndian::read_u32(&self.ram[addr as usize..])
    }

    pub fn write_ram(&mut self, addr: u32, value: u32) {
        BigEndian::write_u32(&mut self.ram[addr as usize..], value);

        // NOTE: The following PIF emualtion is very crude and might be inaccurate

        // The last byte of the following word is the PIF control byte!
        if addr == 0x3C
        {
            let pif_control_byte = value & 0xFF;
            match pif_control_byte
            {
                0x08 => self.ram[0x3f] = 0x00,
                0x10 => {
                    println!("PIF control byte was set to 10, clearing rom!");
                    self.boot_rom = vec![0; PIF_ROM_LENGTH as usize].into_boxed_slice();
                }
                0x30 => self.ram[0x3f] = 0x80,
                0xC0 => self.ram[0x3f] = 0x40,
                _ => panic!("Unknown PIF command 0x{:2X}", pif_control_byte),
            }
        }
        else {
            println!("WARNING: Wrote to PIF at {:#02X} value {:#08X}", addr, value);
        }
    }
}
