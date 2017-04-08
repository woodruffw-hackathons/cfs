mod cfs;

extern crate fuse;
extern crate libc;
extern crate time;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	let mnt = &args[1];

	let cfs = cfs::CFS::new(mnt);
	cfs.start();

	// fuse::mount(CFS, &mnt, &[]).unwrap();
}
