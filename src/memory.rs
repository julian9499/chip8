

const MEMORY_SIZE: usize = 4096;

pub struct Memory {
    mem: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; MEMORY_SIZE]
        }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        self.mem[index] = value;
    }

    pub fn read16(&mut self, counter: usize) -> u16 {
        let number = ((self.mem[counter] as u16) << 8) | self.mem[counter + 1] as u16;
        return number;
    }
}
