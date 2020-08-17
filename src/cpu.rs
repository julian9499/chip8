use crate::op::Op;
use crate::memory::Memory;
use crate::screen::Screen;
use winit::event_loop::EventLoop;

pub struct Cpu {
    v: [u8; 16], // the V registers
    i: u16,      // the I register
    pc: u16,     // the program counter
    sp: u8,      // the stack pointer
    dt: u8,      // delay timer
    st: u8,      // sound timer
    screen: Screen,
    event_loop: EventLoop<()>,}

impl Cpu {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let mut screen = Screen::new("chip 8", &event_loop);
        Self {
            v: [0; 16], // the V registers
            i: 0,      // the I register
            pc: 0x200,     // the program counter
            sp: 0,      // the stack pointer
            dt: 0,      // delay timer
            st: 0,      // sound timer
            screen,
            event_loop,
        }
    }

    pub fn cycle(&mut self, memory: &mut Memory) {
        // Read the 2 byte opcode at PC
        let opcode = memory.read16(self.pc as usize);

        // Increment the program counter
        self.pc += 2;

        // Decode the instruction
        let op = Op::decode(opcode);

        // Parameters pulled out for readability
        let x  = Op::x(opcode);
        let y  = Op::y(opcode);
        let kk = Op::kk(opcode);

        // Execute the instruction
        match op {
            Op::LD => self.v[x] = kk,
            // Op::ADD => self.v[x] = self.v[x] + kk,
            Op::LDR => self.v[x] = self.v[y],
            Op::CLR => self.screen.clear_screen(),
            _ => {}
        }
    }
}
