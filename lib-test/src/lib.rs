#[cfg(test)]
mod tests {
	use std::fs::File;

	use config::{definition::Config, getconf::get_config};
	use metadata::md5sum::md5sum;

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
	#[test]
	fn md5() {
		let file =
			File::open(shellexpand::tilde("~/.config/byrdocs/byrconf.yml").into_owned()).unwrap();
		let md5 = md5sum(&file).unwrap();
		let expectation: u128 = 0x2b181915816194cb3cb82ae809f89dba;
		assert_eq!(md5, expectation);
	}
}
