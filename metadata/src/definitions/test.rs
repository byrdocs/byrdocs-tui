use std::{error::Error, fs::File, io, path::PathBuf};

use crate::{definitions::pretype::*, md5sum::md5sum};
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Test {
	md5: u128,
	title: String,
	college: Option<Vec<String>>,
	course: Option<Course>,
	time: Time,
	filetype: FileType,
	content: Vec<TestContentType>,
}

impl Test {
	pub fn new() -> Test {
		Test {
			md5: 0,
			title: String::new(),
			college: None,
			course: None,
			time: Time::new(),
			filetype: FileType::Pdf,
			content: Vec::<TestContentType>::new(),
		}
	}
	pub fn from(md5: u128, filetype: FileType) -> Test {
		Test {
			md5,
			title: String::new(),
			college: None,
			course: None,
			time: Time::new(),
			filetype,
			content: Vec::<TestContentType>::new(),
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
	pub fn get_college(&mut self, college: &Vec<String>) -> Result<(), Box<dyn Error>> {
		Ok(self.college = Some(college.clone()))
	}
	pub fn get_course(&mut self, course: &Option<Course>) -> Result<(), Box<dyn Error>> {
		Ok(self.course = course.clone())
	}
	pub fn get_time(&mut self, time: &Time) -> Result<(), Box<dyn Error>> {
		Ok(self.time = time.clone())
	}
	pub fn get_content(&mut self, content: &Vec<TestContentType>) -> Result<(), Box<dyn Error>> {
		Ok(self.content = content.clone())
	}
	pub fn md5(&self) -> u128 {
		self.md5
	}
	pub fn title(&self) -> String {
		self.title.clone()
	}
	pub fn college(&self) -> Option<Vec<String>> {
		self.college.clone()
	}
	pub fn course(&self) -> Option<Course> {
		self.course.clone()
	}
	pub fn time(&self) -> Time {
		self.time.clone()
	}
	pub fn filetype(&self) -> FileType {
		self.filetype.clone()
	}
	pub fn content(&self) -> Vec<TestContentType> {
		self.content.clone()
	}
}
