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

impl Cpu {
    fn zero_flag(&self) -> bool {
        (self.f & 0b10000000) > 0
    }
    fn subtract_flag(&self) -> bool {
        (self.f & 0b01000000) > 0
    }
    fn halfcarry_flag(&self) -> bool {
        (self.f & 0b00100000) > 0
    }
    fn carry_flag(&self) -> bool {
        (self.f & 0b00010000) > 0
    }
}
