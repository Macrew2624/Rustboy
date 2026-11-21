use rustboy::{
    CARRY_FLAG_MASK, Cpu, HALFCARRY_FLAG_MASK, Memory, Reg, SUBTRACT_FLAG_MASK, ZERO_FLAG_MASK,
    reg_from_bits,
};

use crate::common::start_cpu;
mod common;

#[test]
fn cpu_new() {
    // initialize a new Cpu object
    let cpu = Cpu::new();

    // verify that all registers are zero and that the cpu is not halted or stopped
    assert_eq!(cpu.pc, 0);
    assert_eq!(cpu.sp, 0);
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.f, 0);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0);
    assert!(!cpu.halted);
    assert!(!cpu.stopped);
}

#[test]
fn cpu_zero_flag() {
    // initialize a new Cpu object
    let mut cpu = Cpu::new();

    // verify that the last 4 bits of F aren't relevant
    cpu.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());

    // verify that bit 7 is the Zero Flag
    cpu.f = 0x80;
    assert!(cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
}

#[test]
fn cpu_subtract_flag() {
    // initialize a new Cpu object
    let mut cpu = Cpu::new();

    // verify that the last 4 bits of F aren't relevant
    cpu.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());

    // verify that bit 6 is the Subtract Flag
    cpu.f = 0x40;
    assert!(!cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
}

#[test]
fn cpu_halfcarry_flag() {
    // initialize a new Cpu object
    let mut cpu = Cpu::new();

    // verify that the last 4 bits of F aren't relevant
    cpu.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());

    // verify that bit 5 is the Half-Carry Flag
    cpu.f = 0x20;
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
}

#[test]
fn cpu_carry_flag() {
    // initialize a new Cpu object
    let mut cpu = Cpu::new();

    // verify that the last 4 bits of F aren't relevant
    cpu.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());

    // verify that bit 4 is the Carry Flag
    cpu.f = 0x10;
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(cpu.carry_flag());
}

#[test]
fn cpu_get_reg() {
    // initialize the cpu
    let cpu = common::start_cpu();

    // verify that get_reg returns the correct value for each register
    assert_eq!(cpu.get_reg(Reg::A), 1);
    assert_eq!(cpu.get_reg(Reg::B), 2);
    assert_eq!(cpu.get_reg(Reg::C), 3);
    assert_eq!(cpu.get_reg(Reg::D), 4);
    assert_eq!(cpu.get_reg(Reg::E), 5);
    assert_eq!(cpu.get_reg(Reg::H), 6);
    assert_eq!(cpu.get_reg(Reg::L), 7);

    // verify that the registers were mapped correctly
    assert_eq!(cpu.get_reg(Reg::A), cpu.a);
    assert_eq!(cpu.get_reg(Reg::B), cpu.b);
    assert_eq!(cpu.get_reg(Reg::C), cpu.c);
    assert_eq!(cpu.get_reg(Reg::D), cpu.d);
    assert_eq!(cpu.get_reg(Reg::E), cpu.e);
    assert_eq!(cpu.get_reg(Reg::H), cpu.h);
    assert_eq!(cpu.get_reg(Reg::L), cpu.l);
}

#[test]
fn cpu_set_reg() {
    // initialize an array that contains all registers
    const ALL_REGS: [Reg; 7] = [Reg::A, Reg::B, Reg::C, Reg::D, Reg::E, Reg::H, Reg::L];

    // initialize a vector to store failed function calls
    let mut failed = Vec::new();

    // iterate through all registers, setting each
    for (i, reg) in ALL_REGS.into_iter().enumerate() {
        // initialize an new cpu for every iteration
        let mut cpu = start_cpu();
        let mut cpu_expected = start_cpu();

        // set register r to index + 1
        cpu.set_reg(reg, i as u8 + 1);

        // simulate the same set
        match reg {
            Reg::A => cpu_expected.a = i as u8 + 1,
            Reg::B => cpu_expected.b = i as u8 + 1,
            Reg::C => cpu_expected.c = i as u8 + 1,
            Reg::D => cpu_expected.d = i as u8 + 1,
            Reg::E => cpu_expected.e = i as u8 + 1,
            Reg::H => cpu_expected.h = i as u8 + 1,
            Reg::L => cpu_expected.l = i as u8 + 1,
        }

        // verify that nothing else changed
        if cpu != cpu_expected {
            failed.push(reg);
        }
    }

    // verify that all operations completed successfully
    assert!(failed.is_empty(), "failed registers: {:?}", failed);
}

#[test]
fn cpu_hl() {
    // initialize a new Cpu object
    let mut cpu = Cpu::new();

    // verify that HL starts as 0 since both H and L are 0
    assert_eq!(cpu.hl(), 0);

    // verify that setting register H modifies HL's top nibble
    cpu.h = 0x14;
    assert_eq!(cpu.hl(), 0x1400);

    // verify that setting register L modifies HL's bottom nibble
    cpu.l = 0x23;
    assert_eq!(cpu.hl(), 0x1423);

    // verify that HL is unsigned
    cpu.h = 0xFF;
    assert_eq!(cpu.hl(), 0xFF23);

    // verify that changing both H and L completely change HL
    cpu.h = 0x12;
    cpu.l = 0x80;
    assert_eq!(cpu.hl(), 0x1280);
}

#[test]
fn cpu_set_hl() {
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();

    cpu.set_hl(0x1234);
    cpu_expected.h = 0x12;
    cpu_expected.l = 0x34;

    // verify that setting HL changes HL, H, L and nothing else
    assert_eq!(cpu.hl(), 0x1234);
    assert_eq!(cpu.h, 0x12);
    assert_eq!(cpu.l, 0x34);
    assert_eq!(cpu, cpu_expected);

    cpu.set_hl(0x0987);
    cpu_expected.h = 0x09;
    cpu_expected.l = 0x87;

    // verify that setting HL doesn't merge the new values into the old ones, completely changing them
    assert_eq!(cpu.hl(), 0x0987);
    assert_eq!(cpu.h, 0x09);
    assert_eq!(cpu.l, 0x87);
    assert_eq!(cpu, cpu_expected);
}

#[test]
fn cpu_set_flag() {
    // init cpu and cpu_expected
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();

    // set the first 4 bits as 1 to prove it doesn't do anything
    cpu.f = 0x0F;
    cpu_expected.f = 0x0F;

    // verify that setting the Zero Flag true makes it true and doesn't change anything else
    cpu.set_flag(true, ZERO_FLAG_MASK);
    cpu_expected.f = 0x8F;
    assert!(cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == cpu_expected);

    // verify that setting the Subtract Flag true makes it true and doesn't change anything else
    cpu.set_flag(true, SUBTRACT_FLAG_MASK);
    cpu_expected.f = 0xCF;
    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == cpu_expected);

    // verify that setting the Half-Carry Flag true makes it true and doesn't change anything else
    cpu.set_flag(true, HALFCARRY_FLAG_MASK);
    cpu_expected.f = 0xEF;
    assert!(cpu.zero_flag());
    assert!(cpu.halfcarry_flag());
    assert!(cpu.subtract_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == cpu_expected);

    // verify that setting the Carry Flag true makes it true and doesn't change anything else
    cpu.set_flag(true, CARRY_FLAG_MASK);
    cpu_expected.f = 0xFF;
    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());
    assert!(cpu.carry_flag());
    assert!(cpu == cpu_expected);

    // verify that setting already true flags doesn't change anything
    cpu.set_flag(true, 0xF0);
    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());
    assert!(cpu.carry_flag());
    assert!(cpu == cpu_expected);

    // verify that setting the Carry Flag false makes it false and doesn't change anything else
    cpu.set_flag(false, CARRY_FLAG_MASK);
    cpu_expected.f = 0xEF;
    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == cpu_expected);

    // verify that setting the Half-Carry Flag false makes it false and doesn't change anything else
    cpu.set_flag(false, HALFCARRY_FLAG_MASK);
    cpu_expected.f = 0xCF;
    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == cpu_expected);

    // verify that setting the Subtract Flag false makes it false and doesn't change anything else
    cpu.set_flag(false, SUBTRACT_FLAG_MASK);
    cpu_expected.f = 0x8F;
    assert!(cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == cpu_expected);

    // verify that setting the Zero Flag false makes it false and doesn't change anything else
    cpu.set_flag(false, ZERO_FLAG_MASK);
    cpu_expected.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == cpu_expected);

    // verify that setting already false flags doesn't change anythign
    cpu.set_flag(false, 0xF0);
    cpu_expected.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == cpu_expected);
}

#[test]
fn cpu_fetch_byte() {
    // initialize cpu, cpu_expected, memory and mem_expected
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    // verify that the cpu's pc starts at 0
    assert_eq!(cpu.pc, 0);

    // prepare the non-zero values to prove that fetch_byte returns them properly
    memory.write_byte(0, 1);
    memory.write_byte(1, 2);
    memory.write_byte(2, 3);
    mem_expected.write_byte(0, 1);
    mem_expected.write_byte(1, 2);
    mem_expected.write_byte(2, 3);

    // verify that fetching returns correctly, pc moves by 1 and nothing else changes
    cpu_expected.pc += 1;
    assert_eq!(cpu.fetch_byte(&memory), 1);
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // verify that fetching increments pc (doesn't set it to 1) and reads the memory correctly
    cpu_expected.pc += 1;
    assert_eq!(cpu.fetch_byte(&memory), 2);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare pc to fetch from an arbitrary adress
    cpu.pc = 0x1234;
    cpu_expected.pc = 0x1234;

    // prepare memory to fetch from an arbitrary adress
    memory.write_byte(0x1234, 4);
    mem_expected.write_byte(0x1234, 4);

    // verify that fetching from an arbitrary adress returns the correct value,
    // increases pc by 1 and doesn't change anything else
    cpu_expected.pc += 1;
    assert_eq!(cpu.fetch_byte(&memory), 4);
    assert_eq!(cpu.pc, 0x1235);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare pc to fetch the last adress possible
    cpu.pc = 0xFFFF;
    cpu_expected.pc = 0xFFFF;

    // prepare memory to fetch the last adress possible
    memory.write_byte(0xFFFF, 67);
    mem_expected.write_byte(0xFFFF, 67);

    // verify that fetching from the last possible adress returns the correct value,
    // overflows pc and doesn't change anything else
    cpu_expected.pc = 0;
    assert_eq!(cpu.fetch_byte(&memory), 67);
    assert_eq!(cpu.pc, 0);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);
}

#[test]
fn cpu_load_immediate() {
    // initialize an array that contains all registers
    const ALL_REGS: [Reg; 7] = [Reg::A, Reg::B, Reg::C, Reg::D, Reg::E, Reg::H, Reg::L];

    // initialize a vector to store failed function calls
    let mut failed = Vec::new();

    // iterate through all registers, loading each register with a value from memory
    for (i, reg) in ALL_REGS.into_iter().enumerate() {
        // initialize a new cpu and memory for every iteration
        let mut cpu = start_cpu();
        let mut cpu_expected = start_cpu();
        let mut memory = Memory::new();
        let mut mem_expected = Memory::new();

        // prepare the memory to load a distinct nonzero value
        memory.write_byte(0, i as u8 + 1);
        mem_expected.write_byte(0, i as u8 + 1);

        // load the value into the coresponding register, increasing pc by 1
        cpu.load_immediate(reg, &memory);

        // simulate the same load
        cpu_expected.pc += 1;
        match reg {
            Reg::A => cpu_expected.a = 1,
            Reg::B => cpu_expected.b = 2,
            Reg::C => cpu_expected.c = 3,
            Reg::D => cpu_expected.d = 4,
            Reg::E => cpu_expected.e = 5,
            Reg::H => cpu_expected.h = 6,
            Reg::L => cpu_expected.l = 7,
        }

        // since we have a new cpu every iteration, verify that pc increased by 1
        assert_eq!(cpu.pc, 1);

        // verify that nothing else was changed
        if cpu != cpu_expected || memory != mem_expected {
            failed.push(reg);
        }
    }

    // verify that all operations completed successfully
    assert!(failed.is_empty(), "failed registers: {:?}", failed);
}

#[test]
fn cpu_load_reg() {
    // initialize an array that contains all registers
    const ALL_REGS: [Reg; 7] = [Reg::A, Reg::B, Reg::C, Reg::D, Reg::E, Reg::H, Reg::L];

    // initialize a vector to store failed function calls
    let mut failed = Vec::new();

    // iterate through all registers, loading each register with a value from another register
    for reg1 in ALL_REGS {
        for reg2 in ALL_REGS {
            // initialize a new cpu for every iteration
            let mut cpu = start_cpu();
            let mut cpu_expected = start_cpu();

            // load register 2's value into register 1
            cpu.load_reg(reg1, reg2);

            // simulate the same load
            let value = cpu_expected.get_reg(reg2);
            cpu_expected.set_reg(reg1, value);

            // verify nothing else changed
            if cpu != cpu_expected {
                failed.push((reg1, reg2));
            }
        }
    }

    // verify that all operations completed successfully
    assert!(failed.is_empty(), "failed registers: {:?}", failed);
}

#[test]
fn cpu_add_to_reg() {
    // initialize a new cpu
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();

    // prepare the registers to test 1 + 1
    cpu.set_reg(Reg::A, 0x01);
    cpu.add_to_reg(Reg::A, 0x01);

    // recreate the addition
    cpu_expected.set_reg(Reg::A, 0x02);
    cpu_expected.f = 0x00;

    // assert no flags were set and nothing else was changed
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert_eq!(cpu, cpu_expected);

    // prepare the registers to test 16 + 16
    cpu.set_reg(Reg::A, 0x10);
    cpu.add_to_reg(Reg::A, 0x10);

    // recreate the addition
    cpu_expected.set_reg(Reg::A, 0x20);
    cpu_expected.f = 0x00;

    // assert no flags were set and nothing else was changed
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert_eq!(cpu, cpu_expected);

    // prepare the registers for 8 + 8
    cpu.set_reg(Reg::A, 0x08);
    cpu.add_to_reg(Reg::A, 0x08);

    // recreate the addition
    cpu_expected.set_reg(Reg::A, 0x10);
    cpu_expected.f = 0x20;

    // verify that only the Half-Carry Flag was set and nothing else
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert_eq!(cpu, cpu_expected);

    // prepare the registers for 240 + 16
    cpu.set_reg(Reg::A, 0xF0);
    cpu.add_to_reg(Reg::A, 0x10);

    // recreate the addition
    cpu_expected.set_reg(Reg::A, 0x00);
    cpu_expected.f = 0x90;

    // verify that only the Zero and Carry Flag were changed and nothing else
    assert!(cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(cpu.carry_flag());
    assert_eq!(cpu, cpu_expected);

    // prepare the registers for 255 + 1
    cpu.set_reg(Reg::A, 0xFF);
    cpu.add_to_reg(Reg::A, 0x01);

    // recreate the addition
    cpu_expected.set_reg(Reg::A, 0x00);
    cpu_expected.f = 0xB0;

    // verify that only the Zero, Half-Carry and Carry Flags were changed and nothing else
    assert!(cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());
    assert!(cpu.carry_flag());
    assert_eq!(cpu, cpu_expected);

    // prepare the registers for 0 + 0
    cpu.set_reg(Reg::A, 0x00);
    cpu.add_to_reg(Reg::A, 0x00);

    // recreate the addition
    cpu_expected.set_reg(Reg::A, 0x00);
    cpu_expected.f = 0x80;

    // verify that only the Zero Flag was changed and nothing else
    assert!(cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert_eq!(cpu, cpu_expected);

    // initialize an array that contains all registers
    const ALL_REGS: [Reg; 7] = [Reg::A, Reg::B, Reg::C, Reg::D, Reg::E, Reg::H, Reg::L];

    // initialize a vector to store failed function calls
    let mut failed = Vec::new();

    // iterate through all registers, setting each
    for reg in ALL_REGS {
        // initialize an new cpu for every iteration
        let mut cpu = start_cpu();
        let mut cpu_expected = start_cpu();

        // add 255 to the register
        cpu.add_to_reg(reg, 0xFF);

        // simulate the same add
        match reg {
            Reg::A => cpu_expected.a = cpu_expected.a.wrapping_add(0xFF),
            Reg::B => cpu_expected.b = cpu_expected.b.wrapping_add(0xFF),
            Reg::C => cpu_expected.c = cpu_expected.c.wrapping_add(0xFF),
            Reg::D => cpu_expected.d = cpu_expected.d.wrapping_add(0xFF),
            Reg::E => cpu_expected.e = cpu_expected.e.wrapping_add(0xFF),
            Reg::H => cpu_expected.h = cpu_expected.h.wrapping_add(0xFF),
            Reg::L => cpu_expected.l = cpu_expected.l.wrapping_add(0xFF),
        }

        if reg == Reg::A {
            cpu_expected.f = 0xB0;
        } else {
            cpu_expected.f = 0x30;
        }

        // verify that nothing else changed
        if cpu != cpu_expected {
            failed.push(reg);
        }
    }

    // verify that all operations completed successfully
    assert!(failed.is_empty(), "failed registers: {:?}", failed);
}

#[test]
fn cpu_inc_reg() {
    // initialize an array that contains all registers
    const ALL_REGS: [Reg; 7] = [Reg::A, Reg::B, Reg::C, Reg::D, Reg::E, Reg::H, Reg::L];

    // initialize a vector to store failed function calls
    let mut failed = Vec::new();

    // iterate through all registers, increasing each by 1
    for reg in ALL_REGS {
        // initialize a new cpu for every iteration
        let mut cpu = start_cpu();
        let mut cpu_expected = start_cpu();

        // set the register to 0xFF
        match reg {
            Reg::A => cpu.a = 0xFF,
            Reg::B => cpu.b = 0xFF,
            Reg::C => cpu.c = 0xFF,
            Reg::D => cpu.d = 0xFF,
            Reg::E => cpu.e = 0xFF,
            Reg::H => cpu.h = 0xFF,
            Reg::L => cpu.l = 0xFF,
        }

        //increase reg by 1
        cpu.inc_reg(reg);

        // simulate the same inc
        cpu_expected.set_reg(reg, 0);
        cpu_expected.f = 0xB0;

        // verify that nothing else changed
        if cpu != cpu_expected {
            failed.push(reg);
        }
    }

    // verify that all operations completed successfully
    assert!(failed.is_empty(), "failed registers: {:?}", failed);
}

#[test]
fn helper_reg_from_bits() {
    // iterate through all values an unsigned 8 bit integer can have
    for index in 0..=255 {
        // verify that the 7 registers are chosen correctly and that anything else returns an error
        let value = index as u8;
        match value {
            0 => assert_eq!(reg_from_bits(value).unwrap(), Reg::B),
            1 => assert_eq!(reg_from_bits(value).unwrap(), Reg::C),
            2 => assert_eq!(reg_from_bits(value).unwrap(), Reg::D),
            3 => assert_eq!(reg_from_bits(value).unwrap(), Reg::E),
            4 => assert_eq!(reg_from_bits(value).unwrap(), Reg::H),
            5 => assert_eq!(reg_from_bits(value).unwrap(), Reg::L),
            7 => assert_eq!(reg_from_bits(value).unwrap(), Reg::A),
            _ => assert!(reg_from_bits(value).is_err()),
        }
    }
}

#[test]
fn memory_new() {
    // initialize a new Memory object
    let memory = Memory::new();

    // verify that memory starts as an array of size 65536 filled with zeroes
    assert_eq!(memory.ram.len(), u16::MAX as usize + 1);
    for val in &memory.ram {
        assert_eq!(*val, 0);
    }
}

#[test]
fn memory_read_write_byte() {
    // initialize a new Memory object
    let mut memory = Memory::new();

    // change every value in memory to semi-distinct non-zero integers
    for i in 0..memory.ram.len() {
        let mut value = (i as u8).wrapping_add(1) ^ ((i >> 8) as u8);
        if value == 0 {
            value = 1;
        }
        memory.write_byte(i as u16, value);
    }

    // verify that every value was changed
    for i in 0..memory.ram.len() {
        let mut value = (i as u8).wrapping_add(1) ^ ((i >> 8) as u8);
        if value == 0 {
            value = 1;
        }
        assert_eq!(memory.read_byte(i as u16), value);
    }
}
