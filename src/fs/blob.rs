use std::io::*;

use crypto::sha1::Sha1;
use crypto::digest::Digest;

use fs::tree::RimFile;

pub fn hash_file(filename: &String) -> Result<String> {
	const BUFFER_SZ: usize = 0x10000usize;
	let mut file = try!(RimFile::open(filename));
	let mut buffer: [u8; BUFFER_SZ] = [0u8; BUFFER_SZ];
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