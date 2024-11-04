use std::error::Error;
use std::io;
use std::path::PathBuf;

use config::getconf::get_config;

pub fn check_root_dir() -> Result<PathBuf, Box<dyn Error>> {
	let yaml = match get_config() {
		| Ok(o) => o,
		| Err(e) => return Err(e),
	};
	let yaml = yaml.first().unwrap();
	let root_dir = PathBuf::from(yaml["root-dir"].as_str().unwrap());
	if !root_dir.exists() {
		return Err(Box::new(io::Error::new(
			io::ErrorKind::NotFound,
			format!("{} doesn't exist.", root_dir.to_str().unwrap()),
		)));
	}
	Ok(root_dir)
}

pub fn check_subdirctory(root_dir:&PathBuf, sub_dir: &str) -> Result<(), Box<dyn Error>> {
	let subdir = root_dir.join(sub_dir);
	if subdir.exists() {
		Ok(())
	} else {
		Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "")))
	}
}
