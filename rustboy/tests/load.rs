/*
    Test every possible LD function
    Verify it functions how it's supposed to.
    Veirfy it doesn't tamper with anything else.
    Opcodes to test:
    0x01 LD BC, u16
    0x02 LD (BC), A
    0x06 LD B, n
    0x08 LD (u16), SP
    0x0A LD A, (BC)
    0x0E LD C, n
    0x11 LD DE, u16
    0x12 LD (DE), A
    0x16 LD D, n
    0x1A LD A, (DE)
    0x1E LD E, n
    0x21 LD HL, u16
    0x22 LD (HL+), A
    0x26 LD H, n
    0x2A LD A, (HL+)
    0x2E LD L, n
    0x31 LD SP, u16
    0x32 LD (HL-), A
    0x36 LD (HL), n
    0x3A LD A, (HL-)
    0x3E LD A, n
    0x46 LD B, (HL)
    0x4E LD C, (HL)
    0x56 LD D, (HL)
    0x5E LD E, (HL)
    0x66 LD H, (HL)
    0x6E LD L, (HL)
    0x7E LD A, (HL)
    0xE0 LD (FF00+u8), A
    0xE2 LD (FF00+C), A
    0xEA LD (u16), A
    0xF0 LD A, (FF00+u8)
    0xF2 LD A, (FF00+C)
    0xF8 LD HL, SP+i8
    0xF9 LD SP, HL
    0xFA LD A, (u16)
*/

use rustboy::{Cpu, LAST_THREE_BITS_MASK, Memory, Reg, reg_from_bits};

#[test]
fn ld_rd_rs_works() {
    let start = Cpu {
        pc: 0, sp: 0,
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        h: 6,
        l: 7,
        f: 0,
        halted: false,
        stopped: false,
    };
    let mut memory = Memory::new();

    let mut failed = Vec::new();
    for opcode in 0x40..=0x7F {
        if opcode & LAST_THREE_BITS_MASK != 0b110 && !(0x70..=0x77).contains(&opcode) {
            let mut cpu = start.clone();
            let mut expected = cpu.clone();
            expected.pc += 1;

            let src = reg_from_bits(opcode & rustboy::LAST_THREE_BITS_MASK).unwrap();
            let dst = reg_from_bits((opcode >> 3) & rustboy::LAST_THREE_BITS_MASK).unwrap();

            expected.set_reg(dst, expected.get_reg(src));

            memory.write_byte(0, opcode);
            cpu.step(&mut memory).unwrap();

            if cpu != expected {
                failed.push(opcode);
            }
        }
    }

    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}

#[test]
fn ld_hl_r_works() {
    let cpu_start = Cpu {
        pc: 0,
        sp: 0,
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        h: 6,
        l: 7,
        f: 0,
        halted: false,
        stopped: false,
    };
    let mem_start = Memory::new();

    let mut failed = Vec::new();
    for opcode in 0x70..=0x77 {
        if opcode != 0x76 {
            let mut cpu = cpu_start.clone();
            let mut memory = mem_start.clone();
            let mut cpu_expected = cpu.clone();
            let mut mem_expected = memory.clone();
            cpu_expected.pc += 1;

            let src = match opcode & LAST_THREE_BITS_MASK {
                0b000 => Reg::B,
                0b001 => Reg::C,
                0b010 => Reg::D,
                0b011 => Reg::E,
                0b100 => Reg::H,
                0b101 => Reg::L,
                0b111 => Reg::A,
                _ => unreachable!(),
            };

            mem_expected.write_byte(cpu_expected.hl(), cpu_expected.get_reg(src));

            memory.write_byte(0, opcode);
            cpu.step(&mut memory).unwrap();

            if cpu != cpu_expected || memory != mem_expected {
                failed.push(opcode);
            }
        }
    }

    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}
