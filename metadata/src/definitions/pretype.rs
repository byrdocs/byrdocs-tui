#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
	Pdf,
	Zip,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CourseType {
	本科,
	研究生,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Course {
	pub r#type: Option<CourseType>,
	pub name: String,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemesterType {
	First,
	Second,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StageType {
	期中,
	期末,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
	pub start: String,
	pub end: String,
	pub semester: Option<SemesterType>,
	pub stage: Option<StageType>,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestContentType {
	原题,
	答案,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocContentType {
	思维导图,
	题库,
	答案,
	知识点,
	课件,
}
impl Course {
	pub fn new() -> Course {
		Course {
			r#type: None,
			name: String::new(),
		}
	}
}
impl Time {
	pub fn new() -> Time {
		Time {
			start: String::new(),
			end: String::new(),
			semester: None,
			stage: None,
		}
	}
}
