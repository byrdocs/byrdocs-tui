use std::error::Error;

use yaml_rust::Yaml;

use crate::definitions::doc::Doc;

pub fn read(yaml: &Yaml) -> Result<Doc, Box<dyn Error>> {
	let mut doc = Doc::new();
	Ok(doc)
}
