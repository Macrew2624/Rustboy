use rustboy::Cpu;

pub fn start_cpu() -> Cpu {
    Cpu {
        pc: 0,
        sp: 8,
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        h: 6,
        l: 7,
        f: 0xF0,
        halted: false,
        stopped: false,
    }
}
