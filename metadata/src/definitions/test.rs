use std::{error::Error, fs::File, io, path::PathBuf};

use crate::{definitions::pretype::*, md5sum::md5sum};
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Test {
	md5: u128,
	title: String,
	college: Option<Vec<String>>,
	course: CourseType,
	time: Time,
	filetype: FileType,
	content: Vec<TestContentType>,
}

impl Test {
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
	fn get_college(&mut self, college: &Option<Vec<String>>) -> Result<(), Box<dyn Error>> {
		Ok(self.college = college.clone())
	}
	fn get_course(&mut self, course: &CourseType) -> Result<(), Box<dyn Error>> {
		Ok(self.course = course.clone())
	}
	fn get_time(&mut self, time: &Time) -> Result<(), Box<dyn Error>> {
		Ok(self.time = time.clone())
	}
	fn get_content(&mut self, content: &Vec<TestContentType>) -> Result<(), Box<dyn Error>> {
		Ok(self.content = content.clone())
	}
	fn md5(&self) -> u128 {
		self.md5
	}
	fn title(&self) -> String {
		self.title.clone()
	}
	fn college(&self) -> Option<Vec<String>> {
		self.college.clone()
	}
	fn course(&self) -> CourseType {
		self.course.clone()
	}
	fn time(&self) -> Time {
		self.time.clone()
	}
	fn filetype(&self) -> FileType {
		self.filetype.clone()
	}
	fn content(&self) -> Vec<TestContentType> {
		self.content.clone()
	}
}
