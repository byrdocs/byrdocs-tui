use core::error;
use std::error::Error;
use std::path::PathBuf;
use std::{fs, io};

use home::home_dir;

#[derive(Debug)]
pub enum ErrorKind {
	IO(String),
	FS(String),
}

pub fn get_root_dir() -> Result<PathBuf, Box<dyn Error>> {
	let home = home_dir();
	if let Some(home) = home {
		let conf_path: PathBuf = [home, PathBuf::from(".config/byrdocs/root-dir")]
			.iter()
			.collect();
		if !conf_path.exists() {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::NotFound,
				format!("{} doesn't exist.", conf_path.to_str().unwrap()),
			)));
		}
		let target_path = PathBuf::from(fs::read_to_string(conf_path).unwrap().trim());
		if !target_path.exists() {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::NotFound,
				format!("{} doesn't exist.", target_path.to_str().unwrap()),
			)));
		}
		Ok(target_path)
	} else {
		Err(Box::new(io::Error::new(
			io::ErrorKind::Other,
			String::from("Couldn't extract home_dir()"),
		)))
	}
}
