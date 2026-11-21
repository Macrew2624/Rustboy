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
    // initialize cpu, cpu_expected, memory and mem_expected
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    // prepare memory to execute a 0x00 (NOP)
    memory.write_byte(0, 0x00);
    mem_expected.write_byte(0, 0x00);

    // execute the 0x00
    cpu.step(&mut memory).unwrap();
    cpu_expected.pc += 1;

    // verify that the operation changed pc and nothing else
    assert_eq!(cpu.pc, 1);
    assert!(memory == mem_expected);
    assert!(cpu == cpu_expected);
}

#[test]
fn stop() {
    // initialize cpu, cpu_expected, memory and mem_expected
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    // prepare the memory to execute a 0x10 (STOP)
    memory.write_byte(0, 0x10);
    mem_expected.write_byte(0, 0x10);

    // execute the stop
    cpu.step(&mut memory).unwrap();
    cpu_expected.stopped = true;
    cpu_expected.pc += 2;

    // verify that the cpu is stopped, the operation moved pc by 2 and nothing else was changed
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 2);
    assert!(cpu == cpu_expected);
    assert!(memory == mem_expected);

    cpu.step(&mut memory).unwrap();

    //verify that taking another step does nothing
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 2);
    assert!(cpu == cpu_expected);
    assert!(memory == mem_expected);
}

#[test]
fn halt() {
    // initialize cpu, cpu_expected, memory and mem_expected
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    // prepare the memory to execute a 0x76 (HALT)
    memory.write_byte(0, 0x76);
    mem_expected.write_byte(0, 0x76);

    // execute the operation
    cpu.step(&mut memory).unwrap();
    cpu_expected.halted = true;
    cpu_expected.pc += 1;

    // verify that the cpu is halted, the operation moved pc by 1 and nothing else changed
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 1);
    assert!(cpu == cpu_expected);
    assert!(mem_expected == memory);

    cpu.step(&mut memory).unwrap();

    // verify that taking another step does nothing
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 1);
    assert!(cpu == cpu_expected);
    assert!(mem_expected == memory);
}

#[test]
fn interrupt_wakes_a_stopped_cpu() {
    // initialize cpu, cpu_expected, memory and mem_expected
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    // prepare the memory to set the cpu as stopped
    memory.write_byte(0, 0x10);
    memory.write_byte(1, 0xFF);
    mem_expected.write_byte(0, 0x10);
    mem_expected.write_byte(1, 0xFF);

    // execute 0x10 (STOP), skip the next byte (this moves pc by 2 and stops the cpu)
    cpu.step(&mut memory).unwrap();
    cpu_expected.stopped = true;
    cpu_expected.pc += 2;

    // verify the cpu is stopped and nothing else in the cpu or memory was tampered with
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to interrupt the cpu (so it's not stopped anymore) and to execute a NOP (0x00)
    memory.write_byte(0xFF0F, 0b00010000);
    memory.write_byte(0xFFFF, 0b00010000);
    memory.write_byte(2, 0x00);
    mem_expected.write_byte(0xFF0F, 0b00010000);
    mem_expected.write_byte(0xFFFF, 0b00010000);
    mem_expected.write_byte(2, 0x00);

    // execute 0x00 (NOP) after checking IE and IF (this moves pc by 1)
    cpu.step(&mut memory).unwrap();
    cpu_expected.stopped = false;
    cpu_expected.pc += 1;

    // verify the cpu is not stopped anymore, nothing else was tampered with
    assert!(!cpu.stopped);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to stop the cpu again
    memory.write_byte(3, 0x10);
    mem_expected.write_byte(3, 0x10);

    // execute 0x10 (STOP), skip the next byte (this moves pc by 2 and stops the cpu)
    cpu.step(&mut memory).unwrap();
    cpu_expected.stopped = true;
    cpu_expected.pc += 2;

    // verify the cpu is stopped and nothing else in the cpu or memory was tampered with
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 5);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to test that only bit 4 can wake the cpu from a stop
    memory.write_byte(0xFF0F, 0b11101111);
    memory.write_byte(0xFFFF, 0b11101111);
    mem_expected.write_byte(0xFF0F, 0b11101111);
    mem_expected.write_byte(0xFFFF, 0b11101111);

    cpu.step(&mut memory).unwrap();

    // verify that step did nothing, and nothing else was changed
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 5);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to test that setting only IF doesn't wake the cpu
    memory.write_byte(0xFF0F, 0b00010000);
    mem_expected.write_byte(0xFF0F, 0b00010000);

    cpu.step(&mut memory).unwrap();

    // verify that the cpu did not wake up, and that nothing else was changed
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 5);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to test that setting only IE doesn't wake the cpu
    memory.write_byte(0xFF0F, 0b00000000);
    memory.write_byte(0xFFFF, 0b00010000);
    mem_expected.write_byte(0xFF0F, 0b00000000);
    mem_expected.write_byte(0xFFFF, 0b00010000);

    cpu.step(&mut memory).unwrap();

    // verify that the cpu did not wake up, and that nothing else was changed
    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 5);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);
}

#[test]
fn interrupt_wakes_a_halted_cpu() {
    // initialize cpu, cpu_expected, memory and mem_expected
    let mut cpu = start_cpu();
    let mut cpu_expected = start_cpu();
    let mut memory = Memory::new();
    let mut mem_expected = Memory::new();

    // prepare the memory to halt the cpu
    memory.write_byte(0, 0x76);
    mem_expected.write_byte(0, 0x76);

    // execute the operation, increasing pc by 1
    cpu.step(&mut memory).unwrap();
    cpu_expected.halted = true;
    cpu_expected.pc += 1;

    // verify that the cpu is halted, pc increased by 1 and nothing else changed
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to wake the cpu up and to execute a 0x00 (NOP)
    memory.write_byte(0xFF0F, 0b00011111);
    memory.write_byte(0xFFFF, 0b00011111);
    memory.write_byte(1, 0x00);
    mem_expected.write_byte(0xFF0F, 0b00011111);
    mem_expected.write_byte(0xFFFF, 0b00011111);
    mem_expected.write_byte(1, 0x00);

    // wake the cpu up and execute the operation
    cpu.step(&mut memory).unwrap();
    cpu_expected.halted = false;
    cpu_expected.pc += 1;

    // verify that the cpu is no longer halted, pc increased by 1 and nothing else changed
    assert!(!cpu.halted);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to halt the cpu again
    memory.write_byte(2, 0x76);
    mem_expected.write_byte(2, 0x76);

    // execute the operation, increasing pc by 1
    cpu.step(&mut memory).unwrap();
    cpu_expected.halted = true;
    cpu_expected.pc += 1;

    // verify that the cpu is halted, pc increased by 1 and nothing else changed
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to prove that setting bit 5, 6 and 7 in IE and IF does nothing
    memory.write_byte(0xFF0F, 0b11100000);
    memory.write_byte(0xFFFF, 0b11100000);
    mem_expected.write_byte(0xFF0F, 0b11100000);
    mem_expected.write_byte(0xFFFF, 0b11100000);

    cpu.step(&mut memory).unwrap();

    // verify that the cpu did not wake up
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to prove that setting only IF doesn't wake up the cpu
    memory.write_byte(0xFF0F, 0b00000001);
    mem_expected.write_byte(0xFF0F, 0b00000001);

    cpu.step(&mut memory).unwrap();

    // verify that the cpu did not wake up
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);

    // prepare the memory to prove that setting only IE doesn't wake up the cpu
    memory.write_byte(0xFF0F, 0b00000000);
    memory.write_byte(0xFFFF, 0b00000001);
    mem_expected.write_byte(0xFF0F, 0b00000000);
    mem_expected.write_byte(0xFFFF, 0b00000001);

    cpu.step(&mut memory).unwrap();

    // verify that the cpu did not wake up
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu, cpu_expected);
    assert_eq!(memory, mem_expected);
}
