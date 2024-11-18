use std::error::Error;

use macros::formatln;

use crate::definitions::{
	doc::Doc,
	pretype::{CourseType, DocContentType, FileType},
};

pub fn write(doc: Doc) -> Result<String, Box<dyn Error>> {
	let mut yaml = String::new();
	yaml.push_str(formatln!("id: {:032x}", doc.md5()).as_str());
	yaml.push_str(
		formatln!(
			"url: https://byrdocs.org/files/{:032x}.{}",
			doc.md5(),
			match doc.filetype() {
				| FileType::Pdf => "pdf",
				| FileType::Zip => "zip",
			}
		)
		.as_str(),
	);
	yaml.push_str(formatln!("type: doc").as_str());
	yaml.push_str(formatln!("data:").as_str());
	yaml.push_str(formatln!("  title: {}", doc.title()).as_str());
	yaml.push_str(
		formatln!(
			"  filetype: {}",
			match doc.filetype() {
				| FileType::Pdf => "pdf",
				| FileType::Zip => "zip",
			}
		)
		.as_str(),
	);
	yaml.push_str(formatln!("  course:").as_str());
	for course in doc.course() {
		yaml.push_str(
			formatln!(
				"    - type: {}",
				match course.r#type {
					| Some(CourseType::本科) => "本科",
					| Some(CourseType::研究生) => "研究生",
					| None => "null",
				}
			)
			.as_str(),
		);
		yaml.push_str(formatln!("      name: {}", course.name).as_str());
	}
	yaml.push_str(formatln!("  content:").as_str());
	for content in doc.content() {
		yaml.push_str(
			formatln!(
				"    - {}",
				match content {
					| DocContentType::思维导图 => "思维导图",
					| DocContentType::知识点 => "知识点",
					| DocContentType::答案 => "答案",
					| DocContentType::课件 => "课件",
					| DocContentType::题库 => "题库",
				}
			)
			.as_str(),
		);
	}
	Ok(yaml)
}
