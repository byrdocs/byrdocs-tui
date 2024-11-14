use std::{error::Error, fs::File, io, path::PathBuf};

use crate::{definitions::pretype::*, md5sum::md5sum};
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Doc {
	md5: u128,
	title: String,
	course: Vec<Course>,
	filetype: FileType,
	content: Vec<DocContentType>,
}

impl Doc {
	pub fn new() -> Doc {
		Doc {
			md5: 0,
			title: String::new(),
			course: Vec::<Course>::new(),
			filetype: FileType::Pdf,
			content: Vec::<DocContentType>::new(),
		}
	}
	pub fn from(md5: u128, filetype: FileType) -> Doc {
		Doc {
			md5,
			title: String::new(),
			course: Vec::<Course>::new(),
			filetype,
			content: Vec::<DocContentType>::new(),
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
	pub fn get_course(&mut self, course: &Vec<Course>) -> Result<(), Box<dyn Error>> {
		Ok(self.course = course.clone())
	}
	pub fn get_content(&mut self, content: &Vec<DocContentType>) -> Result<(), Box<dyn Error>> {
		Ok(self.content = content.clone())
	}
	pub fn md5(&self) -> u128 {
		self.md5
	}
	pub fn title(&self) -> String {
		self.title.clone()
	}
	pub fn course(&self) -> Vec<Course> {
		self.course.clone()
	}
	pub fn filetype(&self) -> FileType {
		self.filetype.clone()
	}
	pub fn content(&self) -> Vec<DocContentType> {
		self.content.clone()
	}
}
