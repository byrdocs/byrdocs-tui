pub mod definition;
use std::io::{Read,Write};
pub use definition::*;
pub fn get(json_path:std::path::PathBuf)->Result<Metadata,serde_json::Error>{
	let mut file=std::fs::File::open(json_path).unwrap();
	let mut data=String::new();
	file.read_to_string(&mut data).unwrap();
	// file.read_to_string(&mut data).unwrap();
	let metadata:Metadata=serde_json::from_str(&data).expect("JSON was not well-formated");
	return Ok(metadata);
}
pub fn write(json_path:std::path::PathBuf,metadata:&Metadata)->Result<(),serde_json::Error>{
	let data = serde_json::to_string_pretty(metadata)?;
	let mut file = std::fs::OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(json_path).unwrap();
	file.write_all(data.as_bytes()).unwrap();
	return Ok(());
}
pub fn insert(json_path:std::path::PathBuf,piece:Piece)->Result<(),std::io::Error>{
	let mut metadata=get(json_path.clone()).unwrap();
	match piece {
		Piece::Books(piece)=>{
			metadata.books.push(piece);
			metadata.books.sort();
		}
		Piece::Tests(piece)=>{
			metadata.tests.push(piece);
			metadata.tests.sort();
		}
		Piece::Docs(piece)=>{
			metadata.docs.push(piece);
			metadata.docs.sort();
		}
	}
	write(json_path.clone(),&metadata).unwrap();
	Ok(())
}
pub fn query(json_path:std::path::PathBuf,category:Piece,data:String)->Result<(),std::io::Error>{
	let mut metadata=get(json_path.clone()).unwrap();
	match category{
		Piece::Books(_)=>{
			let vec:Vec<&Books>=metadata.books.iter().filter(|book|book.contains_data(data.clone())).collect();
			for piece in vec{
				println!("{piece}");
			}
		}
		Piece::Tests(_)=>{
			let vec:Vec<&Tests>=metadata.tests.iter().filter(|test|test.contains_data(data.clone())).collect();
			for piece in vec{
				println!("{piece}");
			}
		}
		Piece::Docs(_)=>{
			let vec:Vec<&Docs>=metadata.docs.iter().filter(|docs|docs.contains_data(data.clone())).collect();
			for piece in vec{
				println!("{piece}");
			}
		}
	}
	Ok(())
}
pub fn remove(json_path:std::path::PathBuf,md5_to_remove:String)->Result<(),std::io::Error>{
	let mut metadata=get(json_path.clone()).unwrap();
	metadata.books.retain(|book|book.md5!=md5_to_remove);
	metadata.tests.retain(|test|test.md5!=md5_to_remove);
	metadata.docs.retain(|doc|doc.md5!=md5_to_remove);
	write(json_path.clone(),&metadata).unwrap();
	Ok(())
}