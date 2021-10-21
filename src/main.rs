use riscv::memory::*;
// use std::env;

struct RIV32System {
    bus: SystemInterface,
}

fn main() {
    let mut rv = RIV32System {
        bus: SystemInterface::new(),
    };

    // Overflows
    let ov_add = MemoryMap::RAMStart as u32 + ram::RAM_SIZE as u32;
    rv.bus.write(ov_add, 0xffcafe);
    let val = rv.bus.read(MemoryMap::RAMStart as u32);
    println!("{:#0x}", val);
}
