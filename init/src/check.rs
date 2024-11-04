use std::error::Error;
use std::io;
use std::path::PathBuf;

pub fn check_root_dir(root_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
	if !root_dir.exists() {
		return Err(Box::new(io::Error::new(
			io::ErrorKind::NotFound,
			format!("{} doesn't exist.", root_dir.to_str().unwrap()),
		)));
	}
	Ok(())
}

pub fn check_subdirctory(root_dir: &PathBuf, sub_dir: &str) -> Result<(), Box<dyn Error>> {
	let subdir = root_dir.join(sub_dir);
	if subdir.exists() {
		Ok(())
	} else {
		Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "")))
	}
}
