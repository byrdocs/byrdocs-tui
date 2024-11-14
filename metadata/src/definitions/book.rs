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
	pub fn new() -> Book {
		Book {
			md5: 0,
			title: String::new(),
			authors: Vec::new(),
			translators: None,
			edition: None,
			publisher: None,
			publish_year: None,
			isbn: None,
			filetype: FileType::Pdf,
		}
	}
	pub fn from(md5: u128, filetype: FileType) -> Book {
		Book {
			md5,
			title: String::new(),
			authors: Vec::new(),
			translators: None,
			edition: None,
			publisher: None,
			publish_year: None,
			isbn: None,
			filetype,
		}
	}
	pub fn get_file_info(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
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
	pub fn get_title(&mut self, title: &String) -> Result<(), Box<dyn Error>> {
		Ok(self.title = title.clone())
	}
	pub fn get_authors(&mut self, authors: &Vec<String>) -> Result<(), Box<dyn Error>> {
		Ok(self.authors = authors.clone())
	}
	pub fn get_translators(&mut self, translators: &Vec<String>) -> Result<(), Box<dyn Error>> {
		Ok(self.translators = Some(translators.clone()))
	}
	pub fn get_edition(&mut self, edition: &String) -> Result<(), Box<dyn Error>> {
		Ok(self.edition = Some(edition.clone()))
	}
	pub fn get_publisher(&mut self, publisher: &String) -> Result<(), Box<dyn Error>> {
		Ok(self.publisher = Some(publisher.clone()))
	}
	pub fn get_publish_year(&mut self, publish_year: &String) -> Result<(), Box<dyn Error>> {
		Ok(self.publish_year = Some(publish_year.clone()))
	}
	pub fn get_isbn(&mut self, isbn: &Vec<String>) -> Result<(), Box<dyn Error>> {
		Ok(self.isbn = Some(isbn.clone()))
	}
	pub fn md5(&self) -> u128 {
		self.md5
	}
	pub fn title(&self) -> String {
		self.title.clone()
	}
	pub fn authors(&self) -> Vec<String> {
		self.authors.clone()
	}
	pub fn translators(&self) -> Option<Vec<String>> {
		self.translators.clone()
	}
	pub fn edition(&self) -> Option<String> {
		self.edition.clone()
	}
	pub fn publisher(&self) -> Option<String> {
		self.publisher.clone()
	}
	pub fn publish_year(&self) -> Option<String> {
		self.publish_year.clone()
	}
	pub fn isbn(&self) -> Option<Vec<String>> {
		self.isbn.clone()
	}
	pub fn filetype(&self) -> FileType {
		self.filetype.clone()
	}
}
