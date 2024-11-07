use std::{error::Error, fs::File, io::Write, path::PathBuf};

pub fn gen_readme(readme_path: &PathBuf) -> Result<(), Box<dyn Error>> {
	let readme: &str = include_str!("../../docs/workspace-readme.md");
	let mut file = File::create(readme_path)?;
	file.write_all(readme.as_bytes())?;
	Ok(())
}
