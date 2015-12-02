extern crate crypto;

use std::fs::File;
use std::io::*;
use std::env;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

fn hash_file(filename: &String) -> Result<String> {
	const buffer_sz: usize = 0x10000usize;
	let mut file = try!(File::open(filename));
	let mut buffer: [u8; buffer_sz] = [0u8; buffer_sz];
	let mut sh = Sha1::new();
	loop {
		let count = try!(file.read(&mut buffer));
		if count == 0 {
			break;
		}
		sh.input(&buffer);
	}
	Ok(sh.result_str())
}

fn main() {
	let mut args = env::args();
	if args.len() < 2 {
		println!("Usage: rim [filenames...](required)");
	}
	args.next();
	for arg in args {
		let filename: String = arg;
		let sha1result: String;
		match hash_file(&filename) {
			Ok(res) => {
				sha1result = res;
				println!("{} {}", sha1result, filename);
			},
			Err(err) => {
				println!("{}: {}", filename, err);
				return;
			}
		}
	}
}
