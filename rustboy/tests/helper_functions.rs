/*
    Test every possible helper function
    Verify it functions how it's supposed to.
    Veirfy it doesn't tamper with anything else.
    Functions to test:
    Cpu
        set_flag
        fetch_byte
        load_immediate, load_reg
        add_to_reg, inc_reg, dec_reg
        inc_value, dec_value
    Memory
        new
        read_byte, write_byte
    Free function
        reg_from_bits
*/

use rustboy::{CARRY_FLAG_MASK, Cpu, HALFCARRY_FLAG_MASK, Reg, SUBTRACT_FLAG_MASK, ZERO_FLAG_MASK};

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
    let cpu = Cpu {
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
    let start = Cpu {
        pc: 99,
        sp: 66,
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        h: 6,
        l: 7,
        f: 8,
        halted: false,
        stopped: false,
    };
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
    let start = Cpu {
        pc: 99,
        sp: 66,
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

    let mut cpu = start.clone();
    let mut expected = start.clone();

    cpu.set_flag(true, ZERO_FLAG_MASK);
    expected.f = 0x80;

    assert!(cpu.zero_flag());
    assert!(cpu == expected);

    cpu.set_flag(true, SUBTRACT_FLAG_MASK);
    expected.f += 0x40;

    assert!(cpu.subtract_flag());
    assert!(cpu == expected);

    cpu.set_flag(true, HALFCARRY_FLAG_MASK);
    expected.f += 0x20;

    assert!(cpu.halfcarry_flag());
    assert!(cpu == expected);

    cpu.set_flag(true, CARRY_FLAG_MASK);
    expected.f += 0x10;

    assert!(cpu.carry_flag());
    assert!(cpu == expected);
}
