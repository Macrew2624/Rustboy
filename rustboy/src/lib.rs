#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cpu {
    pub pc: u16, // program counter (PC)
    pub sp: u16, // stack pointer (SP)
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8, // FLAGS: Z - zero, N - subtract, H - half-carry, C - carry, 4 unused bits.
    pub halted: bool, // determines if the cpu is halted. wakes on interrupt
    pub stopped: bool, // determines if the cpu is stopped. wakes on input
}

pub const ZERO_FLAG_MASK: u8 = 0b10000000;
pub const SUBTRACT_FLAG_MASK: u8 = 0b01000000;
pub const HALFCARRY_FLAG_MASK: u8 = 0b00100000;
pub const CARRY_FLAG_MASK: u8 = 0b00010000;
pub const LAST_THREE_BITS_MASK: u8 = 0b00000111;

#[derive(Clone, Copy, Debug)]
pub enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub fn reg_from_bits(bits: u8) -> Result<Reg, String> {
    match bits {
        0b000 => Ok(Reg::B),
        0b001 => Ok(Reg::C),
        0b010 => Ok(Reg::D),
        0b011 => Ok(Reg::E),
        0b100 => Ok(Reg::H),
        0b101 => Ok(Reg::L),
        0b111 => Ok(Reg::A),
        _ => Err(format!("Unrecognized register {:#0b}.", bits)),
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            halted: false,
            stopped: false,
        }
    }

    pub fn zero_flag(&self) -> bool {
        (self.f & ZERO_FLAG_MASK) > 0
    }
    pub fn subtract_flag(&self) -> bool {
        (self.f & SUBTRACT_FLAG_MASK) > 0
    }
    pub fn halfcarry_flag(&self) -> bool {
        (self.f & HALFCARRY_FLAG_MASK) > 0
    }
    pub fn carry_flag(&self) -> bool {
        (self.f & CARRY_FLAG_MASK) > 0
    }
    pub fn get_reg(&self, reg: Reg) -> u8 {
        match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
        }
    }

    pub fn hl(&self) -> u16 {
        return ((self.h as u16) << 8) | (self.l as u16);
    }
    pub fn set_hl(&mut self, value: u16) {
        self.l = value as u8;
        self.h = (value >> 8) as u8;
    }

    pub fn set_flag(&mut self, value: bool, mask: u8) {
        if value {
            self.f |= mask
        } else {
            self.f &= !mask
        }
    }
    pub fn set_reg(&mut self, reg: Reg, value: u8) {
        match reg {
            Reg::A => self.a = value,
            Reg::B => self.b = value,
            Reg::C => self.c = value,
            Reg::D => self.d = value,
            Reg::E => self.e = value,
            Reg::H => self.h = value,
            Reg::L => self.l = value,
        }
    }

    pub fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let fetched = memory.read_byte(self.pc);
        self.pc += 1;
        fetched
    }
    pub fn load_immediate(&mut self, reg: Reg, memory: &Memory) {
        let value = self.fetch_byte(memory);
        self.set_reg(reg, value);
    }
    pub fn load_reg(&mut self, dest: Reg, src: Reg) {
        self.set_reg(dest, self.get_reg(src));
    }

    pub fn add_to_reg(&mut self, reg: Reg, operand: u8) {
        let register = self.get_reg(reg);
        let (result, did_overflow) = register.overflowing_add(operand);

        let half_carry_check = (register & 0x0F) + (operand & 0x0F);
        self.set_flag((half_carry_check >> 4) > 0, HALFCARRY_FLAG_MASK);

        self.set_reg(reg, result);
        self.set_flag(did_overflow, CARRY_FLAG_MASK);
        self.set_flag(result == 0, ZERO_FLAG_MASK);
        self.set_flag(false, SUBTRACT_FLAG_MASK);
    }
    pub fn inc_reg(&mut self, reg: Reg) {
        let register = self.get_reg(reg);
        let result = self.inc_value(register);
        self.set_reg(reg, result);
    }
    pub fn dec_reg(&mut self, reg: Reg) {
        let register = self.get_reg(reg);
        let result = self.dec_value(register);
        self.set_reg(reg, result);
    }

    pub fn inc_value(&mut self, value: u8) -> u8 {
        let (result, _) = value.overflowing_add(1);

        let half_carry_check = (value & 0x0F) + 1;
        self.set_flag((half_carry_check >> 4) > 0, HALFCARRY_FLAG_MASK);

        self.set_flag(result == 0, ZERO_FLAG_MASK);
        self.set_flag(false, SUBTRACT_FLAG_MASK);

        return result;
    }
    pub fn dec_value(&mut self, value: u8) -> u8 {
        let (result, _) = value.overflowing_sub(1);

        self.set_flag(value & 0x0F == 0, HALFCARRY_FLAG_MASK);

        self.set_flag(result == 0, ZERO_FLAG_MASK);
        self.set_flag(true, SUBTRACT_FLAG_MASK);

        return result;
    }

    pub fn step(&mut self, memory: &mut Memory) -> Result<(), String> {
        let interrupt_flag = memory.read_byte(0xFF0F);
        let interrupt_enable = memory.read_byte(0xFFFF);
        if (interrupt_enable & interrupt_flag) & 0b00011111 != 0 {
            self.halted = false;
        }
        if (interrupt_enable & interrupt_flag) & 0b00010000 != 0 {
            self.stopped = false;
        }
        if self.halted || self.stopped {
            return Ok(());
        }
        let instruction_start = self.pc;
        let opcode = self.fetch_byte(memory);
        match opcode {
            0x00 => {
                // NOP - do nothing
                Ok(())
            }
            0x34 => {
                // INC (HL)
                let value = memory.read_byte(self.hl());
                memory.write_byte(self.hl(), self.inc_value(value));
                Ok(())
            }
            0x35 => {
                // DEC (HL)
                let value = memory.read_byte(self.hl());
                memory.write_byte(self.hl(), self.dec_value(value));
                Ok(())
            }
            0x36 => {
                // LD (HL), n - load the u8 value n into memory at the adress HL
                memory.write_byte(self.hl(), self.fetch_byte(memory));
                Ok(())
            }
            0x04..=0x3C if (opcode & LAST_THREE_BITS_MASK == 0b100) => {
                // INC r - increase register r's value by 1 without modifying the carry flag
                let destination_reg = reg_from_bits((opcode >> 3) & LAST_THREE_BITS_MASK)
                    .map_err(|e| format!("{} Instruction start: {:#06x}.", e, instruction_start))?;
                self.inc_reg(destination_reg);
                Ok(())
            }
            0x05..=0x3D if (opcode & LAST_THREE_BITS_MASK == 0b101) => {
                // DEC r
                let destination_reg = reg_from_bits((opcode >> 3) & LAST_THREE_BITS_MASK)
                    .map_err(|e| format!("{} Instruction start: {:#06x}.", e, instruction_start))?;
                self.dec_reg(destination_reg);
                Ok(())
            }
            0x06..=0x3E if (opcode & LAST_THREE_BITS_MASK == 0b110) => {
                // LD r, n - load value n from memory into register r
                let destination_reg = reg_from_bits((opcode >> 3) & LAST_THREE_BITS_MASK)
                    .map_err(|e| format!("{} Instruction start: {:#06x}.", e, instruction_start))?;
                self.load_immediate(destination_reg, memory);
                Ok(())
            }
            0x10 => {
                // STOP
                self.stopped = true;
                self.fetch_byte(memory);
                Ok(())
            }
            0x76 => {
                // HALT
                self.halted = true;
                Ok(())
            }
            0x70..=0x77 => {
                // LD (HL), r - load register r's value into memory at adress HL
                let source_reg = reg_from_bits(opcode & LAST_THREE_BITS_MASK)
                    .map_err(|e| format!("{} Instruction start: {:#06x}.", e, instruction_start))?;
                memory.write_byte(self.hl(), self.get_reg(source_reg));
                Ok(())
            }
            0x40..=0x7F if (opcode & LAST_THREE_BITS_MASK == 0b110) => {
                // LD r, (HL) - load the value from memory at adress hl into register r
                let destination_reg = reg_from_bits((opcode >> 3) & LAST_THREE_BITS_MASK)
                    .map_err(|e| format!("{} Instruction start: {:#06x}.", e, instruction_start))?;
                self.set_reg(destination_reg, memory.read_byte(self.hl()));
                Ok(())
            }
            0x40..=0x7F => {
                // LD rd, rs - load register rs's value into register rd depending on the digits of the opcode
                // first, it checks the last 3 bits of the opcode to determine the source
                // then, the opcode is shifted to the right by 3 and then it check the last 3 bits again to determine the destination
                // 000 -> B; 001 -> C; 010 -> D; 011 -> E; 100 -> H; 101 -> L; 110 -> HL (handled above); 111 -> A;
                let source_reg = reg_from_bits(opcode & LAST_THREE_BITS_MASK)
                    .map_err(|e| format!("{} Instruction start: {:#06x}.", e, instruction_start))?;
                let destination_reg = reg_from_bits((opcode >> 3) & LAST_THREE_BITS_MASK)
                    .map_err(|e| format!("{} Instruction start: {:#06x}.", e, instruction_start))?;
                self.load_reg(destination_reg, source_reg);
                Ok(())
            }
            0x86 => {
                // ADD A, (HL) - Add the value found at the adress HL to the value in register a
                self.add_to_reg(Reg::A, memory.read_byte(self.hl()));
                Ok(())
            }
            0x80..=0x87 => {
                // ADD A, r - Add register r's value to the value in register a
                let source_reg = reg_from_bits(opcode & LAST_THREE_BITS_MASK)
                    .map_err(|e| format!("{} Instruction start: {:#06x}.", e, instruction_start))?;
                self.add_to_reg(Reg::A, self.get_reg(source_reg));
                Ok(())
            }
            _ => {
                // Not implemented or unrecognized opcode
                Err(format!(
                    "Operation with opcode {:#04x} is not implemented/recognized. Instruction start: {:#06x}",
                    opcode, instruction_start
                ))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Memory {
    pub ram: [u8; 1 << 16],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { ram: [0; 1 << 16] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value
    }
}
