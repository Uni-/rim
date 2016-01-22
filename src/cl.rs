use std::env;

use fs::tree::*;
use fs::blob::*;

fn print_builtin_help() {
	macro_rules! builtin_help_message {
		() => ("Usage: rim <command>\n")
	};
	print!(builtin_help_message![]);
}

pub fn parse_args() -> Vec<String> {
	let mut args = env::args();
	args.next();
	match args.next().as_ref().map(|s| &s[..]) {
		None => {
			print_builtin_help();
		},
		Some("status") => {
			let wd_contents = scan_wd();
			let wd_count = wd_contents.len();
			for path in wd_contents {
				println!("{}", path);
			}
			println!("{} files and folders found. done.", wd_count);
		},
		Some("hash") => {
			for filename in args {
				match hash_file(&filename) {
					Ok(sha1_res) => println!("{} {}", sha1_res, filename),
					Err(err) => println!("{}: {}", filename, err),
				}
			}
		},
		Some(s) => {
			println!("{}: no such command, please check usage", s);
		},
	}

	vec![]
}
