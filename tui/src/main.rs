use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let _config = match init::check() {
		| Ok(conf) => conf,
		| Err(e) => return Err(e),
	};
	Ok(())
}

#[cfg(test)]
mod tests {
	use std::fs::File;

	use config::{definition::Config, getconf::get_config};
	use metadata::{
		definitions::pretype::FileType,
		md5sum::md5sum,
		read::{book, doc, test},
	};
	use yaml_rust::YamlLoader;

	#[test]
	fn config() {
		let config = get_config().unwrap();
		let expectation = Config {
			root_dir: String::from(shellexpand::tilde("~/BYRDOCS")),
			archive_dir: String::from(shellexpand::tilde("~/BYRDOCS/archive")),
			cache_dir: String::from(shellexpand::tilde("~/BYRDOCS/.cache")),
			stockpile_dir: String::from(shellexpand::tilde("~/BYRDOCS/stockpile")),
		};
		assert_eq!(config, expectation);
	}
	#[test]
	fn md5() {
		let file =
			File::open(shellexpand::tilde("~/.config/byrdocs/byrconf.yml").into_owned()).unwrap();
		let md5 = md5sum(&file).unwrap();
		let expectation: u128 = 0x2b181915816194cb3cb82ae809f89dba;
		assert_eq!(md5, expectation);
	}
	#[test]
	fn metadata_book() {
		let book = match book::read(
			&YamlLoader::load_from_str(include_str!(
				"../../.assets/06fa27d50ba4a4baae1b665ea48fd677.yml",
			))
			.unwrap()
			.first()
			.unwrap(),
		) {
			| Ok(book) => {
				assert_eq!(book.md5(), 0x06fa27d50ba4a4baae1b665ea48fd677);
				assert_eq!(book.title(), "理论物理学教程 第10卷 物理动理学 第2版");
				assert_eq!(book.authors().len(), 2);
				assert_eq!(book.authors()[0], "Лифшиц, Евгений Михайлович(粟弗席兹)");
				assert_eq!(book.authors()[1], "Питаевский, Лев Петрович(皮塔耶夫斯基)");
				if let Some(translators) = book.translators() {
					assert_eq!(translators.len(), 3);
					assert_eq!(translators[0], "徐锡申");
					assert_eq!(translators[1], "徐春华");
					assert_eq!(translators[2], "黄京民");
				} else {
					assert!(false, "translators of {} shold not be empty", book.md5());
				}
				assert_eq!(book.edition(), Some("1".to_string()));
				assert_eq!(book.publisher(), Some("高等教育出版社".to_string()));
				assert_eq!(book.publish_year(), Some("2008".to_string()));
				if let Some(isbn) = book.isbn() {
					assert_eq!(isbn[0], "978-7-04-023069-7");
				}
				assert_eq!(book.filetype(), FileType::Pdf);
				book
			}
			| Err(e) => return assert!(false, "{}", e)
		};
		assert_eq!(
			metadata::write::book::write(book).unwrap(),"\
id: 06fa27d50ba4a4baae1b665ea48fd677
url: https://byrdocs.org/files/06fa27d50ba4a4baae1b665ea48fd677.pdf
type: book
data:
  title: 理论物理学教程 第10卷 物理动理学 第2版
  authors:
    - Лифшиц, Евгений Михайлович(粟弗席兹)
    - Питаевский, Лев Петрович(皮塔耶夫斯基)
  translators:
    - 徐锡申
    - 徐春华
    - 黄京民
  edition: '1'
  publisher: 高等教育出版社
  publish_year: '2008'
  isbn:
    - 978-7-04-023069-7
  filetype: pdf
"
		);
	}
	#[test]
	fn metadata_test() {
		let _ = match test::read(
			&YamlLoader::load_from_str(include_str!(
				"../../.assets/154a930358251fe7bf774dd194bd9a00.yml",
			))
			.unwrap()
			.first()
			.unwrap(),
		) {
			| Ok(test) => {
				assert_eq!(test.md5(), 0x154a930358251fe7bf774dd194bd9a00);
				assert_eq!(test.title(), "2023-2024 第二学期 概率论与数理统计 期末");
				if let Some(college) = test.college() {
					assert_eq!(college.len(), 3);
					assert_eq!(college[0], "人工智能学院");
					assert_eq!(college[1], "自动化学院");
					assert_eq!(college[2], "信息与通信工程学院");
				} else {
					assert!(false, "colleges of {} should not be empty", test.md5());
				}
				assert_eq!(test.course().r#type, None);
				assert_eq!(test.course().name, "概率论与数理统计");
				assert_eq!(test.time().start, "2023");
				assert_eq!(test.time().end, "2024");
				assert_eq!(
					test.time().semester,
					Some(metadata::definitions::pretype::SemesterType::Second)
				);
				assert_eq!(
					test.time().stage,
					Some(metadata::definitions::pretype::StageType::期末)
				);
				assert_eq!(test.filetype(), FileType::Pdf);
				assert_eq!(test.content().len(), 1);
				assert_eq!(
					test.content()[0],
					metadata::definitions::pretype::TestContentType::原题
				)
			}
			| Err(e) => assert!(false, "{}", e),
		};
	}
	#[test]
	fn metadata_doc() {
		let _ = match doc::read(
			&YamlLoader::load_from_str(include_str!(
				"../../.assets/3d6fa4f7305fb91e9c014e468037310c.yml",
			))
			.unwrap()
			.first()
			.unwrap(),
		) {
			| Ok(doc) => {
				assert_eq!(doc.md5(), 0x3d6fa4f7305fb91e9c014e468037310c);
				assert_eq!(doc.course().len(), 1);
				assert_eq!(doc.course()[0].r#type, None);
				assert_eq!(doc.course()[0].name, "工程图学");
				assert_eq!(doc.content().len(), 2);
				assert_eq!(
					doc.content()[0],
					metadata::definitions::pretype::DocContentType::题库
				);
				assert_eq!(
					doc.content()[1],
					metadata::definitions::pretype::DocContentType::答案
				)
			}
			| Err(e) => assert!(false, "{}", e),
		};
	}
}
