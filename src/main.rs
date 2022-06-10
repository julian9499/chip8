mod memory;
mod op;
mod cpu;
mod chip8;
mod audio;

use chip8::CHIP8;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut chip = CHIP8::new();
    match chip.load_rom("./roms/audio.ch8") {
        Ok(()) => println!("successfully read rom"),
        Err(a) => panic!(a)
    }
    chip.load_font();

    chip.start();



}
