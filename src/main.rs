extern crate crypto;

pub mod cl;
pub mod fs {
	pub mod blob;
	pub mod tree;
}

use cl::*;

fn main() {
	parse_args();
}
