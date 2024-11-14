pub mod book;
pub mod doc;
pub mod test;

use std::{error::Error, fs, io};

use yaml_rust::YamlLoader;

use crate::definitions::DocType;

pub fn read(path: &String) -> Result<DocType, Box<dyn Error>> {
	let metadata = match fs::read_to_string(&path) {
		| Ok(o) => o,
		| Err(_) => {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::NotFound,
				format!("Metadata not found or unavalible"),
			)))
		}
	};
	let yaml = match YamlLoader::load_from_str(metadata.as_str()) {
		| Ok(o) => o.first().unwrap().clone(),
		| Err(e) => return Err(Box::new(e)),
	};
	match yaml["type"].as_str() {
		| Some("book") => match book::read(&yaml) {
			| Ok(o) => Ok(DocType::Book(o)),
			| Err(e) => Err(e),
		},
		| Some("test") => match test::read(&yaml) {
			| Ok(o) => Ok(DocType::Test(o)),
			| Err(e) => Err(e),
		},
		| Some("doc") => match doc::read(&yaml) {
			| Ok(o) => Ok(DocType::Doc(o)),
			| Err(e) => Err(e),
		},
		| Some(other) => Err(Box::new(io::Error::new(
			io::ErrorKind::Other,
			format!("Unknown file type {} ", other),
		))),
		| None => Err(Box::new(io::Error::new(
			io::ErrorKind::NotFound,
			format!("File type not found in metadata!"),
		))),
	}
}
