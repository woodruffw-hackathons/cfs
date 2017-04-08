use fuse::*;
use cfs;

const GENERIC_FILE_ATTR: FileAttr = FileAttr {
	ino: 1,
	size: 0,
	blocks: 0,
	atime: cfs::CREATE_TIME,
	mtime: cfs::CREATE_TIME,
	ctime: cfs::CREATE_TIME,
	crtime: cfs::CREATE_TIME,
	kind: FileType::RegularFile,
	perm: 0o755,
	nlink: 2,
	uid: 501,
	gid: 20,
	rdev: 0,
	flags: 0,
};

pub struct File {
	name: str,
	// kind: CFSKind,
	attr: FileAttr,
}

impl File {
	pub fn new(name: str) -> File {
		File { name: name, attr: GENERIC_FILE_ATTR }
	}
}
