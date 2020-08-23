mod screen;
mod memory;
mod op;
mod cpu;
mod chip8;
use chip8::CHIP8;

fn main() {
    let mut chip = CHIP8::new();
    match chip.load_rom("./roms/maze.ch8") {
        Ok(()) => println!("successfully read rom"),
        Err(a) => panic!(a)
    }
    chip.load_font();

    chip.start();


}
