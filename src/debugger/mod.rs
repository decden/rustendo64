mod command;

use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::borrow::Cow;
use n64::cpu::Instruction;
use n64::cpu::opcode::Opcode::*;
use n64::mem_map;
use n64::mem_map::Addr::*;
use n64::N64;
use middleware::MostRecentFrameSink;
use self::command::Command;

use minifb::{WindowOptions, Window, Key, KeyRepeat, Scale};

pub struct Debugger {
    n64: N64,

    last_command: Option<Command>,

    window: Option<Window>,
    window_size: (u32, u32),
}

impl Debugger {
    pub fn new(n64: N64) -> Debugger {
        Debugger {
            n64: n64,

            last_command: None,

            window: None,
            window_size: (0, 0),
        }
    }

    pub fn run(&mut self) {
        // Run the emulator when pressing enter
        self.last_command = Some(Command::Step(1000000000000));
        loop {
            print!("r64> ");
            stdout().flush().unwrap();

            let command = match (read_stdin().parse(), self.last_command) {
                (Ok(Command::Repeat), Some(c)) => Ok(c),
                (Ok(Command::Repeat), None) => Err("No last command".into()),
                (Ok(c), _) => Ok(c),
                (Err(e), _) => Err(e),
            };

            match command {
                Ok(Command::Step(count)) => self.step(count),
                Ok(Command::Memdump(addr, count)) => self.memdump(addr, count),
                Ok(Command::CpuInfo) => self.cpuinfo(),
                Ok(Command::Exit) => break,
                Ok(Command::Repeat) => unreachable!(),
                Err(ref e) => println!("{}", e),
            }

            self.last_command = command.ok();
        }
    }

    pub fn step(&mut self, count: usize) {
        for _ in 0..count {
            let current_pc = self.n64.cpu().current_pc_phys();
            let instr = Instruction(self.n64.interconnect().read_word_debug(current_pc as u32).unwrap());

            print!("{:018X}: ", current_pc);

            match instr.opcode() {
                Special => print!("{:?} (Special)", instr.special_op()),
                RegImm => print!("{:?} (RegImm)", instr.reg_imm_op()),
                _ => print!("{:?}", instr),
            }

            if self.n64.cpu().will_execute_from_delay_slot() {
                println!(" (DELAY)");
            } else {
                println!("");
            }

            let mut frame_sink = MostRecentFrameSink::new();
            self.n64.step(&mut frame_sink);

            if let Some(frame) = frame_sink.into_frame() {
                if self.window_size != (frame.width, frame.height) {
                    self.window = Some(Window::new("Rustendo64", frame.width as usize, frame.height as usize, WindowOptions {
                        borderless: true,
                        title: true,
                        resize: false,
                        scale: Scale::X2,
                    }).unwrap());
                    self.window_size = (frame.width, frame.height);
                }

                if let Some(window) = self.window.as_mut() {
                    window.update_with_buffer(&frame.argb_data);
                }
            }
        }
    }

    pub fn memdump(&mut self, addr: Option<usize>, size: usize) {
        let dump_addr = addr.unwrap_or(self.n64.cpu().current_pc_phys() as usize) as u32;
        println!("Dumping memory at {:016X}", dump_addr);

        for i in 0..(size as u32 / 4) {
            let addr = dump_addr + i * 4;
            let word = self.n64.interconnect().read_word_debug(addr);
            match word {
                Some(word) => print!("{:08X} ", word),
                None => print!("???????? "),
            };
        }
    }

    pub fn cpuinfo(&mut self) {
        println!("{:?}", self.n64.cpu());
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}
