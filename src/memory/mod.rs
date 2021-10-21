pub mod ram;
pub mod rom;

pub trait MMIODevice {
  fn read(&self, address: u32) -> u32;
  fn write(&mut self, address: u32, value: u32);
}

pub enum MemoryMap {
  ProgramROMStart = 0x10000000,
  ProgramROMEnd = 0x1fffffff,
  RAMStart = 0x20000000,
  RAMEnd = 0x2fffffff,
}

pub struct SystemInterface {
  rom: rom::ROMDevice,
  ram: ram::RAM,
}

impl SystemInterface {
  pub fn new() -> SystemInterface {
    let rom = rom::ROMDevice::new();
    let ram = ram::RAM::new();
    SystemInterface { rom, ram }
  }
}

impl MMIODevice for SystemInterface {
  fn read(&self, address: u32) -> u32 {
    if (address & 0b11) != 0 {
      panic!("Unaligned read from address {}", address)
    }

    if address & MemoryMap::ProgramROMStart as u32 == MemoryMap::ProgramROMStart as u32 {
      return self.rom.read((address & 0x0fffffff) >> 2);
    }

    if address & MemoryMap::RAMStart as u32 == MemoryMap::RAMStart as u32 {
      return self.ram.read((address & 0x0fffffff) >> 2);
    }

    panic!("Not supported");
  }

  fn write(&mut self, address: u32, value: u32) {
    if (address & 0b11) != 0 {
      panic!("Unaligned write from address {}, value {}", address, value)
    }

    if address & MemoryMap::RAMStart as u32 == MemoryMap::RAMStart as u32 {
      self.ram.write((address & 0x0fffffff) >> 2, value);
    }
  }
}
