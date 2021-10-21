use std::{fs::File, io::prelude::*, io::BufReader, str};

pub fn read_file(name: String) {
	let f = File::open(name).unwrap();
	let mut rdr = BufReader::new(f);
	let mut buf: [u8; 1] = Default::default();
	rdr.read_exact(&mut buf).unwrap();
	println!("{}", str::from_utf8(&buf).unwrap());
}
