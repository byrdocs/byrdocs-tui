#[cfg(test)]
mod tests {
	use init::check::check_root_dir;
	use std::path::PathBuf;

	#[test]
	fn root_dir() {
		assert_eq!(
			PathBuf::from("/home/cpphusky/BYRDOCS"),
			check_root_dir().unwrap()
		);
	}
}
