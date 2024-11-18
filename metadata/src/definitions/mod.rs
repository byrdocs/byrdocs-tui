pub mod book;
pub mod doc;
pub mod pretype;
pub mod test;

pub enum DocType {
	Book(book::Book),
	Test(test::Test),
	Doc(doc::Doc),
}

impl From<book::Book> for DocType {
	fn from(book: book::Book) -> DocType {
		Self::Book(book)
	}
}
impl From<test::Test> for DocType {
	fn from(test: test::Test) -> DocType {
		Self::Test(test)
	}
}
impl From<doc::Doc> for DocType {
	fn from(doc: doc::Doc) -> DocType {
		Self::Doc(doc)
	}
}
impl From<DocType> for Option<book::Book> {
	fn from(document: DocType) -> Option<book::Book> {
		match document {
			| DocType::Book(book) => Some(book),
			| _ => None,
		}
	}
}
impl From<DocType> for Option<test::Test> {
	fn from(document: DocType) -> Option<test::Test> {
		match document {
			| DocType::Test(test) => Some(test),
			| _ => None,
		}
	}
}
impl From<DocType> for Option<doc::Doc> {
	fn from(document: DocType) -> Option<doc::Doc> {
		match document {
			| DocType::Doc(doc) => Some(doc),
			| _ => None,
		}
	}
}
