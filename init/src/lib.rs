use std::{error::Error, io, path::PathBuf};

use check::{check_root_dir, check_subdirctory};
use create::{
	create_archive_dir, create_cache_dir, create_conf, create_root_dir, create_stockpile_dir,
};
use config::getconf::get_config;

pub mod check;
pub mod create;

pub fn check() -> Result<PathBuf, Box<dyn Error>> {
	if let Err(_) = get_config() {
		println!("Warning: It seems that There is no BYRDOCS workspace in your computer, or the configuration file is deleted.");
		println!("Input a directory so that we can create the workspace: ");
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read the path");
		if let Err(e) = create_conf(PathBuf::from(input)) {
			return Err(e);
		}
	}
	let root_dir = match check_root_dir() {
		| Ok(o) => o,
		| Err(_) => match create_root_dir() {
			| Ok(o) => o,
			| Err(e) => return Err(e),
		},
	};
	if let Err(_) = check_subdirctory(root_dir.clone(), "archive") {
		if let Err(e) = create_archive_dir(root_dir.clone()) {
			return Err(e);
		}
	}
	if let Err(_) = check_subdirctory(root_dir.clone(), ".cache") {
		if let Err(e) = create_cache_dir(root_dir.clone()) {
			return Err(e);
		}
	}
	if let Err(_) = check_subdirctory(root_dir.clone(), "stockpile") {
		if let Err(e) = create_stockpile_dir(root_dir.clone()) {
			return Err(e);
		}
	}
	Ok(root_dir)
}
