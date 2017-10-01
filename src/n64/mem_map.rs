const RDRAM_START: u32 =            0x0000_0000;
pub const RDRAM_LENGTH: u32 =       0x0080_0000; // 8 or 4 MB depending on whether there is an expansion pack
const RDRAM_END: u32 =              RDRAM_START + RDRAM_LENGTH - 1;
const RDRAM_MODE_REG: u32 =         0x03f0_000c;

pub const SP_DMEM_START: u32 =      0x0400_0000;
pub const SP_DMEM_LENGTH: u32 =     0x0000_1000;
const SP_DMEM_END: u32 =            SP_DMEM_START + SP_DMEM_LENGTH - 1;

pub const SP_IMEM_START: u32 =      0x0400_1000;
pub const SP_IMEM_LENGTH: u32 =     0x0000_1000;
const SP_IMEM_END: u32 =            SP_IMEM_START + SP_IMEM_LENGTH - 1;

const SP_MEM_ADDR_REG: u32 =        0x0404_0000;
const SP_DRAM_ADDR_REG: u32 =       0x0404_0004;
const SP_RD_LEN_REG: u32 =          0x0404_0008;
const SP_WR_LEN_REG: u32 =          0x0404_000C;
const SP_STATUS_REG: u32 =          0x0404_0010;
const SP_DMA_BUSY_REG: u32 =        0x0404_0018;
const SP_SEMAPHORE_REG: u32 =       0x0404_001C;

const SP_PC_REG: u32 =              0x0408_0000;

const DPC_BASE_REG: u32 =           0x0410_0000;
const DPC_STATUS_REG: u32 =         0x0410_000c;

const MI_MODE_REG: u32 =            0x0430_0000;
const MI_VERSION_REG: u32 =         0x0430_0004;
const MI_INTR_MASK_REG: u32 =       0x0430_000C;

const AI_BASE_REG: u32 =            0x0450_0000;
const AI_DRAM_ADDR_REG: u32 =       0x0450_0000;
const AI_LEN_REG: u32 =             0x0450_0004;
const AI_STATUS_REG: u32 =          0x0450_000C;

const VI_STATUS_REG: u32 =          0x0440_0000;
const VI_ORIGIN_REG: u32 =          0x0440_0004;
const VI_WIDTH_REG: u32 =           0x0440_0008;
const VI_INTR_REG: u32 =            0x0440_000c;
const VI_CURRENT_REG: u32 =         0x0440_0010;
const VI_TIMING_REG: u32 =          0x0440_0014;
const VI_V_SYNC_REG: u32 =          0x0440_0018;
const VI_H_SYNC_REG: u32 =          0x0440_001C;
const VI_H_SYNC_LEAP_REG: u32 =     0x0440_0020;
const VI_H_START_REG: u32 =         0x0440_0024;
const VI_V_START_REG: u32 =         0x0440_0028;
const VI_V_BURST_REG: u32 =         0x0440_002C;
const VI_X_SCALE_REG: u32 =         0x0440_0030;
const VI_Y_SCALE_REG: u32 =         0x0440_0034;

const PI_DRAM_ADDRESS_REG: u32 =    0x0460_0000;
const PI_CART_ADDR_REG: u32 =       0x0460_0004;
const PI_WR_LEN_REG: u32 =          0x0460_000C;
const PI_STATUS_REG: u32 =          0x0460_0010;
const PI_BSD_DOM1_LAT_REG: u32 =    0x0460_0014;
const PI_BSD_DOM1_PWD_REG: u32 =    0x0460_0018;
const PI_BSD_DOM1_PGS_REG: u32 =    0x0460_001c;
const PI_BSD_DOM1_RLS_REG: u32 =    0x0460_0020;

const RI_MODE_REG: u32 =            0x0470_0000;
const RI_CONFIG_REG: u32 =          0x0470_0004;
const RI_CURRENT_LOAD_REG: u32 =    0x0470_0008;
const RI_SELECT_REG: u32 =          0x0470_000C;
const RI_REFRESH_REG: u32 =         0x0470_0010;

const SI_BASE_REG: u32 =            0x0480_0000;
const SI_STATUS_REG: u32 =          0x0480_0018;

const CART_DOM1_ADDR2_START: u32 =  0x1000_0000;
const CART_DOM1_ADDR2_LENGTH: u32 = 0x0fc0_0000;
const CART_DOM1_ADDR2_END: u32 =    CART_DOM1_ADDR2_START + CART_DOM1_ADDR2_LENGTH - 1;

const PIF_ROM_START: u32 =          0x1fc0_0000;
pub const PIF_ROM_LENGTH: u32 =     0x0000_07c0;
const PIF_ROM_END: u32 =            PIF_ROM_START + PIF_ROM_LENGTH - 1;

const PIF_RAM_START: u32 =          0x1fc0_07c0;
pub const PIF_RAM_LENGTH: u32 =     0x0000_0040;
const PIF_RAM_END: u32 =            PIF_RAM_START + PIF_RAM_LENGTH - 1;


#[derive(Debug)]
pub enum Addr {
    RdramMemory(u32),
    RdramModeReg,
    RdramUnknownReg(u32),

    SpDmem(u32),
    SpImem(u32),

    SpMemAddrReg,
    SpDramAddrReg,
    SpRdLenReg,
    SpWrLenReg,
    SpStatusReg,
    SpDmaBusyReg,
    SpSemaphoreReg,
    SpPcReg,

    DpcStatusReg,

    MiModeReg,
    MiVersionReg,
    MiIntrMaskReg,

    AiDramAddrReg,
    AiLenReg,
    AiStatusReg,

    ViStatusReg,
    ViOriginReg,
    ViWidthReg,
    ViIntrReg,
    ViCurrentReg,
    ViTimingReg,
    ViVSyncReg,
    ViHSyncReg,
    ViHSyncLeapReg,
    ViHStartReg,
    ViVStartReg,
    ViVBurstReg,
    ViXScaleReg,
    ViYScaleReg,

    PiDramAddrReg,
    PiCartAddrReg,
    PiWrLenReg,
    PiStatusReg,
    PiBsdDom1LatReg,
    PiBsdDom1PwdReg,
    PiBsdDom1PgsReg,
    PiBsdDom1RlsReg,

    RiModeReg,
    RiConfigReg,
    RiCurrentLoadReg,
    RiRefreshReg,
    RiSelectReg,

    SiStatusReg,

    CartDom1(u32),

    PifRom(u32),
    PifRam(u32),
}

pub fn map_addr(addr: u32) -> Addr {
    match addr {
        RDRAM_START ... RDRAM_END =>
            Addr::RdramMemory(addr - RDRAM_START),

        RDRAM_MODE_REG => Addr::RdramModeReg,
        0x03f0_4004 => Addr::RdramUnknownReg(addr),
        0x03f8_0004 => Addr::RdramUnknownReg(addr),
        0x03f8_000c => Addr::RdramUnknownReg(addr),
        0x03f8_0008 => Addr::RdramUnknownReg(addr),
        0x03f8_0014 => Addr::RdramUnknownReg(addr),

        SP_DMEM_START ... SP_DMEM_END =>
            Addr::SpDmem(addr - SP_DMEM_START),

        SP_IMEM_START ... SP_IMEM_END =>
            Addr::SpImem(addr - SP_IMEM_START),

        SP_MEM_ADDR_REG => Addr::SpMemAddrReg,
        SP_DRAM_ADDR_REG => Addr::SpDramAddrReg,
        SP_RD_LEN_REG => Addr::SpRdLenReg,
        SP_WR_LEN_REG => Addr::SpWrLenReg,
        SP_STATUS_REG => Addr::SpStatusReg,
        SP_DMA_BUSY_REG => Addr::SpDmaBusyReg,
        SP_SEMAPHORE_REG => Addr::SpSemaphoreReg,
        SP_PC_REG => Addr::SpPcReg,

        DPC_STATUS_REG => Addr::DpcStatusReg,

        MI_MODE_REG => Addr::MiModeReg,
        MI_VERSION_REG => Addr::MiVersionReg,
        MI_INTR_MASK_REG => Addr::MiIntrMaskReg,

        AI_DRAM_ADDR_REG => Addr::AiDramAddrReg,
        AI_LEN_REG => Addr::AiLenReg,
        AI_STATUS_REG => Addr::AiStatusReg,

        VI_STATUS_REG => Addr::ViStatusReg,
        VI_ORIGIN_REG => Addr::ViOriginReg,
        VI_WIDTH_REG => Addr::ViWidthReg,
        VI_INTR_REG => Addr::ViIntrReg,
        VI_CURRENT_REG => Addr::ViCurrentReg,
        VI_TIMING_REG => Addr::ViTimingReg,
        VI_V_SYNC_REG => Addr::ViVSyncReg,
        VI_H_SYNC_REG => Addr::ViHSyncReg,
        VI_H_SYNC_LEAP_REG => Addr::ViHSyncLeapReg,
        VI_H_START_REG => Addr::ViHStartReg,
        VI_V_START_REG => Addr::ViVStartReg,
        VI_V_BURST_REG => Addr::ViVBurstReg,
        VI_X_SCALE_REG => Addr::ViXScaleReg,
        VI_Y_SCALE_REG => Addr::ViYScaleReg,

        PI_DRAM_ADDRESS_REG => Addr::PiDramAddrReg,
        PI_CART_ADDR_REG => Addr::PiCartAddrReg,
        PI_WR_LEN_REG => Addr::PiWrLenReg,
        PI_STATUS_REG => Addr::PiStatusReg,
        PI_BSD_DOM1_LAT_REG => Addr::PiBsdDom1LatReg,
        PI_BSD_DOM1_PWD_REG => Addr::PiBsdDom1PwdReg,
        PI_BSD_DOM1_PGS_REG => Addr::PiBsdDom1PgsReg,
        PI_BSD_DOM1_RLS_REG => Addr::PiBsdDom1RlsReg,

        RI_MODE_REG => Addr::RiModeReg,
        RI_CONFIG_REG => Addr::RiConfigReg,
        RI_CURRENT_LOAD_REG => Addr::RiCurrentLoadReg,
        RI_REFRESH_REG => Addr::RiRefreshReg,
        RI_SELECT_REG => Addr::RiSelectReg,

        SI_STATUS_REG => Addr::SiStatusReg,

        CART_DOM1_ADDR2_START ... CART_DOM1_ADDR2_END =>
            Addr::CartDom1(addr - CART_DOM1_ADDR2_START),
        PIF_ROM_START ... PIF_ROM_END =>
            Addr::PifRom(addr - PIF_ROM_START),
        PIF_RAM_START ... PIF_RAM_END =>
            Addr::PifRam(addr - PIF_RAM_START),

        _ => panic!("Unrecognized physical address: {:#08X}", addr),
    }
}
