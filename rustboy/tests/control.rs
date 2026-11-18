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
    let mut memory = Memory::new();

    memory.write_byte(0, 0x10);
    cpu.step(&mut memory).unwrap();

    assert!(cpu.stopped);
}
