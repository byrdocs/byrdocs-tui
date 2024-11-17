use std::{error::Error, io};

use yaml_rust::Yaml;

use crate::definitions::{
	pretype::{Course, CourseType, FileType, SemesterType, StageType, TestContentType, Time},
	test::Test,
};

pub fn read(yaml: &Yaml) -> Result<Test, Box<dyn Error>> {
	let md5sum = match u128::from_str_radix(yaml["id"].as_str().unwrap(), 16) {
		| Ok(u) => u,
		| Err(e) => return Err(Box::new(e)),
	};
	let mut test = Test::from(
		md5sum,
		match yaml["data"]["filetype"].as_str() {
			| Some("pdf") => FileType::Pdf,
			| Some(ext) => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::Other,
					format!("Unsupported file extension: {} for test {}", ext, md5sum),
				)))
			}
			| None => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::NotFound,
					format!("File extension not found for {}", md5sum),
				)))
			}
		},
	);
	if let Some(title) = yaml["data"]["title"].as_str() {
		test.get_title(&String::from(title))?
	} else {
		return Err(Box::new(io::Error::new(
			io::ErrorKind::InvalidData,
			format!("Invalid title of {}", test.md5()),
		)));
	};
	if let Some(college) = yaml["data"]["college"].as_vec() {
		if college.len() > 0 {
			let mut vec = Vec::<String>::new();
			for c in college {
				vec.push(c.as_str().unwrap().to_string());
			}
			test.get_college(&vec)?
		}
	}
	if let Some(course) = yaml["data"]["course"].as_hash() {
		let r#type = match course[&Yaml::from_str("type")].as_str() {
			| Some("本科") => Some(CourseType::本科),
			| Some("研究生") => Some(CourseType::研究生),
			| None => None,
			| Some(_) => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::InvalidData,
					format!("Invalid type of course of {}", test.md5()),
				)));
			}
		};
		let name = match course[&Yaml::from_str("name")].as_str() {
			| Some(n) => n.to_string(),
			| None => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::InvalidData,
					format!("Name of course of {} must not be empty", test.md5()),
				)))
			}
		};
		test.get_course(&Course::from(name, r#type))?
	}
	if let Some(time) = yaml["data"]["time"].as_hash() {
		let start = match time[&Yaml::from_str("start")].as_str() {
			| Some(s) => s.to_string(),
			| None => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::InvalidData,
					format!("Start time of {} is invalid", test.md5()),
				)))
			}
		};
		let end = match time[&Yaml::from_str("end")].as_str() {
			| Some(e) => e.to_string(),
			| None => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::InvalidData,
					format!("End time of {} is invalid", test.md5()),
				)))
			}
		};
		let semester = match time[&Yaml::from_str("semester")].as_str() {
			| Some("First") => Some(SemesterType::First),
			| Some("Second") => Some(SemesterType::Second),
			| Some(_) => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::InvalidData,
					format!("Semester time of {} is invalid", test.md5()),
				)))
			}
			| None => None,
		};
		let stage = match time[&Yaml::from_str("stage")].as_str() {
			| Some("期中") => Some(StageType::期中),
			| Some("期末") => Some(StageType::期末),
			| Some(_) => {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::InvalidData,
					format!("Stage time of {} is invalid", test.md5()),
				)))
			}
			| None => None,
		};
		test.get_time(&Time::from(start, end, semester, stage))?
	} else {
		return Err(Box::new(io::Error::new(
			io::ErrorKind::InvalidData,
			format!("Invalid time of {}", test.md5()),
		)));
	}
	if let Some(content) = yaml["data"]["content"].as_vec() {
		if content.len() > 0 {
			let mut vec = Vec::<TestContentType>::new();
			for c in content {
				match c.as_str() {
					| Some("原题") => vec.push(TestContentType::原题),
					| Some("答案") => vec.push(TestContentType::答案),
					| Some(_) | None => {
						return Err(Box::new(io::Error::new(
							io::ErrorKind::InvalidData,
							format!("Invalid content type of {}", test.md5()),
						)))
					}
				}
			}
			test.get_content(&vec)?
		} else {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("Content of test {} must be not empty", test.md5()),
			)));
		}
	} else {
		return Err(Box::new(io::Error::new(
			io::ErrorKind::InvalidData,
			format!("Content of test {} must be not empty", test.md5()),
		)));
	}
	Ok(test)
}
