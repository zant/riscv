use super::*;
use std::boxed::Box;

pub const RAM_SIZE: usize = 1024 * 1024 * 2;

pub struct RAM {
	cells: Box<[u32; RAM_SIZE / 4]>,
}

impl MMIODevice for RAM {
	fn read(&self, address: u32) -> u32 {
		return self.cells[(address as usize) & RAM_SIZE / 4 - 1];
	}
	fn write(&mut self, address: u32, value: u32) {
		println!("address {:#0x}", address);
		println!("masked {:#0x}", (address as usize) & RAM_SIZE / 4 - 1);
		self.cells[(address as usize) & RAM_SIZE / 4 - 1] = value;
	}
}

impl RAM {
	pub fn new() -> RAM {
		RAM {
			cells: Box::new([0; RAM_SIZE / 4]),
		}
	}
}
