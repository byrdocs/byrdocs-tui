// Common Types required by Categories
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
	Pdf,
	Zip,
}

// Books
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Book {
	md5: u128,
	title: String,
	authors: Vec<String>,
	translators: Option<Vec<String>>,
	edition: Option<String>,
	publisher: Option<String>,
	publish_year: Option<String>,
	isbn: Option<String>,
	filetype: FileType,
}

// Tests
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CourseType {
	本科,
	研究生,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Course {
	r#type: Option<CourseType>,
	name: String,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemesterType {
	Fiest,
	Second,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StageType {
	期中,
	期末,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
	start: String,
	end: String,
	semester: Option<SemesterType>,
	stage: Option<StageType>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestContentType {
	原题,
	答案,
}
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Tests {
	md5: u128,
	title: String,
	college: Option<Vec<String>>,
	course: CourseType,
	time: Time,
	filetype: FileType,
	content: Vec<TestContentType>,
}

// Docs
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocContentType {
	思维导图,
	题库,
	答案,
	知识点,
	课件,
}
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Doc {
	md5: u128,
	title: String,
	course: Vec<CourseType>,
	filetype: FileType,
	content: Vec<DocContentType>,
}
