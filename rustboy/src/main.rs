use rustboy::{Cpu, Memory};

fn main() {
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

    // HALT.
    memory.write_byte(5, 0x76);

    loop {
        match cpu.step(&mut memory) {
            Ok(()) => {
                if cpu.halted {
                    break;
                }
            }
            Err(message) => {
                eprintln!("Error: {}", message);
                break;
            }
        }
    }
    println!("{} {}", cpu.a, cpu.b)
}
