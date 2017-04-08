use cfs::file::File;

pub struct Context {
	files: Vec<File>
}

impl Context {
	pub fn new() -> Context {
		Context { files: Vec::new() }
	}

	pub fn add_file(&mut self, filename: str) {
		let file = File::new(filename);
		self.files.push(file);
	}
}
