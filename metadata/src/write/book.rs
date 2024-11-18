use std::error::Error;

use macros::formatln;

use crate::definitions::{book::Book, pretype::FileType};

pub fn write(book: Book) -> Result<String, Box<dyn Error>> {
	let mut content = String::new();
	content.push_str(formatln!("id: {:032x}", book.md5()).as_str());
	content.push_str(
		formatln!(
			"url: https://byrdocs.org/files/{:032x}.{}",
			book.md5(),
			match book.filetype() {
				| FileType::Pdf => "pdf",
				| FileType::Zip => "zip",
			}
		)
		.as_str(),
	);
	content.push_str(formatln!("type: book").as_str());
	content.push_str(formatln!("data:").as_str());
	content.push_str(formatln!("  title: {}", book.title()).as_str());
	content.push_str(formatln!("  authors:").as_str());
	for author in book.authors() {
		content.push_str(formatln!("    - {}", author).as_str());
	}
	if let Some(translators) = book.translators() {
		content.push_str(formatln!("  translators:").as_str());
		for translator in translators {
			content.push_str(formatln!("    - {}", translator).as_str());
		}
	} else {
		content.push_str(formatln!("  translators: null").as_str());
	}
	if let Some(edition) = book.edition() {
		content.push_str(formatln!("  edition: '{}'", edition).as_str());
	} else {
		content.push_str(formatln!("  edition: null").as_str());
	}
	if let Some(publisher) = book.publisher() {
		content.push_str(formatln!("  publisher: {}", publisher).as_str());
	} else {
		content.push_str(formatln!("  publisher: null").as_str());
	}
	if let Some(publish_year) = book.publish_year() {
		content.push_str(formatln!("  publish_year: '{}'", publish_year).as_str());
	} else {
		content.push_str(formatln!("  publish_year: null").as_str());
	}
	if let Some(isbn) = book.isbn() {
		content.push_str(formatln!("  isbn:").as_str());
		for i in isbn {
			content.push_str(formatln!("    - {}", i).as_str());
		}
	} else {
		content.push_str(formatln!("  isbn: null").as_str());
	}
	content.push_str(
		formatln!(
			"  filetype: {}",
			match book.filetype() {
				| FileType::Pdf => "pdf",
				| FileType::Zip => "zip",
			}
		)
		.as_str(),
	);
	Ok(content)
}
