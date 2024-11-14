use std::error::Error;

use yaml_rust::Yaml;

use crate::definitions::test::Test;

pub fn read(yaml: &Yaml) -> Result<Test, Box<dyn Error>> {
	let mut test = Test::new();
	Ok(test)
}
