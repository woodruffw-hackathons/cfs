use cfs::file::File;

pub struct Context {
	files: Vec<File>
}

impl Context {
	pub fn new() -> Context {
		Context { files: Vec::new() }
	}
}
