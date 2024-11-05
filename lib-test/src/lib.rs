#[cfg(test)]
mod tests {
	use config::{definition::Config, getconf::get_config};

	#[test]
	fn config() {
		let config = get_config().unwrap();
		let expectation = Config {
			root_dir: String::from(shellexpand::tilde("~/BYRDOCS")),
			archive_dir: String::from(shellexpand::tilde("~/BYRDOCS/archive")),
			cache_dir: String::from(shellexpand::tilde("~/BYRDOCS/.cache")),
			stockpile_dir: String::from(shellexpand::tilde("~/BYRDOCS/stockpile")),
		};
		assert_eq!(config, expectation);
	}
}
