/*
    Test every possible control function
    Verify it functions how it's supposed to.
    Veirfy it doesn't tamper with anything else.
    Opcodes to test:
    0x00 NOP
    0x10 STOP
    0x76 HALT
*/

mod common;
use common::start_cpu;
use rustboy::Memory;

#[test]
fn nop() {
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    memory.write_byte(0, 0x00);
    mem_expected.write_byte(0, 0x00);

    cpu.step(&mut memory).unwrap();
    cpu_expected.pc += 1;

    assert!(memory == mem_expected);
    assert!(cpu == cpu_expected);
}

#[test]
fn stop() {
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    mem_expected.write_byte(0, 0x10);

    memory.write_byte(0, 0x10);
    cpu.step(&mut memory).unwrap();
    cpu_expected.stopped = true;
    cpu_expected.pc += 2;

    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 2);
    assert!(cpu == cpu_expected);
    assert!(memory == mem_expected);

    cpu.step(&mut memory).unwrap();
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 2);
    assert!(cpu == cpu_expected);
    assert!(memory == mem_expected);
}

#[test]
fn halt() {
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    mem_expected.write_byte(0, 0x76);

    memory.write_byte(0, 0x76);
    cpu.step(&mut memory).unwrap();
    cpu_expected.halted = true;
    cpu_expected.pc += 1;

    assert!(cpu.halted);
    assert_eq!(cpu.pc, 1);
    assert!(cpu == cpu_expected);
    assert!(mem_expected == memory);

    cpu.step(&mut memory).unwrap();
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 1);
    assert!(cpu == cpu_expected);
    assert!(mem_expected == memory);
}

#[test]
fn interrupt_wakes_a_stopped_cpu() {
    let mut cpu = start_cpu();
    let mut memory = Memory::new();

    memory.write_byte(0, 0x10);
    memory.write_byte(1, 0xFF);
    cpu.step(&mut memory).unwrap();

    assert!(cpu.stopped);

    memory.write_byte(0xFF0F, 0b00010000);
    memory.write_byte(0xFFFF, 0b00010000);

    cpu.step(&mut memory).unwrap();
    assert!(!cpu.stopped);
    assert_eq!(cpu.pc, 3);

    memory.write_byte(3, 0x10);
    cpu.step(&mut memory).unwrap();

    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 5);

    memory.write_byte(0xFF0F, 0b11101111);
    memory.write_byte(0xFFFF, 0b11101111);

    cpu.step(&mut memory).unwrap();
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 5);

    memory.write_byte(0xFF0F, 0b00000001);

    cpu.step(&mut memory).unwrap();
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 5);

    memory.write_byte(0xFF0F, 0b00000000);
    memory.write_byte(0xFFFF, 0b00000001);

    cpu.step(&mut memory).unwrap();
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn interrupt_wakes_a_halted_cpu() {
    let mut cpu = start_cpu();
    let mut memory = Memory::new();

    memory.write_byte(0, 0x76);
    cpu.step(&mut memory).unwrap();

    assert!(cpu.halted);

    memory.write_byte(0xFF0F, 0b00000001);
    memory.write_byte(0xFFFF, 0b00000001);

    cpu.step(&mut memory).unwrap();
    assert!(!cpu.halted);
    assert_eq!(cpu.pc, 2);

    memory.write_byte(2, 0x76);
    cpu.step(&mut memory).unwrap();

    assert!(cpu.halted);
    assert_eq!(cpu.pc, 3);

    memory.write_byte(0xFF0F, 0b11100000);
    memory.write_byte(0xFFFF, 0b11100000);

    cpu.step(&mut memory).unwrap();
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 3);

    memory.write_byte(0xFF0F, 0b00000001);

    cpu.step(&mut memory).unwrap();
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 3);

    memory.write_byte(0xFF0F, 0b00000000);
    memory.write_byte(0xFFFF, 0b00000001);

    cpu.step(&mut memory).unwrap();
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 3);
}
