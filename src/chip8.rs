use std::fs::File;
use std::io;
use std::io::Read;
use crate::memory::Memory;
use crate::cpu::Cpu;
use std::{thread, time};

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
        let mut ten_millis = time::Duration::from_millis(16);
        let mut now = time::Instant::now();

        loop {
            if now.elapsed() >= ten_millis {
                now = time::Instant::now();
                self.cpu.timer();
            }
            self.cpu.cycle(&mut self.mem);
            thread::sleep(ten_millis);

        }
    }

    pub fn load_font(&mut self) -> () {
        let font: [u8; 80] =
            [0xF0,
                0x90,
                0x90,
                0x90,
                0xF0,
                0x20,
                0x60,
                0x20,
                0x20,
                0x70,
                0xF0,
                0x10,
                0xF0,
                0x80,
                0xF0,
                0xF0,
                0x10,
                0xF0,
                0x10,
                0xF0,
                0x90,
                0x90,
                0xF0,
                0x10,
                0x10,
                0xF0,
                0x80,
                0xF0,
                0x10,
                0xF0,
                0xF0,
                0x80,
                0xF0,
                0x90,
                0xF0,
                0xF0,
                0x10,
                0x20,
                0x40,
                0x40,
                0xF0,
                0x90,
                0xF0,
                0x90,
                0xF0,
                0xF0,
                0x90,
                0xF0,
                0x10,
                0xF0,
                0xF0,
                0x90,
                0xF0,
                0x90,
                0x90,
                0xE0,
                0x90,
                0xE0,
                0x90,
                0xE0,
                0xF0,
                0x80,
                0x80,
                0x80,
                0xF0,
                0xE0,
                0x90,
                0x90,
                0x90,
                0xE0,
                0xF0,
                0x80,
                0xF0,
                0x80,
                0xF0,
                0xF0,
                0x80,
                0xF0,
                0x80,
                0x80];

        for i in 0..font.len() {
            self.mem.set(i, font[i]);
        }
    }

}
