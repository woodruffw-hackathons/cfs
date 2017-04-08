extern crate fuse;
extern crate libc;
extern crate time;

use std::env;
use std::ffi::OsStr;
use fuse::*;
use libc::{ENOENT, EACCES};
use std::path::Path;
use time::Timespec;


const CREATE_TIME: Timespec = Timespec { sec: 1381237736, nsec: 0 };

const GENERIC_DIR_ATTR: FileAttr = FileAttr {
	ino: 1,
	size: 0,
	blocks: 0,
	atime: CREATE_TIME,
	mtime: CREATE_TIME,
	ctime: CREATE_TIME,
	crtime: CREATE_TIME,
	kind: FileType::Directory,
	perm: 0o755,
	nlink: 2,
	uid: 501,
	gid: 20,
	rdev: 0,
	flags: 0,
};

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

pub struct CFS {
	pub files: Vec<String>
}

impl Filesystem for CFS {
	fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
		// if parent == 1 && name.to_str() == Some("hello.txt") {
		// 	reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
		// } else {
		// 	reply.error(ENOENT);
		// }
		reply.error(ENOENT);
	}

	fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
		match ino {
			0 | 1 => reply.attr(&TTL, &GENERIC_DIR_ATTR),
			_ => reply.error(ENOENT),
		}
	}

	fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: u64, _size: u32, reply: ReplyData) {
		// if ino == 2 {
		// 	reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
		// } else {
		// 	reply.error(ENOENT);
		// }
		reply.error(ENOENT);
	}

	fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: u64, mut reply: ReplyDirectory) {
		if ino == 1 {
			if offset == 0 {
				reply.add(1, 0, FileType::Directory, ".");
				reply.add(1, 1, FileType::Directory, "..");
				// reply.add(2, 2, FileType::RegularFile, "hello.txt");
			}
			reply.ok();
		} else {
			reply.error(ENOENT);
		}
	}

	fn create(&mut self, _req: &Request, _parent: u64, name: &OsStr, _mode: u32, _flags: u32, reply: ReplyCreate) {
		print!("{:?}\n", name);
		self.files.push(name.to_string_lossy().into_owned());
		print!("{:?}\n", self.files);
		reply.error(EACCES);
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let mnt = Path::new(&args[1]);

	let cfs = CFS { files: Vec::new() };

	fuse::mount(cfs, &mnt, &[]).unwrap();
}
