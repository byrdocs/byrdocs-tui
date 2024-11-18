use std::error::Error;

use crate::definitions::{book::Book, pretype::FileType};

pub fn write(book: Book) -> Result<String, Box<dyn Error>> {
	let mut content = String::new();
	content.push_str(format!("id: {:032x}\n", book.md5()).as_str());
	content.push_str(
		format!(
			"url: https://byrdocs.org/files/{:032x}.{}\n",
			book.md5(),
			match book.filetype() {
				| FileType::Pdf => "pdf",
				| FileType::Zip => "zip",
			}
		)
		.as_str(),
	);
	content.push_str(format!("type: book\ndata:\n").as_str());
	content.push_str(format!("  title: {}\n", book.title()).as_str());
	content.push_str(format!("  authors:\n").as_str());
	for author in book.authors() {
		content.push_str(format!("    - {}\n", author).as_str());
	}
	if let Some(translators) = book.translators() {
		content.push_str(format!("  translators:\n").as_str());
		for translator in translators {
			content.push_str(format!("    - {}\n", translator).as_str());
		}
	} else {
		content.push_str(format!("  translators: null\n").as_str());
	}
	if let Some(edition) = book.edition() {
		content.push_str(format!("  edition: '{}'\n", edition).as_str());
	} else {
		content.push_str(format!("  edition: null\n").as_str());
	}
	if let Some(publisher) = book.publisher() {
		content.push_str(format!("  publisher: {}\n", publisher).as_str());
	} else {
		content.push_str(format!("  publisher: null\n").as_str());
	}
	if let Some(publish_year) = book.publish_year() {
		content.push_str(format!("  publish_year: '{}'\n", publish_year).as_str());
	} else {
		content.push_str(format!("  publish_year: null\n").as_str());
	}
	if let Some(isbn) = book.isbn() {
		content.push_str(format!("  isbn:\n").as_str());
		for i in isbn {
			content.push_str(format!("    - {}\n", i).as_str());
		}
	} else {
		content.push_str(format!("  isbn: null\n").as_str());
	}
	content.push_str(
		format!(
			"  filetype: {}\n",
			match book.filetype() {
				| FileType::Pdf => "pdf",
				| FileType::Zip => "zip",
			}
		)
		.as_str(),
	);
	Ok(content)
}
