pub enum Op {
    CLR,    // Clear the screen
    RET,    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    JP,     // The interpreter sets the program counter to nnn.
    CALL,   // 2nnn - The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    SE,     // 3xkk - The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    SNE,    // 4xkk - The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    SER,    // 5xy0 - SE Vx, Vy
    LD,     // 6xkk - LD Vx, byte
    ADD,    // 7xkk - ADD Vx, byte
    LDR,    // 8xy0 - LD Vx, Vy
    OR,     // 8xy1 - OR Vx, Vy
    AND,    // 8xy2 - AND Vx, Vy
    XOR,    // 8xy3 - XOR Vx, Vy
    ADDR,   // 8xy4 - ADD Vx, Vy
    SUB,    // 8xy5 - SUB Vx, Vy
    SHR,    // 8xy6 - If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    SUBN,   // 8xy7 - SUBN Vx, Vy
    SHL,    // 8xyE - SHL Vx {, Vy}
    SNER,   // 9xy0 - SNE Vx, Vy
    LDI,    // Annn - LD I, addr
    JPA,    // Bnnn - JP V0, addr
    RND,    // Cxkk - RND Vx, byte
    DRW,    // Dxyn - DRW Vx, Vy, nibble
    SKP,    // Ex9E - SKP Vx
    SKNP,   // ExA1 - SKNP Vx
    LDD,    // load delay timer value into vx
    LDK,    // wait for key press store into vx
    LDDT,   // load value in vx into delay timer
    LDST,   // load vx into sound timer
    ADDI,   // Set I = I + Vx which is stored in I
    LDF,    // Set I = location of sprite for digit Vx
    LDB,    // Store "BCD"(normal) representation of Vx in memory location  I, I+1 and I+2
    LDII,   // Store registers V0 through Vx in memory starting at location I
    LDVX,   // Read registers V0 through Vx into memory starting at location I

}

impl Op {
    pub fn decode(opcode: u16) -> Self {
        match opcode & 0xF000 {
            0x0000 => match opcode & 0xFFFF {
                0x00E0 => Self::CLR,
                0x00EE => Self::RET,
                _ => panic!("Invalid opcode: {:04X}", opcode),
            }
            0x1000 => Self::JP,
            0x2000 => Self::CALL,
            0x3000 => Self::SE,
            0x4000 => Self::SNE,
            0x5000 => Self::SER,
            0x6000 => Self::LD,
            0x7000 => Self::ADD,
            0x8000 => Self::decode8(opcode),
            0x9000 => Self::SNE,
            0xA000 => Self::LDI,
            0xB000 => Self::JPA,
            0xC000 => Self::RND,
            0xD000 => Self::DRW,
            0xE000 => match opcode & 0x00FF {
                0x009E => Self::SKP,
                0x00A1 => Self::SKNP,
                _ => panic!("Invalid opcode: {:04X}", opcode),
            }, //multiple
            0xF000 => Self::decode_f(opcode), // multiple
            _ => panic!("Invalid opcode: {:04X}", opcode),
        }
    }

    pub fn decode8(opcode: u16) -> Self {
        match opcode & 0x000F {
            0x0000 => Self::LDR,
            0x0001 => Self::OR,
            0x0002 => Self::AND,
            0x0003 => Self::XOR,
            0x0004 => Self::ADDR,
            0x0005 => Self::SUB,
            0x0006 => Self::SHR,
            0x0007 => Self::SUBN,
            0x000E => Self::SHL,
            _ => panic!("Invalid opcode: {:04X}", opcode),
        }
    }

    pub fn decode_f(opcode: u16) -> Self {
        match opcode & 0x00FF {
            0x0007 => Self::LDD,
            0x000A => Self::LDK,
            0x0015 => Self::LDDT,
            0x0018 => Self::LDST,
            0x001E => Self::ADDI,
            0x0029 => Self::LDF,
            0x0033 => Self::LDB,
            0x0055 => Self::LDII,
            0x0065 => Self::LDVX,
            _ => panic!("Invalid opcode: {:04X}", opcode),
        }
    }

    pub fn x(opcode: u16) -> usize {
        ((opcode & 0x0F00) >> 8) as usize
    }

    pub fn y(opcode: u16) -> usize {
        ((opcode & 0x00F0) >> 4) as usize
    }

    pub fn kk(opcode: u16) -> u8 {
        (opcode & 0x00FF) as u8
    }

    pub fn nnn(opcode: u16) -> u16 {
        (opcode & 0x0FFF) as u16
    }

    pub fn nibble(opcode: u16) -> usize {
        (opcode & 0x000F) as usize
    }
}
