#[derive(
	serde::Serialize,serde::Deserialize,
	PartialEq,Eq,PartialOrd,Ord,
	display_json::DisplayAsJson
)]
pub struct Books{
	pub md5:String,
	pub name:String,
	pub author:String,
	pub translator:String,
	pub edition:u16,
	pub publisher:String,
	pub isbn:String,
	pub format:String,
	pub size:u64,
}
impl Books{
	pub fn new(
		md5:String,
		name:String,
		author:String,
		translator:String,
		edition:u16,
		publisher:String,
		isbn:String,
		format:String,
		size:u64
	)->Self{
		return Books{
			md5:md5,
			name:name,
			author:author,
			translator:translator,
			edition:edition,
			publisher:publisher,
			isbn:isbn,
			format:format,
			size:size,
		};
	}
	pub fn empty()->Self{
		return Books{
			md5:String::new(),
			name:String::new(),
			author:String::new(),
			translator:String::new(),
			edition:0,
			publisher:String::new(),
			isbn:String::new(),
			format:String::new(),
			size:0,
		};
	}
	pub fn contains_data(&self,data:String,md5_only:bool)->bool{
		if md5_only{
			return self.md5.contains(&data);
		}
		else{
			return self.md5.contains(&data)
				||self.name.contains(&data)
				||self.author.contains(&data)
				||self.translator.contains(&data)
				||self.edition.to_string().contains(&data)
				||self.publisher.contains(&data)
				||self.isbn.contains(&data)
				||self.format.contains(&data);
		}
	}
}
#[derive(
	serde::Serialize,serde::Deserialize,
	PartialEq,Eq,PartialOrd,Ord,
	display_json::DisplayAsJson
)]
pub struct Tests{
	pub md5:String,
	pub field:String,
	pub school:String,
	pub term:String,
	pub class:String,
	pub stage:String,
	pub kind:String,
	pub format:String,
	pub size:u64,
}
impl Tests{
	pub fn new(
		md5:String,
		field:String,
		school:String,
		term:String,
		class:String,
		stage:String,
		kind:String,
		format:String,
		size:u64
	)->Self{
		return Tests{
			md5:md5,
			field:field,
			school:school,
			term:term,
			class:class,
			stage:stage,
			kind:kind,
			format:format,
			size:size,
		}
	}
	pub fn empty()->Self{
		return Tests{
			md5:String::new(),
			field:String::new(),
			school:String::new(),
			term:String::new(),
			class:String::new(),
			stage:String::new(),
			kind:String::new(),
			format:String::new(),
			size:0,
		}
	}
	pub fn contains_data(&self,data:String,md5_only:bool)->bool{
		if md5_only{
			return self.md5.contains(&data);
		}
		else{
			return self.md5.contains(&data)
				||self.field.contains(&data)
				||self.school.contains(&data)
				||self.term.contains(&data)
				||self.class.contains(&data)
				||self.stage.contains(&data)
				||self.kind.contains(&data)
				||self.format.contains(&data);
		}
	}
}
#[derive(
	serde::Serialize,serde::Deserialize,
	PartialEq,Eq,PartialOrd,Ord,
	display_json::DisplayAsJson
)]
pub struct Docs{
	pub md5:String,
	pub field:String,
	pub class:String,
	pub name:String,
	pub kind:String,
	pub format:String,
	pub size:u64,
}
impl Docs{
	pub fn new(
		md5:String,
		field:String,
		class:String,
		name:String,
		kind:String,
		format:String,
		size:u64
	)->Self{
		return Docs{
			md5:md5,
			field:field,
			class:class,
			name:name,
			kind:kind,
			format:format,
			size:size,
		}
	}
	pub fn empty()->Self{
		return Docs{
			md5:String::new(),
			field:String::new(),
			class:String::new(),
			name:String::new(),
			kind:String::new(),
			format:String::new(),
			size:0,
		}
	}
	pub fn contains_data(&self,data:String,md5_only:bool)->bool{
		if md5_only{
			return self.md5.contains(&data);
		}
		else{
			return self.md5.contains(&data)
				||self.field.contains(&data)
				||self.class.contains(&data)
				||self.name.contains(&data)
				||self.kind.contains(&data)
				||self.format.contains(&data);
		}
	}
}
pub enum Piece{
	Books(Books),
	Tests(Tests),
	Docs(Docs),
}
impl From<Books> for Piece{
	fn from(book:Books)->Self{
		Piece::Books(book)
	}
}
impl From<Tests> for Piece{
	fn from(test:Tests)->Self{
		Piece::Tests(test)
	}
}
impl From<Docs> for Piece{
	fn from(doc:Docs)->Self{
		Piece::Docs(doc)
	}
}
pub enum Category{
	Books,
	Tests,
	Docs,
}
#[derive(serde::Serialize,serde::Deserialize)]
pub struct Metadata{
	pub books:Vec<Books>,
	pub tests:Vec<Tests>,
	pub docs:Vec<Docs>,
}
