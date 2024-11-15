use std::{error::Error, io};

use yaml_rust::Yaml;

use crate::definitions::{
	pretype::{Course, CourseType, FileType, SemesterType, StageType, TestContentType, Time},
	test::Test,
};

pub fn read(yaml: &Yaml) -> Result<Test, Box<dyn Error>> {
	let mut test = Test::from(
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
		| Some(title) => test.get_title(&String::from(title))?,
		| None => {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("Invalid title of {}", test.md5()),
			)))
		}
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
	let _ = match yaml["data"]["course"].as_hash() {
		| Some(course) => {
			let name = match course[&Yaml::from_str("name")].as_str() {
				| Some(n) => n.to_string(),
				| None => {
					return Err(Box::new(io::Error::new(
						io::ErrorKind::InvalidData,
						format!("Invalid course name of {}", test.md5()),
					)))
				}
			};
			let _ = match course[&Yaml::from_str("type")].as_str() {
				| Some("本科") => test.get_course(&Course::from(name, CourseType::本科)),
				| Some("研究生") => test.get_course(&Course::from(name, CourseType::研究生)),
				| Some(_) => {
					return Err(Box::new(io::Error::new(
						io::ErrorKind::InvalidData,
						format!("Invalid course type of {}", test.md5()),
					)))
				}
				| None => test.get_course(&Course::from(name, None)),
			};
		}
		| None => {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("Invalid course of {}", test.md5()),
			)))
		}
	};
	let _ = match yaml["data"]["time"].as_hash() {
		| Some(time) => {
			let start = match time[&Yaml::from_str("start")].as_str() {
				| Some(s) => s.to_string(),
				| None => {
					return Err(Box::new(io::Error::new(
						io::ErrorKind::InvalidData,
						format!("Invalid start time of {}", test.md5()),
					)))
				}
			};
			let end = match time[&Yaml::from_str("end")].as_str() {
				| Some(e) => e.to_string(),
				| None => {
					return Err(Box::new(io::Error::new(
						io::ErrorKind::InvalidData,
						format!("Invalid end time of {}", test.md5()),
					)))
				}
			};
			let semester = match time[&Yaml::from_str("semester")].as_str() {
				| Some("First") => Some(SemesterType::First),
				| Some("Second") => Some(SemesterType::Second),
				| Some(_) => {
					return Err(Box::new(io::Error::new(
						io::ErrorKind::InvalidData,
						format!("Invalid semester time of {}", test.md5()),
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
						format!("Invalid stage time of {}", test.md5()),
					)))
				}
				| None => None,
			};
			test.get_time(&Time::from(start, end, semester, stage))
		}
		| None => {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("Invalid time of {}", test.md5()),
			)));
		}
	};
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
				format!("Invalid content of {}", test.md5()),
			)));
		}
	}
	Ok(test)
}
