use std::ffi::OsStr;
use fuse::*;
use libc::{ENOENT, EACCES};
use cfs;

pub struct FS;

impl Filesystem for FS {
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
			1 => reply.attr(&cfs::TTL, &cfs::GENERIC_DIR_ATTR),
			// 2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
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

	fn create(&mut self, _req: &Request, _parent: u64, _name: &OsStr, _mode: u32, _flags: u32, reply: ReplyCreate) {
		print!("create: {:?}\n", _name);
		reply.error(EACCES);
	}
}
