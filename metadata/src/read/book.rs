use std::{error::Error, io};

use yaml_rust::Yaml;

use crate::definitions::{book::Book, pretype::FileType};

pub fn read(yaml: &Yaml) -> Result<Book, Box<dyn Error>> {
	let mut book = Book::from(
		u128::from_str_radix(yaml["id"].as_str().unwrap(), 16)?,
		match yaml["data"]["filetype"].as_str() {
			| Some("pdf") => FileType::Pdf,
			| Some("zip") => FileType::Zip,
			| Some(ext) => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::Other,
					format!("Unsupported file extension: {}", ext),
				)))
			}
			| None => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::NotFound,
					format!("File extension not found"),
				)))
			}
		},
	);
	let _ = match yaml["data"]["title"].as_str() {
		| Some(title) => book.get_title(&String::from(title))?,
		| None => {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("Invalid title of {}", book.md5()),
			)))
		}
	};
	let _ = match yaml["data"]["authors"].as_vec() {
		| Some(authors) => {
			if authors.len() > 0 {
				let mut vec = Vec::<String>::new();
				for a in authors {
					vec.push(String::from(a.as_str().unwrap()));
				}
				book.get_authors(&vec)?
			} else {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::InvalidData,
					format!("Authors of {} must be not empty", book.md5()),
				)));
			}
		}
		| None => {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("Invalid authors of {}", book.md5()),
			)))
		}
	};
	if let Some(translators) = yaml["data"]["translators"].as_vec() {
		if translators.len() > 0 {
			let mut vec = Vec::<String>::new();
			for t in translators {
				vec.push(String::from(t.as_str().unwrap()));
			}
			book.get_translators(&vec)?
		}
	}
	if let Some(edition) = yaml["data"]["edition"].as_str() {
		book.get_edition(&String::from(edition))?
	}
	if let Some(publisher) = yaml["data"]["publisher"].as_str() {
		book.get_publisher(&String::from(publisher))?
	}
	if let Some(publish_year) = yaml["data"]["publish_year"].as_str() {
		book.get_publish_year(&String::from(publish_year))?
	}
	if let Some(isbn) = yaml["data"]["isbn"].as_vec() {
		if isbn.len() > 0 {
			let mut vec = Vec::<String>::new();
			for i in isbn {
				vec.push(String::from(i.as_str().unwrap()));
			}
			book.get_isbn(&vec)?
		}
	}
	Ok(book)
}
