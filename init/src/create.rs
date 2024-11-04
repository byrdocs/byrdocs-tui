use std::{
	error::Error,
	fs,
	io::{self, Write},
	path::PathBuf,
};

use config::getconf::get_config;
use git2::Repository;
use home::home_dir;

const ARCHIVE_URL: &str = "https://github.com/byrdocs/byrdocs-archive.git";
const SCRIPTS_UTL: &str = "https://github.com/byrdocs/byrdocs-scripts.git";

/// create_conf() create or recreate the configurations.
/// Settings may be overwritten. Use it cautiously!
pub fn create_conf(root_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
	if let Some(home) = home_dir() {
		let conf_path = home.join(".config/byrdocs/byrconf.yml");
		let mut conf = fs::File::create(&conf_path)?;
		writeln!(conf, "root-dir: {}", root_dir.to_str().unwrap())?;
		Ok(conf_path)
	} else {
		Err(Box::new(io::Error::new(
			io::ErrorKind::Other,
			String::from("Couldn't extract home_dir()"),
		)))
	}
}

/// If root-dir in the configurations not found, then create one.
/// If root-dir already exists, nothing will be created.
pub fn create_root_dir() -> Result<PathBuf, Box<dyn Error>> {
	let yaml = match get_config() {
		| Ok(o) => o,
		| Err(e) => return Err(e),
	};
	let yaml = yaml.first().unwrap();
	let root_dir = PathBuf::from(yaml["root-dir"].as_str().unwrap());
	if !root_dir.exists() {
		fs::create_dir(&root_dir)?;
	}
	Ok(root_dir)
}

pub fn create_archive_dir(root_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
	let archive_dir = root_dir.join("archive");
	println!("Cloning byrdocs-archive from github...");
	match Repository::clone(ARCHIVE_URL, &archive_dir) {
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

pub fn create_cache_dir(root_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
	let cache_dir = root_dir.join(".cache");
	fs::create_dir(&cache_dir)?;
	Ok(cache_dir)
}

pub fn create_stockpile_dir(root_dir:&PathBuf) -> Result<PathBuf, Box<dyn Error>> {
	let stockpile_dir = root_dir.join("stockpile");
	println!("Cloning byrdocs-scripts(stockpile) from github...");
	match Repository::clone(SCRIPTS_UTL,&stockpile_dir) {
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
