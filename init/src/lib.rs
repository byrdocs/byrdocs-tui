use std::{
	error::Error,
	io,
	path::PathBuf,
};

use check::{check_dir, check_file};
use config::{definition::Config, getconf::get_config};
use create::{
	create_archive_dir, create_cache_dir, create_conf, create_root_dir, create_stockpile_dir,
};
use readme::gen_readme;

pub mod check;
pub mod create;
pub mod readme;

pub fn check() -> Result<Config, Box<dyn Error>> {
	let config = match get_config() {
		| Ok(conf) => conf,
		| Err(_) => {
			println!("Warning: It seems that There is no BYRDOCS workspace in your computer, or the configuration file is deleted.");
			println!("Input a directory so that we can create the workspace: ");
			let mut input = String::new();
			io::stdin()
				.read_line(&mut input)
				.expect("Failed to read the path");
			match create_conf(PathBuf::from(shellexpand::tilde(&input).into_owned())) {
				| Ok(_) => get_config().unwrap(),
				| Err(e) => return Err(e),
			}
		}
	};
	if let Err(_) = check_dir(&PathBuf::from(&config.root_dir)) {
		if let Err(e) = create_root_dir(&PathBuf::from(&config.root_dir)) {
			return Err(e);
		};
	};
	if let Err(_) = check_file(&PathBuf::from(&config.root_dir).join("README.md")) {
		if let Err(e) = gen_readme(&PathBuf::from(&config.root_dir).join("README.md")) {
			return Err(e);
		}
	}
	if let Err(_) = check_dir(&PathBuf::from(&config.archive_dir)) {
		if let Err(e) = create_archive_dir(&PathBuf::from(&config.archive_dir)) {
			return Err(e);
		}
	}
	if let Err(_) = check_dir(&PathBuf::from(&config.cache_dir)) {
		if let Err(e) = create_cache_dir(&PathBuf::from(&config.cache_dir)) {
			return Err(e);
		}
	}
	if let Err(_) = check_dir(&PathBuf::from(&config.stockpile_dir)) {
		if let Err(e) = create_stockpile_dir(&PathBuf::from(&config.stockpile_dir)) {
			return Err(e);
		}
	}
	Ok(config)
}
