pub mod file;
pub mod memory;

use std::env::Args;

pub fn handle_args(mut args: Args) {
	args.next();
	match args.next() {
		Some(arg) => file::read_file(arg),
		None => println!("No file"),
	}
}
