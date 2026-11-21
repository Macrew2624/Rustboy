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
mod common;
use common::start_cpu;
use rustboy::{LAST_THREE_BITS_MASK, Memory, reg_from_bits};

#[test]
fn ld_rd_rs() {
    // initialize a vector to store failed function calls
    let mut failed = Vec::new();

    // initialize a vector to count how many operations happened
    let mut counter = 0;

    // iterate through every opcode in range 0x40 to 0x7F
    for opcode in 0x40..=0x7F {
        // exclude operations to/from (HL)
        if opcode & LAST_THREE_BITS_MASK != 0b110 && !(0x70..=0x77).contains(&opcode) {
            // initialize a new cpu and memory every iteration
            let mut cpu = start_cpu();
            let mut cpu_expected = start_cpu();
            let mut memory = Memory::new();
            let mut mem_expected = Memory::new();

            // prepare the memory to execute the opcode
            mem_expected.write_byte(0, opcode);
            memory.write_byte(0, opcode);

            // load the src's register's value into the dst register
            cpu.step(&mut memory).unwrap();

            // compute the source and destination registers for the expected result
            let src = reg_from_bits(opcode & rustboy::LAST_THREE_BITS_MASK).unwrap();
            let dst = reg_from_bits((opcode >> 3) & rustboy::LAST_THREE_BITS_MASK).unwrap();

            //simulate the same load
            cpu_expected.set_reg(dst, cpu_expected.get_reg(src));
            cpu_expected.pc += 1;

            // verify that nothing else changed
            if cpu != cpu_expected || memory != mem_expected {
                failed.push(opcode);
            }

            // increase the number of operations that happened
            counter += 1;
        }
    }

    // verify that the correct number of opcodes were excluded
    assert_eq!(counter, 49);

    //verify that all operations completed successfully
    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}

#[test]
fn ld_hl_r() {
    // initialize a vector to store failed function calls
    let mut failed = Vec::new();

    // iterate through every opcode in range 0x70 to 0x77
    for opcode in 0x70..=0x77 {
        // exclude HALT
        if opcode != 0x76 {
            // initialize a new cpu and memory each iteration
            let mut cpu = start_cpu();
            let mut cpu_expected = start_cpu();
            let mut memory = Memory::new();
            let mut mem_expected = Memory::new();

            // prepare the memory to execute the opcode
            memory.write_byte(0, opcode);

            // load the src register's value into memory at adress (HL)
            cpu.step(&mut memory).unwrap();

            // compute the src register's value for the expected result
            let src = reg_from_bits(opcode & LAST_THREE_BITS_MASK).unwrap();

            // execute the same load
            mem_expected.write_byte(0, opcode);
            mem_expected.write_byte(cpu_expected.hl(), cpu_expected.get_reg(src));
            cpu_expected.pc += 1;

            // verify that nothing else changed
            if cpu != cpu_expected || memory != mem_expected {
                failed.push(opcode);
            }
        }
    }

    // verify that every operation completed successfully
    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}

#[test]
fn ld_r_hl() {
    // initialize a vector to store failed function calls
    let mut failed = Vec::new();

    // iterate through every opcode in range 0x46 to 0x7E
    for opcode in 0x46..=0x7E {
        // only include operations to (HL)
        if opcode & LAST_THREE_BITS_MASK == 0b110 && (opcode >> 3) & LAST_THREE_BITS_MASK != 0b110 {
            // initialize a new cpu and memory for every iteration
            let mut cpu = start_cpu();
            let mut cpu_expected = start_cpu();
            let mut memory = Memory::new();
            let mut mem_expected = Memory::new();

            // prepare the memory to execute the opcode and load value 0x12 into the register
            memory.write_byte(0, opcode);
            memory.write_byte(cpu.hl(), 0x12);

            // load the value from memory at adress (HL) into the dst register
            cpu.step(&mut memory).unwrap();

            // compute the dst register's value for the expected result
            let dst = reg_from_bits((opcode >> 3) & LAST_THREE_BITS_MASK).unwrap();

            // recreate the same load
            mem_expected.write_byte(0, opcode);
            mem_expected.write_byte(cpu_expected.hl(), 0x12);
            cpu_expected.set_reg(dst, 0x12);
            cpu_expected.pc += 1;

            // verify that nothing else was changed
            if cpu != cpu_expected || memory != mem_expected {
                failed.push(opcode);
            }
        }
    }

    // verify that every operation completed successfully
    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}

#[test]
fn ld_r_u8() {
    // initialize a vector to store failed function calls
    let mut failed = Vec::new();

    // initialize a vector to count how many operations happened
    let mut counter = 0;

    // iterate through every opcode in range 0x06 to 0x3E
    for opcode in 0x06..=0x3E {
        // only include the LD r, u8 operations (excluding LD (HL), u8)
        if opcode & LAST_THREE_BITS_MASK == 0b110 && (opcode >> 3) & LAST_THREE_BITS_MASK != 0b110 {
            // initialize a new cpu and memory for every iteration
            let mut cpu = start_cpu();
            let mut cpu_expected = start_cpu();
            let mut memory = Memory::new();
            let mut mem_expected = Memory::new();

            // prepare the memory to execute the opcode and load the value 0x12
            memory.write_byte(0, opcode);
            memory.write_byte(1, 0x12);

            // load the value from memory into the dst register
            cpu.step(&mut memory).unwrap();

            // compute the dst register for the expected value
            let dst = reg_from_bits((opcode >> 3) & LAST_THREE_BITS_MASK).unwrap();

            // recreate the same load
            mem_expected.write_byte(0, opcode);
            mem_expected.write_byte(1, 0x12);
            cpu_expected.set_reg(dst, 0x12);
            cpu_expected.pc += 2;

            // verify that nothing else changed
            if cpu != cpu_expected || memory != mem_expected {
                failed.push(opcode);
            }

            // increase the number of operations that executed
            counter += 1;
        }
    }

    // verify that the correct number of operations were excluded
    assert_eq!(counter, 7);

    // verify that every operation completed successfully
    assert!(failed.is_empty(), "failed opcodes: {:x?}", failed);
}

#[test]
fn ld_hl_u8() {
    // initialize a new cpu and memory
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    // prepare the memory to execute 0x36 (LD (HL), u8), and load value 0x12
    memory.write_byte(0, 0x36);
    memory.write_byte(1, 0x12);
    mem_expected.write_byte(0, 0x36);
    mem_expected.write_byte(1, 0x12);

    // execute the operation, writing 0x12 at the adress (HL) in memory, increasing pc by 2
    cpu.step(&mut memory).unwrap();
    mem_expected.write_byte(cpu_expected.hl(), 0x12);
    cpu_expected.pc += 2;

    // verify that nothing else was changed
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);
}
