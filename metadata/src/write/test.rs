use std::error::Error;

use macros::formatln;

use crate::definitions::{
	pretype::{CourseType, FileType, SemesterType, StageType, TestContentType},
	test::Test,
};

pub fn write(test: Test) -> Result<String, Box<dyn Error>> {
	let mut content = String::new();
	content.push_str(format!("id: {:032x}\n", test.md5()).as_str());
	content.push_str(
		formatln!(
			"url: https://byrdocs.org/files/{:032x}.{}",
			test.md5(),
			match test.filetype() {
				| FileType::Pdf => "pdf",
				| FileType::Zip => "zip",
			}
		)
		.as_str(),
	);
	content.push_str(formatln!("type: test").as_str());
	content.push_str(formatln!("data:").as_str());
	content.push_str(formatln!("  title: {}", test.title()).as_str());
	if let Some(college) = test.college() {
		content.push_str(formatln!("  college:").as_str());
		for c in college {
			content.push_str(formatln!("    - {}", c).as_str());
		}
	} else {
		content.push_str(formatln!("  college: null").as_str());
	}
	if let Some(course) = test.course() {
		content.push_str(formatln!("  course:").as_str());
		content.push_str(
			formatln!(
				"    type: {}",
				match course.r#type {
					| Some(CourseType::本科) => "本科",
					| Some(CourseType::研究生) => "研究生",
					| None => "null",
				}
			)
			.as_str(),
		);
		content.push_str(formatln!("    name: {}", course.name).as_str());
	} else {
		content.push_str(formatln!("  course: null").as_str());
	}
	content.push_str(formatln!("  time:").as_str());
	content.push_str(formatln!("    start: '{}'", test.time().start).as_str());
	content.push_str(formatln!("    end: '{}'", test.time().end).as_str());
	content.push_str(
		formatln!(
			"    semester: {}",
			match test.time().semester {
				| Some(SemesterType::First) => "First",
				| Some(SemesterType::Second) => "Second",
				| None => "null",
			}
		)
		.as_str(),
	);
	content.push_str(
		formatln!(
			"    stage: {}",
			match test.time().stage {
				| Some(StageType::期中) => "期中",
				| Some(StageType::期末) => "期末",
				| None => "null",
			}
		)
		.as_str(),
	);
	content.push_str(
		formatln!(
			"  filetype: {}",
			match test.filetype() {
				| FileType::Pdf => "pdf",
				| FileType::Zip => "zip",
			}
		)
		.as_str(),
	);
	content.push_str(formatln!("  content:").as_str());
	for c in test.content() {
		content.push_str(
			formatln!(
				"    - {}",
				match c {
					| TestContentType::原题 => "原题",
					| TestContentType::答案 => "答案",
				}
			)
			.as_str(),
		);
	}
	Ok(content)
}
