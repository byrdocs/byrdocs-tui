use std::error::Error;

use crate::definitions::DocType;

pub mod book;
pub mod doc;
pub mod test;

pub fn write<T>(file:T)->Result<String,Box<dyn Error>> where T:Into<DocType>{
	match file.into(){
		DocType::Book(book)=>book::write(book),
		DocType::Test(test)=>test::write(test),
		DocType::Doc(doc)=>doc::write(doc),
	}
}
