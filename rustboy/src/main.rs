struct Cpu {
    pc: u16, // program counter (PC)
    sp: u16, // stack pointer (SP)
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8, // FLAGS: Z - zero, N - subtract, H - half-carry, C - carry, 4 unused bits.
}

const ZERO_FLAG_MASK: u8 = 0b10000000;
const SUBTRACT_FLAG_MASK: u8 = 0b01000000;
const HALFCARRY_FLAG_MASK: u8 = 0b00100000;
const CARRY_FLAG_MASK: u8 = 0b00010000;

impl Cpu {
    fn zero_flag(&self) -> bool {
        (self.f & ZERO_FLAG_MASK) > 0
    }
    fn subtract_flag(&self) -> bool {
        (self.f & SUBTRACT_FLAG_MASK) > 0
    }
    fn halfcarry_flag(&self) -> bool {
        (self.f & HALFCARRY_FLAG_MASK) > 0
    }
    fn carry_flag(&self) -> bool {
        (self.f & CARRY_FLAG_MASK) > 0
    }

    fn set_flag(&mut self, value: bool, mask: u8) {
        if value {
            self.f |= mask
        } else {
            self.f &= !mask
        }
    }
}

struct Memory {
    ram: [u8; 1 << 16],
}

impl Memory {
    fn new() -> Memory {
        Memory { ram: [0; 1 << 16] }
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value
    }
}

fn main() {
    println!("Hello, world!");
}
