use std::{error::Error, fs::File, io, path::PathBuf};

use crate::{definitions::pretype::*, md5sum::md5sum};

// Books
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Book {
	md5: u128,
	title: String,
	authors: Vec<String>,
	translators: Option<Vec<String>>,
	edition: Option<String>,
	publisher: Option<String>,
	publish_year: Option<String>,
	isbn: Option<Vec<String>>,
	filetype: FileType,
}

impl Book {
	fn get_file_info(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
		self.filetype = match path.extension().unwrap().to_str() {
			| Some("pdf") => FileType::Pdf,
			| Some("zip") => FileType::Zip,
			| Some(ext) => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::Other,
					format!("Unsupported file type: {}", ext),
				)))
			}
			| None => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::Other,
					format!("Unsupported file type: (unknown)"),
				)))
			}
		};
		let file = File::open(&path)?;
		self.md5 = md5sum(&file)?;
		Ok(())
	}
	fn get_title(&mut self, title: &String) -> Result<(), Box<dyn Error>> {
		Ok(self.title = title.clone())
	}
	fn get_authors(&mut self, authors: &Vec<String>) -> Result<(), Box<dyn Error>> {
		Ok(self.authors = authors.clone())
	}
	fn get_translators(&mut self, translators: &Vec<String>) -> Result<(), Box<dyn Error>> {
		Ok(self.translators = Some(translators.clone()))
	}
	fn get_edition(&mut self, edition: &String) -> Result<(), Box<dyn Error>> {
		Ok(self.edition = Some(edition.clone()))
	}
	fn get_publisher(&mut self, publisher: &String) -> Result<(), Box<dyn Error>> {
		Ok(self.publisher = Some(publisher.clone()))
	}
	fn get_publish_year(&mut self, publish_year: &String) -> Result<(), Box<dyn Error>> {
		Ok(self.publish_year = Some(publish_year.clone()))
	}
	fn get_isbn(&mut self, isbn: &Vec<String>) -> Result<(), Box<dyn Error>> {
		Ok(self.isbn = Some(isbn.clone()))
	}
	fn md5(&self) -> u128 {
		self.md5
	}
	fn title(&self) -> String {
		self.title.clone()
	}
	fn authors(&self) -> Vec<String> {
		self.authors.clone()
	}
	fn translators(&self) -> Option<Vec<String>> {
		self.translators.clone()
	}
	fn edition(&self) -> Option<String> {
		self.edition.clone()
	}
	fn publisher(&self) -> Option<String> {
		self.publisher.clone()
	}
	fn publish_year(&self) -> Option<String> {
		self.publish_year.clone()
	}
	fn isbn(&self) -> Option<Vec<String>> {
		self.isbn.clone()
	}
	fn filetype(&self) -> FileType {
		self.filetype.clone()
	}
}
