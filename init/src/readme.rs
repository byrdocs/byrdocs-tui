use std::{
	error::Error,
	fs::File,
	io::{self, Write},
	path::PathBuf,
};

use crypto::{digest::Digest, md5::Md5};
use metadata::md5sum::md5sum;

const README: &str = include_str!("../../docs/workspace-readme.md");

pub fn check_readme(readme_path: &PathBuf) -> Result<(), Box<dyn Error>> {
	let mut expected = Md5::new();
	expected.input(README.as_bytes());
	let expected = expected.result_str().parse::<u128>()?;
	let file = File::open(readme_path)?;
	let md5 = md5sum(&file)?;
	if md5 == expected {
		Ok(())
	} else {
		Err(Box::new(io::Error::new(io::ErrorKind::Other, "")))
	}
}

pub fn gen_readme(readme_path: &PathBuf) -> Result<(), Box<dyn Error>> {
	let mut file = File::create(readme_path)?;
	file.write_all(README.as_bytes())?;
	Ok(())
}
