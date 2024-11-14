use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// create_conf() create or recreate the configurations.
/// Settings may be overwritten. Use it cautiously!
pub fn create_conf(root_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
	let conf_folder = PathBuf::from(shellexpand::tilde("~/.config/byrdocs").into_owned());
	if !conf_folder.exists() {
		if let Err(_) = fs::create_dir_all(conf_folder) {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::Other,
				format!("Uable to create conf_folder ~/.config/byrdocs",),
			)));
		}
	}
	let conf_path = PathBuf::from(shellexpand::tilde("~/.config/byrdocs/byrconf.yml").into_owned());
	let mut conf = match fs::File::create(&conf_path) {
		| Ok(o) => o,
		| Err(_) => {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::Other,
				format!("Failed to create conf: ~/.config/byrdocs/byrconf.yml"),
			)))
		}
	};
	writeln!(conf, "root-dir: {}", root_dir.to_str().unwrap().trim())?;
	writeln!(
		conf,
		"archive-dir: {}/archive",
		root_dir.to_str().unwrap().trim()
	)?;
	writeln!(
		conf,
		"cache-dir: {}/.cache",
		root_dir.to_str().unwrap().trim()
	)?;
	writeln!(
		conf,
		"stockpile-dir: {}/stockpile",
		root_dir.to_str().unwrap().trim()
	)?;
	Ok(conf_path)
}
