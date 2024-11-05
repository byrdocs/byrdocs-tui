use std::error::Error;
use std::io;
use std::path::PathBuf;

pub fn check_dir(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
	if dir.exists() {
		Ok(())
	} else {
		Err(Box::new(io::Error::new(
			io::ErrorKind::NotFound,
			format!("{} doesn't exist.", dir.to_str().unwrap()),
		)))
	}
}
