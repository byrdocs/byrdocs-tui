pub mod book;
pub mod doc;
pub mod pretype;
pub mod test;

pub enum DocType {
	Book(book::Book),
	Test(test::Test),
	Doc(doc::Doc),
}
