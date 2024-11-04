use std::error::Error;
use std::{fs, io};

use home::home_dir;
use yaml_rust::{Yaml, YamlLoader};

pub fn get_config() -> Result<Vec<Yaml>, Box<dyn Error>> {
	if let Some(home) = home_dir() {
		let conf_path = home.join(".config/byrdocs/byrconf.yml");
		let configs = match fs::read_to_string(conf_path) {
			| Ok(o) => o,
			| Err(e) => {
				return Err(Box::new(e));
			}
		};
		match YamlLoader::load_from_str(configs.as_str()) {
			| Ok(o) => Ok(o),
			| Err(e) => Err(Box::new(e)),
		}
	} else {
		Err(Box::new(io::Error::new(
			io::ErrorKind::Other,
			String::from("Couldn't extract home_dir()"),
		)))
	}
}
