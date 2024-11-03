use std::{error::Error, io, path::PathBuf};

use filesystem::getconf::get_root_dir;

pub fn check_root_dir() -> Result<PathBuf, Box<dyn Error>> {
	get_root_dir()
}
pub fn check_subdirctory(root_dir: PathBuf, sub_dir: &str) -> Result<(), Box<dyn Error>> {
	let subdir = root_dir.join(sub_dir);
	if subdir.exists() {
		Ok(())
	} else {
		Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "")))
	}
}
