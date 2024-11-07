use std::{error::Error, fs::File, io, path::PathBuf};

use crate::{definitions::pretype::*, md5sum::md5sum};
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Doc {
	md5: u128,
	title: String,
	course: Vec<CourseType>,
	filetype: FileType,
	content: Vec<DocContentType>,
}

impl Doc {
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
	fn get_course(&mut self, course: &Vec<CourseType>) -> Result<(), Box<dyn Error>> {
		Ok(self.course = course.clone())
	}
	fn get_content(&mut self, content: &Vec<DocContentType>) -> Result<(), Box<dyn Error>> {
		Ok(self.content = content.clone())
	}
}
