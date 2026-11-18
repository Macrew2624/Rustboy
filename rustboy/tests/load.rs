/*
    Test every possible LD function
    Verify it functions how it's supposed to.
    Veirfy it doesn't tamper with anything else.
    Opcodes to test:
    0x01 LD BC, u16
    0x02 LD (BC), A
    0x08 LD (u16), SP
    0x0A LD A, (BC)
    0x11 LD DE, u16
    0x12 LD (DE), A
    0x1A LD A, (DE)
    0x21 LD HL, u16
    0x22 LD (HL+), A
    0x2A LD A, (HL+)
    0x31 LD SP, u16
    0x32 LD (HL-), A
    0x36 LD (HL), n
    0x3A LD A, (HL-)
    0xE0 LD (FF00+u8), A
    0xE2 LD (FF00+C), A
    0xEA LD (u16), A
    0xF0 LD A, (FF00+u8)
    0xF2 LD A, (FF00+C)
    0xF8 LD HL, SP+i8
    0xF9 LD SP, HL
    0xFA LD A, (u16)
*/
use rustboy::{LAST_THREE_BITS_MASK, Memory, reg_from_bits};

mod common;
use common::start_cpu;

#[test]
fn ld_rd_rs() {
    let start = common::start_cpu();
    let mem_start = Memory::new();

    let mut failed = Vec::new();
    let mut counter = 0;
    for opcode in 0x40..=0x7F {
        if opcode & LAST_THREE_BITS_MASK != 0b110 && !(0x70..=0x77).contains(&opcode) {
            let mut cpu = start.clone();
            let mut memory = mem_start.clone();
            let mut cpu_expected = start.clone();
            let mut mem_expected = mem_start.clone();
            cpu_expected.pc += 1;

            let src = reg_from_bits(opcode & rustboy::LAST_THREE_BITS_MASK).unwrap();
            let dst = reg_from_bits((opcode >> 3) & rustboy::LAST_THREE_BITS_MASK).unwrap();

            cpu_expected.set_reg(dst, cpu_expected.get_reg(src));

            mem_expected.write_byte(0, opcode);
            memory.write_byte(0, opcode);
            cpu.step(&mut memory).unwrap();

            if cpu != cpu_expected || memory != mem_expected {
                failed.push(opcode);
            }
            counter += 1;
        }
    }

    assert_eq!(counter, 49);
    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}

#[test]
fn ld_hl_r() {
    let cpu_start = common::start_cpu();
    let mem_start = Memory::new();

    let mut failed = Vec::new();
    for opcode in 0x70..=0x77 {
        if opcode != 0x76 {
            let mut cpu = cpu_start.clone();
            let mut memory = mem_start.clone();
            let mut cpu_expected = cpu_start.clone();
            let mut mem_expected = mem_start.clone();

            let src = reg_from_bits(opcode & LAST_THREE_BITS_MASK).unwrap();

            mem_expected.write_byte(0, opcode);
            mem_expected.write_byte(cpu_expected.hl(), cpu_expected.get_reg(src));

            memory.write_byte(0, opcode);
            cpu.step(&mut memory).unwrap();
            cpu_expected.pc += 1;

            if cpu != cpu_expected || memory != mem_expected {
                failed.push(opcode);
            }
        }
    }

    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}

#[test]
fn ld_r_hl() {
    let cpu_start = common::start_cpu();
    let mem_start = Memory::new();

    let mut failed = Vec::new();
    for opcode in 0x40..=0x7F {
        if opcode & LAST_THREE_BITS_MASK == 0b110 && (opcode >> 3) & LAST_THREE_BITS_MASK != 0b110 {
            let mut cpu = cpu_start.clone();
            let mut memory = mem_start.clone();
            let mut cpu_expected = cpu_start.clone();
            let mut mem_expected = mem_start.clone();

            memory.write_byte(cpu.hl(), 0x12);
            mem_expected.write_byte(cpu_expected.hl(), 0x12);

            let dst = reg_from_bits((opcode >> 3) & LAST_THREE_BITS_MASK).unwrap();

            mem_expected.write_byte(0, opcode);
            cpu_expected.set_reg(dst, 0x12);

            memory.write_byte(0, opcode);
            cpu.step(&mut memory).unwrap();
            cpu_expected.pc += 1;

            if cpu != cpu_expected || memory != mem_expected {
                failed.push(opcode);
            }
        }
    }

    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}

#[test]
fn ld_r_u8() {
    let cpu_start = common::start_cpu();
    let mem_start = Memory::new();

    let mut failed = Vec::new();
    for opcode in 0x06..=0x3E {
        if opcode & LAST_THREE_BITS_MASK == 0b110 && (opcode >> 3) & LAST_THREE_BITS_MASK != 0b110 {
            let mut cpu = cpu_start.clone();
            let mut memory = mem_start.clone();
            let mut cpu_expected = cpu_start.clone();
            let mut mem_expected = mem_start.clone();

            memory.write_byte(0, opcode);
            mem_expected.write_byte(0, opcode);
            memory.write_byte(1, 0x12);
            mem_expected.write_byte(1, 0x12);

            let dst = reg_from_bits((opcode >> 3) & LAST_THREE_BITS_MASK).unwrap();
            cpu_expected.set_reg(dst, 0x12);

            cpu.step(&mut memory).unwrap();
            cpu_expected.pc += 2;

            if cpu != cpu_expected || memory != mem_expected {
                failed.push(opcode);
            }
        }
    }

    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}

#[test]
fn ld_hl_u8() {
    let cpu_start = start_cpu();
}
