use crate::op::Op;
use crate::memory::Memory;
use crate::screen::Screen;
use winit::event_loop::{EventLoop, ControlFlow};
use rand::Rng;
use winit::event::Event;
use device_query::{DeviceQuery, DeviceState, Keycode};


pub struct Cpu {
    stack: [u16; 16],
    v: [u8; 16],
    // the V registers
    i: u16,
    // the I register
    pc: u16,
    // the program counter
    sp: usize,
    // the stack pointer
    dt: u8,
    // delay timer
    st: u8,
    // sound timer
    screen: Screen,
    event_loop: EventLoop<()>,
    seed: rand::rngs::ThreadRng,
    device_state: DeviceState,
}

impl Cpu {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let mut screen = Screen::new("chip 8", &event_loop);
        Self {
            stack: [0; 16], // the stack
            v: [0; 16], // the V registers
            i: 0,      // the I register
            pc: 0x200,     // the program counter
            sp: 0,      // the stack pointer
            dt: 0,      // delay timer
            st: 0,      // sound timer
            screen,
            event_loop,
            seed: rand::thread_rng(),
            device_state: DeviceState::new(),
        }
    }

    pub fn cycle(&mut self, memory: &mut Memory) {
        // Read the 2 byte opcode at PC
        let opcode = memory.read16(self.pc as usize);

        assert_eq!(self.pc % 2, 0);

        // Increment the program counter
        self.pc += 2;

        // Get current pressed keys
        let keys: Vec<u8> = self.device_state.get_keys()
            .iter()
            .map(|key| Cpu::keycode_to_u8(key))
            .collect();


        // Decode the instruction
        let op = Op::decode(opcode);

        // Parameters pulled out for readability
        let x = Op::x(opcode);
        let y = Op::y(opcode);
        let kk = Op::kk(opcode);
        let nnn = Op::nnn(opcode);
        let nibble = Op::nibble(opcode);

        // Execute the instruction
        match op {
            Op::CLR => self.screen.clear_screen(),
            Op::RET => {
                self.pc = self.stack[self.sp];
                self.sp = self.sp - 1
            }
            Op::JP => self.pc = nnn,
            Op::CALL => {
                self.sp += 1;
                self.stack[self.sp] = self.pc;
                self.pc = nnn;
            }
            Op::SE => {
                if self.v[x] == kk {
                    self.pc += 2;
                }
            }
            Op::SNE => {
                if self.v[x] != kk {
                    self.pc += 2;
                }
            }
            Op::SER => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            Op::LD => self.v[x] = kk,
            Op::ADD => self.v[x] = self.v[x].wrapping_add(kk),
            Op::LDR => self.v[x] = self.v[y].clone(),
            Op::OR => self.v[x] = self.v[x] | self.v[y],
            Op::AND => self.v[x] = self.v[x] & self.v[y],
            Op::XOR => self.v[x] = self.v[x] ^ self.v[y],
            Op::ADDR => {
                let val = (self.v[x] as u16 + self.v[y] as u16) as u8;
                self.v[0xF] = u8::from((self.v[x] as u16 + self.v[y] as u16) > 255);
                self.v[x] = val;
            }
            Op::SUB => {
                let val = self.v[x].wrapping_sub(self.v[y]);
                self.v[0xF] = u8::from(self.v[x] > self.v[y]);
                self.v[x] = val;
            }
            Op::SHR => {
                let val = self.v[x] >> 1;
                self.v[0xF] = u8::from(self.v[x] % 2 != 0);
                self.v[x] = val;
            }
            Op::SUBN => {
                let val = self.v[y].wrapping_sub(self.v[x]);
                self.v[0xF] = u8::from(self.v[y] > self.v[x]);
                self.v[x] = val;
            }
            Op::SHL => {
                let val = self.v[x].wrapping_mul(2);
                self.v[0xF] = u8::from(self.v[x] & 0b10000000 != 0);
                self.v[x] = val;
            }
            Op::SNER => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            Op::LDI => self.i = nnn,
            Op::JPA => self.pc = nnn + self.v[0] as u16,
            Op::RND => {
                let rnd: u8 = self.seed.gen();
                self.v[x] = rnd & kk
            }

            Op::DRW => {
                let sprite = self.read_sprite(self.i, nibble, memory);
                self.screen.draw_sprite(sprite, self.v[x], self.v[y]);
            }
            Op::SKP => {
                if keys.contains(&(self.v[x] as u8)) {
                    self.pc += 2
                }
            }
            Op::SKNP => {
                if !keys.contains(&(self.v[x] as u8)) {
                    self.pc += 2
                }
            }
            Op::LDD => self.v[x] = self.dt,
            Op::LDK => {
                let mut presses = self.device_state.get_keys();
                while presses.is_empty() {
                    presses = self.device_state.get_keys();
                }
                let keycode: Vec<u8> = presses.iter().map(|key| Cpu::keycode_to_u8(key)).collect();
                self.v[x] = keycode.first().get_or_insert(&(20 as u8)).clone();
            }
            Op::LDDT => {
                self.dt = self.v[x];
            }
            Op::LDST => {
                self.st = self.v[x];
            }
            Op::ADDI => {
                self.i = self.i.wrapping_add(self.v[x] as u16);
            }
            Op::LDF => {
                self.i = self.v[x].wrapping_mul(5) as u16;
            }
            Op::LDB => {
                memory.set(self.i as usize, self.v[x] / 100);
                memory.set((self.i + 1) as usize, (self.v[x] % 100) / 100);
                memory.set((self.i + 2) as usize, self.v[x] % 10)
            }
            Op::LDII => {
                let mut index = self.i.clone();
                for v in self.v.iter() {
                    memory.set(index as usize, *v);
                    index += 1;
                }
            }
            Op::LDVX => {
                for index in 0..15 {
                    self.v[index] = memory.read8(self.i as usize + index);
                }
            }
        }
    }

    pub fn timer(&mut self) -> () {
        if self.st != 0 {
            self.st -= 1;
        }
        if self.dt != 0 {
            self.dt -= 1;
        }
    }

    pub fn read_sprite(&mut self, i: u16, nibble: usize, memory: &mut Memory) -> [u8; 15] {
        let mut sprite: [u8; 15] = [0; 15];
        for index in 0..nibble {
            sprite[index] = memory.read8((i as usize) + index);
        }
        return sprite;
    }

    pub fn keycode_to_u8(key: &Keycode) -> u8 {
        return match *key {
            Keycode::Key0 => 0,
            Keycode::Key1 => 1,
            Keycode::Key2 => 2,
            Keycode::Key3 => 3,
            Keycode::Key4 => 4,
            Keycode::Key5 => 5,
            Keycode::Key6 => 6,
            Keycode::Key7 => 7,
            Keycode::Key8 => 8,
            Keycode::Key9 => 9,
            Keycode::A => 10,
            Keycode::B => 11,
            Keycode::C => 12,
            Keycode::D => 13,
            Keycode::E => 14,
            Keycode::F => 15,
            _ => 20
        };
    }
}
