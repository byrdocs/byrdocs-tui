use std::error::Error;

use config::getconf::get_config;

fn main() -> Result<(), Box<dyn Error>> {
	let config = match get_config() {
		| Ok(conf) => conf,
		| Err(e) => return Err(e),
	};
	match init::check(config) {
		| Ok(_) => Ok(()),
		| Err(e) => Err(e),
	}
}
