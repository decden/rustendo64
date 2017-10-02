use byteorder::{BigEndian, ByteOrder};

use super::mem_map::PIF_RAM_LENGTH;
use super::mem_map::PIF_ROM_LENGTH;

pub struct Pif {
    boot_rom: Box<[u8]>,
    ram: Box<[u8]>,
}

fn crc32(rom: &[u8]) -> u32 {
    let mut table: [u32; 256] = [0; 256];

    for n in 0..256 {
        let mut c: u32 = n;
        for k in 0..8 {
            c = if (c & 1 != 0) { 0xEDB88320 ^ (c >> 1) } else { c >> 1 };
        } 

        table[n as usize] = c;
    }

    let mut c: u32 = 0xffffffff;
    for n in 0..rom.len() {
        c = table[((c ^ rom[n] as u32) & 0xFF) as usize] ^ (c >> 8);
    }

    !c
}

impl Pif {
    pub fn new(boot_rom: Box<[u8]>) -> Pif {
        Pif {
            boot_rom: boot_rom,

            ram: vec![0; PIF_RAM_LENGTH as usize].into_boxed_slice(),
        }
    }


    pub fn init_cic_seed(&mut self, rom: &[u8]) {
        let crc = crc32(&rom[0x40..0x1000]);
        let seed = match crc {
            0x90BB_6CB5 => 0x0000_3F3F, // 6102
            0x98BC_2C86 => 0x0000_913F, // 6105
            _ => panic!("WARNING: Unknown CRC {:08X}", crc),
        };

        self.ram[0x24] = ((seed >> 24) & 0xff) as u8;
        self.ram[0x25] = ((seed >> 16) & 0xff) as u8;
        self.ram[0x26] = ((seed >>  8) & 0xff) as u8;
        self.ram[0x27] = ((seed >>  0) & 0xff) as u8;
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
                _ => println!("Unknown PIF command 0x{:2X}", pif_control_byte),
            }
        }
        else {
            println!("WARNING: Wrote to PIF at {:#02X} value {:#08X}", addr, value);
        }
    }
}
