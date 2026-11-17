use rustboy::{CARRY_FLAG_MASK, Cpu, Memory, Reg};

#[test]
fn setting_register_a_leaves_the_rest_unmodified() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 0x06);
    memory.write_byte(1, 42);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.f, 0);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0);
}

#[test]
fn setting_a_register_moves_pc_by_two() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 0x3E);
    memory.write_byte(1, 42);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.pc, 2);

    memory.write_byte(2, 0x06);
    memory.write_byte(3, 42);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.pc, 4);

    memory.write_byte(4, 0x0E);
    memory.write_byte(5, 42);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.pc, 6);
}

#[test]
fn setting_a_register_modifies_it() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 0x3E);
    memory.write_byte(1, 42);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 42);

    memory.write_byte(2, 0x06);
    memory.write_byte(3, 67);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.b, 67);
    assert_eq!(cpu.a, 42);

    memory.write_byte(4, 0x0E);
    memory.write_byte(5, 72);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.c, 72);
    assert_eq!(cpu.b, 67);
    assert_eq!(cpu.a, 42);
}

#[test]
fn after_copying_the_value_of_a_register_to_another_register_they_are_equal() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 0x3E);
    memory.write_byte(1, 42);
    cpu.step(&mut memory).unwrap();

    memory.write_byte(2, 0x06);
    memory.write_byte(3, 67);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.b, 67);
    assert_eq!(cpu.a, 42);

    memory.write_byte(4, 0x78);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.b, 67);
    assert_eq!(cpu.a, 67);
}

#[test]
fn uninmplemented_opcode_returns_error() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 0xFF);

    assert!(cpu.step(&mut memory).is_err());

    memory.write_byte(1, 0xFF);
    let message = cpu.step(&mut memory).unwrap_err();
    assert!(message.contains("0x0001"));
}

#[test]
fn halt_stops_pc_from_advancing() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 0x76);
    cpu.step(&mut memory).unwrap();

    assert!(cpu.halted);
    assert_eq!(cpu.pc, 1);

    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.pc, 1);
}

#[test]
fn non_overflowing_addition_leaves_carry_flag_false() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.set_flag(true, CARRY_FLAG_MASK);
    assert!(cpu.carry_flag());

    // LD A, 1
    memory.write_byte(0, 0x3E);
    memory.write_byte(1, 1);

    // LD B, 2
    memory.write_byte(2, 0x06);
    memory.write_byte(3, 2);

    // ADD A, B
    memory.write_byte(4, 0x80);

    cpu.step(&mut memory).unwrap();
    cpu.step(&mut memory).unwrap();
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 3);
    assert_eq!(cpu.b, 2);
    assert_eq!(cpu.pc, 5);
    assert!(!cpu.carry_flag());
}

#[test]
fn overflowing_addition_leaves_carry_flag_true() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    // LD A, 200
    memory.write_byte(0, 0x3E);
    memory.write_byte(1, 200);

    // LD B, 100
    memory.write_byte(2, 0x06);
    memory.write_byte(3, 100);

    // ADD A, B
    memory.write_byte(4, 0x80);

    cpu.step(&mut memory).unwrap();
    cpu.step(&mut memory).unwrap();
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 44);
    assert_eq!(cpu.b, 100);
    assert_eq!(cpu.pc, 5);
    assert!(cpu.carry_flag());
}

#[test]
fn addition_correctly_sets_flags() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 15);
    cpu.load_immediate(Reg::A, &memory);

    memory.write_byte(1, 1);
    cpu.load_immediate(Reg::B, &memory);

    memory.write_byte(2, 0x80);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 16);
    assert!(cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(!cpu.carry_flag());

    memory.write_byte(3, 255);
    cpu.load_immediate(Reg::A, &memory);

    memory.write_byte(4, 1);
    cpu.load_immediate(Reg::B, &memory);

    memory.write_byte(5, 0x80);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 0);
    assert!(cpu.zero_flag());
    assert!(cpu.carry_flag());
    assert!(!cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());

    memory.write_byte(6, 6);
    cpu.load_immediate(Reg::A, &memory);

    memory.write_byte(7, 4);
    cpu.load_immediate(Reg::B, &memory);

    memory.write_byte(8, 0x80);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 10);
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(!cpu.carry_flag());
}

#[test]
fn hl_works() {
    let mut cpu = Cpu::new();

    cpu.set_hl(0x1234);
    assert_eq!(cpu.h, 0x12);
    assert_eq!(cpu.l, 0x34);
    assert_eq!(cpu.hl(), 0x1234);

    cpu.set_hl(0x00FF);
    assert_eq!(cpu.h, 0x00);
    assert_eq!(cpu.l, 0xFF);
    assert_eq!(cpu.hl(), 0x00FF);
}

#[test]
fn operaion_0x86_works() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0x1234, 7);
    memory.write_byte(0, 30);
    cpu.load_immediate(Reg::A, &memory);

    cpu.set_hl(0x1234);

    memory.write_byte(1, 0x86);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 7);
    assert_eq!(cpu.hl(), 0x1234);
    assert_eq!(cpu.a, 37);
}

#[test]
fn ld_to_hl_works() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 30);
    cpu.load_immediate(Reg::A, &memory);

    cpu.set_hl(0x1234);

    memory.write_byte(1, 0x77);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 30);
    assert_eq!(cpu.hl(), 0x1234);

    memory.write_byte(2, 0x74);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 0x12);
}

#[test]
fn ld_from_hl_works() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.set_hl(0x1234);
    memory.write_byte(0x1234, 99);

    memory.write_byte(0, 0x46);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 99);
    assert_eq!(cpu.b, 99);

    memory.write_byte(1, 0x5E);
    memory.write_byte(0x1234, 55);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 55);
    assert_eq!(cpu.e, 55);
    assert_eq!(cpu.b, 99);
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);
    assert_eq!(cpu.l, 0x34);
    assert_eq!(cpu.h, 0x12);
}

#[test]
fn ld_hl_n_writes_to_memory() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.set_hl(0x1234);
    memory.write_byte(0, 0x36);
    memory.write_byte(1, 77);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 77);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn ld_immediate_works() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 0x3E);
    memory.write_byte(1, 77);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 77);
    assert_eq!(cpu.pc, 2);

    memory.write_byte(2, 0x2E);
    memory.write_byte(3, 67);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.l, 67);
    assert_eq!(cpu.pc, 4);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);

    memory.write_byte(4, 0x16);
    memory.write_byte(5, 20);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.d, 20);
    assert_eq!(cpu.pc, 6);

    memory.write_byte(6, 0x26);
    memory.write_byte(7, 44);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.h, 44);
    assert_eq!(cpu.pc, 8);
    assert_eq!(cpu.hl(), 0x2C43);
    assert_eq!(cpu.l, 67);
    assert_eq!(cpu.a, 77);
    assert_eq!(cpu.d, 20);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
}

#[test]
fn inc_correctly_sets_flags() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.set_flag(true, CARRY_FLAG_MASK);

    memory.write_byte(0, 15);
    cpu.load_immediate(Reg::A, &memory);

    memory.write_byte(1, 1);
    cpu.load_immediate(Reg::B, &memory);

    memory.write_byte(2, 0x04);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.b, 2);

    memory.write_byte(3, 0x3C);
    cpu.step(&mut memory).unwrap();
    assert_eq!(cpu.a, 16);
    assert!(cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());

    memory.write_byte(4, 255);
    cpu.load_immediate(Reg::A, &memory);

    memory.write_byte(5, 1);
    cpu.load_immediate(Reg::B, &memory);

    memory.write_byte(6, 0x3C);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.b, 1);
    assert!(cpu.zero_flag());
    assert!(cpu.carry_flag());
    assert!(!cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());

    memory.write_byte(7, 6);
    cpu.load_immediate(Reg::A, &memory);

    memory.write_byte(8, 4);
    cpu.load_immediate(Reg::B, &memory);

    memory.write_byte(9, 0x3C);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 7);
    assert_eq!(cpu.b, 4);
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());
}

#[test]
fn dec_correctly_sets_flags() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.set_flag(true, CARRY_FLAG_MASK);

    memory.write_byte(0, 15);
    cpu.load_immediate(Reg::A, &memory);

    memory.write_byte(1, 1);
    cpu.load_immediate(Reg::B, &memory);

    memory.write_byte(2, 0x05);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.b, 0);
    assert!(!cpu.halfcarry_flag());
    assert!(cpu.zero_flag());
    assert!(cpu.carry_flag());

    memory.write_byte(3, 0x3D);
    cpu.step(&mut memory).unwrap();
    assert_eq!(cpu.a, 14);
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());

    memory.write_byte(4, 0);
    cpu.load_immediate(Reg::A, &memory);

    memory.write_byte(5, 0);
    cpu.load_immediate(Reg::B, &memory);

    memory.write_byte(6, 0x3D);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 255);
    assert_eq!(cpu.b, 0);
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());
    assert!(cpu.subtract_flag());
    assert!(cpu.halfcarry_flag());

    memory.write_byte(7, 6);
    cpu.load_immediate(Reg::A, &memory);

    memory.write_byte(8, 4);
    cpu.load_immediate(Reg::B, &memory);

    memory.write_byte(9, 0x3D);
    cpu.step(&mut memory).unwrap();

    assert_eq!(cpu.a, 5);
    assert_eq!(cpu.b, 4);
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());
}

#[test]
fn inc_hl_works() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.set_hl(0x1234);
    cpu.set_flag(true, CARRY_FLAG_MASK);
    memory.write_byte(0x1234, 0x0F);

    memory.write_byte(0, 0x34);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 0x10);
    assert!(cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(cpu.carry_flag());
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu.hl(), 0x1234);

    memory.write_byte(0x1234, 0x06);
    memory.write_byte(1, 0x34);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 0x07);
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(!cpu.subtract_flag());
    assert!(cpu.carry_flag());
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.hl(), 0x1234);
}

#[test]
fn dec_hl_works() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.set_hl(0x1234);
    cpu.set_flag(true, CARRY_FLAG_MASK);
    memory.write_byte(0x1234, 0x10);

    memory.write_byte(0, 0x35);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 0x0F);
    assert!(cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(cpu.carry_flag());
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu.hl(), 0x1234);

    memory.write_byte(0x1234, 0x06);
    memory.write_byte(1, 0x35);
    cpu.step(&mut memory).unwrap();

    assert_eq!(memory.read_byte(0x1234), 0x05);
    assert!(!cpu.halfcarry_flag());
    assert!(!cpu.zero_flag());
    assert!(cpu.subtract_flag());
    assert!(cpu.carry_flag());
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.hl(), 0x1234);
}

#[test]
fn stop_consumes_padding_and_stays_stopped() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_byte(0, 0x10);
    memory.write_byte(1, 0x00);
    cpu.step(&mut memory).unwrap();

    assert!(cpu.stopped);
    assert_eq!(cpu.pc, 2);

    memory.write_byte(2, 0x3E);
    memory.write_byte(3, 77);

    cpu.step(&mut memory).unwrap();
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.a, 0);
}
