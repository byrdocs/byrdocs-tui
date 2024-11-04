use std::{error::Error, io, path::PathBuf};

use check::{check_root_dir, check_subdirctory};
use config::{definition::Config, getconf::get_config};
use create::{
	create_archive_dir, create_cache_dir, create_conf, create_root_dir, create_stockpile_dir,
};

pub mod check;
pub mod create;

pub fn check(config: Config) -> Result<(), Box<dyn Error>> {
	if let Err(_) = get_config() {
		println!("Warning: It seems that There is no BYRDOCS workspace in your computer, or the configuration file is deleted.");
		println!("Input a directory so that we can create the workspace: ");
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read the path");
		if let Err(e) = create_conf(PathBuf::from(shellexpand::tilde(&input).into_owned())) {
			return Err(e);
		}
	}
	let root_dir = PathBuf::from(match check_root_dir(&PathBuf::from(&config.root_dir)) {
		| Ok(_) => &config.root_dir,
		| Err(_) => match create_root_dir(&PathBuf::from(&config.root_dir)) {
			| Ok(_) => &config.root_dir,
			| Err(e) => return Err(e),
		},
	});
	if let Err(_) = check_subdirctory(&root_dir, "archive") {
		if let Err(e) = create_archive_dir(&root_dir) {
			return Err(e);
		}
	}
	if let Err(_) = check_subdirctory(&root_dir, ".cache") {
		if let Err(e) = create_cache_dir(&root_dir) {
			return Err(e);
		}
	}
	if let Err(_) = check_subdirctory(&root_dir, "stockpile") {
		if let Err(e) = create_stockpile_dir(&root_dir) {
			return Err(e);
		}
	}
	Ok(())
}
