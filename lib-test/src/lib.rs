#[cfg(test)]
mod tests {
	use filesystem::getconf::get_root_dir;
	use std::path::PathBuf;

	#[test]
	fn root_dir() {
		assert_eq!(
			PathBuf::from("/home/cpphusky/BYRDOCS"),
			get_root_dir().unwrap()
		);
	}
}
