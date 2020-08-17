use std::fs::File;
use std::io;
use std::io::Read;
use crate::memory::Memory;
use crate::cpu::Cpu;
use crate::screen::Screen;
use winit::event_loop::EventLoop;

pub struct CHIP8 {
    mem: Memory,
    cpu: Cpu,
}

impl CHIP8 {
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) -> io::Result<()> {
        let mut rom_file = File::open(path)?;
        let mut rom_data = Vec::new();

        let rom_size = rom_file.read_to_end(&mut rom_data)?;

        // Load the ROM into main memory at 0x200
        for i in 0..rom_size {
            self.mem.set(0x200 + i,rom_data[i]);
        }

        Ok(())
    }

    pub fn start(&mut self) {
        loop {
            self.cpu.cycle(&mut self.mem)
        }
    }
}
