use rustboy::{
    CARRY_FLAG_MASK, Cpu, HALFCARRY_FLAG_MASK, Memory, Reg, SUBTRACT_FLAG_MASK, ZERO_FLAG_MASK,
    reg_from_bits,
};
mod common;

#[test]
fn cpu_new() {
    let cpu = Cpu::new();

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
    let mut cpu = Cpu::new();

    cpu.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.carry_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.subtract_flag());

    cpu.f = 0x80;
    assert!(cpu.zero_flag());
    assert!(!cpu.carry_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.subtract_flag());
}

#[test]
fn cpu_carry_flag() {
    let mut cpu = Cpu::new();

    cpu.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.carry_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.subtract_flag());

    cpu.f = 0x10;
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.subtract_flag());
}

#[test]
fn cpu_halfcarry_flag() {
    let mut cpu = Cpu::new();

    cpu.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.carry_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.subtract_flag());

    cpu.f = 0x20;
    assert!(!cpu.zero_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu.halfcarry_flag());
    assert!(!cpu.subtract_flag());
}

#[test]
fn cpu_subtract_flag() {
    let mut cpu = Cpu::new();

    cpu.f = 0x0F;
    assert!(!cpu.zero_flag());
    assert!(!cpu.carry_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.subtract_flag());

    cpu.f = 0x40;
    assert!(!cpu.zero_flag());
    assert!(!cpu.carry_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(cpu.subtract_flag());
}

#[test]
fn cpu_get_reg() {
    let cpu = common::start_cpu();

    assert_eq!(cpu.get_reg(Reg::A), 1);
    assert_eq!(cpu.get_reg(Reg::B), 2);
    assert_eq!(cpu.get_reg(Reg::C), 3);
    assert_eq!(cpu.get_reg(Reg::D), 4);
    assert_eq!(cpu.get_reg(Reg::E), 5);
    assert_eq!(cpu.get_reg(Reg::H), 6);
    assert_eq!(cpu.get_reg(Reg::L), 7);

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
    let start = Cpu::new();

    const ALL_REGS: [Reg; 7] = [Reg::A, Reg::B, Reg::C, Reg::D, Reg::E, Reg::H, Reg::L];

    let mut failed = Vec::new();
    for (i, reg) in ALL_REGS.into_iter().enumerate() {
        let mut expected = start.clone();
        let mut cpu = start.clone();
        match reg {
            Reg::A => expected.a = i as u8 + 1,
            Reg::B => expected.b = i as u8 + 1,
            Reg::C => expected.c = i as u8 + 1,
            Reg::D => expected.d = i as u8 + 1,
            Reg::E => expected.e = i as u8 + 1,
            Reg::H => expected.h = i as u8 + 1,
            Reg::L => expected.l = i as u8 + 1,
        }

        cpu.set_reg(reg, i as u8 + 1);
        if cpu != expected {
            failed.push(reg);
        }
    }

    assert!(failed.is_empty(), "failed registers: {:?}", failed);
}

#[test]
fn cpu_hl() {
    let mut cpu = Cpu::new();

    assert_eq!(cpu.hl(), 0);

    cpu.h = 0x14;
    assert_eq!(cpu.hl(), 0x1400);

    cpu.l = 0x23;
    assert_eq!(cpu.hl(), 0x1423);

    cpu.h = 0xFF;
    assert_eq!(cpu.hl(), 0xFF23);

    cpu.h = 0x12;
    cpu.l = 0x80;
    assert_eq!(cpu.hl(), 0x1280);
}

#[test]
fn cpu_set_hl() {
    let start = common::start_cpu();
    let mut expected = start.clone();
    let mut cpu = start.clone();

    cpu.set_hl(0x1234);
    expected.h = 0x12;
    expected.l = 0x34;
    assert_eq!(cpu.hl(), 0x1234);
    assert_eq!(cpu.h, 0x12);
    assert_eq!(cpu.l, 0x34);
    assert_eq!(cpu, expected);

    cpu.set_hl(0x0987);
    expected.h = 0x09;
    expected.l = 0x87;
    assert_eq!(cpu.hl(), 0x0987);
    assert_eq!(cpu.h, 0x09);
    assert_eq!(cpu.l, 0x87);
    assert_eq!(cpu, expected);
}

#[test]
fn cpu_set_flag() {
    let mut start = common::start_cpu();
    start.f = 0x0F;
    let mut cpu = start.clone();
    let mut expected = start.clone();

    cpu.set_flag(true, ZERO_FLAG_MASK);
    expected.f = 0x8F;

    assert!(cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == expected);

    cpu.set_flag(true, SUBTRACT_FLAG_MASK);
    expected.f = 0xCF;

    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == expected);

    cpu.set_flag(true, HALFCARRY_FLAG_MASK);
    expected.f = 0xEF;

    assert!(cpu.zero_flag());
    assert!(cpu.halfcarry_flag());
    assert!(cpu.subtract_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == expected);

    cpu.set_flag(true, CARRY_FLAG_MASK);
    expected.f = 0xFF;

    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());
    assert!(cpu.carry_flag());
    assert!(cpu == expected);

    cpu.set_flag(true, 0xF0);
    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());
    assert!(cpu.carry_flag());
    assert!(cpu == expected);

    cpu.set_flag(false, CARRY_FLAG_MASK);
    expected.f = 0xEF;

    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == expected);

    cpu.set_flag(false, HALFCARRY_FLAG_MASK);
    expected.f = 0xCF;

    assert!(cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == expected);

    cpu.set_flag(false, SUBTRACT_FLAG_MASK);
    expected.f = 0x8F;

    assert!(cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == expected);

    cpu.set_flag(false, ZERO_FLAG_MASK);
    expected.f = 0x0F;

    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == expected);

    cpu.set_flag(false, 0xF0);
    expected.f = 0x0F;

    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.carry_flag());
    assert!(cpu == expected);
}

#[test]
fn cpu_fetch_byte() {
    let start = common::start_cpu();

    let mut cpu = start.clone();
    let mut expected = start.clone();
    let mut memory = Memory::new();

    assert_eq!(cpu.pc, 0);

    memory.write_byte(0, 1);
    memory.write_byte(1, 2);
    memory.write_byte(2, 3);

    expected.pc += 1;
    assert_eq!(cpu.fetch_byte(&memory), 1);
    assert_eq!(cpu.pc, 1);
    assert!(expected == cpu);

    expected.pc += 1;
    assert_eq!(cpu.fetch_byte(&memory), 2);
    assert_eq!(cpu.pc, 2);
    assert!(expected == cpu);

    expected.pc += 1;
    assert_eq!(cpu.fetch_byte(&memory), 3);
    assert_eq!(cpu.pc, 3);
    assert!(expected == cpu);

    cpu.pc = 0x1234;
    expected.pc = 0x1234;

    expected.pc += 1;
    memory.write_byte(0x1234, 4);
    assert_eq!(cpu.fetch_byte(&memory), 4);
    assert_eq!(cpu.pc, 0x1235);
    assert!(expected == cpu);

    cpu.pc = 0xFFFF;
    expected.pc = 0xFFFF;

    memory.write_byte(0xFFFF, 67);

    expected.pc = 0;
    assert_eq!(cpu.fetch_byte(&memory), 67);
    assert_eq!(cpu.pc, 0);
    assert!(expected == cpu);
}

#[test]
fn cpu_load_immediate() {
    let start = common::start_cpu();
    let mut memory = Memory::new();

    const ALL_REGS: [Reg; 7] = [Reg::A, Reg::B, Reg::C, Reg::D, Reg::E, Reg::H, Reg::L];

    let mut failed = Vec::new();
    for (i, reg) in ALL_REGS.into_iter().enumerate() {
        let mut expected = start.clone();
        let mut cpu = start.clone();

        memory.write_byte(0, i as u8 + 1);
        cpu.load_immediate(reg, &memory);
        assert!(cpu.pc == 1);
        expected.pc += 1;

        match reg {
            Reg::A => expected.a = 1,
            Reg::B => expected.b = 2,
            Reg::C => expected.c = 3,
            Reg::D => expected.d = 4,
            Reg::E => expected.e = 5,
            Reg::H => expected.h = 6,
            Reg::L => expected.l = 7,
        }

        if cpu != expected {
            failed.push(reg);
        }
    }

    assert!(failed.is_empty(), "failed registers: {:?}", failed);
}

#[test]
fn cpu_load_reg() {
    let start = common::start_cpu();

    const ALL_REGS: [Reg; 7] = [Reg::A, Reg::B, Reg::C, Reg::D, Reg::E, Reg::H, Reg::L];

    let mut failed = Vec::new();
    for reg1 in ALL_REGS {
        for reg2 in ALL_REGS {
            let mut cpu = start.clone();
            let mut expected = start.clone();

            cpu.load_reg(reg1, reg2);

            let value = expected.get_reg(reg2);
            expected.set_reg(reg1, value);

            if cpu != expected {
                failed.push((reg1, reg2));
            }
        }
    }
    assert!(failed.is_empty(), "failed registers: {:?}", failed);
}

#[test]
fn helper_reg_from_bits() {
    for index in 0..=255 {
        let value = index as u8;
        match value {
            0 => assert_eq!(reg_from_bits(value).unwrap(), Reg::B),
            1 => assert_eq!(reg_from_bits(value).unwrap(), Reg::C),
            2 => assert_eq!(reg_from_bits(value).unwrap(), Reg::D),
            3 => assert_eq!(reg_from_bits(value).unwrap(), Reg::E),
            4 => assert_eq!(reg_from_bits(value).unwrap(), Reg::H),
            5 => assert_eq!(reg_from_bits(value).unwrap(), Reg::L),
            6 => assert!(reg_from_bits(value).is_err()),
            7 => assert_eq!(reg_from_bits(value).unwrap(), Reg::A),
            _ => assert!(reg_from_bits(value).is_err()),
        }
    }
}

#[test]
fn memory_new() {
    let memory = Memory::new();

    assert_eq!(memory.ram.len(), u16::MAX as usize + 1);
    for val in &memory.ram {
        assert_eq!(*val, 0);
    }
}

#[test]
fn memory_read_write_byte() {
    let mut memory = Memory::new();

    for i in 0..memory.ram.len() {
        let mut value = (i as u8).wrapping_add(1) ^ ((i >> 8) as u8);
        if value == 0 {
            value = 1;
        }
        memory.write_byte(i as u16, value);
    }

    for i in 0..memory.ram.len() {
        let mut value = (i as u8).wrapping_add(1) ^ ((i >> 8) as u8);
        if value == 0 {
            value = 1;
        }
        assert_eq!(memory.read_byte(i as u16), value);
    }
}
