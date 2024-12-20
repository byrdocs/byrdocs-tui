use std::{error::Error, io};

use yaml_rust::Yaml;

use crate::definitions::{
	doc::Doc,
	pretype::{Course, CourseType, DocContentType, FileType},
};

pub fn read(yaml: &Yaml) -> Result<Doc, Box<dyn Error>> {
	let md5sum = match u128::from_str_radix(yaml["id"].as_str().unwrap(), 16) {
		| Ok(u) => u,
		| Err(e) => return Err(Box::new(e)),
	};
	let mut doc = Doc::from(
		md5sum,
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
	if let Some(title) = yaml["data"]["title"].as_str() {
		doc.get_title(&title.to_string())?
	} else {
		return Err(Box::new(io::Error::new(
			io::ErrorKind::InvalidData,
			format!("Title of {} must be not empty", doc.md5()),
		)));
	}
	if let Some(course) = yaml["data"]["course"].as_vec() {
		if course.len() > 0 {
			let mut vec = Vec::<Course>::new();
			for c in course {
				let r#type = match c["type"].as_str() {
					| Some("本科") => Some(CourseType::本科),
					| Some("研究生") => Some(CourseType::研究生),
					| None => None,
					| Some(_) => {
						return Err(Box::new(io::Error::new(
							io::ErrorKind::InvalidData,
							format!("Invalid type of course of {}", doc.md5()),
						)));
					}
				};
				let name = match c["name"].as_str() {
					| Some(n) => n.to_string(),
					| None => {
						return Err(Box::new(io::Error::new(
							io::ErrorKind::InvalidData,
							format!("Name of course of {} must not be empty", doc.md5()),
						)))
					}
				};
				vec.push(Course::from(name, r#type));
			}
			doc.get_course(&vec)?
		} else {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("Course of doc {} must be not empty", doc.md5()),
			)));
		}
	} else {
		return Err(Box::new(io::Error::new(
			io::ErrorKind::InvalidData,
			format!("Course of doc {} must be not empty", doc.md5()),
		)));
	}
	if let Some(content) = yaml["data"]["content"].as_vec() {
		if content.len() > 0 {
			let mut vec = Vec::<DocContentType>::new();
			for c in content {
				match c.as_str() {
					| Some("思维导图") => vec.push(DocContentType::思维导图),
					| Some("题库") => vec.push(DocContentType::题库),
					| Some("答案") => vec.push(DocContentType::答案),
					| Some("知识点") => vec.push(DocContentType::知识点),
					| Some("课件") => vec.push(DocContentType::课件),
					| Some(_) | None => {
						return Err(Box::new(io::Error::new(
							io::ErrorKind::InvalidData,
							format!("Invalid content type of {}", doc.md5()),
						)))
					}
				}
			}
			doc.get_content(&vec)?
		} else {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("Content of doc {} must be not empty", doc.md5()),
			)));
		}
	} else {
		return Err(Box::new(io::Error::new(
			io::ErrorKind::InvalidData,
			format!("Content of doc {} must be not empty", doc.md5()),
		)));
	}
	Ok(doc)
}
