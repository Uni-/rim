use std::ffi::OsString;
use std::fs::*;

pub type RimFile = File;

pub fn scan_wd() -> Vec<String> {
	let mut wd_all: Vec<String> = vec![".".to_owned()];
	let ignore_starts: Vec<&'static str> = vec![".git", "target"];
	let mut index = 0usize;
	while index != wd_all.len() {
		let meta = metadata(&wd_all[index]).unwrap();
		if meta.is_dir() {
			let subs = read_dir(&wd_all[index]).unwrap();
			let mut append = 0usize;
			for sub in subs {
				let entry: OsString;
				if index == 0usize {
					entry = sub.unwrap().file_name();
				} else {
					entry = sub.unwrap().path().into_os_string();
				}
				let pathstr = entry.into_string().unwrap();
				let mut ignore = false;
				for i in 0..ignore_starts.len() {
					if pathstr.starts_with(ignore_starts[i]) {
						ignore = true;
					}
				}
				if !ignore {
					append += 1;
					wd_all.insert(index + append, pathstr);
				}
			}
		}
		index += 1;
	}
	wd_all.remove(0);
	wd_all
}
