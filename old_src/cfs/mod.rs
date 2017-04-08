extern crate fuse;

use time::Timespec;
use fuse::*;
use std::path::Path;

mod fs;
mod file;
mod context;

pub struct CFS {
	mnt: String,
	fs: fs::FS,
}

impl CFS {
	pub fn new(mnt: &str) -> CFS {
		CFS {
			mnt: mnt.to_string(),
			fs: fs::FS { context: context::Context::new() },
		}
	}

	pub fn start(self) {
		fuse::mount(self.fs, &Path::new(&self.mnt), &[]);
		();
	}
}

const CREATE_TIME: Timespec = Timespec { sec: 1381237736, nsec: 0 };

pub const GENERIC_DIR_ATTR: FileAttr = FileAttr {
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

pub const TTL: Timespec = Timespec { sec: 1, nsec: 0 };
