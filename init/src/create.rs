use std::{error::Error, fs, io, path::PathBuf};

use git2::Repository;

const ARCHIVE_URL: &str = "https://github.com/byrdocs/byrdocs-archive.git";
const SCRIPTS_UTL: &str = "https://github.com/byrdocs/byrdocs-scripts.git";

/// If root-dir in the configurations not found, then create one.
/// If root-dir already exists, nothing will be created.
pub fn create_root_dir(root_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
	if !root_dir.exists() {
		fs::create_dir(&root_dir)?;
	}
	Ok(())
}

pub fn create_archive_dir(archive_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
	println!("Cloning byrdocs-archive from github...");
	match Repository::clone(ARCHIVE_URL, &archive_dir) {
		| Ok(_) => Ok(()),
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

pub fn create_cache_dir(cache_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
	fs::create_dir(&cache_dir)?;
	Ok(())
}

pub fn create_stockpile_dir(stockpile_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
	println!("Cloning byrdocs-scripts(stockpile) from github...");
	match Repository::clone(SCRIPTS_UTL, &stockpile_dir) {
		| Ok(_) => Ok(()),
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
