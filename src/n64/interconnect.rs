use byteorder::{BigEndian, ByteOrder};

use super::{AudioInterface, PeripheralInterface, Pif, Rdp, RspRegs, MipsInterface,
            SerialInterface, RdramInterface, VideoInterface};
use super::mem_map::{self, Addr};
use super::mem_map::RDRAM_LENGTH;
use super::dma::DMARequest;
use super::sinks::{Sink, VideoFrame};
use super::video_interface::{FramebufferFormat};

use std::fmt;

pub struct Interconnect {
    rdram: Box<[u8]>,

    pif: Pif,

    rsp: RspRegs,
    rdp: Rdp,

    mi: MipsInterface,
    ai: AudioInterface,
    vi: VideoInterface,

    pi: PeripheralInterface,
    ri: RdramInterface,
    si: SerialInterface,

    cart_rom: Box<[u8]>,

    steps_to_next_frame: u32,
}

impl Interconnect {
    pub fn new(boot_rom: Box<[u8]>, cart_rom: Box<[u8]>) -> Interconnect {
        let mut interconnect = Interconnect {
            rdram: vec![0; RDRAM_LENGTH as usize].into_boxed_slice(),

            pif: Pif::new(boot_rom),

            rdp: Rdp,
            rsp: RspRegs::new(),

            mi: MipsInterface::new(),
            ai: AudioInterface::default(),
            vi: VideoInterface::new(),

            pi: PeripheralInterface::default(),
            ri: RdramInterface::default(),
            si: SerialInterface::default(),

            cart_rom: cart_rom,

            steps_to_next_frame: 100000,
        };

        interconnect.pif.init_cic_seed(&*interconnect.cart_rom);
        interconnect
    }

    pub fn pif(&self) -> &Pif { &self.pif }
    pub fn rsp(&mut self) -> &mut RspRegs { &mut self.rsp }

    pub fn read_word_debug(&self, addr: u32) -> Option<u32> {
        let mapped_address = mem_map::map_addr(addr);
        match mapped_address {
            Addr::RdramMemory(offset) => Some(BigEndian::read_u32(&self.rdram[offset as usize..])),

            Addr::SpDmem(offset) => Some(self.rsp.read_dmem(offset)),
            Addr::SpImem(offset) => Some(self.rsp.read_imem(offset)),

            Addr::PifRom(offset) => Some(self.pif.read_boot_rom(offset)),
            Addr::PifRam(offset) => Some(self.pif.read_ram(offset)), // This could modify state

            _ => {
                println!("Could not debug address {:?}", mapped_address);
                None
            },
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        let mapped_address = mem_map::map_addr(addr);
        // println!("Reading {:?} ", mapped_address);
        let word = match mapped_address {
            Addr::RdramMemory(offset) => BigEndian::read_u32(&self.rdram[offset as usize..]),
            Addr::RdramModeReg => self.ri.read_rdram_mode_reg(),
            Addr::SpDmem(offset) => self.rsp.read_dmem(offset),
            Addr::SpImem(offset) => self.rsp.read_imem(offset),

            Addr::SpStatusReg => self.rsp.read_status_reg(),
            Addr::SpDmaBusyReg => self.rsp.read_dma_busy_reg(),
            Addr::SpPcReg => self.rsp.read_pc_reg(),

            Addr::DpcStatusReg => self.rdp.read_status_reg(),

            Addr::MiModeReg => self.mi.read_mode_reg(),
            Addr::MiVersionReg => self.mi.read_version_reg(),

            Addr::ViIntrReg => self.vi.read_intr_reg(),
            Addr::ViCurrentReg => self.vi.read_current_reg(),
            Addr::ViHStartReg => self.vi.read_h_start_reg(),

            Addr::AiDramAddrReg => self.ai.read_dram_addr_reg(),
            Addr::AiLenReg => self.ai.read_len_reg(),

            Addr::PiStatusReg => self.pi.read_status_reg(),
            Addr::PiBsdDom1LatReg => self.pi.read_bsd_dom1_lat_reg(),
            Addr::PiBsdDom1PwdReg => self.pi.read_bsd_dom1_pwd_reg(),
            Addr::PiBsdDom1PgsReg => self.pi.read_bsd_dom1_pgs_reg(),
            Addr::PiBsdDom1RlsReg => self.pi.read_bsd_dom1_rls_reg(),
            Addr::PiBsdDom2LatReg => self.pi.read_bsd_dom2_lat_reg(),
            Addr::PiBsdDom2PwdReg => self.pi.read_bsd_dom2_pwd_reg(),
            Addr::PiBsdDom2PgsReg => self.pi.read_bsd_dom2_pgs_reg(),
            Addr::PiBsdDom2RlsReg => self.pi.read_bsd_dom2_rls_reg(),

            Addr::RiSelectReg => self.ri.read_select_reg(),
            Addr::RiRefreshReg => self.ri.read_refresh_reg(),

            Addr::SiStatusReg => self.si.read_status_reg(),

            Addr::CartDom1(offset) => BigEndian::read_u32(&self.cart_rom[offset as usize..]),

            Addr::PifRom(offset) => self.pif.read_boot_rom(offset),
            Addr::PifRam(offset) => self.pif.read_ram(offset),

            _ => panic!("Unknown physical address {:?} for read", mapped_address),
        };
        word
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        let mapped_address = mem_map::map_addr(addr);
        // println!("Writing {:?} {:08X}", mapped_address, value);
        match mapped_address {
            Addr::RdramMemory(offset) => BigEndian::write_u32(&mut self.rdram[offset as usize..], value),
            Addr::RdramModeReg => self.ri.write_rdram_mode_reg(value),
            Addr::RdramUnknownReg(_) => println!("WARNING: Discarding write to {:?}", mapped_address),

            Addr::SpDmem(offset) => self.rsp.write_dmem(offset, value),
            Addr::SpImem(offset) => self.rsp.write_imem(offset, value),

            Addr::SpMemAddrReg => self.rsp.write_mem_addr_reg(value),
            Addr::SpDramAddrReg => self.rsp.write_dram_addr_reg(value),
            Addr::SpRdLenReg => self.rsp.write_rd_len_reg(value),
            Addr::SpStatusReg => self.rsp.write_status_reg(value),
            Addr::SpDmaBusyReg => self.rsp.write_dma_busy_reg(value),
            Addr::SpSemaphoreReg => self.rsp.write_semaphore_reg(value),
            Addr::SpPcReg => self.rsp.write_pc_reg(value),

            Addr::DpcStatusReg => self.rdp.write_status_reg(value),

            Addr::MiModeReg => self.mi.write_mode_reg(value),
            Addr::MiIntrMaskReg => self.mi.write_intr_mask_reg(value),

            Addr::ViStatusReg => self.vi.write_status_reg(value),
            Addr::ViOriginReg => self.vi.write_origin_reg(value),
            Addr::ViWidthReg => self.vi.write_width_reg(value),
            Addr::ViIntrReg => self.vi.write_intr_reg(value),
            Addr::ViCurrentReg => self.vi.write_current_reg(value),
            Addr::ViTimingReg => self.vi.write_timing_reg(value),
            Addr::ViVSyncReg => self.vi.write_v_sync_reg(value),
            Addr::ViHSyncReg => self.vi.write_h_sync_reg(value),
            Addr::ViHSyncLeapReg => self.vi.write_h_sync_leap_reg(value),
            Addr::ViHStartReg => self.vi.write_h_start_reg(value),
            Addr::ViVStartReg => self.vi.write_v_start_reg(value),
            Addr::ViVBurstReg => self.vi.write_v_burst_reg(value),
            Addr::ViXScaleReg => self.vi.write_x_scale_reg(value),
            Addr::ViYScaleReg => self.vi.write_y_scale_reg(value),

            Addr::AiDramAddrReg => self.ai.write_dram_addr_reg(value),
            Addr::AiLenReg => self.ai.write_len_reg(value),
            Addr::AiControlReg => self.ai.write_control_reg(value),
            Addr::AiStatusReg => self.ai.write_status_reg(value),
            Addr::AiDacrateReg => self.ai.write_dacrate_reg(value),
            Addr::AiBitrateReg => self.ai.write_bitrate_reg(value),

            Addr::PiDramAddrReg => self.pi.write_dram_addr_reg(value),
            Addr::PiCartAddrReg => self.pi.write_cart_addr_reg(value),
            Addr::PiWrLenReg => self.pi.write_wr_len_reg(value),
            Addr::PiStatusReg => self.pi.write_status_reg(value),
            Addr::PiBsdDom1LatReg => self.pi.write_bsd_dom1_lat_reg(value),
            Addr::PiBsdDom1PwdReg => self.pi.write_bsd_dom1_pwd_reg(value),
            Addr::PiBsdDom1PgsReg => self.pi.write_bsd_dom1_pgs_reg(value),
            Addr::PiBsdDom1RlsReg => self.pi.write_bsd_dom1_rls_reg(value),
            Addr::PiBsdDom2LatReg => self.pi.write_bsd_dom2_lat_reg(value),
            Addr::PiBsdDom2PwdReg => self.pi.write_bsd_dom2_pwd_reg(value),
            Addr::PiBsdDom2PgsReg => self.pi.write_bsd_dom2_pgs_reg(value),
            Addr::PiBsdDom2RlsReg => self.pi.write_bsd_dom2_rls_reg(value),

            Addr::RiModeReg => self.ri.write_mode_reg(value),
            Addr::RiConfigReg => self.ri.write_config_reg(value),
            Addr::RiCurrentLoadReg => self.ri.write_current_load_reg(value),
            Addr::RiSelectReg => self.ri.write_select_reg(value),
            Addr::RiRefreshReg => self.ri.write_refresh_reg(value),

            Addr::SiStatusReg => self.si.write_status_reg(value),

            Addr::CartDom1(offset) => panic!("Cannot write to cart ROM"),

            Addr::PifRom(_) => panic!("Cannot write to PIF ROM"),
            Addr::PifRam(offset) => self.pif.write_ram(offset, value),

            _ => panic!("Unknown physical address {:?} for write", mapped_address),
        }
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        let mapped_address = mem_map::map_addr(addr);
        let byte = match mapped_address {
            Addr::RdramMemory(offset) => self.rdram[offset as usize],
            Addr::SpDmem(offset) => self.rsp.read_dmem_byte(offset),
            Addr::SpImem(offset) => self.rsp.read_imem_byte(offset),
            Addr::CartDom1(offset) => self.cart_rom[offset as usize],

            _ => panic!("Cannot read byte from {:?}", mapped_address),
        };
        byte
    }

    pub fn write_byte(&mut self, addr: u32, value: u8) {
        let mapped_address = mem_map::map_addr(addr);
        match mapped_address {
            Addr::RdramMemory(offset) => self.rdram[offset as usize] = value,
            Addr::SpDmem(offset) => self.rsp.write_dmem_byte(offset, value),
            Addr::SpImem(offset) => self.rsp.write_imem_byte(offset, value),
            _ => panic!("Cannot write byte to {:?}", mapped_address),
        };
    }

    fn do_dma(&mut self, dma: DMARequest)
    {
        if (dma.length != 0) {
            println!("DMA {:08X} {:08X} {}", dma.from, dma.to, dma.length);
        }

        for i in 0..dma.length {
            let byte = self.read_byte(dma.from + i);
            self.write_byte(dma.to + i, byte);
        }
    }

    pub fn step(&mut self, frame_sink: &mut Sink<VideoFrame>) {
        // Execute DMA
        let dma = self.pi.get_dma_write_chunk();
        self.do_dma(dma);
        let dma = self.rsp.get_dma_read_chunk();
        self.do_dma(dma);


        // Every once in a white, do scan out the framebuffer. Note that this counter is totaly arbitrary right now...
        self.steps_to_next_frame -= 1;
        if self.steps_to_next_frame == 0 {
            self.steps_to_next_frame = 100000;
            if let Some(fb) = self.vi.framebuffer_description() {
                let size = (fb.width * fb.height) as usize;
                let mut argb_data: Box<[u32]> = vec![0; size].into_boxed_slice();

                match fb.format {
                    FramebufferFormat::RGBA32Bit => for i in 0..size {
                        argb_data[i] = self.read_word(fb.origin + i as u32 * 4) >> 8;
                    },
                    FramebufferFormat::RGBA16Bit => for i in 0..(size / 2) {
                        let pixel = self.read_word(fb.origin + i as u32 * 4);
                        argb_data[i*2+0] = ((pixel >> 26) & 0b11111) << (3 + 16) |
                                           ((pixel >> 21) & 0b11111) << (3 +  8) |
                                           ((pixel >> 16) & 0b11111) << (3 +  0);
                        argb_data[i*2+1] = ((pixel >> 10) & 0b11111) << (3 + 16) |
                                           ((pixel >>  5) & 0b11111) << (3 +  8) |
                                           ((pixel >>  0) & 0b11111) << (3 +  0);
                    },
                    _ => {},
                };

                frame_sink.append(VideoFrame {
                    argb_data: argb_data,
                    width: fb.width,
                    height: fb.height,
                });
            }
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: Impl Debug for Interconnect")
    }
}
