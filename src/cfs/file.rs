use fuse::*;

pub struct File {
	name: String,
	// kind: CFSKind,
	attr: FileAttr,
}

impl File {
	fn new(name: &str, attr: FileAttr) -> File {
		File { name: name.to_string(), attr: attr }
	}
}
