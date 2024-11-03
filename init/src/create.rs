use std::{error::Error, fs, io, path::PathBuf};

use git2::Repository;
use home::home_dir;

const ARCHIVE_URL: &str = "https://github.com/byrdocs/byrdocs-archive";
const SCRIPTS_UTL: &str = "https://github.com/byrdocs/byrdocs-scripts";

pub fn create_root_dir() -> Result<PathBuf, Box<dyn Error>> {
	let Some(home_dir) = home_dir() else {
		return Err(Box::new(io::Error::new(
			io::ErrorKind::Other,
			format!("Couldn't get home_dir()"),
		)));
	};
	let root_dir = home_dir.join("BYRDOCS");
	fs::create_dir(root_dir.clone())?;
	Ok(root_dir)
}

pub fn create_archive_dir(root_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
	let archive_dir = root_dir.join("archive");
	match Repository::clone(ARCHIVE_URL, archive_dir.clone()) {
		| Ok(_) => Ok(archive_dir),
		| Err(_) => Err(Box::new(io::Error::new(
			io::ErrorKind::Other,
			format!(
				"Failed to clone {} to {}",
				ARCHIVE_URL,
				archive_dir.to_str().unwrap()
			),
		))),
	}
}

pub fn create_cache_dir(root_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
	let cache_dir = root_dir.join(".cache");
	fs::create_dir(cache_dir.clone())?;
	Ok(cache_dir)
}

pub fn create_stockpile_dir(root_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
	let stockpile_dir = root_dir.join("stockpile");
	match Repository::clone(SCRIPTS_UTL, stockpile_dir.clone()) {
		| Ok(_) => Ok(stockpile_dir),
		| Err(_) => Err(Box::new(io::Error::new(
			io::ErrorKind::Other,
			format!(
				"Failed to clone {} to {}",
				SCRIPTS_UTL,
				stockpile_dir.to_str().unwrap()
			),
		))),
	}
}
