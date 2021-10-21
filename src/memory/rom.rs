use super::*;

// div 4 to be in u32
pub const ROM_SIZE: usize = 1024 * 1024 / 4;

pub struct ROMDevice {
	cells: [u32; ROM_SIZE],
}

impl MMIODevice for ROMDevice {
	fn read(&self, address: u32) -> u32 {
		return self.cells[(address as usize) & ROM_SIZE - 1];
	}
	fn write(&mut self, _: u32, _: u32) {}
}

impl ROMDevice {
	pub fn new() -> ROMDevice {
		ROMDevice {
			cells: [0; ROM_SIZE],
		}
	}
}
