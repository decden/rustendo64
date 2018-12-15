use super::sinks::{Sink, VideoFrame};
use super::{Cpu, Interconnect, Rsp};

#[derive(Debug)]
pub struct N64 {
    cpu: Cpu,
    rsp: Rsp,
    interconnect: Interconnect,
}

impl N64 {
    pub fn new(boot_rom: Box<[u8]>, cart_rom: Box<[u8]>) -> N64 {
        N64 {
            cpu: Cpu::new(),
            rsp: Rsp::new(),
            interconnect: Interconnect::new(boot_rom, cart_rom),
        }
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn interconnect(&self) -> &Interconnect {
        &self.interconnect
    }

    pub fn step(&mut self, frame_sink: &mut Sink<VideoFrame>) {
        self.cpu.step(&mut self.interconnect);
        self.rsp.step(&mut self.interconnect);
        self.interconnect.step(frame_sink);
    }
}
