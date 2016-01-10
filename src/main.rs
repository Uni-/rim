use std::env;

extern crate crypto;

pub mod fs {
	pub mod blob;
	pub mod tree;
}
use fs::tree::*;
use fs::blob::*;

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
	let wd_contents = scan_wd();
	let wd_count = wd_contents.len();
	for path in wd_contents {
		println!("{}", path);
	}
	println!("{} files and folders found. done.", wd_count);
}
