use std::error::Error;
use std::fs;

use yaml_rust::YamlLoader;

use crate::definition::Config;

pub fn get_config() -> Result<Config, Box<dyn Error>> {
	let conf_path = shellexpand::tilde("~/.config/byrdocs/byrconf.yml").into_owned();
	let configs = match fs::read_to_string(conf_path) {
		| Ok(o) => o,
		| Err(e) => {
			return Err(Box::new(e));
		}
	};
	match YamlLoader::load_from_str(configs.as_str()) {
		| Err(e) => Err(Box::new(e)),
		| Ok(yaml) => {
			let yaml = yaml.first().unwrap();
			Ok(Config {
				root_dir: yaml["root-dir"].as_str().unwrap().to_string(),
				archive_dir: yaml["archive-dir"].as_str().unwrap().to_string(),
				cache_dir: yaml["cache-dir"].as_str().unwrap().to_string(),
				stockpile_dir: yaml["stockpile-dir"].as_str().unwrap().to_string(),
			})
		}
	}
}
