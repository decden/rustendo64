#[derive(Debug, Clone)]
pub enum FramebufferFormat {
    Blank,
    RGBA32Bit,
    RGBA16Bit,
}

#[derive(Clone)]
pub struct FramebufferDescription {
    pub format: FramebufferFormat,
    pub origin: u32,
    pub width: u32,
    pub height: u32,
}

pub struct VideoInterface {
    framebuffer: FramebufferDescription,

    interrupt_half_line: u32,

    active_video_start: u32,
    active_video_end: u32,
}

impl VideoInterface {
    pub fn new() -> VideoInterface {
        VideoInterface {
            framebuffer: FramebufferDescription {
                format: FramebufferFormat::Blank,
                origin: 0,
                width: 0,
                height: 0,
            },

            interrupt_half_line: 0,
            active_video_start: 0,
            active_video_end: 0,
        }
    }

    pub fn framebuffer_description(&self) -> Option<FramebufferDescription> {
        match self.framebuffer.width {
            0 => None,
            _ => match self.framebuffer.format {
                FramebufferFormat::Blank => None,
                _ => Some(self.framebuffer.clone()),
            }
        }
    }

    pub fn write_status_reg(&mut self, value: u32) {
        match (value & 0b11) {
            0 => self.framebuffer.format = FramebufferFormat::Blank,
            1 => println!("WARNING: Stub for write VI_STATUS_REG 0 (reserved)"),
            2 => self.framebuffer.format = FramebufferFormat::RGBA16Bit,
            3 => self.framebuffer.format = FramebufferFormat::RGBA32Bit,
            _ => {},
        };
        // NOTE: There is much more to this register...
        println!("WARNING: Stub for write VI_STATUS_REG {:08X} (fb_format: {:?})", value, self.framebuffer.format);
    }

    pub fn write_origin_reg(&mut self, value: u32) {
        self.framebuffer.origin = value & 0x00ff_ffff;
    }

    pub fn write_width_reg(&mut self, value: u32) {
        self.framebuffer.width = value & 0x0000_0fff;
        // TODO: Verify how the height of the FB can actually be determined
        self.framebuffer.height = self.framebuffer.width * 3 / 4;
    }

    pub fn read_intr_reg(&self) -> u32 {
        self.interrupt_half_line
    }

    pub fn write_intr_reg(&mut self, value: u32) {
        self.interrupt_half_line = value & 0x0000_03ff;
    }

    pub fn read_current_reg(&self) -> u32 {
        // TODO: Proper impl
        0
    }

    pub fn write_current_reg(&mut self, value: u32) {
        // TODO: Clear interrupt
    }

    pub fn write_timing_reg(&mut self, value: u32) {
        println!("WARNING: Stub for write VI_TIMING_REG {:08X}", value);
    }

    pub fn write_v_sync_reg(&mut self, value: u32) {
        println!("WARNING: Stub for write VI_V_SYNC_REG {:08X}", value);
    }

    pub fn write_h_sync_reg(&mut self, value: u32) {
        println!("WARNING: Stub for write VI_H_SYNC_REG {:08X}", value);
    }

    pub fn write_h_sync_leap_reg(&mut self, value: u32) {
        println!("WARNING: Stub for write VI_H_SYNC_LEAP_REG {:08X}", value);
    }

    pub fn read_h_start_reg(&self) -> u32 {
        (self.active_video_start << 16) | self.active_video_end
    }

    pub fn write_h_start_reg(&mut self, value: u32) {
        self.active_video_start = (value >> 16) & 0x0000_03ff;
        self.active_video_end = value & 0x0000_03ff;
    }

    pub fn write_v_start_reg(&mut self, value: u32) {
        println!("WARNING: Stub for write VI_V_START_REG {:08X}", value);
    }

    pub fn write_v_burst_reg(&mut self, value: u32) {
        println!("WARNING: Stub for write VI_V_BURST_REG {:08X}", value);
    }

    pub fn write_x_scale_reg(&mut self, value: u32) {
        println!("WARNING: Stub for write VI_X_SCALE_REG {:08X}", value);
    }

    pub fn write_y_scale_reg(&mut self, value: u32) {
        println!("WARNING: Stub for write VI_Y_SCALE_REG {:08X}", value);
    }
}
